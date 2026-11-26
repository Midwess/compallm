use crate::llm::text::component::*;
use compallm_serde_rules::borrowed;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub(crate) struct ToolWire<'a, S: AsRef<str>> {
    name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
    input_schema: InputSchema<'a, S>,
}

enum InputSchema<'a, S> {
    Supplied(&'a crate::json::JsonValue<S>),
    Default,
}

impl<S: AsRef<str>> Serialize for InputSchema<'_, S> {
    fn serialize<T: serde::Serializer>(&self, serializer: T) -> Result<T::Ok, T::Error> {
        match self {
            Self::Supplied(value) => value.serialize(serializer),
            Self::Default => {
                use serde::ser::SerializeMap;
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("type", "object")?;
                map.end()
            }
        }
    }
}

impl<'a, S: AsRef<str>> From<&'a ToolDefinition<S>> for ToolWire<'a, S> {
    fn from(tool: &'a ToolDefinition<S>) -> Self {
        Self {
            name: tool.function.name.as_ref(),
            description: tool.function.description.as_ref().map(AsRef::as_ref),
            input_schema: tool
                .function
                .parameters
                .as_ref()
                .map(InputSchema::Supplied)
                .unwrap_or(InputSchema::Default),
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
    name: &'a str,
    #[serde(default, borrow, deserialize_with = "borrowed::optional_text")]
    description: Option<&'a str>,
    #[serde(borrow)]
    input_schema: crate::json::JsonValue<&'a str>,
    #[serde(default)]
    strict: Option<bool>,
}

impl<'a> From<ToolInput<'a>> for ToolDefinition<&'a str> {
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
        name: &'a str,
        #[serde(default)]
        disable_parallel_tool_use: Option<bool>,
    },
}

impl<'a> ChoiceInput<'a> {
    pub(crate) fn into_components(self) -> (ToolChoice<&'a str>, Option<bool>) {
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
