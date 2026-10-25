use super::AccessToken;
use crate::auth::AuthError;
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Clone, Serialize, Deserialize)]
pub struct RefreshToken {
    pub user_id: u64,
    pub session_id: u64,
    pub token_id: String,
    pub issued_at: u64,
    pub expires_at: u64,
}

impl RefreshToken {
    pub fn new(
        user_id: u64,
        session_id: u64,
        now: u64,
        lifetime: Duration,
    ) -> Result<Self, AuthError> {
        let mut id = [0u8; 32];
        rand::rng().fill_bytes(&mut id);
        let token = Self {
            user_id,
            session_id,
            token_id: URL_SAFE_NO_PAD.encode(id),
            issued_at: now,
            expires_at: AccessToken::expiration(now, lifetime)?,
        };
        token.validate(now)?;
        Ok(token)
    }

    /// The session workflow must additionally check revocation/consumption.
    pub fn validate(&self, now: u64) -> Result<(), AuthError> {
        AccessToken::validate_ids(self.user_id, self.session_id)?;
        AccessToken::validate_times(self.issued_at, self.expires_at, now)?;
        if URL_SAFE_NO_PAD
            .decode(&self.token_id)
            .map_err(|_| AuthError::InvalidToken)?
            .len()
            != 32
        {
            return Err(AuthError::InvalidToken);
        }
        Ok(())
    }
}

impl TryFrom<AccessToken> for RefreshToken {
    type Error = AuthError;
    fn try_from(token: AccessToken) -> Result<Self, Self::Error> {
        match token {
            AccessToken::Refresh(token) => Ok(token),
            _ => Err(AuthError::WrongTokenVariant),
        }
    }
}
