//! Database lifecycle contract. SQL and ORM types belong to database adapters.

use super::db_context::PermissionLoader;
use async_once_cell::Lazy;
use async_trait::async_trait;
use std::{any::Any, error::Error, future::Future, pin::Pin, sync::Arc};

type IdentityResult = Result<crate::authentication::Session, Arc<dyn Error + Send + Sync>>;
pub type IdentityLoader =
    Arc<Lazy<IdentityResult, Pin<Box<dyn Future<Output = IdentityResult> + Send>>>>;

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
    fn is_transaction(&self) -> bool;
    fn scoped(&self) -> Arc<dyn Connection>;
    fn is_restricted(&self) -> bool;
    fn identify(&self, identity: IdentityLoader) -> Result<(), ConnectionError>;
    fn restrict(&self, permission: PermissionLoader) -> Result<(), ConnectionError>;
    fn ensure_open(&self) -> Result<(), ConnectionError>;
    async fn begin(&self) -> Result<Arc<dyn Connection>, ConnectionError>;
    async fn commit(&self) -> Result<(), ConnectionError>;
    async fn rollback(&self) -> Result<(), ConnectionError>;
    /// Release uncommitted ownership synchronously on cancellation or panic.
    fn cancel(&self);
}
