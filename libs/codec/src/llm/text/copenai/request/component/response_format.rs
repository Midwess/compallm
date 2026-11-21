use super::{deserialize, serialize};
use crate::llm::text::component::{JsonSchema, ResponseFormat};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::{DeserializeAs, de::DeserializeAsWrap};

#[derive(Serialize)]
#[serde(
    remote = "ResponseFormat",
    tag = "type",
    content = "json_schema",
    rename_all = "snake_case"
)]
pub(crate) enum ResponseFormatWire {
    Text,
    JsonObject,
    JsonSchema(#[serde(with = "JsonSchemaWire")] JsonSchema),
}

impl<'de> DeserializeAs<'de, ResponseFormat> for ResponseFormatWire {
    fn deserialize_as<D: Deserializer<'de>>(deserializer: D) -> Result<ResponseFormat, D::Error> {
        FormatInput::deserialize(deserializer)?
            .try_into()
            .map_err(serde::de::Error::custom)
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct FormatInput {
    #[serde(rename = "type")]
    kind: FormatKind,
    #[serde(default, deserialize_with = "deserialize::present")]
    json_schema: Option<DeserializeAsWrap<JsonSchema, JsonSchemaWire>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum FormatKind {
    Text,
    JsonObject,
    JsonSchema,
}

impl TryFrom<FormatInput> for ResponseFormat {
    type Error = &'static str;
    fn try_from(value: FormatInput) -> Result<Self, Self::Error> {
        match (value.kind, value.json_schema) {
            (FormatKind::Text, None) => Ok(Self::Text),
            (FormatKind::JsonObject, None) => Ok(Self::JsonObject),
            (FormatKind::JsonSchema, Some(schema)) => Ok(Self::JsonSchema(schema.into_inner())),
            _ => Err("invalid response format"),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "JsonSchema", deny_unknown_fields)]
struct JsonSchemaWire {
    #[serde(deserialize_with = "deserialize::nonempty")]
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    strict: Option<bool>,
    #[serde(
        deserialize_with = "deserialize::schema",
        serialize_with = "serialize::serialize_raw"
    )]
    schema: String,
}

impl<'de> DeserializeAs<'de, JsonSchema> for JsonSchemaWire {
    fn deserialize_as<D: Deserializer<'de>>(deserializer: D) -> Result<JsonSchema, D::Error> {
        Self::deserialize(deserializer)
    }
}
