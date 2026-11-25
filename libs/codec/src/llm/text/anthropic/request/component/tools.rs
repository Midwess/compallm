use crate::llm::text::component::*;
use compallm_serde_rules::borrowed;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Serialize)]
pub(crate) struct ToolWire<'a> {
    name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
    #[serde(serialize_with = "compallm_serde_rules::serialize::serialize_raw")]
    input_schema: &'a str,
}

impl<'a, S: AsRef<str>> From<&'a ToolDefinition<S>> for ToolWire<'a> {
    fn from(tool: &'a ToolDefinition<S>) -> Self {
        Self {
            name: tool.function.name.as_ref(),
            description: tool.function.description.as_ref().map(AsRef::as_ref),
            input_schema: tool
                .function
                .parameters
                .as_ref()
                .map(AsRef::as_ref)
                .unwrap_or(r#"{"type":"object"}"#),
        }
    }
}

#[derive(Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum Choice<'a> {
    Auto,
    None,
    Any,
    Tool { name: &'a str },
}

impl<'a, S: AsRef<str>> From<&'a ToolChoice<S>> for Choice<'a> {
    fn from(choice: &'a ToolChoice<S>) -> Self {
        match choice {
            ToolChoice::Auto => Self::Auto,
            ToolChoice::None => Self::None,
            ToolChoice::Required => Self::Any,
            ToolChoice::Function(name) => Self::Tool {
                name: name.as_ref(),
            },
        }
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ToolInput<'a> {
    #[serde(borrow, deserialize_with = "borrowed::nonempty")]
    name: Cow<'a, str>,
    #[serde(default, borrow, deserialize_with = "borrowed::optional_text")]
    description: Option<Cow<'a, str>>,
    #[serde(borrow, deserialize_with = "borrowed::schema")]
    input_schema: Cow<'a, str>,
    #[serde(default)]
    strict: Option<bool>,
}

impl<'a> From<ToolInput<'a>> for ToolDefinition<Cow<'a, str>> {
    fn from(value: ToolInput<'a>) -> Self {
        Self {
            kind: ToolType::Function,
            function: FunctionDefinition {
                name: value.name,
                description: value.description,
                parameters: Some(value.input_schema),
                strict: value.strict,
            },
        }
    }
}

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub(crate) enum ChoiceInput<'a> {
    Auto {
        #[serde(default)]
        disable_parallel_tool_use: Option<bool>,
    },
    None,
    Any {
        #[serde(default)]
        disable_parallel_tool_use: Option<bool>,
    },
    Tool {
        #[serde(borrow, deserialize_with = "borrowed::text")]
        name: Cow<'a, str>,
        #[serde(default)]
        disable_parallel_tool_use: Option<bool>,
    },
}

impl<'a> ChoiceInput<'a> {
    pub(crate) fn into_components(self) -> (ToolChoice<Cow<'a, str>>, Option<bool>) {
        match self {
            Self::Auto {
                disable_parallel_tool_use,
            } => (ToolChoice::Auto, disable_parallel_tool_use.map(|v| !v)),
            Self::None => (ToolChoice::None, None),
            Self::Any {
                disable_parallel_tool_use,
            } => (ToolChoice::Required, disable_parallel_tool_use.map(|v| !v)),
            Self::Tool {
                name,
                disable_parallel_tool_use,
            } => (
                ToolChoice::Function(name),
                disable_parallel_tool_use.map(|v| !v),
            ),
        }
    }
}
