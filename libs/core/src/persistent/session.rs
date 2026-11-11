use super::ConnectionError;
use crate::authentication::Session;
use crate::organization::OrganizationMember;

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
    /// One primary projection scoped by organization, user and session; None if revoked/missing.
    async fn authorization_version(
        &self,
        session: &Session,
    ) -> Result<Option<u32>, SessionRepositoryError>;

    /// Read active session, user, organization, membership and all assignments
    /// from one primary statement snapshot. Never combine multiple query results.
    async fn find_policy_membership(
        &self,
        session: &Session,
    ) -> Result<Option<(Session, OrganizationMember)>, SessionRepositoryError>;

    /// Increment stored versions once for every non-revoked session in this scope.
    /// Join the current connection/transaction; return the affected-session count.
    async fn invalidate_permissions(
        &self,
        organization_id: u64,
        user_ids: &[u64],
    ) -> Result<u64, SessionRepositoryError>;

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
