use bytes::Bytes;

use super::event::RequestEvent;
use crate::{codec::DecodeStep, error::CodecError};

pub trait RequestDecoder {
    /// Consume a prefix of `input`, returning at most one bounded event.
    ///
    /// Advance `input` to its remaining suffix. NeedInput requires an empty
    /// suffix and is invalid at EOF. Finished requires EOF and no remaining
    /// bytes. Retain bounded state, validate trailing input, and reject semantics
    /// that cannot be represented instead of silently discarding them.
    fn advance(
        &mut self,
        input: &mut Bytes,
        eof: bool,
    ) -> Result<DecodeStep<RequestEvent>, CodecError>;
}
