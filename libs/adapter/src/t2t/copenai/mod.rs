//! OpenAI-compatible Chat request codecs with function tools and reasoning history.
mod decoder;
mod encoder;
mod schema;

pub use decoder::CompatibleOpenAIRequestDecoder;
pub use encoder::CompatibleOpenAIRequestEncoder;
