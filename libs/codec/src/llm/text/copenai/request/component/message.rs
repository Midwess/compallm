use super::tools::ToolTypeWire;
use crate::llm::text::component::*;
use compallm_serde_rules::deserialize;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_with::{DeserializeAs, SerializeAs, serde_as};

#[serde_as]
#[derive(Serialize, Deserialize)]
#[serde(remote = "Message", deny_unknown_fields)]
pub(crate) struct MessageWire {
    #[serde(with = "RoleWire")]
    role: Role,
    #[serde(
        default,
        deserialize_with = "content",
        serialize_with = "serde_with::As::<Option<ContentWire>>::serialize",
        skip_serializing_if = "Option::is_none"
    )]
    content: Option<Content>,
    #[serde_as(as = "Option<Vec<ToolCallWire>>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_calls: Option<Vec<ToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_call_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reasoning: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reasoning_content: Option<String>,
    #[serde_as(as = "Option<Vec<ReasoningWire>>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    reasoning_details: Option<Vec<ReasoningDetail>>,
}

impl<'de> DeserializeAs<'de, Message> for MessageWire {
    fn deserialize_as<D: Deserializer<'de>>(deserializer: D) -> Result<Message, D::Error> {
        let value = Self::deserialize(deserializer)?;
        let history = value
            .tool_calls
            .as_ref()
            .is_some_and(|calls| !calls.is_empty())
            || value.reasoning.is_some()
            || value.reasoning_content.is_some()
            || value
                .reasoning_details
                .as_ref()
                .is_some_and(|details| !details.is_empty());
        let has_content = value
            .content
            .as_ref()
            .is_some_and(|content| !matches!(content, Content::Null));
        if (value.role != Role::Assistant
            && (value.tool_calls.is_some()
                || value.reasoning.is_some()
                || value.reasoning_content.is_some()
                || value.reasoning_details.is_some()))
            || (value.role == Role::Tool
                && (value.tool_call_id.as_ref().is_none_or(String::is_empty)
                    || !matches!(value.content, Some(Content::Text(_)))))
            || (value.role != Role::Tool && value.tool_call_id.is_some())
            || (!has_content && !(value.role == Role::Assistant && history))
        {
            return Err(serde::de::Error::custom("invalid request message"));
        }
        Ok(value)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "Role", rename_all = "snake_case")]
enum RoleWire {
    System,
    Developer,
    User,
    Assistant,
    Tool,
}

#[serde_as]
#[derive(Serialize)]
#[serde(remote = "Content", untagged)]
pub(super) enum ContentWire {
    Null,
    Text(String),
    Object(#[serde(with = "ObjectMessageWire")] ObjectMessage),
    Array(#[serde_as(as = "Vec<ObjectMessageWire>")] Vec<ObjectMessage>),
}

impl SerializeAs<Content> for ContentWire {
    fn serialize_as<S: Serializer>(value: &Content, serializer: S) -> Result<S::Ok, S::Error> {
        Self::serialize(value, serializer)
    }
}

impl<'de> DeserializeAs<'de, Content> for ContentWire {
    fn deserialize_as<D: Deserializer<'de>>(deserializer: D) -> Result<Content, D::Error> {
        deserializer.deserialize_any(ContentVisitor)
    }
}

struct ContentVisitor;

impl<'de> serde::de::Visitor<'de> for ContentVisitor {
    type Value = Content;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("null, text, a text object, or an array of text objects")
    }

    fn visit_unit<E: serde::de::Error>(self) -> Result<Content, E> {
        Ok(Content::Null)
    }

    fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Content, E> {
        Ok(Content::Text(value.to_owned()))
    }

    fn visit_string<E: serde::de::Error>(self, value: String) -> Result<Content, E> {
        Ok(Content::Text(value))
    }

    fn visit_map<A: serde::de::MapAccess<'de>>(self, map: A) -> Result<Content, A::Error> {
        ObjectMessageWire::deserialize(serde::de::value::MapAccessDeserializer::new(map))
            .map(Content::Object)
    }

    fn visit_seq<A: serde::de::SeqAccess<'de>>(self, mut sequence: A) -> Result<Content, A::Error> {
        let mut parts = Vec::new();
        while let Some(part) = sequence
            .next_element::<serde_with::de::DeserializeAsWrap<ObjectMessage, ObjectMessageWire>>()?
        {
            parts.push(part.into_inner());
        }
        Ok(Content::Array(parts))
    }
}

fn content<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<Content>, D::Error> {
    ContentWire::deserialize_as(deserializer).map(Some)
}

#[derive(Serialize, Deserialize)]
#[serde(
    remote = "ObjectMessage",
    tag = "type",
    rename_all = "snake_case",
    deny_unknown_fields
)]
enum ObjectMessageWire {
    Text { text: String },
}

impl SerializeAs<ObjectMessage> for ObjectMessageWire {
    fn serialize_as<S: Serializer>(
        value: &ObjectMessage,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        Self::serialize(value, serializer)
    }
}

impl<'de> DeserializeAs<'de, ObjectMessage> for ObjectMessageWire {
    fn deserialize_as<D: Deserializer<'de>>(deserializer: D) -> Result<ObjectMessage, D::Error> {
        Self::deserialize(deserializer)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "ToolCall", deny_unknown_fields)]
struct ToolCallWire {
    #[serde(deserialize_with = "deserialize::nonempty")]
    id: String,
    #[serde(rename = "type", with = "ToolTypeWire")]
    kind: ToolType,
    #[serde(with = "FunctionCallWire")]
    function: FunctionCall,
}

impl SerializeAs<ToolCall> for ToolCallWire {
    fn serialize_as<S: Serializer>(value: &ToolCall, serializer: S) -> Result<S::Ok, S::Error> {
        Self::serialize(value, serializer)
    }
}

impl<'de> DeserializeAs<'de, ToolCall> for ToolCallWire {
    fn deserialize_as<D: Deserializer<'de>>(deserializer: D) -> Result<ToolCall, D::Error> {
        Self::deserialize(deserializer)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "FunctionCall", deny_unknown_fields)]
struct FunctionCallWire {
    #[serde(deserialize_with = "deserialize::nonempty")]
    name: String,
    arguments: String,
}

#[derive(Serialize)]
#[serde(remote = "ReasoningDetail")]
struct ReasoningWire {
    #[serde(flatten, with = "ReasoningContentWire")]
    content: ReasoningDetailContent,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    index: Option<u32>,
}

#[derive(Serialize)]
#[serde(remote = "ReasoningDetailContent", tag = "type")]
enum ReasoningContentWire {
    #[serde(rename = "reasoning.summary")]
    Summary { summary: String },
    #[serde(rename = "reasoning.text")]
    Text {
        text: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        signature: Option<String>,
    },
    #[serde(rename = "reasoning.encrypted")]
    Encrypted { data: String },
}

impl SerializeAs<ReasoningDetail> for ReasoningWire {
    fn serialize_as<S: Serializer>(
        value: &ReasoningDetail,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        Self::serialize(value, serializer)
    }
}

impl<'de> DeserializeAs<'de, ReasoningDetail> for ReasoningWire {
    fn deserialize_as<D: Deserializer<'de>>(deserializer: D) -> Result<ReasoningDetail, D::Error> {
        ReasoningInput::deserialize(deserializer)?
            .try_into()
            .map_err(serde::de::Error::custom)
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ReasoningInput {
    #[serde(rename = "type")]
    kind: ReasoningKind,
    #[serde(default, deserialize_with = "deserialize::present")]
    summary: Option<String>,
    #[serde(default, deserialize_with = "deserialize::present")]
    text: Option<String>,
    #[serde(default, deserialize_with = "deserialize::present")]
    data: Option<String>,
    #[serde(default, deserialize_with = "deserialize::present")]
    signature: Option<Option<String>>,
    id: Option<String>,
    format: Option<String>,
    index: Option<u32>,
}

#[derive(Deserialize)]
enum ReasoningKind {
    #[serde(rename = "reasoning.summary", alias = "summary")]
    Summary,
    #[serde(rename = "reasoning.text", alias = "text")]
    Text,
    #[serde(rename = "reasoning.encrypted", alias = "encrypted")]
    Encrypted,
}

impl TryFrom<ReasoningInput> for ReasoningDetail {
    type Error = &'static str;

    fn try_from(value: ReasoningInput) -> Result<Self, Self::Error> {
        let ReasoningInput {
            kind,
            summary,
            text,
            data,
            signature,
            id,
            format,
            index,
        } = value;
        let content = match kind {
            ReasoningKind::Summary if text.is_none() && data.is_none() && signature.is_none() => {
                ReasoningDetailContent::Summary {
                    summary: summary.ok_or("missing summary")?,
                }
            }
            ReasoningKind::Text if summary.is_none() && data.is_none() => {
                ReasoningDetailContent::Text {
                    text: text.ok_or("missing text")?,
                    signature: signature.flatten(),
                }
            }
            ReasoningKind::Encrypted
                if summary.is_none() && text.is_none() && signature.is_none() =>
            {
                ReasoningDetailContent::Encrypted {
                    data: data.ok_or("missing data")?,
                }
            }
            _ => return Err("invalid reasoning detail"),
        };
        Ok(Self {
            content,
            id,
            format,
            index,
        })
    }
}
