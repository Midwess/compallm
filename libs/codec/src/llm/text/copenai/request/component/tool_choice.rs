use super::Decoded;
use super::{
    CompatibleOpenAIRequestComponent, ComponentDeserializeHelper, tools::CompatibleOpenAIToolType,
};
use crate::llm::text::component::{ToolChoice, ToolType};
use serde::Deserialize;

impl<'de> Deserialize<'de> for Decoded<ToolChoice> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::Error;
        CompatibleOpenAIToolChoice::deserialize(deserializer)?
            .try_into()
            .map(Self)
            .map_err(D::Error::custom)
    }
}

#[derive(serde::Deserialize)]
#[serde(untagged, deny_unknown_fields)]
pub(super) enum CompatibleOpenAIToolChoice {
    Name(String),
    Function {
        #[serde(rename = "type", with = "CompatibleOpenAIToolType")]
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

impl TryFrom<CompatibleOpenAIToolChoice> for ToolChoice {
    type Error = &'static str;

    fn try_from(value: CompatibleOpenAIToolChoice) -> Result<Self, Self::Error> {
        match value {
            CompatibleOpenAIToolChoice::Name(value) => match value.as_str() {
                "auto" => Ok(Self::Auto),
                "none" => Ok(Self::None),
                "required" => Ok(Self::Required),
                _ => Err("invalid tool choice"),
            },
            CompatibleOpenAIToolChoice::Function {
                kind: ToolType::Function,
                function,
            } => Ok(Self::Function(function.name)),
        }
    }
}
