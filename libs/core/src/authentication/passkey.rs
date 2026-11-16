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
    user_id: u64,
    session: Option<Session>,
    name: Option<String>,
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
        Ok(Self {
            repository,
            auth,
            webauthn,
            secret,
        })
    }
    fn engine(&self) -> Result<&Webauthn, AuthError> {
        self.webauthn.as_ref().ok_or(AuthError::PasskeyUnavailable)
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
    async fn actor(&self, actor: &UserAccessToken, recent: bool) -> Result<(), AuthError> {
        if !self.auth.validate_access(actor).await? {
            return Err(AuthError::InvalidSession);
        }
        self.active_user(actor.session.user_id).await?;
        self.auth
            .organizations
            .find_membership(actor.session.organization_id, actor.session.user_id)
            .await?
            .ok_or(AuthError::OrganizationAccessDenied)?;
        if recent
            && !self
                .repository
                .recently_verified(
                    &actor.session,
                    unix_timestamp().saturating_sub(CEREMONY_SECONDS),
                )
                .await?
        {
            return Err(AuthError::ReauthenticationRequired);
        }
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
                    && binding.user_id == actor.session.user_id => {}
            (None, None) => {}
            _ => return Err(AuthError::InvalidSession),
        }
        Ok((ceremony, binding))
    }
    #[db_context::root]
    pub async fn methods(&self, email: &str) -> Result<bool, AuthError> {
        let email = compallm_utils::email::normalize_email(email);
        compallm_utils::email::validate_email(&email).map_err(|_| AuthError::InvalidEmail)?;
        let user = self
            .auth
            .users
            .find_by_email(&email)
            .await?
            .ok_or(AuthError::UserNotFound)?;
        self.active_user(user.id).await?;
        Ok(self.webauthn.is_some() && !self.repository.list(user.id).await?.is_empty())
    }
    #[db_context::root]
    pub async fn registration_start(
        &self,
        actor: &UserAccessToken,
        name: &str,
    ) -> Result<CeremonyStart, AuthError> {
        self.actor(actor, true).await?;
        let name = name.trim();
        if name.is_empty() || name.chars().count() > 64 {
            return Err(AuthError::InvalidPasskey);
        }
        let user = self.active_user(actor.session.user_id).await?;
        crate::transaction!(db, {
            let handle = self.repository.lock_user(user.id).await?;
            let credentials = self.repository.list(user.id).await?;
            if credentials.len() >= MAX_PASSKEYS {
                return Err(AuthError::PasskeyLimit);
            }
            let (options, state) = self
                .engine()?
                .start_passkey_registration(
                    handle,
                    &user.email,
                    &user.display_name,
                    Some(
                        credentials
                            .iter()
                            .map(|p| p.credential.cred_id().clone())
                            .collect(),
                    ),
                )
                .map_err(|_| AuthError::InvalidPasskey)?;
            self.store(
                "registration",
                Binding {
                    user_id: user.id,
                    session: Some(actor.session.clone()),
                    name: Some(name.into()),
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
        token: &str,
        response: RegisterPublicKeyCredential,
    ) -> Result<PasskeyRecord, AuthError> {
        self.actor(actor, true).await?;
        let id = hero::distributed_id::try_gen_id().await?;
        crate::transaction!(db, {
            self.repository.lock_user(actor.session.user_id).await?;
            self.actor(actor, true).await?;
            let (ceremony, binding) = self.load(token, "registration", Some(actor)).await?;
            let state: PasskeyRegistration = serde_json::from_value(binding.verification)?;
            let credential = self
                .engine()?
                .finish_passkey_registration(&response, &state)
                .map_err(|_| AuthError::InvalidPasskey)?;
            if self.repository.list(binding.user_id).await?.len() >= MAX_PASSKEYS {
                return Err(AuthError::PasskeyLimit);
            }
            let record = PasskeyRecord {
                id,
                name: binding.name.ok_or(AuthError::InvalidPasskey)?,
                registered_at: unix_timestamp(),
                last_used_at: None,
                credential,
            };
            self.repository
                .create(binding.user_id, record.clone())
                .await?;
            if !self.repository.delete_challenge(ceremony.id).await? {
                return Err(AuthError::InvalidToken);
            }
            Ok(record)
        })
    }
    #[db_context::root]
    pub async fn authentication_start(&self, email: &str) -> Result<CeremonyStart, AuthError> {
        let email = compallm_utils::email::normalize_email(email);
        compallm_utils::email::validate_email(&email).map_err(|_| AuthError::InvalidEmail)?;
        let user = self
            .auth
            .users
            .find_by_email(&email)
            .await?
            .ok_or(AuthError::UserNotFound)?;
        self.active_user(user.id).await?;
        self.start_authentication(user.id, None).await
    }
    #[db_context::root]
    pub async fn reauthentication_start(
        &self,
        actor: &UserAccessToken,
    ) -> Result<CeremonyStart, AuthError> {
        self.actor(actor, false).await?;
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
                user_id,
                session,
                name: None,
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
    ) -> Result<(u64, Uuid), AuthError> {
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
        self.repository.lock_user(preliminary.user_id).await?;
        let (ceremony, binding) = self.load(token, purpose, actor).await?;
        self.active_user(binding.user_id).await?;
        if let Some(actor) = actor {
            self.actor(actor, false).await?;
        }
        let state: PasskeyAuthentication = serde_json::from_value(binding.verification)?;
        let result = self
            .engine()?
            .finish_passkey_authentication(response, &state)
            .map_err(|_| AuthError::InvalidPasskey)?;
        let mut record = self
            .repository
            .list(binding.user_id)
            .await?
            .into_iter()
            .find(|r| r.credential.cred_id() == result.cred_id())
            .ok_or(AuthError::InvalidPasskey)?;
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
        Ok((binding.user_id, ceremony.id))
    }
    #[db_context::root]
    pub async fn authentication_finish(
        &self,
        token: &str,
        response: PublicKeyCredential,
    ) -> Result<UserTokens, AuthError> {
        self.auth.latest_dictionary().await?;
        crate::transaction!(db, {
            let (user_id, ceremony) = self.verify_assertion(token, &response, None).await?;
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
    ) -> Result<(), AuthError> {
        self.actor(actor, false).await?;
        crate::transaction!(db, {
            let (_, ceremony) = self.verify_assertion(token, &response, Some(actor)).await?;
            self.repository
                .record_method(&actor.session, "passkey", unix_timestamp())
                .await?;
            if !self.repository.delete_challenge(ceremony).await? {
                return Err(AuthError::InvalidToken);
            }
            Ok(())
        })
    }
    #[db_context::root]
    pub async fn remove(&self, actor: &UserAccessToken, id: u64) -> Result<u64, AuthError> {
        self.actor(actor, true).await?;
        crate::transaction!(db, {
            self.repository.lock_user(actor.session.user_id).await?;
            self.actor(actor, true).await?;
            if !self.repository.remove(actor.session.user_id, id).await? {
                return Err(AuthError::InvalidPasskey);
            }
            Ok(id)
        })
    }
    #[db_context::root]
    pub async fn email_request(&self, actor: &UserAccessToken) -> Result<CeremonyStart, AuthError> {
        if db.postgres().is_transaction() {
            return Err(AuthError::InvalidSession);
        }
        self.actor(actor, false).await?;
        let user = self.active_user(actor.session.user_id).await?;
        let claims = super::EmailOtpAccessToken::new(user.email.clone())?;
        let mut mac = Hmac::<Sha256>::new_from_slice(self.secret.as_bytes())
            .map_err(|_| AuthError::InvalidConfiguration)?;
        let id = Uuid::new_v4();
        mac.update(id.as_bytes());
        mac.update(claims.code.as_bytes());
        let hash = URL_SAFE_NO_PAD.encode(mac.finalize().into_bytes());
        let expires_at = unix_timestamp() + CEREMONY_SECONDS;
        self.auth
            .email
            .ok_or(crate::mail::EmailError::Unavailable)?
            .send(crate::mail::EmailRequest {
                to: user.email,
                subject: "Verify your Compallm security change".into(),
                text: crate::mail::EmailTemplate::OtpCode(crate::mail::OtpCode::new(
                    claims.code,
                    std::time::Duration::from_secs(CEREMONY_SECONDS),
                )),
            })
            .await?;
        let ticket = Ticket {
            id,
            purpose: "email_reauthentication".into(),
            expires_at,
        };
        let token = self.sign(&ticket)?;
        self.repository
            .insert_challenge(Ceremony {
                id,
                purpose: ticket.purpose,
                expires_at,
                state: serde_json::to_value(Binding {
                    user_id: user.id,
                    session: Some(actor.session.clone()),
                    name: None,
                    verification: serde_json::json!(hash),
                    attempts: 0,
                })?,
            })
            .await?;
        Ok(CeremonyStart {
            token,
            options: serde_json::json!({}),
            expires_at,
        })
    }
    #[db_context::root]
    pub async fn email_confirm(
        &self,
        actor: &UserAccessToken,
        token: &str,
        code: &str,
    ) -> Result<(), AuthError> {
        self.actor(actor, false).await?;
        let accepted = crate::transaction!(db, {
            self.repository.lock_user(actor.session.user_id).await?;
            self.actor(actor, false).await?;
            let (mut ceremony, mut binding) = self
                .load(token, "email_reauthentication", Some(actor))
                .await?;
            let signature = binding
                .verification
                .as_str()
                .ok_or(AuthError::InvalidToken)?;
            let mut mac = Hmac::<Sha256>::new_from_slice(self.secret.as_bytes())
                .map_err(|_| AuthError::InvalidConfiguration)?;
            mac.update(ceremony.id.as_bytes());
            mac.update(code.as_bytes());
            if mac
                .verify_slice(&URL_SAFE_NO_PAD.decode(signature)?)
                .is_err()
            {
                binding.attempts += 1;
                if binding.attempts >= 5 {
                    self.repository.delete_challenge(ceremony.id).await?;
                } else {
                    ceremony.state = serde_json::to_value(binding)?;
                    self.repository.update_challenge(ceremony).await?;
                }
                Ok::<bool, AuthError>(false)
            } else {
                self.repository
                    .record_method(&actor.session, "email_code", unix_timestamp())
                    .await?;
                self.repository.delete_challenge(ceremony.id).await?;
                Ok(true)
            }
        })?;
        if accepted {
            Ok(())
        } else {
            Err(AuthError::IncorrectCode)
        }
    }
    #[db_context::root]
    pub async fn cleanup(&self) -> Result<(), AuthError> {
        self.repository.cleanup(unix_timestamp()).await
    }
}
