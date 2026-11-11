//! Compatible Chat request profile with streamed function tools and reasoning.
mod decoder;
mod encoder;
mod schema;

pub use decoder::CompatibleOpenAIRequestDecoder;
pub use encoder::CompatibleOpenAIRequestEncoder;
