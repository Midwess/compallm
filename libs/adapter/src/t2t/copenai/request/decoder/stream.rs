use crate::decoder::DecoderError;
use crate::json::JsonToken;
use crate::t2t::component::TextRequestComponent;

use super::CompatibleOpenAIRequestDecoder;

impl CompatibleOpenAIRequestDecoder {
    pub(super) fn stream(token: JsonToken<'_>) -> Result<TextRequestComponent, DecoderError> {
        match token {
            JsonToken::Boolean(value) => Ok(TextRequestComponent::Stream(value)),
            _ => Err(DecoderError::InvalidInput("stream must be boolean")),
        }
    }
}
