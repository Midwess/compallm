use super::super::component::CompatibleOpenAIRequestComponent;
use crate::decoder::DecoderError;
use crate::json::JsonToken;
use crate::llm::text::component::Provider;

#[derive(Debug, Default)]
pub(super) struct ProviderState {
    pub(super) active: bool,
    require_parameter: Option<bool>,
}

impl ProviderState {
    pub(super) fn proceed(
        &mut self,
        token: JsonToken<'_>,
    ) -> Result<Option<CompatibleOpenAIRequestComponent>, DecoderError> {
        if !self.active {
            if !matches!(token, JsonToken::ValueObject(None)) {
                return Err(DecoderError::InvalidInput("provider must be an object"));
            }
            self.active = true;
            self.require_parameter = None;
            return Ok(None);
        }
        match token {
            JsonToken::ValueBoolean(Some("require_parameter" | "require_parameters"), value) => {
                if self.require_parameter.is_some() {
                    return Err(DecoderError::InvalidInput("duplicate provider field"));
                }
                self.require_parameter = Some(value);
                Ok(None)
            }
            JsonToken::ObjectEnd => {
                let value = self
                    .require_parameter
                    .take()
                    .ok_or(DecoderError::InvalidInput("missing provider requirement"))?;
                self.active = false;
                Ok(Some(CompatibleOpenAIRequestComponent::Provider(
                    Provider::RequireParameter(value),
                )))
            }
            _ => Err(DecoderError::InvalidInput("invalid provider field")),
        }
    }
}
