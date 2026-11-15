use crate::decoder::DecoderError;
use crate::json::JsonToken;
use crate::t2t::component::TextRequestComponent;

use super::CompatibleOpenAIRequestDecoder;

impl CompatibleOpenAIRequestDecoder {
    pub(super) fn max_completion_tokens(
        token: JsonToken<'_>,
    ) -> Result<TextRequestComponent, DecoderError> {
        if let JsonToken::Number(text) = token
            && text.bytes().all(|byte| byte.is_ascii_digit())
            && let Ok(value @ 1..=u32::MAX) = text.parse::<u32>()
        {
            return Ok(TextRequestComponent::MaxCompletionTokens(value));
        }
        Err(DecoderError::InvalidInput(
            "token limit must be a positive u32 integer",
        ))
    }
}
