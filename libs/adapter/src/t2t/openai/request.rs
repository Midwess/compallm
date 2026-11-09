//! Incremental text-only OpenAI requests. Unsupported semantics fail explicitly;
//! these codecs do not implement the complete Chat or Responses APIs.

mod decoder;
mod encoder;
mod json;

use crate::{
    codec::{DecodeStep, EncodeStep},
    error::CodecError,
    t2t::request::{decoder::RequestDecoder, encoder::RequestEncoder, event::RequestEvent},
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Format {
    Chat,
    Responses,
}

const MODEL_BYTES: usize = 1024;

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
            fn advance(
                &mut self,
                input: &mut &[u8],
                eof: bool,
            ) -> Result<DecodeStep<RequestEvent>, CodecError> {
                self.0.advance(input, eof)
            }
        }
    };
}
request_decoder!(
    ChatCompletionsRequestDecoder,
    Format::Chat,
    "Chat Completions string-content request decoder with bounded deferral."
);
request_decoder!(
    ResponsesRequestDecoder,
    Format::Responses,
    "Responses string/message-array request decoder with bounded deferral."
);

macro_rules! request_encoder {
    ($name:ident, $format:expr, $description:literal) => {
        #[doc = $description]
        pub struct $name(encoder::Encoder);
        impl $name {
            /// Select the target model and bound each accepted decoded fragment.
            /// The model is copied once after validating its 1024-byte bound.
            /// Emits stream=false; sampling/tools/media are not represented.
            pub fn new(model: impl AsRef<str>, fragment_bytes: usize) -> Result<Self, CodecError> {
                Ok(Self(encoder::Encoder::new(
                    $format,
                    model.as_ref(),
                    fragment_bytes,
                )?))
            }
        }
        impl RequestEncoder for $name {
            fn accept(&mut self, event: RequestEvent) -> Result<(), CodecError> {
                self.0.accept(event)
            }
            fn advance(&mut self, output: &mut [u8]) -> Result<EncodeStep, CodecError> {
                self.0.advance(output)
            }
            fn finish(&mut self) -> Result<(), CodecError> {
                self.0.finish()
            }
        }
    };
}
request_encoder!(
    ChatCompletionsRequestEncoder,
    Format::Chat,
    "Canonical Chat Completions role/string-content request encoder."
);
request_encoder!(
    ResponsesRequestEncoder,
    Format::Responses,
    "Canonical Responses message-array request encoder."
);
