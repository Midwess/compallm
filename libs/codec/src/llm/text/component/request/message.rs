use super::{ComponentDeserializer, deserialize};
use serde_json::Value;

use super::ToolType;

/// An owned message emitted only after its fields have been decoded and validated.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Message {
    pub role: Role,
    pub content: Option<Content>,
    pub tool_calls: Option<Vec<ToolCall>>,
    pub tool_call_id: Option<String>,
    pub reasoning: Option<String>,
    pub reasoning_content: Option<String>,
    pub reasoning_details: Option<Vec<ReasoningDetail>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Content {
    Null,
    Text(String),
    Object(ObjectMessage),
    Array(Vec<ObjectMessage>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ObjectMessage {
    Text { text: String },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ToolCall {
    pub id: String,
    pub kind: ToolType,
    pub function: FunctionCall,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Role {
    System,
    Developer,
    User,
    Assistant,
    Tool,
}

deserialize!(Role, value, {
    match value.as_str() {
        Some("system") => Ok(Self::System),
        Some("developer") => Ok(Self::Developer),
        Some("user") => Ok(Self::User),
        Some("assistant") => Ok(Self::Assistant),
        Some("tool") => Ok(Self::Tool),
        _ => Err(ComponentDeserializer::invalid()),
    }
});
deserialize!(ObjectMessage, value, {
    let mut map = ComponentDeserializer::object(value)?;
    let kind: String = ComponentDeserializer::required(&mut map, "type")?;
    if kind != "text" {
        return Err(ComponentDeserializer::invalid());
    }
    let text = ComponentDeserializer::required(&mut map, "text")?;
    ComponentDeserializer::done(map)?;
    Ok(Self::Text { text })
});
deserialize!(Content, value, {
    match value {
        Value::Null => Ok(Self::Null),
        Value::String(text) => Ok(Self::Text(text)),
        value @ Value::Object(_) => Ok(Self::Object(serde_json::from_value(value)?)),
        value @ Value::Array(_) => Ok(Self::Array(serde_json::from_value(value)?)),
        _ => Err(ComponentDeserializer::invalid()),
    }
});
deserialize!(FunctionCall, value, {
    let mut map = ComponentDeserializer::object(value)?;
    let name = ComponentDeserializer::nonempty(ComponentDeserializer::required(&mut map, "name")?)?;
    let arguments = ComponentDeserializer::required(&mut map, "arguments")?;
    ComponentDeserializer::done(map)?;
    Ok(Self { name, arguments })
});
deserialize!(ToolCall, value, {
    let mut map = ComponentDeserializer::object(value)?;
    let id = ComponentDeserializer::nonempty(ComponentDeserializer::required(&mut map, "id")?)?;
    let kind = ComponentDeserializer::required(&mut map, "type")?;
    let function = ComponentDeserializer::required(&mut map, "function")?;
    ComponentDeserializer::done(map)?;
    Ok(Self { id, kind, function })
});
deserialize!(Message, value, {
    let mut map = ComponentDeserializer::object(value)?;
    let role = ComponentDeserializer::required(&mut map, "role")?;
    let content: Option<Content> = map
        .remove("content")
        .map(serde_json::from_value)
        .transpose()?;
    let tool_calls: Option<Vec<ToolCall>> =
        ComponentDeserializer::optional(&mut map, "tool_calls")?;
    let tool_call_id: Option<String> = ComponentDeserializer::optional(&mut map, "tool_call_id")?;
    let reasoning = ComponentDeserializer::optional(&mut map, "reasoning")?;
    let reasoning_content = ComponentDeserializer::optional(&mut map, "reasoning_content")?;
    let reasoning_details: Option<Vec<ReasoningDetail>> =
        ComponentDeserializer::optional(&mut map, "reasoning_details")?;
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
        return Err(ComponentDeserializer::invalid());
    }
    ComponentDeserializer::done(map)?;
    Ok(Self {
        role,
        content,
        tool_calls,
        tool_call_id,
        reasoning,
        reasoning_content,
        reasoning_details,
    })
});
deserialize!(ReasoningDetail, value, {
    let mut map = ComponentDeserializer::object(value)?;
    let kind: String = ComponentDeserializer::required(&mut map, "type")?;
    let id = ComponentDeserializer::optional(&mut map, "id")?;
    let format = ComponentDeserializer::optional(&mut map, "format")?;
    let index = ComponentDeserializer::optional(&mut map, "index")?;
    let content = match kind.as_str() {
        "summary" | "reasoning.summary" => ReasoningDetailContent::Summary {
            summary: ComponentDeserializer::required(&mut map, "summary")?,
        },
        "text" | "reasoning.text" => ReasoningDetailContent::Text {
            text: ComponentDeserializer::required(&mut map, "text")?,
            signature: ComponentDeserializer::optional(&mut map, "signature")?,
        },
        "encrypted" | "reasoning.encrypted" => ReasoningDetailContent::Encrypted {
            data: ComponentDeserializer::required(&mut map, "data")?,
        },
        _ => return Err(ComponentDeserializer::invalid()),
    };
    ComponentDeserializer::done(map)?;
    Ok(Self {
        content,
        id,
        format,
        index,
    })
});
