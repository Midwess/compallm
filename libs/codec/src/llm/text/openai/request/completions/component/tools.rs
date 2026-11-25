use crate::llm::text::component::*;
use compallm_serde_rules::deserialize;
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::DeserializeAs;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::llm::text::openai::request::completions) enum ToolTypeWire {
    Function,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(in crate::llm::text::openai::request::completions) struct ToolInput {
    #[serde(rename = "type")]
    kind: ToolTypeWire,
    function: FunctionInput,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(in crate::llm::text::openai::request::completions) struct FunctionInput {
    #[serde(deserialize_with = "deserialize::nonempty")]
    name: String,
    #[serde(default)]
    description: Option<String>,
    #[serde(default, deserialize_with = "deserialize::optional_schema")]
    parameters: Option<String>,
    #[serde(default)]
    strict: Option<bool>,
}

impl From<ToolInput> for ToolDefinition {
    fn from(value: ToolInput) -> Self {
        let ToolTypeWire::Function = value.kind;
        Self {
            kind: ToolType::Function,
            function: FunctionDefinition {
                name: value.function.name,
                description: value.function.description,
                parameters: value.function.parameters,
                strict: value.function.strict,
            },
        }
    }
}

pub(in crate::llm::text::openai::request::completions) struct ToolsWire;

impl<'de> DeserializeAs<'de, ToolDefinition> for ToolsWire {
    fn deserialize_as<D: Deserializer<'de>>(deserializer: D) -> Result<ToolDefinition, D::Error> {
        ToolInput::deserialize(deserializer).map(Into::into)
    }
}

#[derive(Serialize)]
pub(in crate::llm::text::openai::request::completions) struct ToolOutput<'a> {
    #[serde(rename = "type")]
    kind: &'static str,
    function: FunctionOutput<'a>,
}

impl<'a> From<&'a ToolDefinition> for ToolOutput<'a> {
    fn from(tool: &'a ToolDefinition) -> Self {
        Self {
            kind: "function",
            function: FunctionOutput {
                name: &tool.function.name,
                description: tool.function.description.as_deref(),
                parameters: tool.function.parameters.as_deref(),
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
