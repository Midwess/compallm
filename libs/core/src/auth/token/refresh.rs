use super::TokenValidity;
use crate::auth::{AuthError, Session};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Clone, Serialize, Deserialize)]
pub struct RefreshToken {
    pub session: Session,
    pub validity: TokenValidity,
}

impl RefreshToken {
    pub fn validate(&self, now: u64) -> Result<(), AuthError> {
        self.validity.validate(now)
    }

    pub fn from_session(session: &Session, lifetime: Duration) -> Result<Self, AuthError> {
        if session.revoked_at.is_some() {
            return Err(AuthError::InvalidSession);
        }
        Ok(Self {
            session: session.clone(),
            validity: TokenValidity::new(lifetime)?,
        })
    }
}
