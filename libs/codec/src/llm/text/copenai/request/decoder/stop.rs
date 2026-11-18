use super::super::component::CompatibleOpenAIRequestComponent;
use crate::decoder::DecoderError;
use crate::json::JsonToken;
use crate::llm::text::component::Stop;

#[derive(Debug, Default)]
pub(super) struct StopState {
    pub(super) active: bool,
    values: Vec<String>,
    bytes: usize,
}

impl StopState {
    pub(super) fn proceed(
        &mut self,
        token: JsonToken<'_>,
    ) -> Result<Option<CompatibleOpenAIRequestComponent>, DecoderError> {
        if !self.active {
            if !matches!(token, JsonToken::ValueArray(None)) {
                return Err(DecoderError::InvalidInput("stop must be an array"));
            }
            self.active = true;
            self.bytes = 0;
            return Ok(None);
        }
        match token {
            JsonToken::ValueString(None, value) => {
                if value.is_empty() || self.values.len() == 4 || value.len() > 4096 - self.bytes {
                    return Err(DecoderError::InvalidInput("invalid stop sequences"));
                }
                self.bytes += value.len();
                self.values.push(value.to_owned());
                Ok(None)
            }
            JsonToken::ArrayEnd => {
                self.active = false;
                self.bytes = 0;
                Ok(Some(CompatibleOpenAIRequestComponent::Stop(Stop::Array(
                    std::mem::take(&mut self.values),
                ))))
            }
            _ => Err(DecoderError::InvalidInput("stop array requires strings")),
        }
    }
}
