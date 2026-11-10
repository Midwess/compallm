//! Incremental Anthropic Messages requests for the string-content text subset.
//! Tools, media, content-block arrays, and response streaming are unsupported.

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
    /// Bound emitted fragments and decoded content deferred before its role.
    pub fn new(fragment_bytes: usize, deferred_text_bytes: usize) -> Result<Self, CodecError> {
        Ok(Self(decoder::Decoder::new(
            Format::Anthropic,
            fragment_bytes,
            deferred_text_bytes,
        )?))
    }
}

impl RequestDecoder for AnthropicRequestDecoder {
    fn scratch_requirements(&self) -> (usize, usize) {
        self.0.scratch_requirements()
    }
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
    /// the fallback. Require a positive budget/fragment bound and model <= 1024 bytes.
    pub fn new(
        model: &'model str,
        configuration: Text2TextTranslatorConfiguration,
        fragment_bytes: usize,
    ) -> Result<Self, CodecError> {
        Ok(Self(encoder::Encoder::new(
            Format::Anthropic,
            model,
            fragment_bytes,
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
