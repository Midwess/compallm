#[derive(Debug, thiserror::Error)]
pub enum RegistrationError {
    #[error(transparent)]
    Connection(#[from] crate::persistent::ConnectionError),
    #[error(transparent)]
    Organization(#[from] crate::organization::OrganizationCreationError),
    #[error(
        "user name must contain 1 to 256 email local-part characters; dots cannot be first, last, or repeated"
    )]
    InvalidDisplayName,
    #[error("invalid email address")]
    InvalidEmail,
    #[error(transparent)]
    Persistence(#[from] crate::persistent::user::UserRepositoryError),
    #[error("ID generation failed")]
    IdGeneration(#[from] hero::distributed_id::generator::DistributedIdError),
}
