use super::event::RequestEvent;
use crate::{codec::DecodeStep, error::CodecError};

pub trait RequestDecoder {
    /// Consume a prefix of `input`, returning at most one bounded event.
    ///
    /// Advance `input` to its remaining suffix. NeedInput requires an empty
    /// suffix and is invalid at EOF. Finished requires EOF and no remaining
    /// bytes. Retain bounded state, validate trailing input, and reject semantics
    /// that cannot be represented instead of silently discarding them.
    /// The slice borrows reusable translator storage; do not retain it.
    fn advance(
        &mut self,
        input: &mut &[u8],
        eof: bool,
    ) -> Result<DecodeStep<RequestEvent>, CodecError>;
}

impl<D: RequestDecoder + ?Sized> RequestDecoder for Box<D> {
    fn advance(
        &mut self,
        input: &mut &[u8],
        eof: bool,
    ) -> Result<DecodeStep<RequestEvent>, CodecError> {
        (**self).advance(input, eof)
    }
}
