use super::{deserialize, tools::ToolTypeWire};
use crate::llm::text::component::{ToolChoice, ToolType};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_with::{DeserializeAs, SerializeAs};

pub(crate) struct ToolChoiceWire;

impl SerializeAs<ToolChoice> for ToolChoiceWire {
    fn serialize_as<S: Serializer>(value: &ToolChoice, serializer: S) -> Result<S::Ok, S::Error> {
        ChoiceOutput::from(value).serialize(serializer)
    }
}

impl<'de> DeserializeAs<'de, ToolChoice> for ToolChoiceWire {
    fn deserialize_as<D: Deserializer<'de>>(deserializer: D) -> Result<ToolChoice, D::Error> {
        ChoiceInput::deserialize(deserializer).map(Into::into)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum ChoiceName {
    Auto,
    None,
    Required,
}

#[derive(Serialize)]
#[serde(untagged)]
enum ChoiceOutput<'a> {
    Name(ChoiceName),
    Function {
        #[serde(rename = "type", with = "ToolTypeWire")]
        kind: ToolType,
        function: FunctionName<&'a str>,
    },
}

#[derive(Deserialize)]
#[serde(untagged, deny_unknown_fields)]
enum ChoiceInput {
    Name(ChoiceName),
    Function {
        #[serde(rename = "type", with = "ToolTypeWire")]
        kind: ToolType,
        function: FunctionName,
    },
}

#[derive(Serialize, Deserialize)]
#[serde(
    deny_unknown_fields,
    bound(deserialize = "S: Deserialize<'de> + AsRef<str>")
)]
struct FunctionName<S = String> {
    #[serde(deserialize_with = "deserialize::nonempty")]
    name: S,
}

impl<'a> From<&'a ToolChoice> for ChoiceOutput<'a> {
    fn from(value: &'a ToolChoice) -> Self {
        match value {
            ToolChoice::Auto => Self::Name(ChoiceName::Auto),
            ToolChoice::None => Self::Name(ChoiceName::None),
            ToolChoice::Required => Self::Name(ChoiceName::Required),
            ToolChoice::Function(name) => Self::Function {
                kind: ToolType::Function,
                function: FunctionName { name },
            },
        }
    }
}

impl From<ChoiceInput> for ToolChoice {
    fn from(value: ChoiceInput) -> Self {
        match value {
            ChoiceInput::Name(ChoiceName::Auto) => Self::Auto,
            ChoiceInput::Name(ChoiceName::None) => Self::None,
            ChoiceInput::Name(ChoiceName::Required) => Self::Required,
            ChoiceInput::Function {
                kind: ToolType::Function,
                function,
            } => Self::Function(function.name),
        }
    }
}
