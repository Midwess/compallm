#[derive(Debug, thiserror::Error)]
pub enum OrganizationCreationError {
    #[error("organization name must contain 1 to 63 ASCII letters, digits, or hyphens")]
    InvalidName,
    #[error(transparent)]
    Persistence(#[from] crate::persistent::organization::OrganizationRepositoryError),
    #[error(transparent)]
    Connection(#[from] crate::persistent::ConnectionError),
    #[error("ID generation failed")]
    IdGeneration(#[from] hero::distributed_id::generator::DistributedIdError),
}

#[derive(Debug, thiserror::Error)]
pub enum OrganizationConfigError {
    #[error("organization name must contain 1 to 63 ASCII letters, digits, or hyphens")]
    InvalidName,
    #[error(transparent)]
    Persistence(#[from] crate::persistent::organization::OrganizationRepositoryError),
    #[error(transparent)]
    Connection(#[from] crate::persistent::ConnectionError),
}
