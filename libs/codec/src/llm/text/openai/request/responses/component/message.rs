use crate::llm::text::component::*;
use compallm_serde_rules::borrowed;
use serde::{Deserialize, Serialize, Serializer, ser::SerializeMap};

pub(in crate::llm::text::openai::request::responses) enum InputItem<'a> {
    Message {
        role: InputRole,
        content: &'a str,
    },
    FunctionCall {
        call_id: &'a str,
        name: &'a str,
        arguments: crate::json::JsonArgument<&'a str>,
    },
    FunctionCallOutput {
        call_id: &'a str,
        output: &'a str,
    },
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct InputFields<'a> {
    #[serde(rename = "type")]
    kind: InputKind,
    #[serde(
        default,
        deserialize_with = "compallm_serde_rules::deserialize::present"
    )]
    role: Option<InputRole>,
    #[serde(default, borrow, deserialize_with = "borrowed::present_text")]
    content: Option<&'a str>,
    #[serde(default, borrow, deserialize_with = "borrowed::present_text")]
    call_id: Option<&'a str>,
    #[serde(default, borrow, deserialize_with = "borrowed::present_text")]
    name: Option<&'a str>,
    #[serde(
        default,
        borrow,
        deserialize_with = "compallm_serde_rules::deserialize::present"
    )]
    arguments: Option<crate::json::JsonArgument<&'a str>>,
    #[serde(default, borrow, deserialize_with = "borrowed::present_text")]
    output: Option<&'a str>,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum InputKind {
    Message,
    FunctionCall,
    FunctionCallOutput,
}

impl<'a> TryFrom<InputFields<'a>> for InputItem<'a> {
    type Error = &'static str;

    fn try_from(value: InputFields<'a>) -> Result<Self, Self::Error> {
        let InputFields {
            kind,
            role,
            content,
            call_id,
            name,
            arguments,
            output,
        } = value;
        match (kind, role, content, call_id, name, arguments, output) {
            (InputKind::Message, Some(role), Some(content), None, None, None, None) => {
                Ok(Self::Message { role, content })
            }
            (
                InputKind::FunctionCall,
                None,
                None,
                Some(call_id),
                Some(name),
                Some(arguments),
                None,
            ) => Ok(Self::FunctionCall {
                call_id,
                name,
                arguments,
            }),
            (
                InputKind::FunctionCallOutput,
                None,
                None,
                Some(call_id),
                None,
                None,
                Some(output),
            ) => Ok(Self::FunctionCallOutput { call_id, output }),
            _ => Err("invalid Responses input item"),
        }
    }
}

impl<'de: 'a, 'a> Deserialize<'de> for InputItem<'a> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        InputFields::deserialize(deserializer)?
            .try_into()
            .map_err(serde::de::Error::custom)
    }
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

impl<'a> From<InputItem<'a>> for Message<&'a str> {
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

impl InputItemWire<'_> {
    fn role(value: Role) -> &'static str {
        match value {
            Role::System => "system",
            Role::Developer => "developer",
            Role::User => "user",
            Role::Assistant => "assistant",
            Role::Tool => "tool",
        }
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

impl<'a> InputItemWire<'a> {
    pub(in crate::llm::text::openai::request::responses) fn from_message<T: AsRef<str>>(
        message: &'a Message<T>,
        index: usize,
    ) -> Result<Option<Self>, &'static str> {
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
            return Ok(
                (index == 0).then_some(InputItemWire::FunctionCallOutput { call_id, output })
            );
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
                role: Self::role(message.role),
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
