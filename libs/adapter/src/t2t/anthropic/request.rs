//! Incremental Anthropic Messages requests for the string-content text subset.
//! Tools, media, content-block arrays, and SSE response codecs are unsupported.

use crate::{
    codec::{DecodeStep, EncodeStep},
    error::CodecError,
    t2t::Text2TextTranslatorConfiguration,
    t2t::request::{
        Format, RequestScratch,
        decoder::{self, RequestDecoder},
        encoder::{self, RequestEncoder},
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

/// Canonical Messages encoder with an optional single leading system message.
pub struct AnthropicRequestEncoder<'model>(encoder::Encoder<'model>);

impl<'model> AnthropicRequestEncoder<'model> {
    /// Select target model and fallback settings. Explicit source limits replace
    /// the fallback. Require a positive budget and model <= 1024 bytes.
    pub fn new(
        model: &'model str,
        configuration: Text2TextTranslatorConfiguration,
    ) -> Result<Self, CodecError> {
        Ok(Self(encoder::Encoder::new(
            Format::Anthropic,
            model,
            configuration,
        )?))
    }
}

impl RequestEncoder for AnthropicRequestEncoder<'_> {
    fn accept(&mut self, event: RequestEvent) -> Result<(), CodecError> {
        self.0.accept(event)
    }

    fn advance(&mut self, text: &mut &[u8], output: &mut [u8]) -> Result<EncodeStep, CodecError> {
        self.0.advance(text, output)
    }

    fn finish(&mut self) -> Result<(), CodecError> {
        self.0.finish()
    }
}
