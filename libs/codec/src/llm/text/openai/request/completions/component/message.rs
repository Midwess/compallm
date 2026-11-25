use crate::llm::text::component::*;
use serde::{Serialize, Serializer, ser::SerializeSeq};

/// Encoded Chat Completions message.
#[derive(Serialize)]
pub(in crate::llm::text::openai::request::completions) struct MessageOutput<'a, T: AsRef<str>> {
    role: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<ContentOutput<'a, T>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_calls: Option<ToolCallsOutput<'a, T>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_call_id: Option<&'a str>,
}

impl<'a, T: AsRef<str>> From<&'a Message<T>> for MessageOutput<'a, T> {
    fn from(message: &'a Message<T>) -> Self {
        Self {
            role: role(message.role),
            content: message.content.as_ref().map(ContentOutput),
            tool_calls: message
                .tool_calls
                .as_ref()
                .map(|calls| ToolCallsOutput(calls)),
            tool_call_id: message.tool_call_id.as_ref().map(AsRef::as_ref),
        }
    }
}

pub(in crate::llm::text::openai::request::completions) struct ContentOutput<'a, T>(&'a Content<T>);

impl<T: AsRef<str>> Serialize for ContentOutput<'_, T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self.0 {
            Content::Text(text) => serializer.serialize_str(text.as_ref()),
            Content::Null => serializer.serialize_none(),
            Content::Object(_) | Content::Array(_) => {
                Err(serde::ser::Error::custom("unsupported message content"))
            }
        }
    }
}

struct ToolCallsOutput<'a, T>(&'a [ToolCall<T>]);

impl<T: AsRef<str>> Serialize for ToolCallsOutput<'_, T> {
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

impl<'a, T: AsRef<str>> From<&'a ToolCall<T>> for ToolCallOutput<'a> {
    fn from(call: &'a ToolCall<T>) -> Self {
        Self {
            id: call.id.as_ref(),
            kind: "function",
            function: FunctionCallOutput {
                name: call.function.name.as_ref(),
                arguments: call.function.arguments.as_ref(),
            },
        }
    }
}

#[derive(Serialize)]
pub(in crate::llm::text::openai::request::completions) struct FunctionCallOutput<'a> {
    name: &'a str,
    arguments: &'a str,
}

fn role(value: Role) -> &'static str {
    match value {
        Role::System => "system",
        Role::Developer => "developer",
        Role::User => "user",
        Role::Assistant => "assistant",
        Role::Tool => "tool",
    }
}

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(in crate::llm::text::openai::request::completions) struct MessageInput<'a> {
    #[serde(with = "crate::llm::text::copenai::request::component::RoleWire")]
    role: Role,
    #[serde(
        default,
        borrow,
        deserialize_with = "crate::llm::text::copenai::request::component::content"
    )]
    content: Option<Content<std::borrow::Cow<'a, str>>>,
    #[serde(default, borrow)]
    #[serde_as(
        as = "Option<Vec<serde_with::FromInto<crate::llm::text::copenai::request::component::ToolCallInput<'a>>>>"
    )]
    tool_calls: Option<Vec<ToolCall<std::borrow::Cow<'a, str>>>>,
    #[serde(
        default,
        borrow,
        deserialize_with = "compallm_serde_rules::borrowed::optional_text"
    )]
    tool_call_id: Option<std::borrow::Cow<'a, str>>,
}
impl<'a> TryFrom<MessageInput<'a>> for Message<std::borrow::Cow<'a, str>> {
    type Error = &'static str;
    fn try_from(v: MessageInput<'a>) -> Result<Self, Self::Error> {
        let calls = v.tool_calls.as_ref().is_some_and(|v| !v.is_empty());
        if matches!(v.content, Some(Content::Object(_) | Content::Array(_))) {
            return Err("unsupported message content");
        }
        if v.role == Role::Tool {
            if v.tool_call_id.as_ref().is_none_or(|v| v.is_empty())
                || !matches!(v.content, Some(Content::Text(_)))
            {
                return Err("invalid tool message");
            }
        } else if v.tool_call_id.is_some() {
            return Err("tool call identifier requires a tool message");
        }
        if calls && v.role != Role::Assistant {
            return Err("tool calls require an assistant message");
        }
        if !calls && !matches!(v.content, Some(Content::Text(_))) {
            return Err("message requires text content");
        }
        Ok(Self {
            role: v.role,
            content: v.content,
            tool_calls: v.tool_calls,
            tool_call_id: v.tool_call_id,
            reasoning: None,
            reasoning_content: None,
            reasoning_details: None,
        })
    }
}
