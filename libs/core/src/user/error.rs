#[derive(Debug, thiserror::Error)]
pub enum RegistrationError {
    #[error(transparent)]
    Organization(#[from] crate::organization::OrganizationCreationError),
    #[error("display name must contain between 1 and 256 characters")]
    InvalidDisplayName,
    #[error("invalid email address")]
    InvalidEmail(#[from] compallm_utils::error::EmailError),
    #[error(transparent)]
    Persistence(#[from] crate::persistent::user::UserRepositoryError),
    #[error("ID generation failed")]
    IdGeneration(#[from] hero::distributed_id::generator::DistributedIdError),
}
