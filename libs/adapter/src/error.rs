use thiserror::Error;

/// Codec failures must not include body content or credentials.
#[derive(Debug, Error)]
pub enum CodecError {
    #[error("invalid source body: {0}")]
    InvalidInput(&'static str),
    #[error("unsupported translation: {0}")]
    Unsupported(&'static str),
    #[error("translation limit exceeded: {0}")]
    LimitExceeded(&'static str),
    #[error("codec contract violated: {0}")]
    Contract(&'static str),
}

/// Preserves transport errors without imposing Send, Sync, or static bounds.
#[derive(Debug, Error)]
pub enum StreamError<E> {
    #[error("input body failed: {0}")]
    Input(E),
    #[error(transparent)]
    Codec(#[from] CodecError),
}
