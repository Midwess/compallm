use super::{AuthError, AuthService, Session, UserAccessToken, UserTokens};
use crate::{
    persistent::{db_context, passkey::PasskeyRepository},
    user::UserStatus,
};
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use compallm_utils::time::unix_timestamp;
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use uuid::Uuid;
use webauthn_rs::{Webauthn, WebauthnBuilder, prelude::*};
use webauthn_rs_core::{
    WebauthnCore,
    proto::{
        AttestationConveyancePreference, AuthenticationState, RegistrationState,
        UserVerificationPolicy,
    },
};

pub const CEREMONY_SECONDS: u64 = 300;
pub const MAX_PASSKEYS: usize = 20;

#[derive(Clone)]
pub struct PasskeyRecord {
    pub id: u64,
    pub name: String,
    pub registered_at: u64,
    pub last_used_at: Option<u64>,
    pub credential: Passkey,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct Ceremony {
    pub id: Uuid,
    pub purpose: String,
    pub expires_at: u64,
    pub state: serde_json::Value,
}
#[derive(Serialize, Deserialize)]
struct Binding {
    user_id: Option<u64>,
    session: Option<Session>,
    verification: serde_json::Value,
    attempts: u8,
}
#[derive(Serialize, Deserialize)]
struct Ticket {
    id: Uuid,
    purpose: String,
    expires_at: u64,
}
pub struct CeremonyStart {
    pub token: String,
    pub options: serde_json::Value,
    pub expires_at: u64,
}
pub struct PasskeyService {
    repository: &'static dyn PasskeyRepository,
    auth: &'static AuthService,
    webauthn: Option<Webauthn>,
    discoverable: Option<WebauthnCore>,
    secret: String,
}
impl PasskeyService {
    pub fn new(
        repository: &'static dyn PasskeyRepository,
        auth: &'static AuthService,
        secret: String,
        rp_id: &str,
        origin: &str,
    ) -> Result<Self, AuthError> {
        if rp_id.is_empty() != origin.is_empty() || secret.len() < 32 {
            return Err(AuthError::InvalidConfiguration);
        }
        let webauthn = if rp_id.is_empty() {
            None
        } else {
            let origin = url::Url::parse(origin).map_err(|_| AuthError::InvalidConfiguration)?;
            Some(
                WebauthnBuilder::new(rp_id, &origin)
                    .map_err(|_| AuthError::InvalidConfiguration)?
                    .rp_name("Compallm")
                    .build()
                    .map_err(|_| AuthError::InvalidConfiguration)?,
            )
        };
        // The core adapter enables resident registration and modal discovery with the same validated RP policy.
        let discoverable = webauthn.as_ref().map(|engine| {
            WebauthnCore::new_unsafe_experts_only(
                "Compallm",
                rp_id,
                engine.get_allowed_origins().to_vec(),
                std::time::Duration::from_secs(CEREMONY_SECONDS),
                Some(false),
                Some(false),
            )
        });
        Ok(Self {
            discoverable,
            repository,
            auth,
            webauthn,
            secret,
        })
    }
    fn engine(&self) -> Result<&Webauthn, AuthError> {
        self.webauthn.as_ref().ok_or(AuthError::PasskeyUnavailable)
    }
    fn discovery_engine(&self) -> Result<&WebauthnCore, AuthError> {
        self.discoverable
            .as_ref()
            .ok_or(AuthError::PasskeyUnavailable)
    }
    fn sign(&self, ticket: &Ticket) -> Result<String, AuthError> {
        let payload = URL_SAFE_NO_PAD.encode(serde_json::to_vec(ticket)?);
        let mut mac = Hmac::<Sha256>::new_from_slice(self.secret.as_bytes())
            .map_err(|_| AuthError::InvalidConfiguration)?;
        mac.update(b"compallm:webauthn-ceremony:v1:");
        mac.update(payload.as_bytes());
        Ok(format!(
            "{payload}.{}",
            URL_SAFE_NO_PAD.encode(mac.finalize().into_bytes())
        ))
    }
    fn ticket(&self, token: &str, purpose: &str) -> Result<Ticket, AuthError> {
        if token.len() > 2048 {
            return Err(AuthError::InvalidToken);
        }
        let (payload, signature) = token.split_once('.').ok_or(AuthError::InvalidToken)?;
        let mut mac = Hmac::<Sha256>::new_from_slice(self.secret.as_bytes())
            .map_err(|_| AuthError::InvalidConfiguration)?;
        mac.update(b"compallm:webauthn-ceremony:v1:");
        mac.update(payload.as_bytes());
        mac.verify_slice(&URL_SAFE_NO_PAD.decode(signature)?)
            .map_err(|_| AuthError::InvalidToken)?;
        let ticket: Ticket = serde_json::from_slice(&URL_SAFE_NO_PAD.decode(payload)?)?;
        if ticket.purpose != purpose || ticket.expires_at <= unix_timestamp() {
            return Err(AuthError::InvalidToken);
        }
        Ok(ticket)
    }
    async fn active_user(&self, user_id: u64) -> Result<crate::user::User, AuthError> {
        let user = self
            .auth
            .users
            .find_by_id(user_id)
            .await?
            .ok_or(AuthError::UserNotFound)?;
        if user.status != UserStatus::Active {
            return Err(AuthError::UserDisabled);
        }
        Ok(user)
    }
    async fn actor(&self, actor: &UserAccessToken) -> Result<(), AuthError> {
        if !self.auth.validate_access(actor).await? {
            return Err(AuthError::InvalidSession);
        }
        self.active_user(actor.session.user_id).await?;
        self.auth
            .organizations
            .find_membership(actor.session.organization_id, actor.session.user_id)
            .await?
            .ok_or(AuthError::OrganizationAccessDenied)?;
        Ok(())
    }
    async fn store(
        &self,
        purpose: &str,
        binding: Binding,
        options: serde_json::Value,
    ) -> Result<CeremonyStart, AuthError> {
        let ticket = Ticket {
            id: Uuid::new_v4(),
            purpose: purpose.into(),
            expires_at: unix_timestamp() + CEREMONY_SECONDS,
        };
        let token = self.sign(&ticket)?;
        self.repository
            .insert_challenge(Ceremony {
                id: ticket.id,
                purpose: purpose.into(),
                expires_at: ticket.expires_at,
                state: serde_json::to_value(binding)?,
            })
            .await?;
        Ok(CeremonyStart {
            token,
            options,
            expires_at: ticket.expires_at,
        })
    }
    async fn load(
        &self,
        token: &str,
        purpose: &str,
        actor: Option<&UserAccessToken>,
    ) -> Result<(Ceremony, Binding), AuthError> {
        let ticket = self.ticket(token, purpose)?;
        let ceremony = self
            .repository
            .challenge(ticket.id, true)
            .await?
            .ok_or(AuthError::InvalidToken)?;
        if ceremony.purpose != purpose
            || ceremony.expires_at != ticket.expires_at
            || ceremony.expires_at <= unix_timestamp()
        {
            return Err(AuthError::InvalidToken);
        }
        let binding: Binding = serde_json::from_value(ceremony.state.clone())?;
        match (&binding.session, actor) {
            (Some(session), Some(actor))
                if session.has_same_identity(&actor.session)
                    && binding.user_id == Some(actor.session.user_id) => {}
            (None, None) => {}
            _ => return Err(AuthError::InvalidSession),
        }
        Ok((ceremony, binding))
    }
    fn require_change(&self, actor: &UserAccessToken, token: &str) -> Result<(), AuthError> {
        self.auth
            .verify_passkey_change(token, &actor.session)
            .map_err(|_| AuthError::ReauthenticationRequired)?;
        Ok(())
    }
    #[db_context::root]
    pub async fn registration_start(
        &self,
        actor: &UserAccessToken,
        grant: &str,
    ) -> Result<CeremonyStart, AuthError> {
        self.require_change(actor, grant)?;
        self.actor(actor).await?;
        let user = self.active_user(actor.session.user_id).await?;
        crate::transaction!(db, {
            let handle = self.repository.lock_user(user.id).await?;
            let credentials = self.repository.list(user.id).await?;
            if credentials.len() >= MAX_PASSKEYS {
                return Err(AuthError::PasskeyLimit);
            }
            let display = if user.display_name.is_empty() {
                &user.email
            } else {
                &user.display_name
            };
            let options = self
                .discovery_engine()?
                .new_challenge_register_builder(handle.as_bytes(), &user.email, display)
                .map_err(|_| AuthError::InvalidPasskey)?
                .attestation(AttestationConveyancePreference::None)
                .credential_algorithms(COSEAlgorithm::secure_algs())
                .require_resident_key(true)
                .user_verification_policy(UserVerificationPolicy::Required)
                .reject_synchronised_authenticators(false)
                .exclude_credentials(Some(
                    credentials
                        .iter()
                        .map(|p| p.credential.cred_id().clone())
                        .collect(),
                ))
                .extensions(Some(
                    webauthn_rs_core::proto::RequestRegistrationExtensions {
                        cred_props: Some(true),
                        uvm: Some(true),
                        cred_protect: Some(webauthn_rs_core::proto::CredProtect {
                            credential_protection_policy: webauthn_rs_core::proto::CredentialProtectionPolicy::UserVerificationRequired,
                            enforce_credential_protection_policy: Some(false),
                        }),
                        ..Default::default()
                    },
                ));
            let (options, state) = self
                .discovery_engine()?
                .generate_challenge_register(options)
                .map_err(|_| AuthError::InvalidPasskey)?;
            self.store(
                "registration",
                Binding {
                    user_id: Some(user.id),
                    session: Some(actor.session.clone()),
                    verification: serde_json::to_value(state)?,
                    attempts: 0,
                },
                serde_json::to_value(options)?,
            )
            .await
        })
    }
    #[db_context::root]
    pub async fn registration_finish(
        &self,
        actor: &UserAccessToken,
        grant: &str,
        token: &str,
        response: RegisterPublicKeyCredential,
        name: &str,
    ) -> Result<PasskeyRecord, AuthError> {
        self.require_change(actor, grant)?;
        let name = name.trim();
        if name.is_empty() || name.chars().count() > 64 {
            return Err(AuthError::InvalidPasskey);
        }
        self.actor(actor).await?;
        let id = hero::distributed_id::try_gen_id().await?;
        crate::transaction!(db, {
            self.repository.lock_user(actor.session.user_id).await?;
            self.require_change(actor, grant)?;
            self.actor(actor).await?;
            let (ceremony, binding) = self.load(token, "registration", Some(actor)).await?;
            let state: RegistrationState = serde_json::from_value(binding.verification)?;
            let credential = self
                .discovery_engine()?
                .register_credential(&response, &state, None)
                .map_err(|_| AuthError::InvalidPasskey)?;
            let user_id = binding.user_id.ok_or(AuthError::InvalidToken)?;
            if self.repository.list(user_id).await?.len() >= MAX_PASSKEYS {
                return Err(AuthError::PasskeyLimit);
            }
            let record = PasskeyRecord {
                id,
                name: name.into(),
                registered_at: unix_timestamp(),
                last_used_at: None,
                credential: credential.into(),
            };
            self.repository.create(user_id, record.clone()).await?;
            self.repository
                .record_method(&actor.session, "passkey", unix_timestamp(), Some(record.id))
                .await?;
            if !self.repository.delete_challenge(ceremony.id).await? {
                return Err(AuthError::InvalidToken);
            }
            Ok(record)
        })
    }
    #[db_context::root]
    pub async fn authentication_start(&self) -> Result<CeremonyStart, AuthError> {
        let builder = self
            .discovery_engine()?
            .new_challenge_authenticate_builder(Vec::new(), Some(UserVerificationPolicy::Required))
            .map_err(|_| AuthError::InvalidPasskey)?;
        let (options, state) = self
            .discovery_engine()?
            .generate_challenge_authenticate(builder)
            .map_err(|_| AuthError::InvalidPasskey)?;
        self.store(
            "authentication",
            Binding {
                user_id: None,
                session: None,
                verification: serde_json::to_value(state)?,
                attempts: 0,
            },
            serde_json::to_value(options)?,
        )
        .await
    }
    #[db_context::root]
    pub async fn reauthentication_start(
        &self,
        actor: &UserAccessToken,
    ) -> Result<CeremonyStart, AuthError> {
        self.actor(actor).await?;
        self.start_authentication(actor.session.user_id, Some(actor.session.clone()))
            .await
    }
    async fn start_authentication(
        &self,
        user_id: u64,
        session: Option<Session>,
    ) -> Result<CeremonyStart, AuthError> {
        let records = self.repository.list(user_id).await?;
        if records.is_empty() {
            return Err(AuthError::InvalidPasskey);
        }
        let creds: Vec<_> = records.into_iter().map(|r| r.credential).collect();
        let (options, state) = self
            .engine()?
            .start_passkey_authentication(&creds)
            .map_err(|_| AuthError::InvalidPasskey)?;
        let purpose = if session.is_some() {
            "reauthentication"
        } else {
            "authentication"
        };
        self.store(
            purpose,
            Binding {
                user_id: Some(user_id),
                session,
                verification: serde_json::to_value(state)?,
                attempts: 0,
            },
            serde_json::to_value(options)?,
        )
        .await
    }
    async fn verify_assertion(
        &self,
        token: &str,
        response: &PublicKeyCredential,
        actor: Option<&UserAccessToken>,
    ) -> Result<(u64, Uuid, u64), AuthError> {
        let purpose = if actor.is_some() {
            "reauthentication"
        } else {
            "authentication"
        };
        let ticket = self.ticket(token, purpose)?;
        let preliminary = self
            .repository
            .challenge(ticket.id, false)
            .await?
            .ok_or(AuthError::InvalidToken)?;
        let preliminary: Binding = serde_json::from_value(preliminary.state)?;
        let (user_id, selected) = if actor.is_none() {
            let handle = response
                .get_user_unique_id()
                .and_then(|bytes| Uuid::from_slice(bytes).ok())
                .ok_or(AuthError::InvalidPasskey)?;
            let (user_id, record) = self
                .repository
                .find_discoverable(handle, response.get_credential_id())
                .await?
                .ok_or(AuthError::InvalidPasskey)?;
            (user_id, Some((handle, record.id)))
        } else {
            (preliminary.user_id.ok_or(AuthError::InvalidToken)?, None)
        };
        self.repository.lock_user(user_id).await?;
        let (ceremony, binding) = self.load(token, purpose, actor).await?;
        self.active_user(user_id).await?;
        if let Some(actor) = actor {
            self.actor(actor).await?;
        }
        let records = self.repository.list(user_id).await?;
        let mut record = records
            .into_iter()
            .find(|r| r.credential.cred_id().as_ref() == response.get_credential_id())
            .ok_or(AuthError::InvalidPasskey)?;
        let result = if let Some((handle, id)) = selected {
            let reloaded = self
                .repository
                .find_discoverable(handle, response.get_credential_id())
                .await?
                .ok_or(AuthError::InvalidPasskey)?;
            if reloaded.0 != user_id || reloaded.1.id != id {
                return Err(AuthError::InvalidPasskey);
            }
            let mut state: AuthenticationState = serde_json::from_value(binding.verification)?;
            state.set_allowed_credentials(vec![record.credential.clone().into()]);
            self.discovery_engine()?
                .authenticate_credential(response, &state)
                .map_err(|_| AuthError::InvalidPasskey)?
        } else {
            let state: PasskeyAuthentication = serde_json::from_value(binding.verification)?;
            self.engine()?
                .finish_passkey_authentication(response, &state)
                .map_err(|_| AuthError::InvalidPasskey)?
        };
        let current: Credential = record.credential.clone().into();
        if (current.counter != 0 || result.counter() != 0) && result.counter() <= current.counter {
            return Err(AuthError::InvalidPasskey);
        }
        record
            .credential
            .update_credential(&result)
            .ok_or(AuthError::InvalidPasskey)?;
        record.last_used_at = Some(unix_timestamp());
        self.repository.update(&record).await?;
        Ok((user_id, ceremony.id, record.id))
    }
    #[db_context::root]
    pub async fn authentication_finish(
        &self,
        token: &str,
        response: PublicKeyCredential,
        metadata: super::SessionMetadata,
    ) -> Result<UserTokens, AuthError> {
        self.auth.latest_dictionary().await?;
        crate::transaction!(db, {
            let (user_id, ceremony, credential_id) =
                self.verify_assertion(token, &response, None).await?;
            let (membership, _) = self
                .auth
                .organizations
                .find_first_membership(user_id)
                .await?
                .ok_or(AuthError::OrganizationAccessDenied)?;
            let tokens = self
                .auth
                .start_session_with_method(
                    user_id,
                    membership,
                    self.auth.latest_dictionary().await?,
                    "passkey",
                    Some(credential_id),
                    metadata,
                )
                .await?;
            if !self.repository.delete_challenge(ceremony).await? {
                return Err(AuthError::InvalidToken);
            }
            Ok(tokens)
        })
    }
    #[db_context::root]
    pub async fn reauthentication_finish(
        &self,
        actor: &UserAccessToken,
        token: &str,
        response: PublicKeyCredential,
    ) -> Result<super::IssuedPasskeyChange, AuthError> {
        self.actor(actor).await?;
        crate::transaction!(db, {
            let (_, ceremony, credential_id) =
                self.verify_assertion(token, &response, Some(actor)).await?;
            self.repository
                .record_method(
                    &actor.session,
                    "passkey",
                    unix_timestamp(),
                    Some(credential_id),
                )
                .await?;
            if !self.repository.delete_challenge(ceremony).await? {
                return Err(AuthError::InvalidToken);
            }
            self.auth.issue_passkey_change(&actor.session)
        })
    }
    #[db_context::root]
    pub async fn rename(
        &self,
        actor: &UserAccessToken,
        grant: &str,
        id: u64,
        name: &str,
    ) -> Result<PasskeyRecord, AuthError> {
        self.require_change(actor, grant)?;
        self.actor(actor).await?;
        let name = name.trim();
        if name.is_empty() || name.chars().count() > 64 {
            return Err(AuthError::InvalidPasskey);
        }
        crate::transaction!(db, {
            self.repository.lock_user(actor.session.user_id).await?;
            self.require_change(actor, grant)?;
            self.actor(actor).await?;
            let mut record = self
                .repository
                .list(actor.session.user_id)
                .await?
                .into_iter()
                .find(|p| p.id == id)
                .ok_or(AuthError::InvalidPasskey)?;
            record.name = name.into();
            self.repository
                .rename(actor.session.user_id, id, name)
                .await?;
            Ok(record)
        })
    }
    #[db_context::root]
    pub async fn remove(
        &self,
        actor: &UserAccessToken,
        grant: &str,
        id: u64,
    ) -> Result<u64, AuthError> {
        self.require_change(actor, grant)?;
        self.actor(actor).await?;
        crate::transaction!(db, {
            self.repository.lock_user(actor.session.user_id).await?;
            self.require_change(actor, grant)?;
            self.actor(actor).await?;
            if !self.repository.remove(actor.session.user_id, id).await? {
                return Err(AuthError::InvalidPasskey);
            }
            Ok(id)
        })
    }
    #[db_context::root]
    pub async fn cleanup(&self) -> Result<(), AuthError> {
        self.repository.cleanup(unix_timestamp()).await
    }
}
