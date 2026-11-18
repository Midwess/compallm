use super::super::component::CompatibleOpenAIRequestComponent;
use crate::decoder::DecoderError;
use crate::json::JsonToken;
use crate::llm::text::component::ToolChoice;

#[derive(Debug, Default)]
pub(super) struct ToolChoiceState {
    pub(super) active: bool,
    in_function: bool,
    has_function: bool,
    has_type: bool,
    name: Option<String>,
}

impl ToolChoiceState {
    pub(super) fn proceed(
        &mut self,
        token: JsonToken<'_>,
    ) -> Result<Option<CompatibleOpenAIRequestComponent>, DecoderError> {
        if !self.active {
            if !matches!(token, JsonToken::ValueObject(None)) {
                return Err(DecoderError::InvalidInput("tool choice must be an object"));
            }
            *self = Self {
                active: true,
                ..Self::default()
            };
            return Ok(None);
        }
        if self.in_function {
            match token {
                JsonToken::ValueString(Some("name"), value) if !value.is_empty() => {
                    if self.name.is_some() {
                        return Err(DecoderError::InvalidInput("duplicate function name"));
                    }
                    self.name = Some(value.to_owned());
                }
                JsonToken::ObjectEnd => {
                    if self.name.is_none() {
                        return Err(DecoderError::InvalidInput("missing function name"));
                    }
                    self.in_function = false;
                }
                _ => return Err(DecoderError::InvalidInput("invalid tool choice function")),
            }
            return Ok(None);
        }
        match token {
            JsonToken::ValueString(Some("type"), "function") => {
                if self.has_type {
                    return Err(DecoderError::InvalidInput("duplicate tool choice type"));
                }
                self.has_type = true;
                Ok(None)
            }
            JsonToken::ValueObject(Some("function")) => {
                if self.has_function {
                    return Err(DecoderError::InvalidInput("duplicate tool choice function"));
                }
                self.has_function = true;
                self.in_function = true;
                Ok(None)
            }
            JsonToken::ObjectEnd => {
                if !self.has_type || !self.has_function {
                    return Err(DecoderError::InvalidInput("incomplete tool choice"));
                }
                let name = self
                    .name
                    .take()
                    .ok_or(DecoderError::InvalidInput("missing function name"))?;
                self.active = false;
                Ok(Some(CompatibleOpenAIRequestComponent::ToolChoice(
                    ToolChoice::Function(name),
                )))
            }
            _ => Err(DecoderError::InvalidInput("invalid tool choice field")),
        }
    }
}
