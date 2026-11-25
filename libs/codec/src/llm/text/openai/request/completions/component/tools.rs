use crate::llm::text::component::*;
use serde::Serialize;

#[derive(Serialize)]
pub(in crate::llm::text::openai::request::completions) struct ToolOutput<'a> {
    #[serde(rename = "type")]
    kind: &'static str,
    function: FunctionOutput<'a>,
}

impl<'a, S: AsRef<str>> From<&'a ToolDefinition<S>> for ToolOutput<'a> {
    fn from(tool: &'a ToolDefinition<S>) -> Self {
        Self {
            kind: "function",
            function: FunctionOutput {
                name: tool.function.name.as_ref(),
                description: tool.function.description.as_ref().map(AsRef::as_ref),
                parameters: tool.function.parameters.as_ref().map(AsRef::as_ref),
                strict: tool.function.strict,
            },
        }
    }
}

#[derive(Serialize)]
pub(in crate::llm::text::openai::request::completions) struct FunctionOutput<'a> {
    name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
    #[serde(
        serialize_with = "compallm_serde_rules::serialize::optional_raw",
        skip_serializing_if = "Option::is_none"
    )]
    parameters: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    strict: Option<bool>,
}
