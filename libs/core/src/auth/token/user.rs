use super::TokenValidity;
use crate::auth::AuthError;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Clone, Serialize, Deserialize)]
pub struct UserAccessToken {
    pub user_id: u64,
    pub validity: TokenValidity,
}

impl UserAccessToken {
    pub fn new(user_id: u64, lifetime: Duration) -> Result<Self, AuthError> {
        let token = Self {
            user_id,
            validity: TokenValidity::new(lifetime)?,
        };
        token.validate(token.validity.issued_at)?;
        Ok(token)
    }

    pub fn validate(&self, now: u64) -> Result<(), AuthError> {
        if self.user_id == 0 || self.user_id > i64::MAX as u64 {
            return Err(AuthError::InvalidToken);
        }
        self.validity.validate(now)?;
        Ok(())
    }
}
