//! Incremental standard text-only OpenAI requests. These profiles do not implement
//! the complete Chat or Responses APIs. Compatible extensions live in copenai.

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
            /// Create a decoder using the capacities of caller-owned scratch.
            pub fn new() -> Self {
                Self(decoder::Decoder::new($format))
            }
        }
        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }
        impl RequestDecoder for $name {
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
            /// Select the target model and fallback generation configuration.
            /// The model is borrowed for the encoder lifetime, limited to 1024 bytes.
            /// Configuration supplies fallbacks; explicit source limits replace them.
            /// Request events supply streaming and top_p. Other sampling
            /// settings, tools, and media are outside this standard text profile.
            pub fn new(
                model: &'model str,
                configuration: Text2TextTranslatorConfiguration,
            ) -> Result<Self, CodecError> {
                Ok(Self(encoder::Encoder::new($format, model, configuration)?))
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
