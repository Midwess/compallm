use super::ConnectionError;
use crate::auth::{Session, SessionReference};

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

    /// Match the session, organization, and user together.
    async fn find_by_id(
        &self,
        reference: SessionReference,
    ) -> Result<Option<Session>, SessionRepositoryError>;

    /// Compare-and-swap the current credential of an active, unexpired session.
    /// Preserve created_at; return false if the expected credential is no longer current.
    async fn rotate(
        &self,
        session: &Session,
        previous: SessionReference,
        expected_token_id: &str,
    ) -> Result<bool, SessionRepositoryError>;

    /// Idempotently revoke only a session belonging to this user.
    async fn revoke(
        &self,
        reference: SessionReference,
        at: u64,
    ) -> Result<(), SessionRepositoryError>;
}
