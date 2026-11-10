use hero::distributed_id::{generator::DistributedIdError, try_gen_id};
use std::sync::LazyLock;

use cedar_policy::SchemaFragment;

use crate::authorization::{
    SystemPolicy,
    policy::{EntityType, Policy},
};

use super::{AuthError, RefreshToken};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// A persisted login. CWTs own expiry; ordinary renewal does not change this record.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Session {
    #[serde(rename = "i")]
    pub id: u64,
    #[serde(rename = "u")]
    pub user_id: u64,
    #[serde(rename = "o")]
    pub organization_id: u64,
    /// Display snapshot refreshed on token issuance; not part of session identity.
    #[serde(rename = "n")]
    pub organization_name: String,
    /// Current permission generation, copied from storage when renewing credentials.
    #[serde(rename = "v")]
    pub authorization_version: u32,
    #[serde(rename = "r")]
    pub revoked_at: Option<u64>,
}

impl Session {
    pub async fn new(
        user_id: u64,
        organization_id: u64,
        organization_name: impl Into<String>,
    ) -> Result<Self, DistributedIdError> {
        Ok(Self {
            id: try_gen_id().await?,
            user_id,
            organization_id,
            organization_name: organization_name.into(),
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
        self.has_same_identity(session) && self.revoked_at.is_none()
    }

    pub fn validate_credential(&self, presented: &RefreshToken) -> Result<(), AuthError> {
        presented.validate(compallm_utils::time::unix_timestamp())?;
        if !self.is_active(&presented.session) {
            return Err(AuthError::InvalidSession);
        }
        Ok(())
    }

    /// Renew CWT validity while preserving the database session.
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

impl SystemPolicy for Session {
    /// Declares ownership-scoped account session management, not token access.
    ///
    /// - `session.list`: list safe summaries of the authenticated User's own sessions;
    ///   the container is User, so viewing personal security does not require an
    ///   organization-administrator grant. Queries retain organization scoping as needed.
    /// - `session.metadata.read`: inspect safe metadata for an owned Session; require its `user`
    ///   reference to equal the authenticated principal. Never expose bearer/refresh tokens.
    /// - `session.revoke`: invalidate an owned Session, including current-session signout;
    ///   revocation is irreversible and must not issue replacement authority.
    ///
    /// These are baseline account operations independent of custom roles. Knowing a
    /// session ID or being an organization administrator is not permission to read
    /// another user's sessions. Privileged cross-user revocation is not defined here.
    /// Existing authentication and token checks remain required; the schema itself
    /// neither proves ownership nor isolates baseline decisions from custom forbids.
    fn policy() -> SchemaFragment {
        static DEFINITION: LazyLock<SchemaFragment> =
            LazyLock::new(|| Policy::new(EntityType::Session).into());
        DEFINITION.clone()
    }
}
