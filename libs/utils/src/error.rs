#[derive(Debug, thiserror::Error)]
pub enum EmailError {
    #[error("invalid email address")]
    InvalidEmail,
}
