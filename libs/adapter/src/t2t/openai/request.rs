//! Incremental text-only OpenAI requests. Unsupported semantics fail explicitly;
//! these codecs do not implement the complete Chat or Responses APIs.

use crate::t2t::request::{Format, decoder, encoder};

use crate::{
    codec::{DecodeStep, EncodeStep},
    error::CodecError,
    t2t::Text2TextTranslatorConfiguration,
    t2t::request::{
        RequestScratch, decoder::RequestDecoder, encoder::RequestEncoder, event::RequestEvent,
    },
};

macro_rules! request_decoder {
    ($name:ident, $format:expr, $description:literal) => {
        #[doc = $description]
        pub struct $name(decoder::Decoder);
        impl $name {
            /// Bound emitted fragments and decoded text deferred before role.
            /// Zero deferral permits only empty content before role.
            pub fn new(
                fragment_bytes: usize,
                deferred_text_bytes: usize,
            ) -> Result<Self, CodecError> {
                Ok(Self(decoder::Decoder::new(
                    $format,
                    fragment_bytes,
                    deferred_text_bytes,
                )?))
            }
        }
        impl RequestDecoder for $name {
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
    };
}
request_decoder!(
    OpenAICompletionRequestDecoder,
    Format::Chat,
    "Chat Completions string-content request decoder with bounded deferral."
);
request_decoder!(
    OpenAIResponseRequestDecoder,
    Format::Responses,
    "Responses string/message-array request decoder with bounded deferral."
);

macro_rules! request_encoder {
    ($name:ident, $format:expr, $description:literal) => {
        #[doc = $description]
        pub struct $name<'model>(encoder::Encoder<'model>);
        impl<'model> $name<'model> {
            /// Select the target model and bound each accepted decoded fragment.
            /// The model is borrowed for the encoder lifetime, limited to 1024 bytes.
            /// Configuration supplies fallbacks; explicit source limits replace them.
            /// Emits stream=false; sampling/tools/media are not represented.
            pub fn new(
                model: &'model str,
                configuration: Text2TextTranslatorConfiguration,
                fragment_bytes: usize,
            ) -> Result<Self, CodecError> {
                Ok(Self(encoder::Encoder::new(
                    $format,
                    model,
                    fragment_bytes,
                    configuration,
                )?))
            }
        }
        impl RequestEncoder for $name<'_> {
            fn accept(&mut self, event: RequestEvent) -> Result<(), CodecError> {
                self.0.accept(event)
            }
            fn advance(
                &mut self,
                text: &mut &[u8],
                output: &mut [u8],
            ) -> Result<EncodeStep, CodecError> {
                self.0.advance(text, output)
            }
            fn finish(&mut self) -> Result<(), CodecError> {
                self.0.finish()
            }
        }
    };
}
request_encoder!(
    OpenAICompletionRequestEncoder,
    Format::Chat,
    "Canonical Chat Completions role/string-content request encoder."
);
request_encoder!(
    OpenAIResponseRequestEncoder,
    Format::Responses,
    "Canonical Responses message-array request encoder."
);
