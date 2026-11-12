//! Incremental Anthropic Messages requests with string or text-block content.
//! System text blocks are supported; tools, media, and SSE response codecs are unsupported.

mod decoder;
mod encoder;

pub use decoder::AnthropicRequestDecoder;
pub use encoder::AnthropicRequestEncoder;
