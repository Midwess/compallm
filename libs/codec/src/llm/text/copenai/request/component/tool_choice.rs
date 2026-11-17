use super::Decoded;
use super::{CompatibleOpenAIRequestComponent, ComponentDeserializeHelper, tools::ToolTypeFields};
use crate::llm::text::component::{ToolChoice, ToolType};
use serde::Deserialize;

impl<'de> Deserialize<'de> for Decoded<ToolChoice> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::Error;
        ChoiceFields::deserialize(deserializer)?
            .try_into()
            .map(Self)
            .map_err(D::Error::custom)
    }
}

#[derive(serde::Deserialize)]
#[serde(untagged, deny_unknown_fields)]
pub(super) enum ChoiceFields {
    Name(String),
    Function {
        #[serde(rename = "type", with = "ToolTypeFields")]
        kind: ToolType,
        function: FunctionName,
    },
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct FunctionName {
    #[serde(deserialize_with = "CompatibleOpenAIRequestComponent::nonempty")]
    name: String,
}

impl TryFrom<ChoiceFields> for ToolChoice {
    type Error = &'static str;

    fn try_from(value: ChoiceFields) -> Result<Self, Self::Error> {
        match value {
            ChoiceFields::Name(value) => match value.as_str() {
                "auto" => Ok(Self::Auto),
                "none" => Ok(Self::None),
                "required" => Ok(Self::Required),
                _ => Err("invalid tool choice"),
            },
            ChoiceFields::Function {
                kind: ToolType::Function,
                function,
            } => Ok(Self::Function(function.name)),
        }
    }
}
