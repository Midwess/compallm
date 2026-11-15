pub trait Decoder<E> {
    fn next(&mut self, input: &mut &[u8]) -> Result<DecoderStatus<E>, DecoderError>;

    fn finish(&mut self) -> Result<(), DecoderError>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DecoderStatus<E> {
    Component(E),
    NeedInput,
    Finished,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum DecoderError {
    #[error("invalid request: {0}")]
    InvalidInput(&'static str),
    #[error("unsupported request component: {0}")]
    Unsupported(&'static str),
    #[error("request decoding limit exceeded: {0}")]
    LimitExceeded(&'static str),
    #[error("incomplete request")]
    IncompleteInput,
    #[error("invalid decoder state")]
    InvalidState,
}
