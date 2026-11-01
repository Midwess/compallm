use async_trait::async_trait;

use super::ConnectionError;
use crate::provider::{Provider, ProviderApiKey};

#[derive(Debug, thiserror::Error)]
pub enum ProviderRepositoryError {
    #[error("provider not found")]
    NotFound,
    #[error("provider name is already in use")]
    NameAlreadyExists,
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
