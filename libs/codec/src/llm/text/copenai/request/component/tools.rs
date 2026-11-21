use super::{deserialize, serialize};
use crate::llm::text::component::{FunctionDefinition, ToolDefinition, ToolType};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_with::{DeserializeAs, SerializeAs};

#[derive(Serialize, Deserialize)]
#[serde(remote = "ToolType", rename_all = "snake_case")]
pub(super) enum ToolTypeWire {
    Function,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "ToolDefinition", deny_unknown_fields)]
pub(crate) struct ToolWire {
    #[serde(rename = "type", with = "ToolTypeWire")]
    kind: ToolType,
    #[serde(with = "FunctionDefinitionWire")]
    function: FunctionDefinition,
}

impl SerializeAs<ToolDefinition> for ToolWire {
    fn serialize_as<S: Serializer>(
        value: &ToolDefinition,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        Self::serialize(value, serializer)
    }
}

impl<'de> DeserializeAs<'de, ToolDefinition> for ToolWire {
    fn deserialize_as<D: Deserializer<'de>>(deserializer: D) -> Result<ToolDefinition, D::Error> {
        Self::deserialize(deserializer)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "FunctionDefinition", deny_unknown_fields)]
struct FunctionDefinitionWire {
    #[serde(deserialize_with = "deserialize::nonempty")]
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(
        default,
        deserialize_with = "deserialize::optional_schema",
        serialize_with = "serialize::optional_raw",
        skip_serializing_if = "Option::is_none"
    )]
    parameters: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    strict: Option<bool>,
}
