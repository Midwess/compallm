use crate::llm::text::component::*;
use compallm_serde_rules::deserialize;
use serde::{Deserialize, Serialize, Serializer, ser::SerializeMap, ser::SerializeSeq};

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub(super) enum InputItem {
    Message {
        role: InputRole,
        content: String,
    },
    FunctionCall {
        call_id: String,
        name: String,
        arguments: String,
    },
    FunctionCallOutput {
        call_id: String,
        output: String,
    },
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum InputRole {
    System,
    Developer,
    User,
    Assistant,
}

impl From<InputRole> for Role {
    fn from(value: InputRole) -> Self {
        match value {
            InputRole::System => Self::System,
            InputRole::Developer => Self::Developer,
            InputRole::User => Self::User,
            InputRole::Assistant => Self::Assistant,
        }
    }
}

impl From<InputItem> for Message {
    fn from(value: InputItem) -> Self {
        let mut message = Self {
            role: Role::User,
            content: None,
            tool_calls: None,
            tool_call_id: None,
            reasoning: None,
            reasoning_content: None,
            reasoning_details: None,
        };
        match value {
            InputItem::Message { role, content } => {
                message.role = role.into();
                message.content = Some(Content::Text(content));
            }
            InputItem::FunctionCall {
                call_id,
                name,
                arguments,
            } => {
                message.role = Role::Assistant;
                message.tool_calls = Some(vec![ToolCall {
                    id: call_id,
                    kind: ToolType::Function,
                    function: FunctionCall { name, arguments },
                }]);
            }
            InputItem::FunctionCallOutput { call_id, output } => {
                message.role = Role::Tool;
                message.tool_call_id = Some(call_id);
                message.content = Some(Content::Text(output));
            }
        }
        message
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Tool {
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
pub(super) enum FunctionType {
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

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Text {
    pub format: FormatInput,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct FormatInput {
    #[serde(rename = "type")]
    kind: FormatKind,
    #[serde(default, deserialize_with = "deserialize::optional_schema")]
    schema: Option<String>,
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    strict: Option<bool>,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum FormatKind {
    Text,
    JsonObject,
    JsonSchema,
}

impl TryFrom<FormatInput> for ResponseFormat {
    type Error = &'static str;

    fn try_from(value: FormatInput) -> Result<Self, Self::Error> {
        let FormatInput {
            kind,
            schema,
            name,
            description,
            strict,
        } = value;
        match kind {
            FormatKind::Text | FormatKind::JsonObject => {
                if name.is_some() || description.is_some() || strict.is_some() || schema.is_some() {
                    return Err("invalid response format");
                }
                Ok(match kind {
                    FormatKind::Text => Self::Text,
                    _ => Self::JsonObject,
                })
            }
            FormatKind::JsonSchema => {
                let name = name.filter(|name| !name.is_empty());
                let schema = schema.ok_or("invalid response format")?;
                Ok(Self::JsonSchema(JsonSchema {
                    name: name.ok_or("invalid response format")?,
                    description,
                    strict,
                    schema,
                }))
            }
        }
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
pub(super) enum ToolChoiceInput {
    Name(ChoiceName),
    Function(FunctionChoice),
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum ChoiceName {
    Auto,
    None,
    Required,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct FunctionChoice {
    #[serde(rename = "type")]
    kind: FunctionType,
    #[serde(deserialize_with = "deserialize::nonempty")]
    name: String,
}

impl From<ToolChoiceInput> for ToolChoice {
    fn from(value: ToolChoiceInput) -> Self {
        match value {
            ToolChoiceInput::Name(ChoiceName::Auto) => Self::Auto,
            ToolChoiceInput::Name(ChoiceName::None) => Self::None,
            ToolChoiceInput::Name(ChoiceName::Required) => Self::Required,
            ToolChoiceInput::Function(FunctionChoice { kind, name }) => {
                let FunctionType::Function = kind;
                Self::Function(name)
            }
        }
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct ReasoningInput {
    #[serde(default)]
    effort: Option<EffortInput>,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum EffortInput {
    Minimal,
    Low,
    Medium,
    High,
}

impl From<EffortInput> for ReasoningEffort {
    fn from(value: EffortInput) -> Self {
        match value {
            EffortInput::Minimal => Self::Minimal,
            EffortInput::Low => Self::Low,
            EffortInput::Medium => Self::Medium,
            EffortInput::High => Self::High,
        }
    }
}

impl From<ReasoningInput> for Reasoning {
    fn from(value: ReasoningInput) -> Self {
        Self {
            effort: value.effort.map(Into::into),
            ..Self::default()
        }
    }
}

pub(super) fn effort(value: ReasoningEffort) -> Option<&'static str> {
    match value {
        ReasoningEffort::Minimal => Some("minimal"),
        ReasoningEffort::Low => Some("low"),
        ReasoningEffort::Medium => Some("medium"),
        ReasoningEffort::High => Some("high"),
        _ => None,
    }
}

pub(super) fn role(value: Role) -> &'static str {
    match value {
        Role::System => "system",
        Role::Developer => "developer",
        Role::User => "user",
        Role::Assistant => "assistant",
        Role::Tool => "tool",
    }
}

/// One Responses input item produced from a conversation message.
pub(super) enum InputItemWire<'a> {
    Message {
        role: &'static str,
        content: &'a str,
    },
    FunctionCall {
        call_id: &'a str,
        name: &'a str,
        arguments: &'a str,
    },
    FunctionCallOutput {
        call_id: &'a str,
        output: &'a str,
    },
}

/// Builds the Responses input items that represent one conversation message.
pub(super) fn input_items(message: &Message) -> Result<Vec<InputItemWire<'_>>, &'static str> {
    let mut items = Vec::new();
    if message.role == Role::Tool {
        let call_id = message
            .tool_call_id
            .as_deref()
            .ok_or("tool message requires a tool call identifier")?;
        let output = match message.content.as_ref() {
            Some(Content::Text(output)) => output.as_str(),
            _ => return Err("tool message requires text content"),
        };
        items.push(InputItemWire::FunctionCallOutput { call_id, output });
        return Ok(items);
    }
    if let Some(content) = message.content.as_ref() {
        match content {
            Content::Null => {}
            Content::Text(text) => items.push(InputItemWire::Message {
                role: role(message.role),
                content: text,
            }),
            Content::Object(ObjectMessage::Text { text }) => {
                items.push(InputItemWire::Message {
                    role: role(message.role),
                    content: text,
                });
            }
            Content::Array(_) => return Err("message content parts are not supported"),
        }
    }
    for call in message.tool_calls.iter().flatten() {
        items.push(InputItemWire::FunctionCall {
            call_id: &call.id,
            name: &call.function.name,
            arguments: &call.function.arguments,
        });
    }
    if items.is_empty() {
        return Err("message requires content or tool calls");
    }
    Ok(items)
}

impl Serialize for InputItemWire<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(None)?;
        match self {
            Self::Message { role, content } => {
                map.serialize_entry("type", "message")?;
                map.serialize_entry("role", role)?;
                map.serialize_entry("content", content)?;
            }
            Self::FunctionCall {
                call_id,
                name,
                arguments,
            } => {
                map.serialize_entry("type", "function_call")?;
                map.serialize_entry("call_id", call_id)?;
                map.serialize_entry("name", name)?;
                map.serialize_entry("arguments", arguments)?;
            }
            Self::FunctionCallOutput { call_id, output } => {
                map.serialize_entry("type", "function_call_output")?;
                map.serialize_entry("call_id", call_id)?;
                map.serialize_entry("output", output)?;
            }
        }
        map.end()
    }
}

/// Encoded Responses tool declarations.
pub(super) struct ToolsOutput<'a>(pub &'a [ToolDefinition]);

impl Serialize for ToolsOutput<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut sequence = serializer.serialize_seq(Some(self.0.len()))?;
        for tool in self.0 {
            sequence.serialize_element(&ToolOutput::from(tool))?;
        }
        sequence.end()
    }
}

#[derive(Serialize)]
pub(super) struct ToolOutput<'a> {
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

/// Encoded Responses text configuration.
#[derive(Serialize)]
pub(super) struct TextOutput<'a> {
    pub(super) format: FormatOutput<'a>,
}

/// Encoded Responses response format.
#[derive(Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(super) enum FormatOutput<'a> {
    Text,
    JsonObject,
    JsonSchema {
        name: &'a str,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<&'a str>,
        #[serde(skip_serializing_if = "Option::is_none")]
        strict: Option<bool>,
        #[serde(serialize_with = "compallm_serde_rules::serialize::serialize_raw")]
        schema: &'a str,
    },
}

impl<'a> From<&'a ResponseFormat> for FormatOutput<'a> {
    fn from(format: &'a ResponseFormat) -> Self {
        match format {
            ResponseFormat::Text => Self::Text,
            ResponseFormat::JsonObject => Self::JsonObject,
            ResponseFormat::JsonSchema(schema) => Self::JsonSchema {
                name: &schema.name,
                description: schema.description.as_deref(),
                strict: schema.strict,
                schema: &schema.schema,
            },
        }
    }
}

/// Encoded Responses tool choice.
#[derive(Serialize)]
#[serde(untagged)]
pub(super) enum ToolChoiceOutput<'a> {
    Name(&'static str),
    Function {
        #[serde(rename = "type")]
        kind: &'static str,
        name: &'a str,
    },
}

impl<'a> From<&'a ToolChoice> for ToolChoiceOutput<'a> {
    fn from(choice: &'a ToolChoice) -> Self {
        match choice {
            ToolChoice::Auto => Self::Name("auto"),
            ToolChoice::None => Self::Name("none"),
            ToolChoice::Required => Self::Name("required"),
            ToolChoice::Function(name) => Self::Function {
                kind: "function",
                name,
            },
        }
    }
}

/// Encoded Responses reasoning configuration.
#[derive(Serialize)]
pub(super) struct ReasoningOutput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) effort: Option<&'static str>,
}
