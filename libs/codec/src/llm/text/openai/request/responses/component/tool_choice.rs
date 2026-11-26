use super::tools::FunctionType;
use crate::llm::text::component::*;
use compallm_serde_rules::borrowed;
use serde::{Deserialize, Serialize};
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(in crate::llm::text::openai::request::responses) struct FunctionChoice<'a> {
    #[serde(rename = "type")]
    kind: FunctionType,
    #[serde(borrow, deserialize_with = "borrowed::nonempty")]
    name: &'a str,
}
impl<'a> From<FunctionChoice<'a>> for ToolChoice<&'a str> {
    fn from(v: FunctionChoice<'a>) -> Self {
        let FunctionType::Function = v.kind;
        Self::Function(v.name)
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

impl<'a, T: AsRef<str>> From<&'a ToolChoice<T>> for ToolChoiceOutput<'a> {
    fn from(choice: &'a ToolChoice<T>) -> Self {
        match choice {
            ToolChoice::Auto => Self::Name("auto"),
            ToolChoice::None => Self::Name("none"),
            ToolChoice::Required => Self::Name("required"),
            ToolChoice::Function(name) => Self::Function {
                kind: "function",
                name: name.as_ref(),
            },
        }
    }
}
