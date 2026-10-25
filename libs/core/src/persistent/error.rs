//! Persistence failures that business services need to distinguish.

#[derive(Debug, thiserror::Error)]
pub enum PersistenceError {
    #[error("email is already registered")]
    EmailAlreadyExists,
    #[error("database operation failed")]
    Database(#[from] anyhow::Error),
}

impl PersistenceError {
    pub fn database(source: impl Into<anyhow::Error>) -> Self {
        Self::Database(source.into())
    }
}
