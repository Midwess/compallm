use super::tools::ToolTypeWire;
use crate::llm::text::component::*;
use compallm_serde_rules::deserialize;
use serde::{Deserialize, Deserializer, Serialize, Serializer, ser::SerializeSeq};
use serde_with::DeserializeAs;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::llm::text::openai::request::completions) enum RoleWire {
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

pub(in crate::llm::text::openai::request::completions) fn role(value: Role) -> &'static str {
    match value {
        Role::System => "system",
        Role::Developer => "developer",
        Role::User => "user",
        Role::Assistant => "assistant",
        Role::Tool => "tool",
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
pub(in crate::llm::text::openai::request::completions) enum ContentInput {
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
pub(in crate::llm::text::openai::request::completions) struct MessageInput {
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
pub(in crate::llm::text::openai::request::completions) struct ToolCallInput {
    #[serde(deserialize_with = "deserialize::nonempty")]
    id: String,
    #[serde(rename = "type")]
    kind: ToolTypeWire,
    function: FunctionCallInput,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(in crate::llm::text::openai::request::completions) struct FunctionCallInput {
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

/// Encoded Chat Completions message.
#[derive(Serialize)]
pub(in crate::llm::text::openai::request::completions) struct MessageOutput<'a> {
    role: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<ContentOutput<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_calls: Option<ToolCallsOutput<'a>>,
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
                .map(|calls| ToolCallsOutput(calls)),
            tool_call_id: message.tool_call_id.as_deref(),
        }
    }
}

pub(in crate::llm::text::openai::request::completions) struct ContentOutput<'a>(&'a Content);

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

struct ToolCallsOutput<'a>(&'a [ToolCall]);

impl Serialize for ToolCallsOutput<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut sequence = serializer.serialize_seq(Some(self.0.len()))?;
        for call in self.0 {
            sequence.serialize_element(&ToolCallOutput::from(call))?;
        }
        sequence.end()
    }
}

#[derive(Serialize)]
pub(in crate::llm::text::openai::request::completions) struct ToolCallOutput<'a> {
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
pub(in crate::llm::text::openai::request::completions) struct FunctionCallOutput<'a> {
    name: &'a str,
    arguments: &'a str,
}
