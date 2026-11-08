use async_trait::async_trait;

use super::ConnectionError;
use crate::organization::{Group, Organization, OrganizationMember, Role};

#[derive(Debug, thiserror::Error)]
pub enum OrganizationRepositoryError {
    #[error("organization not found")]
    NotFound,
    #[error("membership assignments changed concurrently")]
    Conflict,
    #[error("database operation failed")]
    Database(#[from] anyhow::Error),
}

impl OrganizationRepositoryError {
    pub fn database(source: impl Into<anyhow::Error>) -> Self {
        Self::Database(source.into())
    }
}

impl From<ConnectionError> for OrganizationRepositoryError {
    fn from(error: ConnectionError) -> Self {
        Self::database(error)
    }
}

/// Use the current DbContext and leave transaction ownership to the caller.
#[async_trait]
pub trait OrganizationRepository: Send + Sync {
    /// Find a role using its organization-scoped identity.
    async fn find_role(
        &self,
        organization_id: u64,
        role_id: u64,
    ) -> Result<Option<Role>, OrganizationRepositoryError>;
    async fn find_roles(
        &self,
        organization_id: u64,
        role_ids: &[u64],
    ) -> Result<Vec<Role>, OrganizationRepositoryError>;
    /// Find a group and its assigned roles using its organization-scoped identity.
    async fn find_group(
        &self,
        organization_id: u64,
        group_id: u64,
    ) -> Result<Option<Group>, OrganizationRepositoryError>;
    /// Insert a group and its role links as one operation.
    async fn create_group(&self, group: Group) -> Result<Group, OrganizationRepositoryError>;
    /// Return the member and active organization, using the compound key.
    async fn find_membership(
        &self,
        organization_id: u64,
        user_id: u64,
    ) -> Result<Option<(OrganizationMember, Organization)>, OrganizationRepositoryError>;
    /// Oldest membership in an active organization, with organization ID as a tie-breaker.
    async fn find_first_membership(
        &self,
        user_id: u64,
    ) -> Result<Option<(OrganizationMember, Organization)>, OrganizationRepositoryError>;
    async fn update_name(
        &self,
        organization_id: u64,
        name: &str,
    ) -> Result<Organization, OrganizationRepositoryError>;
    async fn create(
        &self,
        organization: Organization,
    ) -> Result<Organization, OrganizationRepositoryError>;
    /// Insert initial default-role records within organization creation.
    async fn create_roles(&self, roles: Vec<Role>) -> Result<(), OrganizationRepositoryError>;
    async fn add_member(
        &self,
        member: OrganizationMember,
    ) -> Result<OrganizationMember, OrganizationRepositoryError>;
    /// Save assignment changes only if the previous membership is still current.
    async fn save_member(
        &self,
        member: &OrganizationMember,
        previous: &OrganizationMember,
    ) -> Result<(), OrganizationRepositoryError>;
}
