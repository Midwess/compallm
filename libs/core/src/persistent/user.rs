use super::connection::ConnectionError;
use crate::user::User;
use async_trait::async_trait;
use std::sync::Arc;

#[derive(Debug, thiserror::Error)]
pub enum UserRepositoryError {
    #[error("email is already registered")]
    EmailAlreadyExists,
    #[error("database operation failed")]
    Database(#[from] anyhow::Error),
}

impl UserRepositoryError {
    pub fn database(source: impl Into<anyhow::Error>) -> Self {
        Self::Database(source.into())
    }
}

impl From<ConnectionError> for UserRepositoryError {
    fn from(error: ConnectionError) -> Self {
        Self::database(error)
    }
}

/// Use the active `DbContext`. Never open or commit a separate transaction
/// in these operations.
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, UserRepositoryError>;

    async fn find_by_id(&self, user_id: u64) -> Result<Option<User>, UserRepositoryError>;

    /// Enforce email uniqueness atomically; translate conflicts into
    /// EmailAlreadyExists. Set verification and creation timestamps on insert;
    /// the signup workflow verifies email ownership before calling this method.
    async fn create(&self, user: User) -> Result<User, UserRepositoryError>;

    /// Record a successful signin using the current transaction context.
    async fn record_login(&self, user_id: u64) -> Result<(), UserRepositoryError>;
}

#[async_trait]
impl<R: UserRepository + ?Sized> UserRepository for Arc<R> {
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, UserRepositoryError> {
        self.as_ref().find_by_email(email).await
    }

    async fn find_by_id(&self, user_id: u64) -> Result<Option<User>, UserRepositoryError> {
        self.as_ref().find_by_id(user_id).await
    }

    async fn create(&self, user: User) -> Result<User, UserRepositoryError> {
        self.as_ref().create(user).await
    }

    async fn record_login(&self, user_id: u64) -> Result<(), UserRepositoryError> {
        self.as_ref().record_login(user_id).await
    }
}
