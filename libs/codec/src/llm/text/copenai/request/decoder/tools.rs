use super::super::component::CompatibleOpenAIRequestComponent;
use super::schema::SchemaState;
use crate::decoder::DecoderError;
use crate::json::JsonToken;
use crate::llm::text::component::{FunctionDefinition, ToolDefinition, ToolType};

#[derive(Debug, Default)]
pub(super) struct ToolsState {
    pub(super) active: bool,
    tools: Vec<ToolDefinition>,
    current: Option<ToolDraft>,
}

#[derive(Debug, Default)]
struct ToolDraft {
    has_type: bool,
    has_function: bool,
    in_function: bool,
    name: Option<String>,
    description: Option<String>,
    has_description: bool,
    strict: Option<bool>,
    has_strict: bool,
    parameters: Option<String>,
    has_parameters: bool,
    schema: SchemaState,
}

impl ToolsState {
    pub(super) fn proceed(
        &mut self,
        token: JsonToken<'_>,
    ) -> Result<Option<CompatibleOpenAIRequestComponent>, DecoderError> {
        if !self.active {
            if !matches!(token, JsonToken::ValueArray(None)) {
                return Err(DecoderError::InvalidInput("tools must be an array"));
            }
            self.active = true;
            return Ok(None);
        }
        if let Some(tool) = &mut self.current {
            if tool.proceed(token)? {
                let tool = self.current.take().ok_or(DecoderError::InvalidState)?;
                self.tools.push(tool.try_into()?);
            }
            return Ok(None);
        }
        match token {
            JsonToken::ValueObject(None) => {
                self.current = Some(ToolDraft::default());
                Ok(None)
            }
            JsonToken::ArrayEnd => {
                self.active = false;
                Ok(Some(CompatibleOpenAIRequestComponent::Tools(
                    std::mem::take(&mut self.tools),
                )))
            }
            _ => Err(DecoderError::InvalidInput("tool must be an object")),
        }
    }
}

impl ToolDraft {
    fn proceed(&mut self, token: JsonToken<'_>) -> Result<bool, DecoderError> {
        if self.schema.active {
            if let Some(schema) = self.schema.proceed(token)? {
                self.parameters = Some(schema);
            }
            return Ok(false);
        }
        if self.in_function {
            match token {
                JsonToken::ValueString(Some("name"), value) if !value.is_empty() => {
                    if self.name.is_some() {
                        return Err(DecoderError::InvalidInput("duplicate function name"));
                    }
                    self.name = Some(value.to_owned());
                }
                JsonToken::ValueString(Some("description"), value) => {
                    if self.has_description {
                        return Err(DecoderError::InvalidInput("duplicate function description"));
                    }
                    self.has_description = true;
                    self.description = Some(value.to_owned());
                }
                JsonToken::ValueNull(Some("description")) => {
                    if self.has_description {
                        return Err(DecoderError::InvalidInput("duplicate function description"));
                    }
                    self.has_description = true;
                }
                JsonToken::ValueBoolean(Some("strict"), value) => {
                    if self.has_strict {
                        return Err(DecoderError::InvalidInput("duplicate function strict"));
                    }
                    self.has_strict = true;
                    self.strict = Some(value);
                }
                JsonToken::ValueNull(Some("strict")) => {
                    if self.has_strict {
                        return Err(DecoderError::InvalidInput("duplicate function strict"));
                    }
                    self.has_strict = true;
                }
                JsonToken::ValueObject(Some("parameters")) => {
                    if self.has_parameters {
                        return Err(DecoderError::InvalidInput("duplicate function parameters"));
                    }
                    self.has_parameters = true;
                    self.schema.begin();
                }
                JsonToken::ValueString(Some("parameters"), value) => {
                    if self.has_parameters {
                        return Err(DecoderError::InvalidInput("duplicate function parameters"));
                    }
                    self.has_parameters = true;
                    self.parameters = Some(value.to_owned());
                }
                JsonToken::ValueNull(Some("parameters")) => {
                    if self.has_parameters {
                        return Err(DecoderError::InvalidInput("duplicate function parameters"));
                    }
                    self.has_parameters = true;
                }
                JsonToken::ObjectEnd => {
                    self.in_function = false;
                }
                _ => return Err(DecoderError::InvalidInput("invalid function definition")),
            }
            return Ok(false);
        }
        match token {
            JsonToken::ValueString(Some("type"), "function") => {
                if self.has_type {
                    return Err(DecoderError::InvalidInput("duplicate tool type"));
                }
                self.has_type = true;
                Ok(false)
            }
            JsonToken::ValueObject(Some("function")) => {
                if self.has_function {
                    return Err(DecoderError::InvalidInput("duplicate tool function"));
                }
                self.has_function = true;
                self.in_function = true;
                Ok(false)
            }
            JsonToken::ObjectEnd => Ok(true),
            _ => Err(DecoderError::InvalidInput("invalid tool definition")),
        }
    }
}

impl TryFrom<ToolDraft> for ToolDefinition {
    type Error = DecoderError;
    fn try_from(draft: ToolDraft) -> Result<Self, Self::Error> {
        if !draft.has_type || !draft.has_function {
            return Err(DecoderError::InvalidInput("incomplete tool definition"));
        }
        Ok(Self {
            kind: ToolType::Function,
            function: FunctionDefinition {
                name: draft
                    .name
                    .ok_or(DecoderError::InvalidInput("missing function name"))?,
                description: draft.description,
                parameters: draft.parameters,
                strict: draft.strict,
            },
        })
    }
}
