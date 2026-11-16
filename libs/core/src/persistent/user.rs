use super::connection::ConnectionError;
use crate::user::User;
use async_trait::async_trait;

#[derive(Debug, thiserror::Error)]
pub enum UserRepositoryError {
    #[error("user is missing or disabled")]
    NotFound,
    #[error("username is already in use")]
    UsernameAlreadyExists,
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
    /// Email must already be canonical (trimmed and ASCII-lowercased).
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, UserRepositoryError>;

    async fn find_by_id(&self, user_id: u64) -> Result<Option<User>, UserRepositoryError>;

    /// Enforce email and username uniqueness atomically. Username conflicts must
    /// leave the caller transaction usable so signup can retry a suffix. Set verification and creation timestamps on insert;
    /// the signup workflow verifies email ownership before calling this method.
    /// Persist a prepared domain User without normalizing or validating its fields again.
    async fn create(&self, user: User) -> Result<User, UserRepositoryError>;

    async fn update_profile(
        &self,
        user_id: u64,
        display_name: Option<String>,
        username: Option<String>,
        avatar_path: Option<String>,
    ) -> Result<User, UserRepositoryError>;
}
