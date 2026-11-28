mod component;
pub mod decoder;
pub mod encoder;

pub use decoder::OpenAICompletionRequestDecoder;
pub use encoder::{
    OpenAICompletionRequestEncoder, OpenAICompletionRequestEncoderConfiguration, RequestEncodeError,
};
