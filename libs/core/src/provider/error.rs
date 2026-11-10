#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum ProviderApiKeyError {
    #[error("invalid provider API key")]
    InvalidKey,
    #[error("invalid provider credential encryption configuration")]
    InvalidConfiguration,
    #[error("invalid encrypted provider credential")]
    InvalidCiphertext,
    #[error("provider credential encryption failed")]
    EncryptionFailed,
    #[error("provider credential decryption failed")]
    DecryptionFailed,
}

#[derive(Debug, thiserror::Error)]
pub enum ProviderCreationError {
    #[error(transparent)]
    Permission(#[from] crate::authorization::MemberPermissionError),
    #[error("invalid provider configuration")]
    InvalidInput(#[from] validify::ValidationErrors),
    #[error("provider name is already in use")]
    NameAlreadyExists,
    #[error(transparent)]
    Persistence(crate::persistent::provider::ProviderRepositoryError),
    #[error(transparent)]
    Connection(#[from] crate::persistent::ConnectionError),
    #[error("ID generation failed")]
    IdGeneration(#[from] hero::distributed_id::generator::DistributedIdError),
}

impl From<crate::persistent::provider::ProviderRepositoryError> for ProviderCreationError {
    fn from(error: crate::persistent::provider::ProviderRepositoryError) -> Self {
        match error {
            crate::persistent::provider::ProviderRepositoryError::NameAlreadyExists => {
                Self::NameAlreadyExists
            }
            error => Self::Persistence(error),
        }
    }
}
