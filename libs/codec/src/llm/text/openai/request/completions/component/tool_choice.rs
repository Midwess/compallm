use super::tools::ToolTypeWire;
use crate::llm::text::component::*;
use compallm_serde_rules::deserialize;
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::DeserializeAs;

#[derive(Deserialize)]
#[serde(untagged)]
pub(in crate::llm::text::openai::request::completions) enum ToolChoiceInput {
    Name(ChoiceName),
    Function(FunctionChoice),
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::llm::text::openai::request::completions) enum ChoiceName {
    Auto,
    None,
    Required,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(in crate::llm::text::openai::request::completions) struct FunctionChoice {
    #[serde(rename = "type")]
    kind: ToolTypeWire,
    function: FunctionNameInput,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(in crate::llm::text::openai::request::completions) struct FunctionNameInput {
    #[serde(deserialize_with = "deserialize::nonempty")]
    name: String,
}

impl From<ToolChoiceInput> for ToolChoice {
    fn from(value: ToolChoiceInput) -> Self {
        match value {
            ToolChoiceInput::Name(ChoiceName::Auto) => Self::Auto,
            ToolChoiceInput::Name(ChoiceName::None) => Self::None,
            ToolChoiceInput::Name(ChoiceName::Required) => Self::Required,
            ToolChoiceInput::Function(FunctionChoice { kind, function }) => {
                let ToolTypeWire::Function = kind;
                Self::Function(function.name)
            }
        }
    }
}

impl<'de> DeserializeAs<'de, ToolChoice> for ToolChoiceInput {
    fn deserialize_as<D: Deserializer<'de>>(deserializer: D) -> Result<ToolChoice, D::Error> {
        ToolChoiceInput::deserialize(deserializer).map(Into::into)
    }
}

/// Encoded Chat Completions tool choice.
#[derive(Serialize)]
#[serde(untagged)]
pub(in crate::llm::text::openai::request::completions) enum ToolChoiceOutput<'a> {
    Name(&'static str),
    Function {
        #[serde(rename = "type")]
        kind: &'static str,
        function: FunctionNameOutput<'a>,
    },
}

impl<'a> From<&'a ToolChoice> for ToolChoiceOutput<'a> {
    fn from(choice: &'a ToolChoice) -> Self {
        match choice {
            ToolChoice::Auto => Self::Name("auto"),
            ToolChoice::None => Self::Name("none"),
            ToolChoice::Required => Self::Name("required"),
            ToolChoice::Function(name) => Self::Function {
                kind: "function",
                function: FunctionNameOutput { name },
            },
        }
    }
}

#[derive(Serialize)]
pub(in crate::llm::text::openai::request::completions) struct FunctionNameOutput<'a> {
    name: &'a str,
}
