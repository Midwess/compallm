pub trait RequestDecoder<E> {
    fn flush(&mut self, input: &mut &[u8]) -> Result<RequestDecoderStatus, RequestDecoderError>;

    fn get_component(&self) -> Option<E>;

    fn finish(&mut self) -> Result<(), RequestDecoderError>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RequestDecoderStatus {
    ComponentReady,
    NeedInput,
    Progress,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum RequestDecoderError {
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
