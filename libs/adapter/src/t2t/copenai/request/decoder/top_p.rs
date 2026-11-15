use crate::decoder::DecoderError;
use crate::json::JsonToken;
use crate::t2t::component::TextRequestComponent;

use super::CompatibleOpenAIRequestDecoder;

impl CompatibleOpenAIRequestDecoder {
    pub(super) fn top_p(
        token: JsonToken<'_>,
    ) -> Result<Option<TextRequestComponent>, DecoderError> {
        match token {
            JsonToken::Null => return Ok(None),
            JsonToken::Number(text) => {
                if let Ok(value) = text.parse::<f64>()
                    && value.is_finite()
                    && (0.0..=1.0).contains(&value)
                {
                    return Ok(Some(TextRequestComponent::TopP(value)));
                }
            }
            _ => {}
        }
        Err(DecoderError::InvalidInput(
            "top_p must be between zero and one",
        ))
    }
}
