use crate::llm::text::component::*;
use compallm_serde_rules::deserialize;
use serde::{Deserialize, Serialize, Serializer, ser::SerializeSeq};

pub(crate) struct ToolsWire<'a>(pub &'a [ToolDefinition]);

impl Serialize for ToolsWire<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut sequence = serializer.serialize_seq(Some(self.0.len()))?;
        for tool in self.0 {
            sequence.serialize_element(&ToolWire::from(tool))?;
        }
        sequence.end()
    }
}

#[derive(Serialize)]
pub(crate) struct ToolWire<'a> {
    name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
    #[serde(serialize_with = "compallm_serde_rules::serialize::serialize_raw")]
    input_schema: &'a str,
}

impl<'a> From<&'a ToolDefinition> for ToolWire<'a> {
    fn from(tool: &'a ToolDefinition) -> Self {
        Self {
            name: &tool.function.name,
            description: tool.function.description.as_deref(),
            input_schema: tool
                .function
                .parameters
                .as_deref()
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

impl<'a> From<&'a ToolChoice> for Choice<'a> {
    fn from(choice: &'a ToolChoice) -> Self {
        match choice {
            ToolChoice::Auto => Self::Auto,
            ToolChoice::None => Self::None,
            ToolChoice::Required => Self::Any,
            ToolChoice::Function(name) => Self::Tool { name },
        }
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ToolInput {
    #[serde(deserialize_with = "deserialize::nonempty")]
    name: String,
    description: Option<String>,
    #[serde(deserialize_with = "deserialize::schema")]
    input_schema: String,
    #[serde(default)]
    strict: Option<bool>,
}

impl From<ToolInput> for ToolDefinition {
    fn from(value: ToolInput) -> Self {
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
pub(crate) enum ChoiceInput {
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
        name: String,
        #[serde(default)]
        disable_parallel_tool_use: Option<bool>,
    },
}

impl ChoiceInput {
    pub(crate) fn into_components(self) -> (ToolChoice, Option<bool>) {
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
