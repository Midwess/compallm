use crate::decoder::DecoderError;
use crate::json::JsonToken;
use crate::t2t::component::TextRequestComponent;

use super::CompatibleOpenAIRequestDecoder;

impl CompatibleOpenAIRequestDecoder {
    pub(super) fn parallel_tool_calls(
        token: JsonToken<'_>,
    ) -> Result<TextRequestComponent, DecoderError> {
        match token {
            JsonToken::Boolean(value) => Ok(TextRequestComponent::ParallelToolCalls(value)),
            _ => Err(DecoderError::InvalidInput(
                "parallel_tool_calls must be boolean",
            )),
        }
    }
}
