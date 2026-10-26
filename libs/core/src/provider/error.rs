#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
#[error("provider API key must not be empty")]
pub struct ProviderApiKeyError;

#[derive(Debug, thiserror::Error)]
pub enum ProviderManagementError {
    #[error("provider name must not be blank")]
    InvalidName,
    #[error("provider URL must not be blank")]
    InvalidUrl,
    #[error(transparent)]
    Persistence(#[from] crate::persistent::provider::ProviderRepositoryError),
    #[error(transparent)]
    Connection(#[from] crate::persistent::ConnectionError),
    #[error("ID generation failed")]
    IdGeneration(#[from] hero::distributed_id::generator::DistributedIdError),
}
