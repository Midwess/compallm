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

/// Explicit translator configuration, processing, or I/O failure.
#[derive(Debug, Error)]
pub enum TranslationError {
    #[error("translation failed: {0}")]
    Stream(StreamError<std::io::Error>),
    #[error("target failed: {0}")]
    Target(std::io::Error),
    #[error("missing translator component: {0}")]
    MissingComponent(&'static str),
    #[error("invalid translator operation: {0}")]
    InvalidState(&'static str),
}
