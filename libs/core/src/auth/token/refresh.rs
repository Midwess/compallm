use super::TokenValidity;
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
    pub validity: TokenValidity,
}

impl RefreshToken {
    pub fn new(user_id: u64, session_id: u64, lifetime: Duration) -> Result<Self, AuthError> {
        let mut id = [0u8; 32];
        rand::rng().fill_bytes(&mut id);
        let token = Self {
            user_id,
            session_id,
            token_id: URL_SAFE_NO_PAD.encode(id),
            validity: TokenValidity::new(lifetime)?,
        };
        token.validate(token.validity.issued_at)?;
        Ok(token)
    }

    pub fn validate(&self, now: u64) -> Result<(), AuthError> {
        if self.user_id == 0
            || self.user_id > i64::MAX as u64
            || self.session_id == 0
            || self.session_id > i64::MAX as u64
        {
            return Err(AuthError::InvalidToken);
        }
        self.validity.validate(now)?;
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
