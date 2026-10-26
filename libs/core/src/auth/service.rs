use std::time::Duration;

use super::Session;
use super::{
    AuthError,
    token::{
        AccessToken, EmailOtpAccessToken, EmailOtpChallenge, EncryptedAccessToken, RefreshToken,
        UserTokens,
    },
};
use crate::mail::{EmailError, EmailRequest, EmailService, EmailTemplate, OtpCode};
use crate::organization::OrganizationMember;
use crate::persistent::session::SessionRepository;
use crate::persistent::{DbContext, organization::OrganizationRepository, user::UserRepository};
use crate::transaction;
use crate::user::{RegistrationService, UserStatus};
use compallm_utils::email::verify_email;
use compallm_utils::time::unix_timestamp;

/// Explicit policy supplied by application configuration.
pub struct TokenLifetimes {
    pub user: Duration,
    pub refresh: Duration,
}

pub struct AuthService {
    db: &'static DbContext,
    users: &'static Box<dyn UserRepository>,
    organizations: &'static Box<dyn OrganizationRepository>,
    sessions: &'static Box<dyn SessionRepository>,
    registration: &'static RegistrationService,
    jwt_secret: String,
    lifetimes: TokenLifetimes,
    email: Option<&'static Box<dyn EmailService>>,
}

impl AuthService {
    pub fn new(
        db: &'static DbContext,
        users: &'static Box<dyn UserRepository>,
        organizations: &'static Box<dyn OrganizationRepository>,
        sessions: &'static Box<dyn SessionRepository>,
        registration: &'static RegistrationService,
        jwt_secret: String,
        lifetimes: TokenLifetimes,
        email: Option<&'static Box<dyn EmailService>>,
    ) -> Result<Self, AuthError> {
        if lifetimes.user.as_secs() == 0 || lifetimes.refresh.as_secs() == 0 {
            return Err(AuthError::InvalidConfiguration);
        }
        EncryptedAccessToken::new(&jwt_secret)?;
        Ok(Self {
            db,
            users,
            organizations,
            sessions,
            registration,
            jwt_secret,
            lifetimes,
            email,
        })
    }

    /// Issue an encrypted proof token only after requesting OTP delivery.
    pub async fn signup_email(&self, email: &str) -> Result<String, AuthError> {
        let challenge = EmailOtpChallenge::new(email, &self.jwt_secret)?;
        let sender = self.email.as_ref().ok_or(EmailError::Unavailable)?;
        sender
            .send(EmailRequest {
                to: compallm_utils::email::normalize_email(email)?,
                subject: "Your Compallm signup code".into(),
                text: EmailTemplate::OtpCode(OtpCode::new(
                    challenge.code,
                    Duration::from_secs(EmailOtpAccessToken::TTL_SECONDS),
                )),
            })
            .await?;
        Ok(challenge.access_token)
    }

    pub async fn signin(&self, email_token: &str, code: &str) -> Result<UserTokens, AuthError> {
        let claims =
            EmailOtpAccessToken::try_from(AccessToken::decode(email_token, &self.jwt_secret)?)?;
        verify_email(&claims.email)?;
        claims.verify_code(code, unix_timestamp())?;
        transaction!(self.db, {
            let user = self
                .users
                .find_by_email(&claims.email)
                .await?
                .ok_or(AuthError::UserNotFound)?;
            if user.status != UserStatus::Active {
                return Err(AuthError::UserDisabled);
            }
            let membership = self
                .organizations
                .find_first_admin_membership(user.id)
                .await?
                .ok_or(AuthError::OrganizationAccessDenied)?;
            self.start_session(user.id, membership).await
        })
    }

    pub async fn signup(&self, email_token: &str, code: &str) -> Result<UserTokens, AuthError> {
        let claims =
            EmailOtpAccessToken::try_from(AccessToken::decode(email_token, &self.jwt_secret)?)?;
        verify_email(&claims.email)?;
        claims.verify_code(code, unix_timestamp())?;
        transaction!(self.db, {
            let user = self.registration.register(&claims.email).await?;
            if user.status != UserStatus::Active {
                return Err(AuthError::UserDisabled);
            }
            let membership = self
                .organizations
                .find_first_admin_membership(user.id)
                .await?
                .ok_or(AuthError::OrganizationAccessDenied)?;
            self.start_session(user.id, membership).await
        })
    }

    /// Renew both credentials with a full refresh lifetime starting now.
    /// Configure the refresh lifetime as 30 days for the sliding one-month policy.
    /// Old-credential reuse revokes the session; denial must not roll back revocation.
    pub async fn refresh(
        &self,
        refresh_token: &str,
        organization_id: Option<u64>,
    ) -> Result<UserTokens, AuthError> {
        let claims = RefreshToken::try_from(AccessToken::decode(refresh_token, &self.jwt_secret)?)?;
        if organization_id.is_some_and(|id| id == 0 || id > i64::MAX as u64) {
            return Err(AuthError::OrganizationAccessDenied);
        }
        if DbContext::current()?.postgres().is_transaction() {
            return Err(AuthError::RefreshTransactionAlreadyActive);
        }
        transaction!(self.db, {
            let mut session = self
                .sessions
                .find_by_id(claims.session_id, claims.user_id)
                .await?
                .ok_or(AuthError::InvalidSession)?;
            let now = unix_timestamp();
            if session.id != claims.session_id || !session.is_active(claims.user_id, now) {
                return Err(AuthError::InvalidSession);
            }
            if session.refresh_token_id != claims.token_id {
                self.sessions
                    .revoke(session.id, claims.user_id, now)
                    .await?;
                // Commit the revocation before returning the authentication error.
                return Ok::<_, AuthError>(Err(AuthError::RefreshTokenReused));
            }
            let user = self
                .users
                .find_by_id(claims.user_id)
                .await?
                .ok_or(AuthError::UserNotFound)?;
            if user.status != UserStatus::Active {
                return Err(AuthError::UserDisabled);
            }
            let membership = match organization_id {
                Some(id) => self.organizations.find_membership(id, user.id).await?,
                None => self.organizations.find_first_membership(user.id).await?,
            }
            .ok_or(AuthError::OrganizationAccessDenied)?;
            let refresh = RefreshToken::new(user.id, session.id, self.lifetimes.refresh)?;
            let tokens = UserTokens::new(
                &refresh,
                membership.organization_id,
                membership.role,
                &self.jwt_secret,
                self.lifetimes.user,
            )?;
            session.renew(&refresh);
            if !self.sessions.rotate(&session, &claims.token_id).await? {
                // Another refresh may have won after our read. A still-active
                // session with a different credential must follow the reuse policy.
                let now = unix_timestamp();
                if self
                    .sessions
                    .find_by_id(claims.session_id, claims.user_id)
                    .await?
                    .is_some_and(|current| {
                        current.is_active(claims.user_id, now)
                            && current.refresh_token_id != claims.token_id
                    })
                {
                    self.sessions
                        .revoke(claims.session_id, claims.user_id, now)
                        .await?;
                    return Ok(Err(AuthError::RefreshTokenReused));
                }
                return Err(AuthError::InvalidSession);
            }
            Ok(Ok(tokens))
        })?
    }

    async fn start_session(
        &self,
        user_id: u64,
        membership: OrganizationMember,
    ) -> Result<UserTokens, AuthError> {
        let id = hero::distributed_id::try_gen_id().await?;
        let refresh = RefreshToken::new(user_id, id, self.lifetimes.refresh)?;
        let tokens = UserTokens::new(
            &refresh,
            membership.organization_id,
            membership.role,
            &self.jwt_secret,
            self.lifetimes.user,
        )?;
        self.sessions.create(Session::new(&refresh)).await?;
        Ok(tokens)
    }
}
