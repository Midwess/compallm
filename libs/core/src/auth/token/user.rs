use super::TokenValidity;
use crate::auth::{AuthError, Session};
use crate::organization::OrganizationRole;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Clone, Serialize, Deserialize)]
pub struct UserAccessToken {
    pub session: Session,
    pub organization_role: OrganizationRole,
    pub validity: TokenValidity,
}

impl UserAccessToken {
    pub fn new(
        session: Session,
        organization_role: OrganizationRole,
        lifetime: Duration,
    ) -> Result<Self, AuthError> {
        Ok(Self {
            session,
            organization_role,
            validity: TokenValidity::new(lifetime)?,
        })
    }

    pub fn validate(&self, now: u64) -> Result<(), AuthError> {
        self.validity.validate(now)
    }
}
