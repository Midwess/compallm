impl From<compallm_utils::error::EmailError> for AuthError {
    fn from(error: compallm_utils::error::EmailError) -> Self {
        match error {
            compallm_utils::error::EmailError::InvalidEmail => Self::InvalidEmail,
        }
    }
}

impl From<crate::user::RegistrationError> for AuthError {
    fn from(error: crate::user::RegistrationError) -> Self {
        use crate::user::RegistrationError;
        match error {
            RegistrationError::Connection(error) => Self::Connection(error),
            RegistrationError::Organization(error) => Self::Organization(error),
            RegistrationError::InvalidDisplayName => Self::InvalidDisplayName,
            RegistrationError::InvalidEmail(error) => error.into(),
            RegistrationError::Persistence(error) => Self::Persistence(error),
            RegistrationError::IdGeneration(error) => Self::IdGeneration(error),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error(transparent)]
    Organization(#[from] crate::organization::OrganizationCreationError),
    #[error(transparent)]
    Email(#[from] crate::mail::EmailError),
    #[error("invalid authentication configuration")]
    InvalidConfiguration,
    #[error("invalid or unauthorized email address")]
    InvalidEmail,
    #[error("user not found")]
    UserNotFound,
    #[error("user is disabled")]
    UserDisabled,
    #[error("display name must not be empty")]
    InvalidDisplayName,
    #[error(transparent)]
    Persistence(#[from] crate::persistent::user::UserRepositoryError),
    #[error(transparent)]
    Connection(#[from] crate::persistent::connection::ConnectionError),
    #[error("ID generation failed")]
    IdGeneration(#[from] hero::distributed_id::generator::DistributedIdError),
    #[error("invalid access token")]
    InvalidToken,
    #[error("access token expired")]
    ExpiredToken,
    #[error("incorrect access token variant")]
    WrongTokenVariant,
    #[error("incorrect email code")]
    IncorrectCode,
    #[error("system clock is invalid")]
    InvalidClock(#[from] std::time::SystemTimeError),
    #[error("invalid token encoding")]
    TokenEncoding(#[from] base64::DecodeError),
    #[error("invalid token payload")]
    TokenPayload(#[from] serde_json::Error),
    #[error("token encryption or authentication failed")]
    TokenCrypto(#[from] chacha20poly1305::Error),
    #[error("authentication operation failed")]
    Unexpected(#[from] anyhow::Error),
}
