use crate::llm::text::component::*;
use compallm_serde_rules::borrowed;
use serde::{Deserialize, Serialize, Serializer, ser::SerializeMap};
use std::borrow::Cow;

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub(in crate::llm::text::openai::request::responses) enum InputItem<'a> {
    Message {
        role: InputRole,
        #[serde(borrow, deserialize_with = "borrowed::text")]
        content: Cow<'a, str>,
    },
    FunctionCall {
        #[serde(borrow, deserialize_with = "borrowed::text")]
        call_id: Cow<'a, str>,
        #[serde(borrow, deserialize_with = "borrowed::text")]
        name: Cow<'a, str>,
        #[serde(borrow, deserialize_with = "borrowed::text")]
        arguments: Cow<'a, str>,
    },
    FunctionCallOutput {
        #[serde(borrow, deserialize_with = "borrowed::text")]
        call_id: Cow<'a, str>,
        #[serde(borrow, deserialize_with = "borrowed::text")]
        output: Cow<'a, str>,
    },
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::llm::text::openai::request::responses) enum InputRole {
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

impl<'a> From<InputItem<'a>> for Message<Cow<'a, str>> {
    fn from(value: InputItem<'a>) -> Self {
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

pub(in crate::llm::text::openai::request::responses) fn role(value: Role) -> &'static str {
    match value {
        Role::System => "system",
        Role::Developer => "developer",
        Role::User => "user",
        Role::Assistant => "assistant",
        Role::Tool => "tool",
    }
}

/// One Responses input item produced from a conversation message.
pub(in crate::llm::text::openai::request::responses) enum InputItemWire<'a> {
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

pub(in crate::llm::text::openai::request::responses) fn input_item<T: AsRef<str>>(
    message: &Message<T>,
    index: usize,
) -> Result<Option<InputItemWire<'_>>, &'static str> {
    if message.role == Role::Tool {
        let call_id = message
            .tool_call_id
            .as_ref()
            .map(AsRef::as_ref)
            .ok_or("tool message requires a tool call identifier")?;
        let output = match message.content.as_ref() {
            Some(Content::Text(output)) => output.as_ref(),
            _ => return Err("tool message requires text content"),
        };
        return Ok((index == 0).then_some(InputItemWire::FunctionCallOutput { call_id, output }));
    }
    let text = match message.content.as_ref() {
        None | Some(Content::Null) => None,
        Some(Content::Text(text) | Content::Object(ObjectMessage::Text { text })) => {
            Some(text.as_ref())
        }
        Some(Content::Array(_)) => return Err("message content parts are not supported"),
    };
    if index == 0
        && let Some(content) = text
    {
        return Ok(Some(InputItemWire::Message {
            role: role(message.role),
            content,
        }));
    }
    let call_index = index - usize::from(text.is_some());
    if let Some(call) = message
        .tool_calls
        .as_deref()
        .unwrap_or_default()
        .get(call_index)
    {
        return Ok(Some(InputItemWire::FunctionCall {
            call_id: call.id.as_ref(),
            name: call.function.name.as_ref(),
            arguments: call.function.arguments.as_ref(),
        }));
    }
    if index == 0 {
        return Err("message requires content or tool calls");
    }
    Ok(None)
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
