use std::time::Duration;

use super::{
    AuthError,
    token::{
        AccessToken, EmailOtpAccessToken, EmailOtpChallenge, EncryptedAccessToken, RefreshToken,
        UserTokens,
    },
};
use crate::mail::{EmailError, EmailRequest, EmailService, EmailTemplate, OtpCode};
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
            self.users.record_login(user.id).await?;
            UserTokens::new(
                user.id,
                membership.organization_id,
                membership.role,
                &self.jwt_secret,
                self.lifetimes.user,
                self.lifetimes.refresh,
            )
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
            UserTokens::new(
                user.id,
                membership.organization_id,
                membership.role,
                &self.jwt_secret,
                self.lifetimes.user,
                self.lifetimes.refresh,
            )
        })
    }

    /// Renew both credentials with a full refresh lifetime starting now.
    /// Configure the refresh lifetime as 30 days for the sliding one-month policy.
    /// Previous-token invalidation still requires persistent token tracking.
    pub async fn refresh(
        &self,
        refresh_token: &str,
        organization_id: Option<u64>,
    ) -> Result<UserTokens, AuthError> {
        let claims = RefreshToken::try_from(AccessToken::decode(refresh_token, &self.jwt_secret)?)?;
        if organization_id.is_some_and(|id| id == 0 || id > i64::MAX as u64) {
            return Err(AuthError::OrganizationAccessDenied);
        }
        self.db
            .scope(async {
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
                UserTokens::new(
                    user.id,
                    membership.organization_id,
                    membership.role,
                    &self.jwt_secret,
                    self.lifetimes.user,
                    self.lifetimes.refresh,
                )
            })
            .await
    }
}
