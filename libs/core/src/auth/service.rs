use std::{sync::Arc, time::Duration};

use super::{
    AuthError,
    token::{
        AccessToken, EmailOtpAccessToken, EmailOtpChallenge, EncryptedAccessToken, RefreshToken,
        UserTokens,
    },
};
use crate::mail::{EmailError, EmailRequest, EmailService, EmailTemplate, OtpCode};
use crate::persistent::{
    DbContext,
    user::{UserRepository, UserRepositoryError},
};
use crate::transaction;
use crate::user::{User, UserStatus};
use compallm_utils::email::verify_email;
use compallm_utils::time::unix_timestamp;

/// Explicit policy supplied by application configuration.
pub struct TokenLifetimes {
    pub user: Duration,
    pub refresh: Duration,
}

pub struct AuthService {
    db: DbContext,
    users: Box<dyn UserRepository>,
    jwt_secret: String,
    allowed_email_domain: String,
    lifetimes: TokenLifetimes,
    email: Option<Arc<dyn EmailService>>,
}

impl AuthService {
    pub fn new(
        db: DbContext,
        users: Box<dyn UserRepository>,
        jwt_secret: String,
        allowed_email_domain: String,
        lifetimes: TokenLifetimes,
        email: Option<Arc<dyn EmailService>>,
    ) -> Result<Self, AuthError> {
        if lifetimes.user.as_secs() == 0 || lifetimes.refresh.as_secs() == 0 {
            return Err(AuthError::InvalidConfiguration);
        }
        EncryptedAccessToken::new(&jwt_secret)?;
        let allowed_email_domain = allowed_email_domain.trim().to_ascii_lowercase();
        verify_email(&format!("user@{allowed_email_domain}"))
            .map_err(|_| AuthError::InvalidConfiguration)?;
        Ok(Self {
            db,
            users,
            jwt_secret,
            allowed_email_domain,
            lifetimes,
            email,
        })
    }

    /// Issue an encrypted proof token only after requesting OTP delivery.
    pub async fn signup_email(&self, email: &str) -> Result<String, AuthError> {
        let challenge =
            EmailOtpChallenge::new(email, &self.allowed_email_domain, &self.jwt_secret)?;
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
        if claims.email.rsplit_once('@').map(|(_, domain)| domain)
            != Some(self.allowed_email_domain.as_str())
        {
            return Err(AuthError::InvalidEmail);
        }
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
            self.users.record_login(user.id).await?;
            UserTokens::new(
                user.id,
                &self.jwt_secret,
                self.lifetimes.user,
                self.lifetimes.refresh,
            )
        })
    }

    pub async fn signup(
        &self,
        email_token: &str,
        code: &str,
        display_name: &str,
    ) -> Result<UserTokens, AuthError> {
        let claims =
            EmailOtpAccessToken::try_from(AccessToken::decode(email_token, &self.jwt_secret)?)?;
        verify_email(&claims.email)?;
        if claims.email.rsplit_once('@').map(|(_, domain)| domain)
            != Some(self.allowed_email_domain.as_str())
        {
            return Err(AuthError::InvalidEmail);
        }
        claims.verify_code(code, unix_timestamp())?;
        let display_name = display_name.trim();
        if display_name.is_empty() {
            return Err(AuthError::InvalidDisplayName);
        }
        transaction!(self.db, {
            if self.users.find_by_email(&claims.email).await?.is_some() {
                return Err(UserRepositoryError::EmailAlreadyExists.into());
            }
            let user = self
                .users
                .create(User {
                    id: hero::distributed_id::try_gen_id().await?,
                    email: claims.email,
                    display_name: display_name.into(),
                    status: UserStatus::Active,
                })
                .await?;
            if user.status != UserStatus::Active {
                return Err(AuthError::UserDisabled);
            }
            UserTokens::new(
                user.id,
                &self.jwt_secret,
                self.lifetimes.user,
                self.lifetimes.refresh,
            )
        })
    }

    /// Renew both credentials with a full refresh lifetime starting now.
    /// Configure the refresh lifetime as 30 days for the sliding one-month policy.
    /// Previous-token invalidation still requires persistent token tracking.
    pub async fn refresh(&self, refresh_token: &str) -> Result<UserTokens, AuthError> {
        let claims = RefreshToken::try_from(AccessToken::decode(refresh_token, &self.jwt_secret)?)?;
        let user = self
            .db
            .scope(self.users.find_by_id(claims.user_id))
            .await?
            .ok_or(AuthError::UserNotFound)?;
        if user.status != UserStatus::Active {
            return Err(AuthError::UserDisabled);
        }
        UserTokens::new(
            user.id,
            &self.jwt_secret,
            self.lifetimes.user,
            self.lifetimes.refresh,
        )
    }
}
