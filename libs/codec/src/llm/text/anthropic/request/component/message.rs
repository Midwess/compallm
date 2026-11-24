use crate::llm::text::component::*;
use serde::{Deserialize, Deserializer, Serialize, Serializer, ser::SerializeSeq};
use serde_with::DeserializeAs;

#[derive(Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum Block<'a> {
    Text {
        text: &'a str,
    },
    Thinking {
        thinking: &'a str,
        signature: &'a str,
    },
    ToolUse {
        id: &'a str,
        name: &'a str,
        #[serde(serialize_with = "compallm_serde_rules::serialize::serialize_raw")]
        input: &'a str,
    },
    ToolResult {
        tool_use_id: &'a str,
        content: ContentBlocks<'a>,
    },
}

#[derive(Clone, Copy)]
pub(crate) struct ContentBlocks<'a>(Option<&'a Content>);

impl Serialize for ContentBlocks<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let length = match self.0 {
            Some(Content::Text(_) | Content::Object(_)) => 1,
            Some(Content::Array(parts)) => parts.len(),
            Some(Content::Null) | None => 0,
        };
        let mut sequence = serializer.serialize_seq(Some(length))?;
        match self.0 {
            Some(Content::Text(text) | Content::Object(ObjectMessage::Text { text })) => {
                sequence.serialize_element(&Block::Text { text })?;
            }
            Some(Content::Array(parts)) => {
                for ObjectMessage::Text { text } in parts {
                    sequence.serialize_element(&Block::Text { text })?;
                }
            }
            Some(Content::Null) | None => {}
        }
        sequence.end()
    }
}

pub(crate) fn system(message: &Message) -> ContentBlocks<'_> {
    ContentBlocks(message.content.as_ref())
}

#[derive(Serialize)]
pub(crate) struct MessageWire<'a> {
    role: &'static str,
    content: MessageBlocks<'a>,
}

impl<'a> From<&'a Message> for MessageWire<'a> {
    fn from(message: &'a Message) -> Self {
        Self {
            role: match message.role {
                Role::Assistant => "assistant",
                Role::System => "system",
                Role::Developer => "system",
                _ => "user",
            },
            content: MessageBlocks(message),
        }
    }
}

struct MessageBlocks<'a>(&'a Message);

impl Serialize for MessageBlocks<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let message = self.0;
        if message.role == Role::Tool {
            let mut sequence = serializer.serialize_seq(Some(1))?;
            sequence.serialize_element(&Block::ToolResult {
                tool_use_id: message.tool_call_id.as_deref().unwrap_or(""),
                content: ContentBlocks(message.content.as_ref()),
            })?;
            return sequence.end();
        }

        let thinking = if message.role == Role::Assistant {
            message
                .reasoning_details
                .iter()
                .flatten()
                .filter(|detail| {
                    matches!(
                        detail.content,
                        ReasoningDetailContent::Text {
                            signature: Some(_),
                            ..
                        }
                    )
                })
                .count()
        } else {
            0
        };
        let text = match message.content.as_ref() {
            Some(Content::Text(_) | Content::Object(_)) => 1,
            Some(Content::Array(parts)) => parts.len(),
            Some(Content::Null) | None => 0,
        };
        let calls = if message.role == Role::Assistant {
            message.tool_calls.as_ref().map_or(0, Vec::len)
        } else {
            0
        };
        let mut sequence = serializer.serialize_seq(Some(thinking + text + calls))?;
        if message.role == Role::Assistant {
            for detail in message.reasoning_details.iter().flatten() {
                if let ReasoningDetailContent::Text {
                    text,
                    signature: Some(signature),
                } = &detail.content
                {
                    sequence.serialize_element(&Block::Thinking {
                        thinking: text,
                        signature,
                    })?;
                }
            }
        }
        match message.content.as_ref() {
            Some(Content::Text(text) | Content::Object(ObjectMessage::Text { text })) => {
                sequence.serialize_element(&Block::Text { text })?;
            }
            Some(Content::Array(parts)) => {
                for ObjectMessage::Text { text } in parts {
                    sequence.serialize_element(&Block::Text { text })?;
                }
            }
            Some(Content::Null) | None => {}
        }
        if message.role == Role::Assistant {
            for call in message.tool_calls.iter().flatten() {
                sequence.serialize_element(&Block::ToolUse {
                    id: &call.id,
                    name: &call.function.name,
                    input: &call.function.arguments,
                })?;
            }
        }
        sequence.end()
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct MessageInput {
    role: MessageRole,
    content: MessageContent,
}

#[derive(Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
enum MessageRole {
    User,
    Assistant,
    System,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum MessageContent {
    Text(String),
    Blocks(Vec<BlockInput>),
}

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
enum BlockInput {
    Text {
        text: String,
    },
    Thinking {
        thinking: String,
        signature: String,
    },
    RedactedThinking {
        data: String,
    },
    ToolUse {
        id: String,
        name: String,
        input: serde_json::Value,
    },
    ToolResult {
        tool_use_id: String,
        content: ToolResultContent,
    },
}

#[derive(Deserialize)]
#[serde(untagged)]
enum ToolResultContent {
    Text(String),
    Blocks(Vec<TextBlockInput>),
}

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
enum TextBlockInput {
    Text { text: String },
}

pub(crate) struct MessagesWire;

impl<'de> DeserializeAs<'de, Vec<Message>> for MessagesWire {
    fn deserialize_as<D: Deserializer<'de>>(deserializer: D) -> Result<Vec<Message>, D::Error> {
        let input = MessageInput::deserialize(deserializer)?;
        input.try_into().map_err(serde::de::Error::custom)
    }
}

pub(crate) struct SystemWire;

impl<'de> DeserializeAs<'de, Message> for SystemWire {
    fn deserialize_as<D: Deserializer<'de>>(deserializer: D) -> Result<Message, D::Error> {
        let content = MessageContent::deserialize(deserializer)?;
        let messages: Vec<Message> = MessageInput {
            role: MessageRole::System,
            content,
        }
        .try_into()
        .map_err(serde::de::Error::custom)?;
        messages
            .into_iter()
            .next()
            .ok_or_else(|| serde::de::Error::custom("empty system"))
    }
}

impl TryFrom<MessageInput> for Vec<Message> {
    type Error = &'static str;

    fn try_from(value: MessageInput) -> Result<Self, Self::Error> {
        let role = match value.role {
            MessageRole::User => Role::User,
            MessageRole::Assistant => Role::Assistant,
            MessageRole::System => Role::System,
        };
        let empty = || Message {
            role,
            content: None,
            tool_calls: None,
            tool_call_id: None,
            reasoning: None,
            reasoning_content: None,
            reasoning_details: None,
        };
        if let MessageContent::Text(text) = value.content {
            let mut message = empty();
            message.content = Some(Content::Text(text));
            return Ok(vec![message]);
        }
        let MessageContent::Blocks(blocks) = value.content else {
            unreachable!()
        };
        let mut message = empty();
        let mut text = Vec::new();
        let mut calls = Vec::new();
        let mut reasoning = Vec::new();
        let mut results = Vec::new();
        for block in blocks {
            match block {
                BlockInput::Text { text: value } => text.push(ObjectMessage::Text { text: value }),
                BlockInput::Thinking {
                    thinking,
                    signature,
                } => reasoning.push(ReasoningDetail {
                    content: ReasoningDetailContent::Text {
                        text: thinking,
                        signature: Some(signature),
                    },
                    id: None,
                    format: None,
                    index: None,
                }),
                BlockInput::RedactedThinking { data } => reasoning.push(ReasoningDetail {
                    content: ReasoningDetailContent::Encrypted { data },
                    id: None,
                    format: Some("anthropic".to_owned()),
                    index: None,
                }),
                BlockInput::ToolUse { id, name, input } => calls.push(ToolCall {
                    id,
                    kind: ToolType::Function,
                    function: FunctionCall {
                        name,
                        arguments: input.to_string(),
                    },
                }),
                BlockInput::ToolResult {
                    tool_use_id,
                    content,
                } => {
                    let content = match content {
                        ToolResultContent::Text(text) => Content::Text(text),
                        ToolResultContent::Blocks(blocks) => Content::Array(
                            blocks
                                .into_iter()
                                .map(|block| match block {
                                    TextBlockInput::Text { text } => ObjectMessage::Text { text },
                                })
                                .collect(),
                        ),
                    };
                    results.push(Message {
                        role: Role::Tool,
                        content: Some(content),
                        tool_calls: None,
                        tool_call_id: Some(tool_use_id),
                        reasoning: None,
                        reasoning_content: None,
                        reasoning_details: None,
                    });
                }
            }
        }
        if !results.is_empty() {
            if role != Role::User || !text.is_empty() || !calls.is_empty() || !reasoning.is_empty()
            {
                return Err("mixed tool results are not representable");
            }
            return Ok(results);
        }
        if role != Role::Assistant && (!calls.is_empty() || !reasoning.is_empty()) {
            return Err("assistant content in non-assistant message");
        }
        if !text.is_empty() {
            message.content = Some(Content::Array(text));
        }
        if !calls.is_empty() {
            message.tool_calls = Some(calls);
        }
        if !reasoning.is_empty() {
            message.reasoning_details = Some(reasoning);
        }
        if message.content.is_none()
            && message.tool_calls.is_none()
            && message.reasoning_details.is_none()
        {
            return Err("empty message content");
        }
        Ok(vec![message])
    }
}
