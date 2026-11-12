use super::ListOptions;
use crate::persistent::ConnectionError;
use crate::user::User;
use async_trait::async_trait;

#[derive(Clone, Debug, Default)]
pub struct UserQuery {
    pub id: Option<i64>,
    pub ids: Option<Vec<i64>>,
    pub email: Option<String>,
    pub emails: Option<Vec<String>>,
}

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
    async fn list(
        &self,
        query: UserQuery,
        options: ListOptions,
    ) -> Result<Vec<User>, UserRepositoryError>;

    async fn one(
        &self,
        query: UserQuery,
        options: ListOptions,
    ) -> Result<Option<User>, UserRepositoryError> {
        let users = self.list(query, options.with_limit(1)).await?;
        Ok(users.into_iter().next())
    }

    /// Enforce email uniqueness atomically; translate conflicts into
    /// EmailAlreadyExists. Set verification and creation timestamps on insert;
    /// the signup workflow verifies email ownership before calling this method.
    /// Persist a prepared domain User without normalizing or validating its fields again.
    async fn create(&self, user: User) -> Result<User, UserRepositoryError>;
}
