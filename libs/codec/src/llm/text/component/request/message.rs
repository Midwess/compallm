use super::ComponentDeserializeHelper;
use serde_json::Value;

use super::ToolType;

/// An owned message emitted only after its fields have been decoded and validated.
#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize)]
#[serde(try_from = "serde_json::Value")]
pub struct Message {
    pub role: Role,
    pub content: Option<Content>,
    pub tool_calls: Option<Vec<ToolCall>>,
    pub tool_call_id: Option<String>,
    pub reasoning: Option<String>,
    pub reasoning_content: Option<String>,
    pub reasoning_details: Option<Vec<ReasoningDetail>>,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize)]
#[serde(try_from = "serde_json::Value")]
pub enum Content {
    Null,
    Text(String),
    Object(ObjectMessage),
    Array(Vec<ObjectMessage>),
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize)]
#[serde(try_from = "serde_json::Value")]
pub enum ObjectMessage {
    Text { text: String },
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize)]
#[serde(try_from = "serde_json::Value")]
pub struct ToolCall {
    pub id: String,
    pub kind: ToolType,
    pub function: FunctionCall,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize)]
#[serde(try_from = "serde_json::Value")]
pub struct FunctionCall {
    pub name: String,
    pub arguments: String,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize)]
#[serde(try_from = "serde_json::Value")]
pub struct ReasoningDetail {
    pub content: ReasoningDetailContent,
    pub id: Option<String>,
    pub format: Option<String>,
    pub index: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ReasoningDetailContent {
    Summary {
        summary: String,
    },
    Text {
        text: String,
        signature: Option<String>,
    },
    Encrypted {
        data: String,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Deserialize)]
#[serde(try_from = "serde_json::Value")]
pub enum Role {
    System,
    Developer,
    User,
    Assistant,
    Tool,
}

impl TryFrom<Value> for Role {
    type Error = serde_json::Error;

    fn try_from(value: Value) -> super::Result<Self> {
        match value.as_str() {
            Some("system") => Ok(Self::System),
            Some("developer") => Ok(Self::Developer),
            Some("user") => Ok(Self::User),
            Some("assistant") => Ok(Self::Assistant),
            Some("tool") => Ok(Self::Tool),
            _ => Err(Self::invalid()),
        }
    }
}
impl TryFrom<Value> for ObjectMessage {
    type Error = serde_json::Error;

    fn try_from(value: Value) -> super::Result<Self> {
        let mut map = Self::object(value)?;
        let kind: String = Self::required(&mut map, "type")?;
        if kind != "text" {
            return Err(Self::invalid());
        }
        let text = Self::required(&mut map, "text")?;
        Self::done(map)?;
        Ok(Self::Text { text })
    }
}
impl TryFrom<Value> for Content {
    type Error = serde_json::Error;

    fn try_from(value: Value) -> super::Result<Self> {
        match value {
            Value::Null => Ok(Self::Null),
            Value::String(text) => Ok(Self::Text(text)),
            value @ Value::Object(_) => Ok(Self::Object(serde_json::from_value(value)?)),
            value @ Value::Array(_) => Ok(Self::Array(serde_json::from_value(value)?)),
            _ => Err(Self::invalid()),
        }
    }
}
impl TryFrom<Value> for FunctionCall {
    type Error = serde_json::Error;

    fn try_from(value: Value) -> super::Result<Self> {
        let mut map = Self::object(value)?;
        let name = Self::nonempty(Self::required(&mut map, "name")?)?;
        let arguments = Self::required(&mut map, "arguments")?;
        Self::done(map)?;
        Ok(Self { name, arguments })
    }
}
impl TryFrom<Value> for ToolCall {
    type Error = serde_json::Error;

    fn try_from(value: Value) -> super::Result<Self> {
        let mut map = Self::object(value)?;
        let id = Self::nonempty(Self::required(&mut map, "id")?)?;
        let kind = Self::required(&mut map, "type")?;
        let function = Self::required(&mut map, "function")?;
        Self::done(map)?;
        Ok(Self { id, kind, function })
    }
}
impl TryFrom<Value> for Message {
    type Error = serde_json::Error;

    fn try_from(value: Value) -> super::Result<Self> {
        let mut map = Self::object(value)?;
        let role = Self::required(&mut map, "role")?;
        let content: Option<Content> = map
            .remove("content")
            .map(serde_json::from_value)
            .transpose()?;
        let tool_calls: Option<Vec<ToolCall>> = Self::optional(&mut map, "tool_calls")?;
        let tool_call_id: Option<String> = Self::optional(&mut map, "tool_call_id")?;
        let reasoning = Self::optional(&mut map, "reasoning")?;
        let reasoning_content = Self::optional(&mut map, "reasoning_content")?;
        let reasoning_details: Option<Vec<ReasoningDetail>> =
            Self::optional(&mut map, "reasoning_details")?;
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
            return Err(Self::invalid());
        }
        Self::done(map)?;
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
impl TryFrom<Value> for ReasoningDetail {
    type Error = serde_json::Error;

    fn try_from(value: Value) -> super::Result<Self> {
        let mut map = Self::object(value)?;
        let kind: String = Self::required(&mut map, "type")?;
        let id = Self::optional(&mut map, "id")?;
        let format = Self::optional(&mut map, "format")?;
        let index = Self::optional(&mut map, "index")?;
        let content = match kind.as_str() {
            "summary" | "reasoning.summary" => ReasoningDetailContent::Summary {
                summary: Self::required(&mut map, "summary")?,
            },
            "text" | "reasoning.text" => ReasoningDetailContent::Text {
                text: Self::required(&mut map, "text")?,
                signature: Self::optional(&mut map, "signature")?,
            },
            "encrypted" | "reasoning.encrypted" => ReasoningDetailContent::Encrypted {
                data: Self::required(&mut map, "data")?,
            },
            _ => return Err(Self::invalid()),
        };
        Self::done(map)?;
        Ok(Self {
            content,
            id,
            format,
            index,
        })
    }
}

impl ComponentDeserializeHelper for Message {}
impl ComponentDeserializeHelper for Content {}
impl ComponentDeserializeHelper for ObjectMessage {}
impl ComponentDeserializeHelper for ToolCall {}
impl ComponentDeserializeHelper for FunctionCall {}
impl ComponentDeserializeHelper for ReasoningDetail {}
impl ComponentDeserializeHelper for ReasoningDetailContent {}
impl ComponentDeserializeHelper for Role {}
