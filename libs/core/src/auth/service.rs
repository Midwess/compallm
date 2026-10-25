use std::time::Duration;

use super::{
    AuthError,
    token::{TokenCodec, UserTokens},
};
use crate::entity::user::{NewUser, UserStatus};
use crate::persistent::{DbContext, PersistenceError, user::UserRepository};
use compallm_utils::time::unix_timestamp;

/// Explicit policy supplied by application configuration.
pub struct TokenLifetimes {
    pub user: Duration,
    pub refresh: Duration,
}

pub struct AuthService<R: UserRepository> {
    db: DbContext,
    users: R,
    tokens: TokenCodec,
    lifetimes: TokenLifetimes,
}

impl<R: UserRepository> AuthService<R> {
    pub fn new(
        db: DbContext,
        users: R,
        tokens: TokenCodec,
        lifetimes: TokenLifetimes,
    ) -> Result<Self, AuthError> {
        if lifetimes.user.as_secs() == 0 || lifetimes.refresh.as_secs() == 0 {
            return Err(AuthError::InvalidConfiguration);
        }
        Ok(Self {
            db,
            users,
            tokens,
            lifetimes,
        })
    }

    pub async fn signin(&self, email_token: &str, code: &str) -> Result<UserTokens, AuthError> {
        let claims = self.tokens.verify_email_otp(email_token, code)?;
        self.db
            .transaction(async {
                let user = self
                    .users
                    .find_by_email(&claims.email)
                    .await?
                    .ok_or(AuthError::UserNotFound)?;
                if user.status != UserStatus::Active {
                    return Err(AuthError::UserDisabled);
                }
                let session_id = hero::distributed_id::try_gen_id().await?;
                let tokens = self.tokens.issue_user_tokens(
                    user.id,
                    session_id,
                    self.lifetimes.user,
                    self.lifetimes.refresh,
                )?;
                self.users.record_signin(user.id, unix_timestamp()).await?;
                Ok(tokens)
            })
            .await
    }

    pub async fn signup(
        &self,
        email_token: &str,
        code: &str,
        display_name: &str,
    ) -> Result<UserTokens, AuthError> {
        let claims = self.tokens.verify_email_otp(email_token, code)?;
        let display_name = display_name.trim();
        if display_name.is_empty() {
            return Err(AuthError::InvalidDisplayName);
        }
        self.db
            .transaction(async {
                if self.users.find_by_email(&claims.email).await?.is_some() {
                    return Err(PersistenceError::EmailAlreadyExists.into());
                }
                let user = self
                    .users
                    .create(NewUser {
                        id: hero::distributed_id::try_gen_id().await?,
                        email: claims.email,
                        display_name: display_name.into(),
                        verified_at: unix_timestamp(),
                    })
                    .await?;
                if user.status != UserStatus::Active {
                    return Err(AuthError::UserDisabled);
                }
                let tokens = self.tokens.issue_user_tokens(
                    user.id,
                    hero::distributed_id::try_gen_id().await?,
                    self.lifetimes.user,
                    self.lifetimes.refresh,
                )?;
                self.users.record_signin(user.id, unix_timestamp()).await?;
                Ok(tokens)
            })
            .await
    }

    /// Check the current user without extending the refresh credential's expiry.
    /// Persisted session revocation/refresh rotation are not implemented yet.
    pub async fn refresh(&self, refresh_token: &str) -> Result<UserTokens, AuthError> {
        let claims = self.tokens.parse_refresh(refresh_token)?;
        let user = self
            .db
            .scope(self.users.find_by_id(claims.user_id))
            .await?
            .ok_or(AuthError::UserNotFound)?;
        if user.status != UserStatus::Active {
            return Err(AuthError::UserDisabled);
        }
        Ok(UserTokens {
            access_token: self.tokens.issue_user_access_token(
                user.id,
                claims.session_id,
                self.lifetimes.user,
            )?,
            refresh_token: refresh_token.into(),
        })
    }
}
