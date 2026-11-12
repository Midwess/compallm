use crate::{
    codec::DecodeStep,
    error::CodecError,
    t2t::request::{
        Format, RequestScratch,
        decoder::{self, RequestDecoder},
        event::RequestEvent,
    },
};

/// Bounded Messages decoder. Optional top-level system text must precede messages.
/// Source max_tokens becomes a shared configuration event; model is validated.
pub struct AnthropicRequestDecoder(decoder::Decoder);

impl AnthropicRequestDecoder {
    /// Create a decoder using the capacities of caller-owned scratch.
    pub fn new() -> Self {
        Self(decoder::Decoder::new(Format::Anthropic))
    }
}

impl Default for AnthropicRequestDecoder {
    fn default() -> Self {
        Self::new()
    }
}

impl RequestDecoder for AnthropicRequestDecoder {
    fn advance(
        &mut self,
        input: &mut &[u8],
        eof: bool,
        scratch: &mut RequestScratch,
    ) -> Result<DecodeStep<RequestEvent>, CodecError> {
        self.0.advance(input, eof, scratch)
    }
}
