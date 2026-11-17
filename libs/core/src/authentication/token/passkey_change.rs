use super::TokenValidity;
use crate::authentication::{AuthError, Session};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AuthenticationPurpose {
    #[default]
    Login,
    PasskeyChange,
}
impl AuthenticationPurpose {
    pub fn is_login(&self) -> bool {
        *self == Self::Login
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PasskeyChangeToken {
    #[serde(rename = "u")]
    pub user_id: u64,
    #[serde(rename = "s")]
    pub session_id: u64,
    #[serde(rename = "v")]
    pub validity: TokenValidity,
}
impl PasskeyChangeToken {
    pub const TTL_SECONDS: u64 = 180;
    pub fn new(session: &Session) -> Result<Self, AuthError> {
        Ok(Self {
            user_id: session.user_id,
            session_id: session.id,
            validity: TokenValidity::new(std::time::Duration::from_secs(Self::TTL_SECONDS))?,
        })
    }
    pub fn validate(&self, now: u64) -> Result<(), AuthError> {
        self.validity.validate(now)?;
        if self.user_id == 0
            || self.session_id == 0
            || self
                .validity
                .expires_at
                .saturating_sub(self.validity.issued_at)
                != Self::TTL_SECONDS
        {
            return Err(AuthError::InvalidToken);
        }
        Ok(())
    }
    pub fn authorize(&self, session: &Session) -> Result<(), AuthError> {
        self.validate(compallm_utils::time::unix_timestamp())?;
        if self.user_id != session.user_id || self.session_id != session.id {
            return Err(AuthError::InvalidSession);
        }
        Ok(())
    }
}
pub struct IssuedPasskeyChange {
    pub token: String,
    pub claims: PasskeyChangeToken,
}
