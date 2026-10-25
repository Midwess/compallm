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
