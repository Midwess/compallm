use crate::llm::text::component::*;
use serde::Serialize;

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

impl<'a, T: AsRef<str>> From<&'a ToolChoice<T>> for ToolChoiceOutput<'a> {
    fn from(choice: &'a ToolChoice<T>) -> Self {
        match choice {
            ToolChoice::Auto => Self::Name("auto"),
            ToolChoice::None => Self::Name("none"),
            ToolChoice::Required => Self::Name("required"),
            ToolChoice::Function(name) => Self::Function {
                kind: "function",
                function: FunctionNameOutput {
                    name: name.as_ref(),
                },
            },
        }
    }
}

#[derive(Serialize)]
pub(in crate::llm::text::openai::request::completions) struct FunctionNameOutput<'a> {
    name: &'a str,
}
