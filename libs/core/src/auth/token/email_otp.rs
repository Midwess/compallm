use super::AccessToken;
use crate::auth::AuthError;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use subtle::ConstantTimeEq;

/// Never return these plaintext claims to a browser or log them.
#[derive(Clone, Serialize, Deserialize)]
pub struct EmailOtpAccessToken {
    pub email: String,
    pub code: String,
    pub issued_at: u64,
    pub expires_at: u64,
}

impl EmailOtpAccessToken {
    pub const TTL_SECONDS: u64 = 10 * 60;

    pub fn new(email: String, now: u64) -> Result<Self, AuthError> {
        if !email_address::EmailAddress::is_valid(&email) {
            return Err(AuthError::InvalidEmail);
        }
        Ok(Self {
            email,
            code: format!("{:06}", rand::rng().random_range(0..1_000_000)),
            issued_at: now,
            expires_at: AccessToken::expiration(now, Duration::from_secs(Self::TTL_SECONDS))?,
        })
    }

    pub fn validate(&self, now: u64) -> Result<(), AuthError> {
        AccessToken::validate_times(self.issued_at, self.expires_at, now)?;
        if self.expires_at - self.issued_at != Self::TTL_SECONDS
            || self.code.len() != 6
            || !self.code.bytes().all(|b| b.is_ascii_digit())
            || !email_address::EmailAddress::is_valid(&self.email)
        {
            return Err(AuthError::InvalidToken);
        }
        Ok(())
    }

    pub fn verify_code(&self, code: &str, now: u64) -> Result<(), AuthError> {
        self.validate(now)?;
        if code.len() != 6 || !bool::from(self.code.as_bytes().ct_eq(code.as_bytes())) {
            return Err(AuthError::IncorrectCode);
        }
        Ok(())
    }
}
