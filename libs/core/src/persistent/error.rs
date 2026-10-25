//! Shared error types for all persistence repository interfaces.

#[derive(Debug, thiserror::Error)]
pub enum PersistenceError {
    #[error("email is already registered")]
    EmailAlreadyExists,
    #[error("database operation failed")]
    Database(#[source] anyhow::Error),
    #[error("no database connection scope is active")]
    MissingConnection,
    #[error("transaction scope has already ended")]
    ClosedConnection,
    #[error("transaction still has running database operations")]
    BusyTransaction,
    #[error("{message}")]
    General {
        message: String,
        #[source]
        source: anyhow::Error,
    },
}

impl PersistenceError {
    pub fn database(source: impl Into<anyhow::Error>) -> Self {
        Self::Database(source.into())
    }
    pub fn general(message: impl Into<String>, source: anyhow::Error) -> Self {
        Self::General {
            message: message.into(),
            source,
        }
    }
}

impl From<anyhow::Error> for PersistenceError {
    fn from(source: anyhow::Error) -> Self {
        Self::general("persistence operation failed", source)
    }
}
