use crate::llm::text::component::*;
use compallm_serde_rules::deserialize;
use serde::{Deserialize, Deserializer, Serialize, Serializer, ser::SerializeSeq};
use serde_with::DeserializeAs;

/// Wire types for native OpenAI Chat Completions request bodies.

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum RoleWire {
    System,
    Developer,
    User,
    Assistant,
    Tool,
}

impl From<RoleWire> for Role {
    fn from(value: RoleWire) -> Self {
        match value {
            RoleWire::System => Self::System,
            RoleWire::Developer => Self::Developer,
            RoleWire::User => Self::User,
            RoleWire::Assistant => Self::Assistant,
            RoleWire::Tool => Self::Tool,
        }
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

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum ToolTypeWire {
    Function,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub(super) enum ContentInput {
    Text(String),
    Null,
}

impl From<ContentInput> for Content {
    fn from(value: ContentInput) -> Self {
        match value {
            ContentInput::Text(text) => Self::Text(text),
            ContentInput::Null => Self::Null,
        }
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct MessageInput {
    role: RoleWire,
    #[serde(default, deserialize_with = "content")]
    content: Option<Content>,
    #[serde(default)]
    tool_calls: Option<Vec<ToolCallInput>>,
    #[serde(default)]
    tool_call_id: Option<String>,
}

fn content<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<Content>, D::Error> {
    ContentInput::deserialize(deserializer).map(|value| Some(value.into()))
}

impl TryFrom<MessageInput> for Message {
    type Error = &'static str;

    fn try_from(value: MessageInput) -> Result<Self, Self::Error> {
        let MessageInput {
            role,
            content,
            tool_calls,
            tool_call_id,
        } = value;
        let role = Role::from(role);
        let tool_calls: Option<Vec<ToolCall>> =
            tool_calls.map(|calls| calls.into_iter().map(Into::into).collect());
        let calls = tool_calls.as_ref().is_some_and(|calls| !calls.is_empty());
        if role == Role::Tool {
            if tool_call_id.as_ref().is_none_or(String::is_empty) {
                return Err("tool message requires a tool call identifier");
            }
            if !matches!(content, Some(Content::Text(_))) {
                return Err("tool message requires text content");
            }
        } else if tool_call_id.is_some() {
            return Err("tool call identifier requires a tool message");
        }
        if calls && role != Role::Assistant {
            return Err("tool calls require an assistant message");
        }
        if !calls && !matches!(content, Some(Content::Text(_))) {
            return Err("message requires text content");
        }
        Ok(Self {
            role,
            content,
            tool_calls,
            tool_call_id,
            reasoning: None,
            reasoning_content: None,
            reasoning_details: None,
        })
    }
}

impl<'de> DeserializeAs<'de, Message> for MessageInput {
    fn deserialize_as<D: Deserializer<'de>>(deserializer: D) -> Result<Message, D::Error> {
        MessageInput::deserialize(deserializer)?
            .try_into()
            .map_err(serde::de::Error::custom)
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct ToolCallInput {
    #[serde(deserialize_with = "deserialize::nonempty")]
    id: String,
    #[serde(rename = "type")]
    kind: ToolTypeWire,
    function: FunctionCallInput,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct FunctionCallInput {
    #[serde(deserialize_with = "deserialize::nonempty")]
    name: String,
    arguments: String,
}

impl From<ToolCallInput> for ToolCall {
    fn from(value: ToolCallInput) -> Self {
        let ToolTypeWire::Function = value.kind;
        Self {
            id: value.id,
            kind: ToolType::Function,
            function: FunctionCall {
                name: value.function.name,
                arguments: value.function.arguments,
            },
        }
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct ToolInput {
    #[serde(rename = "type")]
    kind: ToolTypeWire,
    function: FunctionInput,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct FunctionInput {
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

pub(super) struct ToolsWire;

impl<'de> DeserializeAs<'de, Vec<ToolDefinition>> for ToolsWire {
    fn deserialize_as<D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Vec<ToolDefinition>, D::Error> {
        Vec::<ToolInput>::deserialize(deserializer)
            .map(|tools| tools.into_iter().map(Into::into).collect())
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct FormatInput {
    #[serde(rename = "type")]
    kind: FormatKind,
    #[serde(default)]
    json_schema: Option<JsonSchemaInput>,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum FormatKind {
    Text,
    JsonObject,
    JsonSchema,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct JsonSchemaInput {
    #[serde(deserialize_with = "deserialize::nonempty")]
    name: String,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    strict: Option<bool>,
    #[serde(deserialize_with = "deserialize::schema")]
    schema: String,
}

impl TryFrom<FormatInput> for ResponseFormat {
    type Error = &'static str;

    fn try_from(value: FormatInput) -> Result<Self, Self::Error> {
        match (value.kind, value.json_schema) {
            (FormatKind::Text, None) => Ok(Self::Text),
            (FormatKind::JsonObject, None) => Ok(Self::JsonObject),
            (FormatKind::JsonSchema, Some(schema)) => Ok(Self::JsonSchema(JsonSchema {
                name: schema.name,
                description: schema.description,
                strict: schema.strict,
                schema: schema.schema,
            })),
            _ => Err("invalid response format"),
        }
    }
}

impl<'de> DeserializeAs<'de, ResponseFormat> for FormatInput {
    fn deserialize_as<D: Deserializer<'de>>(deserializer: D) -> Result<ResponseFormat, D::Error> {
        FormatInput::deserialize(deserializer)?
            .try_into()
            .map_err(serde::de::Error::custom)
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct StreamOptionsInput {
    #[serde(default)]
    include_usage: Option<bool>,
}

impl From<StreamOptionsInput> for StreamOptions {
    fn from(value: StreamOptionsInput) -> Self {
        Self {
            include_usage: value.include_usage,
        }
    }
}

impl<'de> DeserializeAs<'de, StreamOptions> for StreamOptionsInput {
    fn deserialize_as<D: Deserializer<'de>>(deserializer: D) -> Result<StreamOptions, D::Error> {
        StreamOptionsInput::deserialize(deserializer).map(Into::into)
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
    kind: ToolTypeWire,
    function: FunctionNameInput,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct FunctionNameInput {
    #[serde(deserialize_with = "deserialize::nonempty")]
    name: String,
}

impl From<ToolChoiceInput> for ToolChoice {
    fn from(value: ToolChoiceInput) -> Self {
        match value {
            ToolChoiceInput::Name(ChoiceName::Auto) => Self::Auto,
            ToolChoiceInput::Name(ChoiceName::None) => Self::None,
            ToolChoiceInput::Name(ChoiceName::Required) => Self::Required,
            ToolChoiceInput::Function(FunctionChoice { kind, function }) => {
                let ToolTypeWire::Function = kind;
                Self::Function(function.name)
            }
        }
    }
}

impl<'de> DeserializeAs<'de, ToolChoice> for ToolChoiceInput {
    fn deserialize_as<D: Deserializer<'de>>(deserializer: D) -> Result<ToolChoice, D::Error> {
        ToolChoiceInput::deserialize(deserializer).map(Into::into)
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
pub(super) enum StopInput {
    Null,
    Text(String),
    Array(Vec<String>),
}

impl TryFrom<StopInput> for Stop {
    type Error = &'static str;

    fn try_from(value: StopInput) -> Result<Self, Self::Error> {
        let valid = match &value {
            StopInput::Null => true,
            StopInput::Text(text) => !text.is_empty() && text.len() <= 4096,
            StopInput::Array(values) => {
                values.len() <= 4
                    && values.iter().all(|value| !value.is_empty())
                    && values.iter().map(String::len).sum::<usize>() <= 4096
            }
        };
        if !valid {
            return Err("invalid stop sequences");
        }
        Ok(match value {
            StopInput::Null => Self::Null,
            StopInput::Text(text) => Self::Text(text),
            StopInput::Array(values) => Self::Array(values),
        })
    }
}

impl<'de> DeserializeAs<'de, Stop> for StopInput {
    fn deserialize_as<D: Deserializer<'de>>(deserializer: D) -> Result<Stop, D::Error> {
        StopInput::deserialize(deserializer)?
            .try_into()
            .map_err(serde::de::Error::custom)
    }
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

pub(super) fn effort(value: ReasoningEffort) -> Option<&'static str> {
    match value {
        ReasoningEffort::Minimal => Some("minimal"),
        ReasoningEffort::Low => Some("low"),
        ReasoningEffort::Medium => Some("medium"),
        ReasoningEffort::High => Some("high"),
        _ => None,
    }
}

/// Encoded Chat Completions message.
#[derive(Serialize)]
pub(super) struct MessageOutput<'a> {
    role: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<ContentOutput<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_calls: Option<Vec<ToolCallOutput<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_call_id: Option<&'a str>,
}

impl<'a> From<&'a Message> for MessageOutput<'a> {
    fn from(message: &'a Message) -> Self {
        Self {
            role: role(message.role),
            content: message.content.as_ref().map(ContentOutput),
            tool_calls: message
                .tool_calls
                .as_ref()
                .map(|calls| calls.iter().map(ToolCallOutput::from).collect()),
            tool_call_id: message.tool_call_id.as_deref(),
        }
    }
}

pub(super) struct ContentOutput<'a>(&'a Content);

impl Serialize for ContentOutput<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self.0 {
            Content::Text(text) => serializer.serialize_str(text),
            Content::Null => serializer.serialize_none(),
            Content::Object(_) | Content::Array(_) => {
                Err(serde::ser::Error::custom("unsupported message content"))
            }
        }
    }
}

#[derive(Serialize)]
pub(super) struct ToolCallOutput<'a> {
    id: &'a str,
    #[serde(rename = "type")]
    kind: &'static str,
    function: FunctionCallOutput<'a>,
}

impl<'a> From<&'a ToolCall> for ToolCallOutput<'a> {
    fn from(call: &'a ToolCall) -> Self {
        Self {
            id: &call.id,
            kind: "function",
            function: FunctionCallOutput {
                name: &call.function.name,
                arguments: &call.function.arguments,
            },
        }
    }
}

#[derive(Serialize)]
pub(super) struct FunctionCallOutput<'a> {
    name: &'a str,
    arguments: &'a str,
}

/// Encoded Chat Completions tool declarations.
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
pub(super) struct FunctionOutput<'a> {
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

/// Encoded Chat Completions response format.
#[derive(Serialize)]
#[serde(tag = "type", content = "json_schema", rename_all = "snake_case")]
pub(super) enum ResponseFormatOutput<'a> {
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

impl<'a> From<&'a ResponseFormat> for ResponseFormatOutput<'a> {
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

/// Encoded Chat Completions tool choice.
#[derive(Serialize)]
#[serde(untagged)]
pub(super) enum ToolChoiceOutput<'a> {
    Name(&'static str),
    Function {
        #[serde(rename = "type")]
        kind: &'static str,
        function: FunctionNameOutput<'a>,
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
                function: FunctionNameOutput { name },
            },
        }
    }
}

#[derive(Serialize)]
pub(super) struct FunctionNameOutput<'a> {
    name: &'a str,
}

/// Encoded Chat Completions streaming options.
#[derive(Serialize)]
pub(super) struct StreamOptionsOutput {
    #[serde(skip_serializing_if = "Option::is_none")]
    include_usage: Option<bool>,
}

impl From<&StreamOptions> for StreamOptionsOutput {
    fn from(options: &StreamOptions) -> Self {
        Self {
            include_usage: options.include_usage,
        }
    }
}

/// Encoded Chat Completions stop sequences.
#[derive(Serialize)]
#[serde(untagged)]
pub(super) enum StopOutput<'a> {
    Null,
    Text(&'a str),
    Array(&'a [String]),
}

impl<'a> From<&'a Stop> for StopOutput<'a> {
    fn from(stop: &'a Stop) -> Self {
        match stop {
            Stop::Null => Self::Null,
            Stop::Text(text) => Self::Text(text),
            Stop::Array(values) => Self::Array(values),
        }
    }
}
