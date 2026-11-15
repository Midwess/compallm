pub trait Decoder {
    type Component<'a>
    where
        Self: 'a;

    type Error;

    fn next<'a, 'input: 'a>(
        &'a mut self,
        input: &mut &'input [u8],
        eof: bool,
    ) -> Result<DecoderStatus<Self::Component<'a>>, Self::Error>;

    fn finish(self) -> Result<(), Self::Error>;
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
