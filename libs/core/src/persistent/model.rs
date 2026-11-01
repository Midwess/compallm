use async_trait::async_trait;

use super::ConnectionError;
use crate::model::Model;

#[derive(Debug, thiserror::Error)]
pub enum ModelRepositoryError {
    #[error("model not found")]
    NotFound,
    #[error("model name is already in use")]
    NameAlreadyExists,
    #[error("database operation failed")]
    Database(#[from] anyhow::Error),
}

impl ModelRepositoryError {
    pub fn database(source: impl Into<anyhow::Error>) -> Self {
        Self::Database(source.into())
    }
}

impl From<ConnectionError> for ModelRepositoryError {
    fn from(error: ConnectionError) -> Self {
        Self::database(error)
    }
}

/// Uses the active `DbContext` and leaves transaction ownership to the caller.
#[async_trait]
pub trait ModelRepository: Send + Sync {
    /// Lists every model in the organization for management workflows.
    async fn find_all(
        &self,
        organization_id: u64,
        limit: u64,
    ) -> Result<Vec<Model>, ModelRepositoryError>;

    async fn insert(&self, model: Model) -> Result<Model, ModelRepositoryError>;

    async fn update(&self, model: Model) -> Result<Model, ModelRepositoryError>;

    async fn delete(
        &self,
        organization_id: u64,
        model_id: u64,
    ) -> Result<Model, ModelRepositoryError>;
}
