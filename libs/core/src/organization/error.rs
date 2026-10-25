#[derive(Debug, thiserror::Error)]
pub enum OrganizationCreationError {
    #[error("organization name must not be empty")]
    InvalidName,
    #[error(transparent)]
    Persistence(#[from] crate::persistent::organization::OrganizationRepositoryError),
    #[error(transparent)]
    Connection(#[from] crate::persistent::ConnectionError),
    #[error("ID generation failed")]
    IdGeneration(#[from] hero::distributed_id::generator::DistributedIdError),
}
