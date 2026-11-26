use crate::llm::text::component::*;
use compallm_serde_rules::borrowed;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(in crate::llm::text::openai::request::responses) struct Tool<'a> {
    #[serde(rename = "type")]
    kind: FunctionType,
    #[serde(borrow, deserialize_with = "borrowed::nonempty")]
    name: &'a str,
    #[serde(default, borrow, deserialize_with = "borrowed::optional_text")]
    description: Option<&'a str>,
    #[serde(default, borrow)]
    parameters: Option<crate::json::JsonValue<&'a str>>,
    strict: Option<bool>,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::llm::text::openai::request::responses) enum FunctionType {
    Function,
}

impl<'a> From<Tool<'a>> for ToolDefinition<&'a str> {
    fn from(value: Tool<'a>) -> Self {
        let FunctionType::Function = value.kind;
        Self {
            kind: ToolType::Function,
            function: FunctionDefinition {
                name: value.name,
                description: value.description,
                parameters: value.parameters,
                strict: value.strict,
            },
        }
    }
}

#[derive(Serialize)]
pub(in crate::llm::text::openai::request::responses) struct ToolOutput<'a, S: AsRef<str>> {
    #[serde(rename = "type")]
    kind: &'static str,
    name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<&'a crate::json::JsonValue<S>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    strict: Option<bool>,
}

impl<'a, S: AsRef<str>> From<&'a ToolDefinition<S>> for ToolOutput<'a, S> {
    fn from(tool: &'a ToolDefinition<S>) -> Self {
        Self {
            kind: "function",
            name: tool.function.name.as_ref(),
            description: tool.function.description.as_ref().map(AsRef::as_ref),
            parameters: tool.function.parameters.as_ref(),
            strict: tool.function.strict,
        }
    }
}
