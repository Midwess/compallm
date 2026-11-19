use super::{CompatibleOpenAIRequestComponent, ComponentDeserializeHelper, Decoded};
use crate::llm::text::component::{
    Content, FunctionCall, Message, ObjectMessage, ReasoningDetail, ReasoningDetailContent, Role,
    ToolCall, ToolType,
};
use serde::Deserialize;

impl<'de> Deserialize<'de> for Decoded<Message> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::Error;
        CompatibleOpenAIMessage::deserialize(deserializer)?
            .try_into()
            .map(Self)
            .map_err(D::Error::custom)
    }
}

impl<'de> Deserialize<'de> for Decoded<ObjectMessage> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        CompatibleOpenAIObjectMessage::deserialize(deserializer).map(Self)
    }
}

impl<'de> Deserialize<'de> for Decoded<ToolCall> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        CompatibleOpenAIToolCall::deserialize(deserializer).map(Self)
    }
}

impl<'de> Deserialize<'de> for Decoded<ReasoningDetail> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::Error;
        CompatibleOpenAIReasoningDetail::deserialize(deserializer)?
            .try_into()
            .map(Self)
            .map_err(D::Error::custom)
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct CompatibleOpenAIMessage {
    #[serde(with = "CompatibleOpenAIRole")]
    role: Role,
    #[serde(
        default,
        deserialize_with = "CompatibleOpenAIRequestComponent::content"
    )]
    content: Option<Content>,
    #[serde(
        default,
        deserialize_with = "CompatibleOpenAIRequestComponent::decode::<_, Option<Vec<ToolCall>>>"
    )]
    tool_calls: Option<Vec<ToolCall>>,
    tool_call_id: Option<String>,
    reasoning: Option<String>,
    reasoning_content: Option<String>,
    #[serde(
        default,
        deserialize_with = "CompatibleOpenAIRequestComponent::decode::<_, Option<Vec<ReasoningDetail>>>"
    )]
    reasoning_details: Option<Vec<ReasoningDetail>>,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub(super) enum CompatibleOpenAIContent {
    Null,
    Text(String),
    Object(#[serde(with = "CompatibleOpenAIObjectMessage")] ObjectMessage),
    Array(Vec<Decoded<ObjectMessage>>),
}

impl From<CompatibleOpenAIContent> for Content {
    fn from(value: CompatibleOpenAIContent) -> Self {
        match value {
            CompatibleOpenAIContent::Null => Self::Null,
            CompatibleOpenAIContent::Text(text) => Self::Text(text),
            CompatibleOpenAIContent::Object(object) => Self::Object(object),
            CompatibleOpenAIContent::Array(parts) => {
                Self::Array(parts.into_iter().map(|Decoded(part)| part).collect())
            }
        }
    }
}

#[derive(Deserialize)]
#[serde(
    remote = "ObjectMessage",
    tag = "type",
    rename_all = "snake_case",
    deny_unknown_fields
)]
enum CompatibleOpenAIObjectMessage {
    Text { text: String },
}

#[derive(Deserialize)]
#[serde(remote = "ToolCall", deny_unknown_fields)]
struct CompatibleOpenAIToolCall {
    #[serde(deserialize_with = "CompatibleOpenAIRequestComponent::nonempty")]
    id: String,
    #[serde(rename = "type", with = "super::tools::CompatibleOpenAIToolType")]
    kind: ToolType,
    #[serde(with = "CompatibleOpenAIFunctionCall")]
    function: FunctionCall,
}

#[derive(Deserialize)]
#[serde(remote = "FunctionCall", deny_unknown_fields)]
struct CompatibleOpenAIFunctionCall {
    #[serde(deserialize_with = "CompatibleOpenAIRequestComponent::nonempty")]
    name: String,
    arguments: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CompatibleOpenAIReasoningDetail {
    #[serde(rename = "type")]
    kind: String,
    #[serde(
        default,
        deserialize_with = "CompatibleOpenAIRequestComponent::present"
    )]
    summary: Option<String>,
    #[serde(
        default,
        deserialize_with = "CompatibleOpenAIRequestComponent::present"
    )]
    text: Option<String>,
    #[serde(
        default,
        deserialize_with = "CompatibleOpenAIRequestComponent::present"
    )]
    data: Option<String>,
    #[serde(
        default,
        deserialize_with = "CompatibleOpenAIRequestComponent::present"
    )]
    signature: Option<Option<String>>,
    id: Option<String>,
    format: Option<String>,
    index: Option<u32>,
}

#[derive(Deserialize)]
#[serde(remote = "Role", rename_all = "snake_case")]
enum CompatibleOpenAIRole {
    System,
    Developer,
    User,
    Assistant,
    Tool,
}

impl TryFrom<CompatibleOpenAIMessage> for Message {
    type Error = &'static str;

    fn try_from(value: CompatibleOpenAIMessage) -> Result<Self, Self::Error> {
        let CompatibleOpenAIMessage {
            role,
            content,
            tool_calls,
            tool_call_id,
            reasoning,
            reasoning_content,
            reasoning_details,
        } = value;
        let history = tool_calls.as_ref().is_some_and(|calls| !calls.is_empty())
            || reasoning.is_some()
            || reasoning_content.is_some()
            || reasoning_details
                .as_ref()
                .is_some_and(|details| !details.is_empty());
        let has_content = content
            .as_ref()
            .is_some_and(|content| !matches!(content, Content::Null));
        if (role != Role::Assistant
            && (tool_calls.is_some()
                || reasoning.is_some()
                || reasoning_content.is_some()
                || reasoning_details.is_some()))
            || (role == Role::Tool
                && (tool_call_id.as_ref().is_none_or(String::is_empty)
                    || !matches!(content, Some(Content::Text(_)))))
            || (role != Role::Tool && tool_call_id.is_some())
            || (!has_content && !(role == Role::Assistant && history))
        {
            return Err("invalid request message");
        }
        Ok(Self {
            role,
            content,
            tool_calls,
            tool_call_id,
            reasoning,
            reasoning_content,
            reasoning_details,
        })
    }
}

impl TryFrom<CompatibleOpenAIReasoningDetail> for ReasoningDetail {
    type Error = &'static str;

    fn try_from(value: CompatibleOpenAIReasoningDetail) -> Result<Self, Self::Error> {
        let CompatibleOpenAIReasoningDetail {
            kind,
            summary,
            text,
            data,
            signature,
            id,
            format,
            index,
        } = value;
        let content = match kind.as_str() {
            "summary" | "reasoning.summary"
                if text.is_none() && data.is_none() && signature.is_none() =>
            {
                ReasoningDetailContent::Summary {
                    summary: summary.ok_or("missing summary")?,
                }
            }
            "text" | "reasoning.text" if summary.is_none() && data.is_none() => {
                ReasoningDetailContent::Text {
                    text: text.ok_or("missing text")?,
                    signature: signature.flatten(),
                }
            }
            "encrypted" | "reasoning.encrypted"
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
