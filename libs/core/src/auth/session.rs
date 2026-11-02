use super::{AuthError, RefreshToken};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// A persisted login. JWTs own expiry; ordinary renewal does not change this record.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Session {
    pub id: u64,
    pub user_id: u64,
    pub organization_id: u64,
    /// Display snapshot refreshed on token issuance; not part of session identity.
    pub organization_name: String,
    pub revoked_at: Option<u64>,
}

impl Session {
    pub fn new(
        user_id: u64,
        id: u64,
        organization_id: u64,
        organization_name: impl Into<String>,
    ) -> Self {
        Self {
            id,
            user_id,
            organization_id,
            organization_name: organization_name.into(),
            revoked_at: None,
        }
    }

    /// Match IDs only; organization names and revocation state are not identity.
    pub fn has_same_identity(&self, other: &Self) -> bool {
        self.id == other.id
            && self.user_id == other.user_id
            && self.organization_id == other.organization_id
    }

    pub fn is_active(&self, session: &Self) -> bool {
        self.has_same_identity(session) && self.revoked_at.is_none()
    }

    pub fn validate_credential(&self, presented: &RefreshToken) -> Result<(), AuthError> {
        presented.validate(compallm_utils::time::unix_timestamp())?;
        if !self.is_active(&presented.session) {
            return Err(AuthError::InvalidSession);
        }
        Ok(())
    }

    /// Renew JWT validity while preserving the database session.
    pub fn refresh(
        &self,
        presented: &RefreshToken,
        lifetime: Duration,
    ) -> Result<RefreshToken, AuthError> {
        self.validate_credential(presented)?;
        RefreshToken::from_session(self, lifetime)
    }

    pub fn switch_organization(
        &mut self,
        presented: &RefreshToken,
        organization_id: u64,
        organization_name: impl Into<String>,
    ) -> Result<(), AuthError> {
        self.validate_credential(presented)?;
        self.organization_id = organization_id;
        self.organization_name = organization_name.into();
        Ok(())
    }

    pub fn revoke(&mut self) {
        self.revoked_at
            .get_or_insert_with(compallm_utils::time::unix_timestamp);
    }
}
