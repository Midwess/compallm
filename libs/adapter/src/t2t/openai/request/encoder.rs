use crate::t2t::request::{Format, encoder};

use crate::{
    codec::EncodeStep,
    error::CodecError,
    t2t::Text2TextTranslatorConfiguration,
    t2t::request::{encoder::RequestEncoder, event::RequestEvent},
};

macro_rules! request_encoder {
    ($name:ident, $format:expr, $driver:ident, $description:literal) => {
        #[doc = $description]
        pub struct $name<'model>(encoder::$driver<'model>);
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
                Ok(Self(encoder::$driver::new($format, model, configuration)?))
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
    ProfileEncoder,
    "Chat Completions encoder accepting native messages and supported structured text events."
);
request_encoder!(
    OpenAIResponseRequestEncoder,
    Format::Responses,
    Encoder,
    "Canonical Responses message-array request encoder."
);

impl OpenAIResponseRequestEncoder<'_> {
    /// Preserve opaque reasoning only after the caller verifies source/target compatibility.
    pub fn with_compatible_reasoning(mut self, compatible: bool) -> Self {
        self.0 = self.0.with_compatible_reasoning(compatible);
        self
    }
}
