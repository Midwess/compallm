use std::time::Duration;

use super::{
    AuthError,
    token::{AccessToken, EmailOtpAccessToken, EncryptedAccessToken, RefreshToken, UserTokens},
};
use crate::entity::user::{User, UserStatus};
use crate::persistent::{DbContext, PersistenceError, user::UserRepository};
use crate::transaction;
use compallm_utils::email::verify_email_domain;
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
}

impl AuthService {
    pub fn new(
        db: DbContext,
        users: Box<dyn UserRepository>,
        jwt_secret: String,
        allowed_email_domain: String,
        lifetimes: TokenLifetimes,
    ) -> Result<Self, AuthError> {
        if lifetimes.user.as_secs() == 0 || lifetimes.refresh.as_secs() == 0 {
            return Err(AuthError::InvalidConfiguration);
        }
        EncryptedAccessToken::new(&jwt_secret)?;
        verify_email_domain(
            &format!("user@{}", allowed_email_domain.trim()),
            &allowed_email_domain,
        )?;
        Ok(Self {
            db,
            users,
            jwt_secret,
            allowed_email_domain,
            lifetimes,
        })
    }

    pub async fn signin(&self, email_token: &str, code: &str) -> Result<UserTokens, AuthError> {
        let claims =
            EmailOtpAccessToken::try_from(AccessToken::decode(email_token, &self.jwt_secret)?)?;
        verify_email_domain(&claims.email, &self.allowed_email_domain)?;
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
            let tokens = UserTokens::new(
                user.id,
                &self.jwt_secret,
                self.lifetimes.user,
                self.lifetimes.refresh,
            )?;
            self.users.record_signin(user.id, unix_timestamp()).await?;
            Ok(tokens)
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
        verify_email_domain(&claims.email, &self.allowed_email_domain)?;
        claims.verify_code(code, unix_timestamp())?;
        let display_name = display_name.trim();
        if display_name.is_empty() {
            return Err(AuthError::InvalidDisplayName);
        }
        transaction!(self.db, {
            if self.users.find_by_email(&claims.email).await?.is_some() {
                return Err(PersistenceError::EmailAlreadyExists.into());
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
            let tokens = UserTokens::new(
                user.id,
                &self.jwt_secret,
                self.lifetimes.user,
                self.lifetimes.refresh,
            )?;
            self.users.record_signin(user.id, unix_timestamp()).await?;
            Ok(tokens)
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
