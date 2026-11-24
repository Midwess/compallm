pub trait Encoder<Component> {
    type Error;

    fn next(
        &mut self,
        component: Option<&mut Component>,
        output: &mut [u8],
    ) -> Result<EncoderStatus, Self::Error>;

    fn finish(self) -> Result<(), Self::Error>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EncoderStatus {
    NeedOutput { written: usize },
    ComponentFinished { written: usize },
    Finished { written: usize },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum EncoderError {
    #[error("invalid component: {0}")]
    InvalidInput(&'static str),
    #[error("unsupported component: {0}")]
    Unsupported(&'static str),
    #[error("encoding limit exceeded: {0}")]
    LimitExceeded(&'static str),
    #[error("incomplete input")]
    IncompleteInput,
    #[error("invalid encoder state")]
    InvalidState,
}

impl From<crate::json::JsonEncodeError> for EncoderError {
    fn from(error: crate::json::JsonEncodeError) -> Self {
        match error {
            crate::json::JsonEncodeError::InvalidState => Self::InvalidState,
            crate::json::JsonEncodeError::Incomplete => Self::IncompleteInput,
            crate::json::JsonEncodeError::Serialize(_) => {
                Self::InvalidInput("JSON value serialization failed")
            }
        }
    }
}
