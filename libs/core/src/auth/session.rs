use super::{AuthError, RefreshToken};
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Identifies the session within its organization and owner.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SessionReference {
    pub id: u64,
    pub user_id: u64,
    pub organization_id: u64,
}

/// A persisted login. JWTs own expiry; ordinary renewal does not change this record.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Session {
    pub id: u64,
    pub user_id: u64,
    pub organization_id: u64,
    pub refresh_token_id: String,
    pub created_at: u64,
    pub revoked_at: Option<u64>,
}

impl Session {
    pub fn new(user_id: u64, id: u64, organization_id: u64) -> Self {
        Self {
            id,
            user_id,
            organization_id,
            refresh_token_id: Self::new_credential_id(),
            created_at: compallm_utils::time::unix_timestamp(),
            revoked_at: None,
        }
    }

    fn new_credential_id() -> String {
        let mut id = [0u8; 32];
        rand::rng().fill_bytes(&mut id);
        URL_SAFE_NO_PAD.encode(id)
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
        if !self.is_active(presented.session.reference())
            || self.refresh_token_id != presented.session.refresh_token_id
        {
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
        self.organization_id = organization_id;
        self.refresh_token_id = Self::new_credential_id();
        Ok(())
    }

    pub fn revoke(&mut self) {
        self.revoked_at
            .get_or_insert_with(compallm_utils::time::unix_timestamp);
    }
}
