use crate::decoder::DecoderError;
use crate::json::JsonToken;
use crate::t2t::component::TextRequestComponent;

use super::CompatibleOpenAIRequestDecoder;

impl CompatibleOpenAIRequestDecoder {
    pub(super) fn model(token: JsonToken<'_>) -> Result<TextRequestComponent, DecoderError> {
        match token {
            JsonToken::String(model) if !model.is_empty() => {
                Ok(TextRequestComponent::Model(model.to_owned()))
            }
            _ => Err(DecoderError::InvalidInput(
                "model must be a nonempty string",
            )),
        }
    }
}
