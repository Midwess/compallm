use super::{CompatibleOpenAIRequestComponent, ComponentDeserializeHelper, Decoded};
use crate::llm::text::component::{JsonSchema, ResponseFormat};
use serde::Deserialize;

impl<'de> Deserialize<'de> for Decoded<ResponseFormat> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::Error;
        CompatibleOpenAIResponseFormat::deserialize(deserializer)?
            .try_into()
            .map(Self)
            .map_err(D::Error::custom)
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct CompatibleOpenAIResponseFormat {
    #[serde(rename = "type")]
    kind: String,
    #[serde(
        default,
        deserialize_with = "CompatibleOpenAIRequestComponent::present"
    )]
    json_schema: Option<Decoded<JsonSchema>>,
}

impl TryFrom<CompatibleOpenAIResponseFormat> for ResponseFormat {
    type Error = &'static str;

    fn try_from(value: CompatibleOpenAIResponseFormat) -> Result<Self, Self::Error> {
        match (value.kind.as_str(), value.json_schema) {
            ("text", None) => Ok(Self::Text),
            ("json_object", None) => Ok(Self::JsonObject),
            ("json_schema", Some(Decoded(schema))) => Ok(Self::JsonSchema(schema)),
            _ => Err("invalid response format"),
        }
    }
}

#[derive(Deserialize)]
#[serde(remote = "JsonSchema", deny_unknown_fields)]
struct CompatibleOpenAIJsonSchema {
    #[serde(deserialize_with = "CompatibleOpenAIRequestComponent::nonempty")]
    name: String,
    description: Option<String>,
    strict: Option<bool>,
    #[serde(deserialize_with = "CompatibleOpenAIRequestComponent::schema")]
    schema: String,
}

impl<'de> Deserialize<'de> for Decoded<JsonSchema> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        CompatibleOpenAIJsonSchema::deserialize(deserializer).map(Self)
    }
}
