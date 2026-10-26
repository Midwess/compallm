use super::{AuthError, RefreshToken};
use std::time::Duration;

/// Identifies the session within its organization and owner.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SessionReference {
    pub id: u64,
    pub user_id: u64,
    pub organization_id: u64,
}

/// One successful login. Refresh updates activity without creating another login.
#[derive(Clone, Debug)]
pub struct Session {
    pub id: u64,
    pub user_id: u64,
    pub organization_id: u64,
    pub refresh_token_id: String,
    pub created_at: u64,
    pub last_used_at: u64,
    pub expires_at: u64,
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
            last_used_at: refresh.validity.issued_at,
            expires_at: refresh.validity.expires_at,
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

    pub fn is_active(&self, reference: SessionReference, now: u64) -> bool {
        self.reference() == reference && self.revoked_at.is_none() && now < self.expires_at
    }

    pub fn refresh(
        &mut self,
        presented: &RefreshToken,
        organization_id: u64,
        lifetime: Duration,
    ) -> Result<(), AuthError> {
        let now = compallm_utils::time::unix_timestamp();
        presented.validate(now)?;
        if !self.is_active(presented.session(), now) {
            return Err(AuthError::InvalidSession);
        }
        if self.refresh_token_id != presented.token_id {
            self.revoked_at = Some(now);
            return Err(AuthError::RefreshTokenReused);
        }
        let refresh = RefreshToken::new(self.user_id, self.id, organization_id, lifetime)?;
        self.organization_id = organization_id;
        self.refresh_token_id = refresh.token_id;
        self.last_used_at = refresh.validity.issued_at;
        self.expires_at = refresh.validity.expires_at;
        Ok(())
    }
}
