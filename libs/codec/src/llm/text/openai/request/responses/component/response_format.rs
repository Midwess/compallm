use crate::llm::text::component::*;
use compallm_serde_rules::deserialize;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(in crate::llm::text::openai::request::responses) struct Text {
    pub format: FormatInput,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(in crate::llm::text::openai::request::responses) struct FormatInput {
    #[serde(rename = "type")]
    kind: FormatKind,
    #[serde(default, deserialize_with = "deserialize::optional_schema")]
    schema: Option<String>,
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    strict: Option<bool>,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::llm::text::openai::request::responses) enum FormatKind {
    Text,
    JsonObject,
    JsonSchema,
}

impl TryFrom<FormatInput> for ResponseFormat {
    type Error = &'static str;

    fn try_from(value: FormatInput) -> Result<Self, Self::Error> {
        let FormatInput {
            kind,
            schema,
            name,
            description,
            strict,
        } = value;
        match kind {
            FormatKind::Text | FormatKind::JsonObject => {
                if name.is_some() || description.is_some() || strict.is_some() || schema.is_some() {
                    return Err("invalid response format");
                }
                Ok(match kind {
                    FormatKind::Text => Self::Text,
                    _ => Self::JsonObject,
                })
            }
            FormatKind::JsonSchema => {
                let name = name.filter(|name| !name.is_empty());
                let schema = schema.ok_or("invalid response format")?;
                Ok(Self::JsonSchema(JsonSchema {
                    name: name.ok_or("invalid response format")?,
                    description,
                    strict,
                    schema,
                }))
            }
        }
    }
}

/// Encoded Responses text configuration.
#[derive(Serialize)]
pub(in crate::llm::text::openai::request::responses) struct TextOutput<'a> {
    pub(in crate::llm::text::openai::request::responses) format: FormatOutput<'a>,
}

/// Encoded Responses response format.
#[derive(Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(in crate::llm::text::openai::request::responses) enum FormatOutput<'a> {
    Text,
    JsonObject,
    JsonSchema {
        name: &'a str,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<&'a str>,
        #[serde(skip_serializing_if = "Option::is_none")]
        strict: Option<bool>,
        #[serde(serialize_with = "compallm_serde_rules::serialize::serialize_raw")]
        schema: &'a str,
    },
}

impl<'a> From<&'a ResponseFormat> for FormatOutput<'a> {
    fn from(format: &'a ResponseFormat) -> Self {
        match format {
            ResponseFormat::Text => Self::Text,
            ResponseFormat::JsonObject => Self::JsonObject,
            ResponseFormat::JsonSchema(schema) => Self::JsonSchema {
                name: &schema.name,
                description: schema.description.as_deref(),
                strict: schema.strict,
                schema: &schema.schema,
            },
        }
    }
}
