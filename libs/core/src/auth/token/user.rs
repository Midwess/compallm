use super::TokenValidity;
use crate::auth::{AuthError, Session};
use crate::organization::Role;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Clone, Serialize, Deserialize)]
pub struct UserAccessToken {
    pub session: Session,
    pub organization_role: String,
    /// Required effective authority; refresh credentials never contain this field.
    pub policy_snapshot: crate::authorization::PolicySnapshot,
    pub validity: TokenValidity,
}

impl UserAccessToken {
    pub fn new(
        session: Session,
        organization_role: impl Into<String>,
        policy_snapshot: crate::authorization::PolicySnapshot,
        lifetime: Duration,
    ) -> Result<Self, AuthError> {
        Ok(Self {
            session,
            organization_role: organization_role.into(),
            policy_snapshot,
            validity: TokenValidity::new(lifetime)?,
        })
    }

    pub fn validate(&self, now: u64) -> Result<(), AuthError> {
        if !matches!(
            self.organization_role.as_str(),
            Role::OWNER | Role::ADMIN | Role::MEMBER | Role::CUSTOM
        ) {
            return Err(AuthError::InvalidToken);
        }
        self.validity.validate(now)
    }
}
