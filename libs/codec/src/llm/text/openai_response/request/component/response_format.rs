use crate::llm::text::component::{JsonSchema, ResponseFormat};
use compallm_serde_rules::borrowed;
use serde::{Deserialize, Serialize};
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(in crate::llm::text::openai_response::request) struct TextInput<'a> {
    #[serde(borrow)]
    format: FormatInput<'a>,
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
struct FormatInput<'a> {
    #[serde(rename = "type")]
    kind: FormatKind,
    #[serde(default, borrow, deserialize_with = "borrowed::present_text")]
    name: Option<&'a str>,
    #[serde(default, borrow, deserialize_with = "borrowed::optional_text")]
    description: Option<&'a str>,
    strict: Option<bool>,
    #[serde(default, borrow)]
    schema: Option<crate::json::JsonView<&'a str>>,
}
impl<'a> TryFrom<TextInput<'a>> for ResponseFormat<&'a str> {
    type Error = &'static str;
    fn try_from(v: TextInput<'a>) -> Result<Self, Self::Error> {
        let v = v.format;
        match v.kind {
            FormatKind::Text | FormatKind::JsonObject
                if v.name.is_none()
                    && v.description.is_none()
                    && v.strict.is_none()
                    && v.schema.is_none() =>
            {
                Ok(if matches!(v.kind, FormatKind::Text) {
                    Self::Text
                } else {
                    Self::JsonObject
                })
            }
            FormatKind::JsonSchema => Ok(Self::JsonSchema(JsonSchema {
                name: v
                    .name
                    .filter(|name| !name.is_empty())
                    .ok_or("missing schema name")?,
                description: v.description,
                strict: v.strict,
                schema: v.schema.ok_or("missing schema")?,
            })),
            _ => Err("invalid format"),
        }
    }
}
#[derive(Serialize)]
pub(in crate::llm::text::openai_response::request) struct TextOutput<'a, S: AsRef<str>> {
    format: FormatOutput<'a, S>,
}
#[derive(Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum FormatOutput<'a, S: AsRef<str>> {
    Text,
    JsonObject,
    JsonSchema {
        name: &'a str,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<&'a str>,
        #[serde(skip_serializing_if = "Option::is_none")]
        strict: Option<bool>,
        schema: &'a crate::json::JsonView<S>,
    },
}
impl<'a, S: AsRef<str>> From<&'a ResponseFormat<S>> for TextOutput<'a, S> {
    fn from(v: &'a ResponseFormat<S>) -> Self {
        Self {
            format: match v {
                ResponseFormat::Text => FormatOutput::Text,
                ResponseFormat::JsonObject => FormatOutput::JsonObject,
                ResponseFormat::JsonSchema(v) => FormatOutput::JsonSchema {
                    name: v.name.as_ref(),
                    description: v.description.as_ref().map(AsRef::as_ref),
                    strict: v.strict,
                    schema: &v.schema,
                },
            },
        }
    }
}
