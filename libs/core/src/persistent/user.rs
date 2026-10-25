use super::error::PersistenceError;
use crate::entity::user::User;
use async_trait::async_trait;

/// Use the active `DbContext`. Never open or commit a separate transaction
/// in these operations.
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, PersistenceError>;

    async fn find_by_id(&self, user_id: u64) -> Result<Option<User>, PersistenceError>;

    /// Enforce email uniqueness atomically; translate conflicts into
    /// EmailAlreadyExists. Set verification and creation timestamps on insert;
    /// the signup workflow verifies email ownership before calling this method.
    async fn create(&self, user: User) -> Result<User, PersistenceError>;

    async fn record_signin(&self, user_id: u64, at: u64) -> Result<(), PersistenceError>;
}
