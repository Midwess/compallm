use super::tools::ToolTypeWire;
use crate::llm::text::component::*;
use compallm_serde_rules::borrowed;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_with::{SerializeAs, serde_as};

#[serde_as]
#[derive(Serialize)]
#[serde(remote = "Message", deny_unknown_fields)]
pub(crate) struct MessageWire<T: Serialize + AsRef<str>> {
    #[serde(with = "RoleWire")]
    role: Role,
    #[serde(
        default,
        serialize_with = "serde_with::As::<Option<ContentWire<T>>>::serialize",
        skip_serializing_if = "Option::is_none"
    )]
    content: Option<Content<T>>,
    #[serde_as(as = "Option<Vec<ToolCallWire<T>>>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_calls: Option<Vec<ToolCall<T>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_call_id: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reasoning: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reasoning_content: Option<T>,
    #[serde_as(as = "Option<Vec<ReasoningWire<T>>>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    reasoning_details: Option<Vec<ReasoningDetail<T>>>,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "Role", rename_all = "snake_case")]
pub(crate) enum RoleWire {
    System,
    Developer,
    User,
    Assistant,
    Tool,
}

#[serde_as]
#[derive(Serialize)]
#[serde(remote = "Content", untagged)]
pub(super) enum ContentWire<T: Serialize> {
    Null,
    Text(T),
    Object(#[serde(with = "ObjectMessageWire")] ObjectMessage<T>),
    Array(#[serde_as(as = "Vec<ObjectMessageWire<T>>")] Vec<ObjectMessage<T>>),
}

impl<T: Serialize> SerializeAs<Content<T>> for ContentWire<T> {
    fn serialize_as<S: Serializer>(value: &Content<T>, serializer: S) -> Result<S::Ok, S::Error> {
        Self::serialize(value, serializer)
    }
}

#[derive(Serialize)]
#[serde(
    remote = "ObjectMessage",
    tag = "type",
    rename_all = "snake_case",
    deny_unknown_fields
)]
enum ObjectMessageWire<T: Serialize> {
    Text { text: T },
}

impl<T: Serialize> SerializeAs<ObjectMessage<T>> for ObjectMessageWire<T> {
    fn serialize_as<S: Serializer>(
        value: &ObjectMessage<T>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        Self::serialize(value, serializer)
    }
}

#[derive(Serialize)]
#[serde(remote = "ToolCall", deny_unknown_fields)]
struct ToolCallWire<T: Serialize + AsRef<str>> {
    id: T,
    #[serde(rename = "type", with = "ToolTypeWire")]
    kind: ToolType,
    #[serde(with = "FunctionCallWire")]
    function: FunctionCall<T>,
}

impl<T: Serialize + AsRef<str>> SerializeAs<ToolCall<T>> for ToolCallWire<T> {
    fn serialize_as<S: Serializer>(value: &ToolCall<T>, serializer: S) -> Result<S::Ok, S::Error> {
        Self::serialize(value, serializer)
    }
}

#[derive(Serialize)]
#[serde(remote = "FunctionCall", deny_unknown_fields)]
struct FunctionCallWire<T: Serialize + AsRef<str>> {
    name: T,
    arguments: crate::json::JsonArgument<T>,
}

#[derive(Serialize)]
#[serde(remote = "ReasoningDetail")]
struct ReasoningWire<T: Serialize> {
    #[serde(flatten, with = "ReasoningContentWire")]
    content: ReasoningDetailContent<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    format: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    index: Option<u32>,
}

#[derive(Serialize)]
#[serde(remote = "ReasoningDetailContent", tag = "type")]
enum ReasoningContentWire<T: Serialize> {
    #[serde(rename = "reasoning.summary")]
    Summary { summary: T },
    #[serde(rename = "reasoning.text")]
    Text {
        text: T,
        #[serde(skip_serializing_if = "Option::is_none")]
        signature: Option<T>,
    },
    #[serde(rename = "reasoning.encrypted")]
    Encrypted { data: T },
}

impl<T: Serialize> SerializeAs<ReasoningDetail<T>> for ReasoningWire<T> {
    fn serialize_as<S: Serializer>(
        value: &ReasoningDetail<T>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        Self::serialize(value, serializer)
    }
}

#[serde_as]
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct MessageInput<'a> {
    #[serde(with = "RoleWire")]
    role: Role,
    #[serde(default, borrow, deserialize_with = "ContentVisitor::deserialize")]
    content: Option<Content<&'a str>>,
    #[serde(default, borrow)]
    #[serde_as(as = "Option<Vec<serde_with::FromInto<ToolCallInput<'a>>>>")]
    tool_calls: Option<Vec<ToolCall<&'a str>>>,
    #[serde(default, borrow, deserialize_with = "borrowed::optional_text")]
    tool_call_id: Option<&'a str>,
    #[serde(default, borrow, deserialize_with = "borrowed::optional_text")]
    reasoning: Option<&'a str>,
    #[serde(default, borrow, deserialize_with = "borrowed::optional_text")]
    reasoning_content: Option<&'a str>,
    #[serde(default, borrow)]
    #[serde_as(as = "Option<Vec<serde_with::TryFromInto<ReasoningInput<'a>>>>")]
    reasoning_details: Option<Vec<ReasoningDetail<&'a str>>>,
}

impl<'a> TryFrom<MessageInput<'a>> for Message<&'a str> {
    type Error = &'static str;
    fn try_from(value: MessageInput<'a>) -> Result<Self, Self::Error> {
        let value = Self {
            role: value.role,
            content: value.content,
            tool_calls: value.tool_calls,
            tool_call_id: value.tool_call_id,
            reasoning: value.reasoning,
            reasoning_content: value.reasoning_content,
            reasoning_details: value.reasoning_details,
        };
        let history = value.tool_calls.as_ref().is_some_and(|v| !v.is_empty())
            || value.reasoning.is_some()
            || value.reasoning_content.is_some()
            || value
                .reasoning_details
                .as_ref()
                .is_some_and(|v| !v.is_empty());
        let has_content = value
            .content
            .as_ref()
            .is_some_and(|v| !matches!(v, Content::Null));
        if (value.role != Role::Assistant
            && (value.tool_calls.is_some()
                || value.reasoning.is_some()
                || value.reasoning_content.is_some()
                || value.reasoning_details.is_some()))
            || (value.role == Role::Tool
                && (value.tool_call_id.as_ref().is_none_or(|v| v.is_empty())
                    || !matches!(value.content, Some(Content::Text(_)))))
            || (value.role != Role::Tool && value.tool_call_id.is_some())
            || (!has_content && !(value.role == Role::Assistant && history))
        {
            return Err("invalid request message");
        }
        Ok(value)
    }
}

impl ContentVisitor {
    pub(crate) fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<Content<&'de str>>, D::Error> {
        deserializer.deserialize_any(ContentVisitor).map(Some)
    }
}
pub(crate) struct ContentVisitor;
impl<'de> serde::de::Visitor<'de> for ContentVisitor {
    type Value = Content<&'de str>;
    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("null, text, a text object, or an array of text objects")
    }
    fn visit_unit<E: serde::de::Error>(self) -> Result<Self::Value, E> {
        Ok(Content::Null)
    }
    fn visit_borrowed_str<E: serde::de::Error>(self, v: &'de str) -> Result<Self::Value, E> {
        Ok(Content::Text(v))
    }

    fn visit_map<A: serde::de::MapAccess<'de>>(self, map: A) -> Result<Self::Value, A::Error> {
        TextPart::deserialize(serde::de::value::MapAccessDeserializer::new(map))
            .map(|v| Content::Object(v.into()))
    }
    fn visit_seq<A: serde::de::SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let mut parts = Vec::new();
        while let Some(part) = seq.next_element::<TextPart<'de>>()? {
            parts.push(part.into());
        }
        Ok(Content::Array(parts))
    }
}
#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
enum TextPart<'a> {
    Text {
        #[serde(borrow)]
        text: &'a str,
    },
}
impl<'a> From<TextPart<'a>> for ObjectMessage<&'a str> {
    fn from(TextPart::Text { text }: TextPart<'a>) -> Self {
        Self::Text { text }
    }
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ToolCallInput<'a> {
    #[serde(borrow, deserialize_with = "borrowed::nonempty")]
    id: &'a str,
    #[serde(rename = "type", with = "ToolTypeWire")]
    kind: ToolType,
    #[serde(borrow)]
    function: FunctionCallInput<'a>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct FunctionCallInput<'a> {
    #[serde(borrow, deserialize_with = "borrowed::nonempty")]
    name: &'a str,
    #[serde(borrow)]
    arguments: crate::json::JsonArgument<&'a str>,
}
impl<'a> From<ToolCallInput<'a>> for ToolCall<&'a str> {
    fn from(v: ToolCallInput<'a>) -> Self {
        Self {
            id: v.id,
            kind: v.kind,
            function: FunctionCall {
                name: v.function.name,
                arguments: v.function.arguments,
            },
        }
    }
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ReasoningInput<'a> {
    #[serde(rename = "type")]
    kind: ReasoningKind,
    #[serde(
        default,
        borrow,
        deserialize_with = "compallm_serde_rules::borrowed::present_text"
    )]
    summary: Option<&'a str>,
    #[serde(
        default,
        borrow,
        deserialize_with = "compallm_serde_rules::borrowed::present_text"
    )]
    text: Option<&'a str>,
    #[serde(
        default,
        borrow,
        deserialize_with = "compallm_serde_rules::borrowed::present_text"
    )]
    data: Option<&'a str>,
    #[serde(
        default,
        borrow,
        deserialize_with = "compallm_serde_rules::borrowed::present_optional_text"
    )]
    signature: Option<Option<&'a str>>,
    #[serde(default, borrow, deserialize_with = "borrowed::optional_text")]
    id: Option<&'a str>,
    #[serde(default, borrow, deserialize_with = "borrowed::optional_text")]
    format: Option<&'a str>,
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
impl<'a> TryFrom<ReasoningInput<'a>> for ReasoningDetail<&'a str> {
    type Error = &'static str;
    fn try_from(v: ReasoningInput<'a>) -> Result<Self, Self::Error> {
        let content = match v.kind {
            ReasoningKind::Summary
                if v.text.is_none() && v.data.is_none() && v.signature.is_none() =>
            {
                ReasoningDetailContent::Summary {
                    summary: v.summary.ok_or("missing summary")?,
                }
            }
            ReasoningKind::Text if v.summary.is_none() && v.data.is_none() => {
                ReasoningDetailContent::Text {
                    text: v.text.ok_or("missing text")?,
                    signature: v.signature.flatten(),
                }
            }
            ReasoningKind::Encrypted
                if v.summary.is_none() && v.text.is_none() && v.signature.is_none() =>
            {
                ReasoningDetailContent::Encrypted {
                    data: v.data.ok_or("missing data")?,
                }
            }
            _ => return Err("invalid reasoning detail"),
        };
        Ok(Self {
            content,
            id: v.id,
            format: v.format,
            index: v.index,
        })
    }
}
