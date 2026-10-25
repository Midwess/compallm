use crate::auth::AuthError;
use compallm_utils::time::unix_timestamp;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Issuance and expiration timestamps with their shared validation rules.
#[derive(Clone, Serialize, Deserialize)]
pub struct TokenValidity {
    pub issued_at: u64,
    pub expires_at: u64,
}

impl TokenValidity {
    pub fn new(lifetime: Duration) -> Result<Self, AuthError> {
        let issued_at = unix_timestamp();
        let expires_at = issued_at
            .checked_add(lifetime.as_secs())
            .filter(|expires_at| *expires_at > issued_at)
            .ok_or(AuthError::InvalidConfiguration)?;
        Ok(Self {
            issued_at,
            expires_at,
        })
    }

    pub fn validate(&self, now: u64) -> Result<(), AuthError> {
        if self.issued_at > now || self.expires_at <= self.issued_at {
            return Err(AuthError::InvalidToken);
        }
        if now >= self.expires_at {
            return Err(AuthError::ExpiredToken);
        }
        Ok(())
    }
}
