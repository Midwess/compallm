use super::super::component::CompatibleOpenAIRequestComponent;
use crate::decoder::DecoderError;
use crate::json::JsonToken;
use crate::llm::text::component::StreamOptions;

#[derive(Debug, Default)]
pub(super) struct StreamOptionsState {
    pub(super) active: bool,
    has_include_usage: bool,
    include_usage: Option<bool>,
}

impl StreamOptionsState {
    pub(super) fn proceed(
        &mut self,
        token: JsonToken<'_>,
    ) -> Result<Option<CompatibleOpenAIRequestComponent>, DecoderError> {
        if !self.active {
            if !matches!(token, JsonToken::ValueObject(None)) {
                return Err(DecoderError::InvalidInput(
                    "stream options must be an object",
                ));
            }
            self.active = true;
            self.has_include_usage = false;
            self.include_usage = None;
            return Ok(None);
        }
        match token {
            JsonToken::ValueBoolean(Some("include_usage"), value) => {
                if self.has_include_usage {
                    return Err(DecoderError::InvalidInput("duplicate stream option"));
                }
                self.has_include_usage = true;
                self.include_usage = Some(value);
                Ok(None)
            }
            JsonToken::ValueNull(Some("include_usage")) => {
                if self.has_include_usage {
                    return Err(DecoderError::InvalidInput("duplicate stream option"));
                }
                self.has_include_usage = true;
                self.include_usage = None;
                Ok(None)
            }
            JsonToken::ObjectEnd => {
                self.active = false;
                Ok(Some(CompatibleOpenAIRequestComponent::StreamOptions(Some(
                    StreamOptions {
                        include_usage: self.include_usage.take(),
                    },
                ))))
            }
            _ => Err(DecoderError::InvalidInput("invalid stream option")),
        }
    }
}
