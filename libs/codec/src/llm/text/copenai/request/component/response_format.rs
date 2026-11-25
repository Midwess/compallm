use crate::llm::text::component::{JsonSchema, ResponseFormat};
use compallm_serde_rules::{borrowed, deserialize, serialize};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Serialize)]
#[serde(
    remote = "ResponseFormat",
    tag = "type",
    content = "json_schema",
    rename_all = "snake_case"
)]
pub(crate) enum ResponseFormatWire<S: AsRef<str> + Serialize> {
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
    #[serde(serialize_with = "serialize::serialize_raw")]
    schema: S,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct FormatInput<'a> {
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
    name: Cow<'a, str>,
    #[serde(default, borrow, deserialize_with = "borrowed::optional_text")]
    description: Option<Cow<'a, str>>,
    strict: Option<bool>,
    #[serde(borrow, deserialize_with = "borrowed::schema")]
    schema: Cow<'a, str>,
}

impl<'a> TryFrom<FormatInput<'a>> for ResponseFormat<Cow<'a, str>> {
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
