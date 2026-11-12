//! Incremental standard text-only OpenAI requests. These profiles do not implement
//! the complete Chat or Responses APIs. Compatible extensions live in copenai.

mod decoder;
mod encoder;

pub use decoder::{OpenAICompletionRequestDecoder, OpenAIResponseRequestDecoder};
pub use encoder::{OpenAICompletionRequestEncoder, OpenAIResponseRequestEncoder};
