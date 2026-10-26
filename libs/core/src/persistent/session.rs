use super::ConnectionError;
use crate::auth::Session;

#[derive(Debug, thiserror::Error)]
pub enum SessionRepositoryError {
    #[error("session database operation failed")]
    Database(#[from] anyhow::Error),
}

impl From<ConnectionError> for SessionRepositoryError {
    fn from(error: ConnectionError) -> Self {
        Self::Database(error.into())
    }
}

#[async_trait::async_trait]
pub trait SessionRepository: Send + Sync {
    async fn create(&self, session: Session) -> Result<(), SessionRepositoryError>;

    /// Must run in a transaction and lock the owned row until completion.
    async fn find_for_update(
        &self,
        id: u64,
        user_id: u64,
    ) -> Result<Option<Session>, SessionRepositoryError>;

    /// Compare-and-swap the current credential of an active, unexpired session.
    /// Preserve created_at; return false if the expected credential is no longer current.
    async fn rotate(
        &self,
        session: &Session,
        expected_token_id: &str,
    ) -> Result<bool, SessionRepositoryError>;

    /// Idempotently revoke only a session belonging to this user.
    async fn revoke(&self, id: u64, user_id: u64, at: u64) -> Result<(), SessionRepositoryError>;
}
