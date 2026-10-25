use super::AccessToken;
use crate::auth::AuthError;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Clone, Serialize, Deserialize)]
pub struct UserAccessToken {
    pub user_id: u64,
    pub session_id: u64,
    pub issued_at: u64,
    pub expires_at: u64,
}

impl UserAccessToken {
    pub fn new(
        user_id: u64,
        session_id: u64,
        now: u64,
        lifetime: Duration,
    ) -> Result<Self, AuthError> {
        let token = Self {
            user_id,
            session_id,
            issued_at: now,
            expires_at: AccessToken::expiration(now, lifetime)?,
        };
        token.validate(now)?;
        Ok(token)
    }

    pub fn validate(&self, now: u64) -> Result<(), AuthError> {
        AccessToken::validate_ids(self.user_id, self.session_id)?;
        AccessToken::validate_times(self.issued_at, self.expires_at, now)
    }
}

impl TryFrom<AccessToken> for UserAccessToken {
    type Error = AuthError;
    fn try_from(token: AccessToken) -> Result<Self, Self::Error> {
        match token {
            AccessToken::User(token) => Ok(token),
            _ => Err(AuthError::WrongTokenVariant),
        }
    }
}
