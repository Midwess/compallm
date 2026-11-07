use crate::organization::Role;

#[derive(Debug, thiserror::Error)]
pub enum RoleRepositoryError {
    #[error("role was not found or changed concurrently")]
    Conflict,
    #[error("role persistence failed")]
    Database(#[from] anyhow::Error),
    #[error(transparent)]
    Connection(#[from] super::ConnectionError),
}

#[async_trait::async_trait]
pub trait RoleRepository: Send + Sync {
    async fn list(&self, organization_id: u64) -> Result<Vec<Role>, RoleRepositoryError>;
    async fn find(
        &self,
        organization_id: u64,
        id: u64,
    ) -> Result<Option<Role>, RoleRepositoryError>;
    async fn create(&self, role: Role) -> Result<Role, RoleRepositoryError>;
    /// Conditional update; policy changes atomically invalidate every affected session.
    async fn update(&self, role: Role) -> Result<Role, RoleRepositoryError>;
}
