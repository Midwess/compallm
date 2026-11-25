use crate::llm::text::component::*;
use serde::{Deserialize, Deserializer, Serialize, Serializer, ser::SerializeSeq};
use std::borrow::Cow;

#[derive(Serialize)]
#[serde(tag = "type", rename_all = "snake_case", bound(serialize = ""))]
pub(crate) enum Block<'a, T: AsRef<str>> {
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
        content: ContentBlocks<'a, T>,
    },
}

#[derive(Clone, Copy)]
pub(crate) struct ContentBlocks<'a, T: AsRef<str>>(Option<&'a Content<T>>);

impl<T: AsRef<str>> Serialize for ContentBlocks<'_, T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let length = match self.0 {
            Some(Content::Text(_) | Content::Object(_)) => 1,
            Some(Content::Array(parts)) => parts.len(),
            Some(Content::Null) | None => 0,
        };
        let mut sequence = serializer.serialize_seq(Some(length))?;
        match self.0 {
            Some(Content::Text(text) | Content::Object(ObjectMessage::Text { text })) => {
                sequence.serialize_element(&Block::<T>::Text {
                    text: text.as_ref(),
                })?;
            }
            Some(Content::Array(parts)) => {
                for ObjectMessage::Text { text } in parts {
                    sequence.serialize_element(&Block::<T>::Text {
                        text: text.as_ref(),
                    })?;
                }
            }
            Some(Content::Null) | None => {}
        }
        sequence.end()
    }
}

pub(crate) fn system<T: AsRef<str>>(message: &Message<T>) -> ContentBlocks<'_, T> {
    ContentBlocks(message.content.as_ref())
}

#[derive(Serialize)]
pub(crate) struct MessageWire<'a, T: AsRef<str>> {
    role: &'static str,
    content: MessageBlocks<'a, T>,
}

impl<'a, T: AsRef<str>> From<&'a Message<T>> for MessageWire<'a, T> {
    fn from(message: &'a Message<T>) -> Self {
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

struct MessageBlocks<'a, T: AsRef<str>>(&'a Message<T>);

impl<T: AsRef<str>> Serialize for MessageBlocks<'_, T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let message = self.0;
        if message.role == Role::Tool {
            let mut sequence = serializer.serialize_seq(Some(1))?;
            sequence.serialize_element(&Block::ToolResult {
                tool_use_id: message
                    .tool_call_id
                    .as_ref()
                    .map(AsRef::as_ref)
                    .unwrap_or(""),
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
                    sequence.serialize_element(&Block::<T>::Thinking {
                        thinking: text.as_ref(),
                        signature: signature.as_ref(),
                    })?;
                }
            }
        }
        match message.content.as_ref() {
            Some(Content::Text(text) | Content::Object(ObjectMessage::Text { text })) => {
                sequence.serialize_element(&Block::<T>::Text {
                    text: text.as_ref(),
                })?;
            }
            Some(Content::Array(parts)) => {
                for ObjectMessage::Text { text } in parts {
                    sequence.serialize_element(&Block::<T>::Text {
                        text: text.as_ref(),
                    })?;
                }
            }
            Some(Content::Null) | None => {}
        }
        if message.role == Role::Assistant {
            for call in message.tool_calls.iter().flatten() {
                sequence.serialize_element(&Block::<T>::ToolUse {
                    id: call.id.as_ref(),
                    name: call.function.name.as_ref(),
                    input: call.function.arguments.as_ref(),
                })?;
            }
        }
        sequence.end()
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields, bound(deserialize = "'de: 'a, 'a: 'de"))]
struct MessageInput<'a> {
    role: MessageRole,
    #[serde(borrow)]
    content: MessageContent<'a>,
}

#[derive(Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
enum MessageRole {
    User,
    Assistant,
    System,
}

enum MessageContent<'a> {
    Text(Cow<'a, str>),
    Blocks(Vec<BlockInput<'a>>),
}

enum BlockInput<'a> {
    Text {
        text: Cow<'a, str>,
    },
    Thinking {
        thinking: Cow<'a, str>,
        signature: Cow<'a, str>,
    },
    RedactedThinking {
        data: Cow<'a, str>,
    },
    ToolUse {
        id: Cow<'a, str>,
        name: Cow<'a, str>,
        input: Cow<'a, str>,
    },
    ToolResult {
        tool_use_id: Cow<'a, str>,
        content: ToolResultContent<'a>,
    },
}

#[derive(Deserialize)]
#[serde(untagged)]
enum ToolResultContent<'a> {
    Text(#[serde(borrow)] Cow<'a, str>),
    Blocks(#[serde(borrow)] Vec<TextBlockInput<'a>>),
}

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
enum TextBlockInput<'a> {
    Text {
        #[serde(borrow)]
        text: Cow<'a, str>,
    },
}

impl<'a> TryFrom<MessageInput<'a>> for Vec<Message<Cow<'a, str>>> {
    type Error = &'static str;

    fn try_from(value: MessageInput<'a>) -> Result<Self, Self::Error> {
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
                    format: Some(Cow::Borrowed("anthropic")),
                    index: None,
                }),
                BlockInput::ToolUse { id, name, input } => calls.push(ToolCall {
                    id,
                    kind: ToolType::Function,
                    function: FunctionCall {
                        name,
                        arguments: input,
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

pub(crate) struct MessagesInput<'a>(pub Vec<Message<Cow<'a, str>>>);
impl<'de> Deserialize<'de> for MessagesInput<'de> {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        MessageInput::deserialize(d)?
            .try_into()
            .map(Self)
            .map_err(serde::de::Error::custom)
    }
}
pub(crate) struct SystemInput<'a>(pub Message<Cow<'a, str>>);
impl<'de> Deserialize<'de> for SystemInput<'de> {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let content = MessageContent::deserialize(d)?;
        let messages: Vec<Message<Cow<'de, str>>> = MessageInput {
            role: MessageRole::System,
            content,
        }
        .try_into()
        .map_err(serde::de::Error::custom)?;
        messages
            .into_iter()
            .next()
            .map(Self)
            .ok_or_else(|| serde::de::Error::custom("empty system"))
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct BlockFields<'a> {
    #[serde(rename = "type")]
    kind: BlockKind,
    #[serde(
        default,
        borrow,
        deserialize_with = "compallm_serde_rules::borrowed::present_text"
    )]
    text: Option<Cow<'a, str>>,
    #[serde(
        default,
        borrow,
        deserialize_with = "compallm_serde_rules::borrowed::present_text"
    )]
    thinking: Option<Cow<'a, str>>,
    #[serde(
        default,
        borrow,
        deserialize_with = "compallm_serde_rules::borrowed::present_text"
    )]
    signature: Option<Cow<'a, str>>,
    #[serde(
        default,
        borrow,
        deserialize_with = "compallm_serde_rules::borrowed::present_text"
    )]
    data: Option<Cow<'a, str>>,
    #[serde(
        default,
        borrow,
        deserialize_with = "compallm_serde_rules::borrowed::present_text"
    )]
    id: Option<Cow<'a, str>>,
    #[serde(
        default,
        borrow,
        deserialize_with = "compallm_serde_rules::borrowed::present_text"
    )]
    name: Option<Cow<'a, str>>,
    #[serde(
        default,
        borrow,
        deserialize_with = "compallm_serde_rules::borrowed::present_schema"
    )]
    input: Option<Cow<'a, str>>,
    #[serde(
        default,
        borrow,
        deserialize_with = "compallm_serde_rules::borrowed::present_text"
    )]
    tool_use_id: Option<Cow<'a, str>>,
    #[serde(default, borrow)]
    content: Option<ToolResultContent<'a>>,
}
#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum BlockKind {
    Text,
    Thinking,
    RedactedThinking,
    ToolUse,
    ToolResult,
}

impl<'de> Deserialize<'de> for BlockInput<'de> {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let v = BlockFields::deserialize(d)?;
        use serde::de::Error;
        let result = match (
            v.kind,
            v.text,
            v.thinking,
            v.signature,
            v.data,
            v.id,
            v.name,
            v.input,
            v.tool_use_id,
            v.content,
        ) {
            (BlockKind::Text, Some(text), None, None, None, None, None, None, None, None) => {
                Self::Text { text }
            }
            (
                BlockKind::Thinking,
                None,
                Some(thinking),
                Some(signature),
                None,
                None,
                None,
                None,
                None,
                None,
            ) => Self::Thinking {
                thinking,
                signature,
            },
            (
                BlockKind::RedactedThinking,
                None,
                None,
                None,
                Some(data),
                None,
                None,
                None,
                None,
                None,
            ) => Self::RedactedThinking { data },
            (
                BlockKind::ToolUse,
                None,
                None,
                None,
                None,
                Some(id),
                Some(name),
                Some(input),
                None,
                None,
            ) => Self::ToolUse { id, name, input },
            (
                BlockKind::ToolResult,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(tool_use_id),
                Some(content),
            ) => Self::ToolResult {
                tool_use_id,
                content,
            },
            _ => return Err(D::Error::custom("invalid message block")),
        };
        Ok(result)
    }
}

impl<'de> Deserialize<'de> for MessageContent<'de> {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MessageContent<'de>;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("text or message blocks")
            }
            fn visit_borrowed_str<E: serde::de::Error>(
                self,
                v: &'de str,
            ) -> Result<Self::Value, E> {
                Ok(MessageContent::Text(Cow::Borrowed(v)))
            }

            fn visit_seq<A: serde::de::SeqAccess<'de>>(
                self,
                mut seq: A,
            ) -> Result<Self::Value, A::Error> {
                let mut blocks = Vec::new();
                while let Some(block) = seq.next_element::<BlockInput<'de>>()? {
                    blocks.push(block);
                }
                Ok(MessageContent::Blocks(blocks))
            }
        }
        d.deserialize_any(Visitor)
    }
}
