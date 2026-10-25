use async_trait::async_trait;

use super::ConnectionError;
use crate::organization::{Organization, OrganizationMember};

#[derive(Debug, thiserror::Error)]
pub enum OrganizationRepositoryError {
    #[error("organization not found")]
    NotFound,
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
    /// Return an active organization membership, using the compound key.
    async fn find_membership(
        &self,
        organization_id: u64,
        user_id: u64,
    ) -> Result<Option<OrganizationMember>, OrganizationRepositoryError>;
    /// Oldest membership in an active organization, with organization ID as a tie-breaker.
    async fn find_first_membership(
        &self,
        user_id: u64,
    ) -> Result<Option<OrganizationMember>, OrganizationRepositoryError>;
    /// Oldest owner/admin membership in an active organization.
    async fn find_first_admin_membership(
        &self,
        user_id: u64,
    ) -> Result<Option<OrganizationMember>, OrganizationRepositoryError>;
    async fn update_name(
        &self,
        organization_id: u64,
        name: &str,
    ) -> Result<Organization, OrganizationRepositoryError>;
    async fn create(
        &self,
        organization: Organization,
    ) -> Result<Organization, OrganizationRepositoryError>;
    async fn add_member(
        &self,
        member: OrganizationMember,
    ) -> Result<OrganizationMember, OrganizationRepositoryError>;
}
