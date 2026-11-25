use crate::llm::text::component::*;
use compallm_serde_rules::deserialize;
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::DeserializeAs;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(in crate::llm::text::openai::request::completions) struct FormatInput {
    #[serde(rename = "type")]
    kind: FormatKind,
    #[serde(default)]
    json_schema: Option<JsonSchemaInput>,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::llm::text::openai::request::completions) enum FormatKind {
    Text,
    JsonObject,
    JsonSchema,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(in crate::llm::text::openai::request::completions) struct JsonSchemaInput {
    #[serde(deserialize_with = "deserialize::nonempty")]
    name: String,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    strict: Option<bool>,
    #[serde(deserialize_with = "deserialize::schema")]
    schema: String,
}

impl TryFrom<FormatInput> for ResponseFormat {
    type Error = &'static str;

    fn try_from(value: FormatInput) -> Result<Self, Self::Error> {
        match (value.kind, value.json_schema) {
            (FormatKind::Text, None) => Ok(Self::Text),
            (FormatKind::JsonObject, None) => Ok(Self::JsonObject),
            (FormatKind::JsonSchema, Some(schema)) => Ok(Self::JsonSchema(JsonSchema {
                name: schema.name,
                description: schema.description,
                strict: schema.strict,
                schema: schema.schema,
            })),
            _ => Err("invalid response format"),
        }
    }
}

impl<'de> DeserializeAs<'de, ResponseFormat> for FormatInput {
    fn deserialize_as<D: Deserializer<'de>>(deserializer: D) -> Result<ResponseFormat, D::Error> {
        FormatInput::deserialize(deserializer)?
            .try_into()
            .map_err(serde::de::Error::custom)
    }
}

/// Encoded Chat Completions response format.
#[derive(Serialize)]
#[serde(tag = "type", content = "json_schema", rename_all = "snake_case")]
pub(in crate::llm::text::openai::request::completions) enum ResponseFormatOutput<'a> {
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

impl<'a> From<&'a ResponseFormat> for ResponseFormatOutput<'a> {
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
