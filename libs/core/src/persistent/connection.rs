//! Database lifecycle contract. SQL and ORM types belong to database adapters.

use async_trait::async_trait;
use std::{any::Any, sync::Arc};

#[derive(Debug, thiserror::Error)]
pub enum ConnectionError {
    #[error("database operation failed")]
    Database(#[from] anyhow::Error),
}

impl ConnectionError {
    pub fn database(source: impl Into<anyhow::Error>) -> Self {
        Self::Database(source.into())
    }
}

#[async_trait]
pub trait Connection: Any + Send + Sync {
    /// Adapter access without exposing its concrete types to Core.
    fn as_any(&self) -> &dyn Any;
    fn ensure_open(&self) -> Result<(), ConnectionError>;
    async fn begin(&self) -> Result<Arc<dyn Connection>, ConnectionError>;
    async fn commit(&self) -> Result<(), ConnectionError>;
    async fn rollback(&self) -> Result<(), ConnectionError>;
    /// Release uncommitted ownership synchronously on cancellation or panic.
    fn cancel(&self);
}
