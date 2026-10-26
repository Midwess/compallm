use super::TokenValidity;
use crate::auth::AuthError;
use crate::user::User;
use rand::Rng;
use serde::{Deserialize, Serialize};
use subtle::ConstantTimeEq;

/// Never return these plaintext claims to a browser or log them.
#[derive(Clone, Serialize, Deserialize)]
pub struct EmailOtpAccessToken {
    pub email: String,
    pub code: String,
    pub validity: TokenValidity,
}

impl EmailOtpAccessToken {
    pub const TTL_SECONDS: u64 = 10 * 60;

    pub fn new(email: String) -> Result<Self, AuthError> {
        User::validate_email(&email).map_err(|_| AuthError::InvalidEmail)?;
        Ok(Self {
            email,
            code: format!("{:06}", rand::rng().random_range(0..1_000_000)),
            validity: TokenValidity::new(std::time::Duration::from_secs(Self::TTL_SECONDS))?,
        })
    }

    pub fn validate(&self, now: u64) -> Result<(), AuthError> {
        self.validity.validate(now)?;
        if self.validity.expires_at - self.validity.issued_at != Self::TTL_SECONDS
            || self.code.len() != 6
            || !self.code.bytes().all(|b| b.is_ascii_digit())
            || User::validate_email(&self.email).is_err()
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
