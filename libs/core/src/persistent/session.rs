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

    /// Match the session, organization, and user together.
    async fn find(&self, session: &Session) -> Result<Option<Session>, SessionRepositoryError>;

    /// Persist changed fields using the previous state for concurrency checks.
    /// Preserve identity and creation time; revocation is irreversible.
    /// Return false on a conflict or missing row. Do not overwrite unchanged fields.
    async fn update(
        &self,
        session: &Session,
        previous: &Session,
    ) -> Result<bool, SessionRepositoryError>;
}
