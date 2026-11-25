use crate::llm::text::component::*;
use compallm_serde_rules::deserialize;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(in crate::llm::text::openai::request::responses) struct Tool {
    #[serde(rename = "type")]
    kind: FunctionType,
    #[serde(deserialize_with = "deserialize::nonempty")]
    name: String,
    description: Option<String>,
    #[serde(default, deserialize_with = "deserialize::optional_schema")]
    parameters: Option<String>,
    strict: Option<bool>,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::llm::text::openai::request::responses) enum FunctionType {
    Function,
}

impl From<Tool> for ToolDefinition {
    fn from(value: Tool) -> Self {
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
pub(in crate::llm::text::openai::request::responses) struct ToolOutput<'a> {
    #[serde(rename = "type")]
    kind: &'static str,
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

impl<'a> From<&'a ToolDefinition> for ToolOutput<'a> {
    fn from(tool: &'a ToolDefinition) -> Self {
        Self {
            kind: "function",
            name: &tool.function.name,
            description: tool.function.description.as_deref(),
            parameters: tool.function.parameters.as_deref(),
            strict: tool.function.strict,
        }
    }
}
