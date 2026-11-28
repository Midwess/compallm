use crate::llm::text::component::{JsonSchema, ResponseFormat};
use compallm_serde_rules::{borrowed, deserialize};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[serde(
    remote = "ResponseFormat",
    tag = "type",
    content = "json_schema",
    rename_all = "snake_case"
)]
pub(in crate::llm::text::openai_completion::request) enum ResponseFormatWire<
    S: AsRef<str> + Serialize,
> {
    Text,
    JsonObject,
    JsonSchema(#[serde(with = "JsonSchemaWire")] JsonSchema<S>),
}

#[derive(Serialize)]
#[serde(remote = "JsonSchema", bound(serialize = "S: Serialize + AsRef<str>"))]
struct JsonSchemaWire<S> {
    name: S,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<S>,
    #[serde(skip_serializing_if = "Option::is_none")]
    strict: Option<bool>,
    schema: crate::json::JsonView<S>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(in crate::llm::text::openai_completion::request) struct FormatInput<'a> {
    #[serde(rename = "type")]
    kind: FormatKind,
    #[serde(default, borrow, deserialize_with = "deserialize::present")]
    json_schema: Option<JsonSchemaInput<'a>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum FormatKind {
    Text,
    JsonObject,
    JsonSchema,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct JsonSchemaInput<'a> {
    #[serde(borrow, deserialize_with = "borrowed::nonempty")]
    name: &'a str,
    #[serde(default, borrow, deserialize_with = "borrowed::optional_text")]
    description: Option<&'a str>,
    strict: Option<bool>,
    #[serde(borrow)]
    schema: crate::json::JsonView<&'a str>,
}

impl<'a> TryFrom<FormatInput<'a>> for ResponseFormat<&'a str> {
    type Error = &'static str;
    fn try_from(value: FormatInput<'a>) -> Result<Self, Self::Error> {
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
