use super::TokenValidity;
use crate::auth::{AuthError, SessionReference};
use crate::organization::OrganizationRole;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Clone, Serialize, Deserialize)]
pub struct UserAccessToken {
    pub user_id: u64,
    pub session_id: u64,
    pub organization_id: u64,
    pub organization_role: OrganizationRole,
    pub validity: TokenValidity,
}

impl UserAccessToken {
    pub fn new(
        user_id: u64,
        session_id: u64,
        organization_id: u64,
        organization_role: OrganizationRole,
        lifetime: Duration,
    ) -> Result<Self, AuthError> {
        let token = Self {
            user_id,
            session_id,
            organization_id,
            organization_role,
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
            || self.organization_id == 0
            || self.organization_id > i64::MAX as u64
        {
            return Err(AuthError::InvalidToken);
        }
        self.validity.validate(now)?;
        Ok(())
    }

    pub fn session(&self) -> SessionReference {
        SessionReference {
            id: self.session_id,
            user_id: self.user_id,
            organization_id: self.organization_id,
        }
    }
}
