use crate::llm::text::component::*;
use serde::Serialize;

/// Encoded Chat Completions response format.
#[derive(Serialize)]
#[serde(tag = "type", content = "json_schema", rename_all = "snake_case")]
pub(in crate::llm::text::openai::request::completions) enum ResponseFormatOutput<'a, S: AsRef<str>>
{
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

impl<'a, S: AsRef<str>> From<&'a ResponseFormat<S>> for ResponseFormatOutput<'a, S> {
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
