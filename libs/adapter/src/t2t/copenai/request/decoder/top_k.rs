use crate::decoder::DecoderError;
use crate::json::JsonToken;
use crate::t2t::component::TextRequestComponent;

use super::CompatibleOpenAIRequestDecoder;

impl CompatibleOpenAIRequestDecoder {
    pub(super) fn top_k(token: JsonToken<'_>) -> Result<TextRequestComponent, DecoderError> {
        if let JsonToken::Number(text) = token
            && text.bytes().all(|byte| byte.is_ascii_digit())
            && let Ok(value) = text.parse::<u32>()
        {
            return Ok(TextRequestComponent::TopK(value));
        }
        Err(DecoderError::InvalidInput("top_k must be a u32 integer"))
    }
}
