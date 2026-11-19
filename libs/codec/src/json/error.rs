#[derive(Debug, thiserror::Error)]
pub enum JsonDecodeError {
    #[error("invalid JSON token")]
    InvalidToken,
    #[error("incomplete JSON token")]
    IncompleteToken,
    #[error("invalid JSON decoder state")]
    InvalidState,
    #[error("JSON value deserialization failed: {0}")]
    Deserialize(#[source] serde_json::Error),
}
