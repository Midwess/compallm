use super::{AuthError, RefreshToken};
use std::time::Duration;

/// Identifies the session within its organization and owner.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SessionReference {
    pub id: u64,
    pub user_id: u64,
    pub organization_id: u64,
}

/// A persisted login. JWTs own expiry; ordinary renewal does not change this record.
#[derive(Clone, Debug)]
pub struct Session {
    pub id: u64,
    pub user_id: u64,
    pub organization_id: u64,
    pub refresh_token_id: String,
    pub created_at: u64,
    pub revoked_at: Option<u64>,
}

impl Session {
    pub fn new(refresh: &RefreshToken) -> Self {
        Self {
            id: refresh.session_id,
            user_id: refresh.user_id,
            organization_id: refresh.organization_id,
            refresh_token_id: refresh.token_id.clone(),
            created_at: refresh.validity.issued_at,
            revoked_at: None,
        }
    }

    pub fn reference(&self) -> SessionReference {
        SessionReference {
            id: self.id,
            user_id: self.user_id,
            organization_id: self.organization_id,
        }
    }

    pub fn is_active(&self, reference: SessionReference) -> bool {
        self.reference() == reference && self.revoked_at.is_none()
    }

    pub fn validate_credential(&self, presented: &RefreshToken) -> Result<(), AuthError> {
        presented.validate(compallm_utils::time::unix_timestamp())?;
        if !self.is_active(presented.session()) || self.refresh_token_id != presented.token_id {
            return Err(AuthError::InvalidSession);
        }
        Ok(())
    }

    /// Renew JWT validity while preserving the database session and credential identifier.
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
    ) -> Result<(), AuthError> {
        self.validate_credential(presented)?;
        if organization_id == 0 || organization_id > i64::MAX as u64 {
            return Err(AuthError::OrganizationAccessDenied);
        }
        self.organization_id = organization_id;
        self.refresh_token_id = RefreshToken::new_credential_id();
        Ok(())
    }

    pub fn revoke(&mut self) {
        self.revoked_at
            .get_or_insert_with(compallm_utils::time::unix_timestamp);
    }
}
