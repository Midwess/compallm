use hero::distributed_id::{generator::DistributedIdError, try_gen_id};

use super::{AuthError, RefreshToken};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Sign-in request hints used only to help the account owner recognize a login.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionMetadata {
    #[serde(rename = "p")]
    pub ip_address: Option<std::net::IpAddr>,
    #[serde(rename = "a")]
    pub user_agent: Option<String>,
}

impl SessionMetadata {
    pub fn new(ip_address: Option<std::net::IpAddr>, user_agent: Option<&str>) -> Self {
        Self {
            ip_address,
            user_agent: user_agent
                .filter(|value| !value.is_empty() && value.len() <= 512)
                .map(str::to_owned),
        }
    }
}

/// A persisted login with a server-enforced renewal deadline.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Session {
    #[serde(rename = "e")]
    pub expires_at: u64,
    #[serde(rename = "l")]
    pub last_seen_at: u64,
    #[serde(rename = "m")]
    pub metadata: SessionMetadata,
    #[serde(rename = "i")]
    pub id: u64,
    #[serde(rename = "u")]
    pub user_id: u64,
    #[serde(rename = "o")]
    pub organization_id: u64,
    /// Current permission generation, copied from storage when renewing credentials.
    #[serde(rename = "v")]
    pub authorization_version: u32,
    #[serde(rename = "r")]
    pub revoked_at: Option<u64>,
}

impl Session {
    pub async fn new(user_id: u64, organization_id: u64) -> Result<Self, DistributedIdError> {
        Ok(Self {
            expires_at: 0,
            last_seen_at: compallm_utils::time::unix_timestamp(),
            metadata: SessionMetadata::default(),
            id: try_gen_id().await?,
            user_id,
            organization_id,
            authorization_version: 1,
            revoked_at: None,
        })
    }

    /// Match IDs only; organization names and revocation state are not identity.
    pub fn has_same_identity(&self, other: &Self) -> bool {
        self.id == other.id
            && self.user_id == other.user_id
            && self.organization_id == other.organization_id
    }

    pub fn is_active(&self, session: &Self) -> bool {
        self.has_same_identity(session)
            && self.revoked_at.is_none()
            && self.expires_at > compallm_utils::time::unix_timestamp()
    }

    pub fn validate_credential(&self, presented: &RefreshToken) -> Result<(), AuthError> {
        presented.validate(compallm_utils::time::unix_timestamp())?;
        if !self.is_active(&presented.session) {
            return Err(AuthError::InvalidSession);
        }
        Ok(())
    }

    /// Renew credentials and record the corresponding activity and deadline.
    pub fn refresh(
        &self,
        presented: &RefreshToken,
        lifetime: Duration,
    ) -> Result<RefreshToken, AuthError> {
        self.validate_credential(presented)?;
        let mut token = RefreshToken::from_session(self, lifetime)?;
        token.session.last_seen_at = token.validity.issued_at;
        token.session.expires_at = token.validity.expires_at;
        Ok(token)
    }

    pub fn switch_organization(
        &mut self,
        presented: &RefreshToken,
        organization_id: u64,
    ) -> Result<(), AuthError> {
        self.validate_credential(presented)?;
        self.organization_id = organization_id;
        Ok(())
    }

    pub fn revoke(&mut self) {
        self.revoked_at
            .get_or_insert_with(compallm_utils::time::unix_timestamp);
    }
}
