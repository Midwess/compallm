//! Database lifecycle contract. SQL and ORM types belong to database adapters.

use async_trait::async_trait;
use std::{any::Any, sync::Arc};

use super::PersistenceError;

#[async_trait]
pub trait Connection: Any + Send + Sync {
    /// Adapter access without exposing its concrete types to Core.
    fn as_any(&self) -> &dyn Any;
    fn ensure_open(&self) -> Result<(), PersistenceError>;
    async fn begin(&self) -> Result<Arc<dyn Connection>, PersistenceError>;
    async fn commit(&self) -> Result<(), PersistenceError>;
    async fn rollback(&self) -> Result<(), PersistenceError>;
    /// Release uncommitted ownership synchronously on cancellation or panic.
    fn cancel(&self);
}
