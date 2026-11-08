use super::TokenValidity;
use crate::auth::{AuthError, Session};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Clone, Serialize, Deserialize)]
pub struct UserAccessToken {
    #[serde(rename = "s")]
    pub session: Session,
    /// Required effective authority; refresh credentials never contain this field.
    #[serde(rename = "p")]
    pub policy_snapshot: crate::authorization::PolicySnapshot,
    #[serde(rename = "v")]
    pub validity: TokenValidity,
}

impl UserAccessToken {
    pub fn new(
        session: Session,
        policy_snapshot: crate::authorization::PolicySnapshot,
        lifetime: Duration,
    ) -> Result<Self, AuthError> {
        Ok(Self {
            session,
            policy_snapshot,
            validity: TokenValidity::new(lifetime)?,
        })
    }

    pub fn validate(&self, now: u64) -> Result<(), AuthError> {
        self.validity.validate(now)
    }
}
