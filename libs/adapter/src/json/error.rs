#[derive(Debug, thiserror::Error)]
pub enum JsonDecodeError {
    #[error("invalid JSON token")]
    InvalidToken,
    #[error("incomplete JSON token")]
    IncompleteToken,
    #[error("invalid JSON decoder state")]
    InvalidState,
}
