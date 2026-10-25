//! Hero-signed JWT payloads. Only email OTP claims need extra encryption.
//! These internal claim schemas are not HTTP response DTOs.

use super::AuthError;
use serde::{Deserialize, Serialize};
use std::time::Duration;

mod codec;
mod email_otp;
mod encrypted;
mod refresh;
mod user;

pub use codec::{EmailOtpChallenge, TokenCodec, UserTokens};
pub use email_otp::EmailOtpAccessToken;
pub use encrypted::EncryptedAccessToken;
pub use refresh::RefreshToken;
pub use user::UserAccessToken;

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccessToken {
    Encrypted(EncryptedToken),
    /// Internal payload: must be encrypted before signing a browser credential.
    EmailOtp(EmailOtpAccessToken),
    User(UserAccessToken),
    Refresh(RefreshToken),
}

/// Opaque ciphertext nested in the signed JWT. Deliberately not Debug.
#[derive(Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EncryptedToken(pub String);

impl AccessToken {
    /// Validate decrypted claims; nested encrypted wrappers are not accepted.
    pub(crate) fn validate_plain(&self, now: u64) -> Result<(), AuthError> {
        match self {
            Self::EmailOtp(token) => token.validate(now),
            Self::User(token) => token.validate(now),
            Self::Refresh(token) => token.validate(now),
            Self::Encrypted(_) => Err(AuthError::InvalidToken),
        }
    }

    pub(super) fn expiration(issued_at: u64, lifetime: Duration) -> Result<u64, AuthError> {
        if lifetime.as_secs() == 0 {
            return Err(AuthError::InvalidConfiguration);
        }
        issued_at
            .checked_add(lifetime.as_secs())
            .ok_or(AuthError::InvalidConfiguration)
    }

    pub(super) fn validate_times(
        issued_at: u64,
        expires_at: u64,
        now: u64,
    ) -> Result<(), AuthError> {
        if issued_at > now || expires_at <= issued_at {
            return Err(AuthError::InvalidToken);
        }
        if now >= expires_at {
            return Err(AuthError::ExpiredToken);
        }
        Ok(())
    }

    pub(super) fn validate_ids(user_id: u64, session_id: u64) -> Result<(), AuthError> {
        if user_id == 0
            || session_id == 0
            || user_id > i64::MAX as u64
            || session_id > i64::MAX as u64
        {
            return Err(AuthError::InvalidToken);
        }
        Ok(())
    }
}
