use super::super::component::CompatibleOpenAIRequestComponent;
use super::schema::SchemaState;
use crate::decoder::DecoderError;
use crate::json::JsonToken;
use crate::llm::text::component::{JsonSchema, ResponseFormat};

#[derive(Debug, Default)]
pub(super) struct ResponseFormatState {
    pub(super) active: bool,
    kind: Option<FormatKind>,
    has_schema: bool,
    in_schema: bool,
    name: Option<String>,
    has_description: bool,
    description: Option<String>,
    has_strict: bool,
    strict: Option<bool>,
    schema: Option<String>,
    schema_value: SchemaState,
}

#[derive(Debug)]
enum FormatKind {
    Text,
    JsonObject,
    JsonSchema,
}

impl ResponseFormatState {
    pub(super) fn proceed(
        &mut self,
        token: JsonToken<'_>,
    ) -> Result<Option<CompatibleOpenAIRequestComponent>, DecoderError> {
        if !self.active {
            if !matches!(token, JsonToken::ValueObject(None)) {
                return Err(DecoderError::InvalidInput(
                    "response format must be an object",
                ));
            }
            *self = Self {
                active: true,
                ..Self::default()
            };
            return Ok(None);
        }
        if self.schema_value.active {
            if let Some(schema) = self.schema_value.proceed(token)? {
                self.schema = Some(schema);
            }
            return Ok(None);
        }
        if self.in_schema {
            match token {
                JsonToken::ValueString(Some("name"), value) if !value.is_empty() => {
                    if self.name.is_some() {
                        return Err(DecoderError::InvalidInput("duplicate schema name"));
                    }
                    self.name = Some(value.to_owned());
                }
                JsonToken::ValueString(Some("description"), value) => {
                    if self.has_description {
                        return Err(DecoderError::InvalidInput("duplicate schema description"));
                    }
                    self.has_description = true;
                    self.description = Some(value.to_owned());
                }
                JsonToken::ValueNull(Some("description")) => {
                    if self.has_description {
                        return Err(DecoderError::InvalidInput("duplicate schema description"));
                    }
                    self.has_description = true;
                }
                JsonToken::ValueBoolean(Some("strict"), value) => {
                    if self.has_strict {
                        return Err(DecoderError::InvalidInput("duplicate schema strict"));
                    }
                    self.has_strict = true;
                    self.strict = Some(value);
                }
                JsonToken::ValueNull(Some("strict")) => {
                    if self.has_strict {
                        return Err(DecoderError::InvalidInput("duplicate schema strict"));
                    }
                    self.has_strict = true;
                }
                JsonToken::ValueObject(Some("schema")) => {
                    if self.schema.is_some() {
                        return Err(DecoderError::InvalidInput("duplicate schema"));
                    }
                    self.schema_value.begin();
                }
                JsonToken::ValueString(Some("schema"), value) => {
                    if self.schema.is_some() {
                        return Err(DecoderError::InvalidInput("duplicate schema"));
                    }
                    self.schema = Some(value.to_owned());
                }
                JsonToken::ObjectEnd => {
                    if self.name.is_none() || self.schema.is_none() {
                        return Err(DecoderError::InvalidInput("incomplete JSON schema"));
                    }
                    self.in_schema = false;
                }
                _ => return Err(DecoderError::InvalidInput("invalid JSON schema field")),
            }
            return Ok(None);
        }
        match token {
            JsonToken::ValueString(Some("type"), value) => {
                if self.kind.is_some() {
                    return Err(DecoderError::InvalidInput("duplicate response format type"));
                }
                self.kind = Some(match value {
                    "text" => FormatKind::Text,
                    "json_object" => FormatKind::JsonObject,
                    "json_schema" => FormatKind::JsonSchema,
                    _ => return Err(DecoderError::InvalidInput("invalid response format type")),
                });
                Ok(None)
            }
            JsonToken::ValueObject(Some("json_schema")) => {
                if self.has_schema {
                    return Err(DecoderError::InvalidInput("duplicate JSON schema"));
                }
                self.has_schema = true;
                self.in_schema = true;
                Ok(None)
            }
            JsonToken::ObjectEnd => {
                let format = match (self.kind.take(), self.has_schema) {
                    (Some(FormatKind::Text), false) => ResponseFormat::Text,
                    (Some(FormatKind::JsonObject), false) => ResponseFormat::JsonObject,
                    (Some(FormatKind::JsonSchema), true) => {
                        ResponseFormat::JsonSchema(JsonSchema {
                            name: self
                                .name
                                .take()
                                .ok_or(DecoderError::InvalidInput("missing schema name"))?,
                            description: self.description.take(),
                            strict: self.strict.take(),
                            schema: self
                                .schema
                                .take()
                                .ok_or(DecoderError::InvalidInput("missing schema"))?,
                        })
                    }
                    _ => return Err(DecoderError::InvalidInput("invalid response format")),
                };
                self.active = false;
                Ok(Some(CompatibleOpenAIRequestComponent::ResponseFormat(
                    format,
                )))
            }
            _ => Err(DecoderError::InvalidInput("invalid response format field")),
        }
    }
}
