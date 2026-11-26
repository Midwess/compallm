use crate::llm::text::component::*;
use compallm_serde_rules::borrowed;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(in crate::llm::text::openai::request::responses) struct Text<'a> {
    #[serde(borrow)]
    pub format: FormatInput<'a>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(in crate::llm::text::openai::request::responses) struct FormatInput<'a> {
    #[serde(rename = "type")]
    kind: FormatKind,
    #[serde(default, borrow)]
    schema: Option<crate::json::JsonValue<&'a str>>,
    #[serde(default, borrow, deserialize_with = "borrowed::optional_text")]
    name: Option<&'a str>,
    #[serde(default, borrow, deserialize_with = "borrowed::optional_text")]
    description: Option<&'a str>,
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

impl<'a> TryFrom<FormatInput<'a>> for ResponseFormat<&'a str> {
    type Error = &'static str;

    fn try_from(value: FormatInput<'a>) -> Result<Self, Self::Error> {
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
pub(in crate::llm::text::openai::request::responses) struct TextOutput<'a, S: AsRef<str>> {
    pub(in crate::llm::text::openai::request::responses) format: FormatOutput<'a, S>,
}

/// Encoded Responses response format.
#[derive(Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(in crate::llm::text::openai::request::responses) enum FormatOutput<'a, S: AsRef<str>> {
    Text,
    JsonObject,
    JsonSchema {
        name: &'a str,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<&'a str>,
        #[serde(skip_serializing_if = "Option::is_none")]
        strict: Option<bool>,
        schema: &'a crate::json::JsonValue<S>,
    },
}

impl<'a, S: AsRef<str>> From<&'a ResponseFormat<S>> for FormatOutput<'a, S> {
    fn from(format: &'a ResponseFormat<S>) -> Self {
        match format {
            ResponseFormat::Text => Self::Text,
            ResponseFormat::JsonObject => Self::JsonObject,
            ResponseFormat::JsonSchema(schema) => Self::JsonSchema {
                name: schema.name.as_ref(),
                description: schema.description.as_ref().map(AsRef::as_ref),
                strict: schema.strict,
                schema: &schema.schema,
            },
        }
    }
}
