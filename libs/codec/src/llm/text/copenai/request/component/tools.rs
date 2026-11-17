use super::{CompatibleOpenAIRequestComponent, ComponentDeserializeHelper, Decoded};
use crate::llm::text::component::{FunctionDefinition, ToolDefinition, ToolType};
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(remote = "ToolDefinition", deny_unknown_fields)]
struct ToolFields {
    #[serde(rename = "type", with = "ToolTypeFields")]
    kind: ToolType,
    #[serde(with = "FunctionFields")]
    function: FunctionDefinition,
}

#[derive(Deserialize)]
#[serde(remote = "FunctionDefinition", deny_unknown_fields)]
struct FunctionFields {
    #[serde(deserialize_with = "CompatibleOpenAIRequestComponent::nonempty")]
    name: String,
    description: Option<String>,
    #[serde(
        default,
        deserialize_with = "CompatibleOpenAIRequestComponent::optional_schema"
    )]
    parameters: Option<String>,
    strict: Option<bool>,
}

#[derive(Deserialize)]
#[serde(remote = "ToolType", rename_all = "snake_case")]
pub(super) enum ToolTypeFields {
    Function,
}

impl<'de> Deserialize<'de> for Decoded<ToolDefinition> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        ToolFields::deserialize(deserializer).map(Self)
    }
}
