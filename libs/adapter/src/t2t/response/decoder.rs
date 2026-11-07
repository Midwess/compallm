use bytes::Bytes;

use super::event::ResponseEvent;
use crate::{codec::DecodeStep, error::CodecError};

pub trait ResponseDecoder {
    /// Consume a prefix and return at most one bounded event. NeedInput requires
    /// empty input and is invalid at EOF. Finished requires validated EOF and
    /// no remaining bytes. Validate framing and terminal markers for the source
    /// response mode; reject unsupported semantics. Each call must be bounded.
    fn advance(
        &mut self,
        input: &mut Bytes,
        eof: bool,
    ) -> Result<DecodeStep<ResponseEvent>, CodecError>;
}
