use crate::repository::ListOptions;
use std::time::Duration;

use super::Session;
use super::{
    AuthError,
    token::{
        AccessToken, EmailOtpAccessToken, EncryptedAccessToken, IssuedEmailOtp, RefreshToken,
        UserAccessToken, UserTokens,
    },
};
use crate::authorization::PolicySnapshot;
use crate::compression::{DCompression, DictionaryType};
use crate::mail::{EmailError, EmailRequest, EmailService, EmailTemplate, OtpCode};
use crate::organization::{Organization, OrganizationMember};
use crate::persistent::dictionary::DictionaryRepository;
use crate::persistent::session::SessionRepository;
use crate::user::{RegistrationService, UserStatus};
use crate::{
    persistent::{DbContext, organization::OrganizationRepository},
    repository::user::{UserQuery, UserRepository},
};
use compallm_utils::time::unix_timestamp;

/// Explicit policy supplied by application configuration.
pub struct TokenLifetimes {
    pub user: Duration,
    pub refresh: Duration,
}

pub struct AuthService {
    users: &'static Box<dyn UserRepository>,
    organizations: &'static Box<dyn OrganizationRepository>,
    sessions: &'static Box<dyn SessionRepository>,
    dictionaries: &'static Box<dyn DictionaryRepository>,
    registration: &'static RegistrationService,
    jwt_secret: String,
    lifetimes: TokenLifetimes,
    email: Option<&'static Box<dyn EmailService>>,
}

impl AuthService {
    /// Return true for current access, false for stale access; reject missing/revoked/ahead state.
    pub async fn validate_access(&self, claims: &UserAccessToken) -> Result<bool, AuthError> {
        claims.validate(unix_timestamp())?;
        let version = self
            .sessions
            .authorization_version(&claims.session)
            .await?
            .ok_or(AuthError::InvalidSession)?;
        if claims.session.authorization_version > version {
            return Err(AuthError::InvalidSession);
        }
        Ok(claims.session.authorization_version == version)
    }

    /// Invalidate selected members' session snapshots in one organization.
    /// The caller determines affected users and owns the transaction containing
    /// authority changes. This increments versions without revoking or renewing.
    pub async fn invalidate_permissions(
        &self,
        organization_id: u64,
        user_ids: &[u64],
    ) -> Result<u64, AuthError> {
        Ok(self
            .sessions
            .invalidate_permissions(organization_id, user_ids)
            .await?)
    }

    /// Revoke only the persisted session identified by the verified access credential.
    pub async fn signout(&self, claims: &UserAccessToken) -> Result<(), AuthError> {
        claims.validate(unix_timestamp())?;
        let mut session = self
            .sessions
            .find(&claims.session)
            .await?
            .ok_or(AuthError::InvalidSession)?;
        if !session.has_same_identity(&claims.session) {
            return Err(AuthError::InvalidSession);
        }
        if session.revoked_at.is_some() {
            return Ok(());
        }
        let previous = session.clone();
        session.revoke();
        if !self.sessions.update(&session, &previous).await? {
            return Err(AuthError::InvalidSession);
        }
        Ok(())
    }

    pub fn new(
        users: &'static Box<dyn UserRepository>,
        organizations: &'static Box<dyn OrganizationRepository>,
        sessions: &'static Box<dyn SessionRepository>,
        dictionaries: &'static Box<dyn DictionaryRepository>,
        registration: &'static RegistrationService,
        jwt_secret: String,
        lifetimes: TokenLifetimes,
        email: Option<&'static Box<dyn EmailService>>,
    ) -> Result<Self, AuthError> {
        EncryptedAccessToken::new(&jwt_secret)?;
        Ok(Self {
            users,
            organizations,
            sessions,
            dictionaries,
            registration,
            jwt_secret,
            lifetimes,
            email,
        })
    }

    /// Reject existing accounts before requesting OTP delivery for signup.
    pub async fn signup_email(&self, email: &str) -> Result<IssuedEmailOtp, AuthError> {
        let email = compallm_utils::email::normalize_email(email);
        if self
            .users
            .one(
                UserQuery {
                    email: Some(email.clone()),
                    ..Default::default()
                },
                ListOptions::default(),
            )
            .await?
            .is_some()
        {
            return Err(crate::repository::user::UserRepositoryError::EmailAlreadyExists.into());
        }
        self.send_email_otp(&email).await
    }

    /// Require an existing active account before admitting signin email.
    pub async fn signin_email(&self, email: &str) -> Result<IssuedEmailOtp, AuthError> {
        let email = compallm_utils::email::normalize_email(email);
        compallm_utils::email::validate_email(&email).map_err(|_| AuthError::InvalidEmail)?;
        let user = self
            .users
            .one(
                UserQuery {
                    email: Some(email.clone()),
                    ..Default::default()
                },
                ListOptions::default(),
            )
            .await?
            .ok_or(AuthError::UserNotFound)?;
        if user.status != UserStatus::Active {
            return Err(AuthError::UserDisabled);
        }
        self.send_email_otp(&email).await
    }

    async fn send_email_otp(&self, email: &str) -> Result<IssuedEmailOtp, AuthError> {
        let claims = EmailOtpAccessToken::new(compallm_utils::email::normalize_email(email))?;
        let dictionary = self.latest_dictionary().await?;
        let encrypted = AccessToken::EmailOtp(claims.clone())
            .encrypt(&EncryptedAccessToken::new(&self.jwt_secret)?)
            .await?;
        let token = encrypted.compress(dictionary).await?.encode(
            &self.jwt_secret,
            Duration::from_secs(EmailOtpAccessToken::TTL_SECONDS),
        )?;
        let sender = self.email.as_ref().ok_or(EmailError::Unavailable)?;
        sender
            .send(EmailRequest {
                to: compallm_utils::email::normalize_email(email),
                subject: "Your Compallm verification code".into(),
                text: EmailTemplate::OtpCode(OtpCode::new(
                    claims.code,
                    Duration::from_secs(EmailOtpAccessToken::TTL_SECONDS),
                )),
            })
            .await?;
        Ok(IssuedEmailOtp {
            token,
            validity: claims.validity,
        })
    }

    pub async fn signin(
        &self,
        claims: &EmailOtpAccessToken,
        code: &str,
    ) -> Result<UserTokens, AuthError> {
        claims.verify_code(code, unix_timestamp())?;
        let user = self
            .users
            .one(
                UserQuery {
                    email: Some(claims.email.clone()),
                    ..Default::default()
                },
                ListOptions::default(),
            )
            .await?
            .ok_or(AuthError::UserNotFound)?;
        if user.status != UserStatus::Active {
            return Err(AuthError::UserDisabled);
        }
        let (membership, organization) = self
            .organizations
            .find_first_membership(user.id)
            .await?
            .ok_or(AuthError::OrganizationAccessDenied)?;
        let dictionary = self.latest_dictionary().await?;
        self.start_session(user.id, membership, organization, dictionary)
            .await
    }

    pub async fn signup(
        &self,
        claims: &EmailOtpAccessToken,
        code: &str,
    ) -> Result<UserTokens, AuthError> {
        claims.verify_code(code, unix_timestamp())?;
        let dictionary = self.latest_dictionary().await?;
        crate::transaction!(DbContext::current()?, {
            let user = self.registration.register(&claims.email).await?;
            if user.status != UserStatus::Active {
                return Err(AuthError::UserDisabled);
            }
            let (membership, organization) = self
                .organizations
                .find_first_membership(user.id)
                .await?
                .ok_or(AuthError::OrganizationAccessDenied)?;
            self.start_session(user.id, membership, organization, dictionary)
                .await
        })
    }

    /// Verify the envelope and inner claims, preserving the concrete token variant.
    pub async fn authenticate_token(&self, token: &str) -> Result<AccessToken, AuthError> {
        AccessToken::decode(token, &self.jwt_secret, self.dictionaries.as_ref()).await
    }

    /// Verify and decompress access claims; dictionary versions are cached after loading.
    pub async fn authenticate_access(&self, token: &str) -> Result<UserAccessToken, AuthError> {
        UserAccessToken::try_from(self.authenticate_token(token).await?)
    }

    /// Read-only renewal: validates CWT/session state and never consumes the credential.
    pub async fn refresh(&self, refresh_token: &str) -> Result<UserTokens, AuthError> {
        let claims = RefreshToken::try_from(self.authenticate_token(refresh_token).await?)?;
        let (session, membership) = self
            .sessions
            .find_policy_membership(&claims.session)
            .await?
            .ok_or(AuthError::InvalidSession)?;
        session.validate_credential(&claims)?;
        if claims.session.authorization_version > session.authorization_version
            || membership.user_id != session.user_id
            || membership.organization_id != session.organization_id
        {
            return Err(AuthError::InvalidSession);
        }
        let snapshot = PolicySnapshot::new(&membership);
        let renewed = session.refresh(&claims, self.lifetimes.refresh)?;
        let dictionary = self.latest_dictionary().await?;
        self.issue_tokens(&renewed, snapshot, dictionary).await
    }

    pub async fn switch_organization(
        &self,
        refresh_token: &str,
        organization_id: u64,
    ) -> Result<UserTokens, AuthError> {
        let claims = RefreshToken::try_from(self.authenticate_token(refresh_token).await?)?;
        let mut session = self
            .sessions
            .find(&claims.session)
            .await?
            .ok_or(AuthError::InvalidSession)?;

        session.validate_credential(&claims)?;
        let user = self
            .users
            .one(
                UserQuery {
                    id: Some(
                        i64::try_from(claims.session.user_id)
                            .map_err(crate::repository::user::UserRepositoryError::database)?,
                    ),
                    ..Default::default()
                },
                ListOptions::default(),
            )
            .await?
            .ok_or(AuthError::UserNotFound)?;
        if user.status != UserStatus::Active {
            return Err(AuthError::UserDisabled);
        }

        let (membership, organization) = self
            .organizations
            .find_membership(organization_id, user.id)
            .await?
            .ok_or(AuthError::OrganizationAccessDenied)?;

        let previous = session.clone();
        session.switch_organization(&claims, organization_id, organization.name)?;

        let dictionary = self.latest_dictionary().await?;
        let refresh = RefreshToken::from_session(&session, self.lifetimes.refresh)?;
        let tokens = self
            .issue_tokens(&refresh, PolicySnapshot::new(&membership), dictionary)
            .await?;

        if !self.sessions.update(&session, &previous).await? {
            return Err(AuthError::InvalidSession);
        }

        Ok(tokens)
    }

    async fn start_session(
        &self,
        user_id: u64,
        membership: OrganizationMember,
        organization: Organization,
        dictionary: &DCompression,
    ) -> Result<UserTokens, AuthError> {
        let session = Session::new(user_id, membership.organization_id, organization.name).await?;
        let refresh = RefreshToken::from_session(&session, self.lifetimes.refresh)?;
        let tokens = self
            .issue_tokens(&refresh, PolicySnapshot::new(&membership), dictionary)
            .await?;
        self.sessions.create(session).await?;
        Ok(tokens)
    }

    async fn latest_dictionary(&self) -> Result<&DCompression, AuthError> {
        self.dictionaries
            .latest(DictionaryType::AccessToken)
            .await
            .map_err(|_| AuthError::DictionaryUnavailable)?
            .ok_or(AuthError::DictionaryUnavailable)
    }

    async fn issue_tokens(
        &self,
        refresh: &RefreshToken,
        snapshot: PolicySnapshot,
        dictionary: &DCompression,
    ) -> Result<UserTokens, AuthError> {
        refresh.validate(unix_timestamp())?;
        let user = UserAccessToken::new(refresh.session.clone(), snapshot, self.lifetimes.user)?;
        let access_validity = user.validity.clone();
        let refresh_lifetime =
            Duration::from_secs(refresh.validity.expires_at.saturating_sub(unix_timestamp()));
        Ok(UserTokens {
            access_validity,
            refresh_validity: refresh.validity.clone(),
            access_token: AccessToken::User(user)
                .compress(dictionary)
                .await?
                .encode(&self.jwt_secret, self.lifetimes.user)?,
            refresh_token: AccessToken::Refresh(refresh.clone())
                .compress(dictionary)
                .await?
                .encode(&self.jwt_secret, refresh_lifetime)?,
        })
    }
}
