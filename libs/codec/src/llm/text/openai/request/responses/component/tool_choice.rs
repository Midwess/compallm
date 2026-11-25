use super::tools::FunctionType;
use crate::llm::text::component::*;
use compallm_serde_rules::deserialize;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(untagged)]
pub(in crate::llm::text::openai::request::responses) enum ToolChoiceInput {
    Name(ChoiceName),
    Function(FunctionChoice),
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::llm::text::openai::request::responses) enum ChoiceName {
    Auto,
    None,
    Required,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(in crate::llm::text::openai::request::responses) struct FunctionChoice {
    #[serde(rename = "type")]
    kind: FunctionType,
    #[serde(deserialize_with = "deserialize::nonempty")]
    name: String,
}

impl From<ToolChoiceInput> for ToolChoice {
    fn from(value: ToolChoiceInput) -> Self {
        match value {
            ToolChoiceInput::Name(ChoiceName::Auto) => Self::Auto,
            ToolChoiceInput::Name(ChoiceName::None) => Self::None,
            ToolChoiceInput::Name(ChoiceName::Required) => Self::Required,
            ToolChoiceInput::Function(FunctionChoice { kind, name }) => {
                let FunctionType::Function = kind;
                Self::Function(name)
            }
        }
    }
}

/// Encoded Responses tool choice.
#[derive(Serialize)]
#[serde(untagged)]
pub(in crate::llm::text::openai::request::responses) enum ToolChoiceOutput<'a> {
    Name(&'static str),
    Function {
        #[serde(rename = "type")]
        kind: &'static str,
        name: &'a str,
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
                name,
            },
        }
    }
}
