use crate::t2t::request::{Format, decoder};

use crate::{
    codec::DecodeStep,
    error::CodecError,
    t2t::request::{RequestScratch, decoder::RequestDecoder, event::RequestEvent},
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
    "Chat Completions text request decoder with bounded string deferral."
);
request_decoder!(
    OpenAIResponseRequestDecoder,
    Format::Responses,
    "Responses text request decoder with input/output text blocks and bounded string deferral."
);
