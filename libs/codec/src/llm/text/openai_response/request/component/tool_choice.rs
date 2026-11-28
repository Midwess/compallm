use super::tools::ToolTypeWire;
use crate::llm::text::component::{ToolChoice, ToolType};
use compallm_serde_rules::borrowed;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[serde(untagged)]
pub(in crate::llm::text::openai_response::request) enum ChoiceOutput<'a> {
    Name(&'static str),
    Function {
        #[serde(rename = "type", with = "ToolTypeWire")]
        kind: ToolType,
        name: &'a str,
    },
}
impl<'a, T: AsRef<str>> From<&'a ToolChoice<T>> for ChoiceOutput<'a> {
    fn from(v: &'a ToolChoice<T>) -> Self {
        match v {
            ToolChoice::Auto => Self::Name("auto"),
            ToolChoice::None => Self::Name("none"),
            ToolChoice::Required => Self::Name("required"),
            ToolChoice::Function(name) => Self::Function {
                kind: ToolType::Function,
                name: name.as_ref(),
            },
        }
    }
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(in crate::llm::text::openai_response::request) struct FunctionChoice<'a> {
    #[serde(rename = "type", with = "ToolTypeWire")]
    kind: ToolType,
    #[serde(borrow, deserialize_with = "borrowed::nonempty")]
    name: &'a str,
}
impl<'a> From<FunctionChoice<'a>> for ToolChoice<&'a str> {
    fn from(v: FunctionChoice<'a>) -> Self {
        let ToolType::Function = v.kind;
        Self::Function(v.name)
    }
}
