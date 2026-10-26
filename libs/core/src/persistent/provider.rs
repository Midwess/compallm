use async_trait::async_trait;

use super::ConnectionError;
use crate::provider::{Provider, ProviderApiKey};

pub const MAX_PROVIDERS_PER_ORGANIZATION: u64 = 128;
pub const MAX_PROVIDER_FIND_LIMIT: u64 = 128;

#[derive(Debug, thiserror::Error)]
pub enum ProviderRepositoryError {
    #[error("provider not found")]
    NotFound,
    #[error("provider name is already in use")]
    NameAlreadyExists,
    #[error("organization has reached its provider limit")]
    MaximumProvidersReached,
    #[error("provider query limit must be between 1 and {MAX_PROVIDER_FIND_LIMIT}")]
    InvalidLimit,
    #[error("database operation failed")]
    Database(#[from] anyhow::Error),
}

impl ProviderRepositoryError {
    pub fn database(source: impl Into<anyhow::Error>) -> Self {
        Self::Database(source.into())
    }
}

impl From<ConnectionError> for ProviderRepositoryError {
    fn from(error: ConnectionError) -> Self {
        Self::database(error)
    }
}

/// Uses the active `DbContext` and leaves transaction ownership to the caller.
/// `insert` must run inside a transaction so locking can enforce the
/// per-organization provider limit under concurrent requests.
#[async_trait]
pub trait ProviderRepository: Send + Sync {
    async fn find_all(
        &self,
        organization_id: u64,
        limit: u64,
    ) -> Result<Vec<Provider>, ProviderRepositoryError>;

    async fn insert(
        &self,
        provider: Provider,
        api_key: ProviderApiKey,
    ) -> Result<Provider, ProviderRepositoryError>;

    async fn update(
        &self,
        provider: Provider,
        api_key: Option<ProviderApiKey>,
    ) -> Result<Provider, ProviderRepositoryError>;

    async fn delete(
        &self,
        organization_id: u64,
        provider_id: u64,
    ) -> Result<Provider, ProviderRepositoryError>;
}
