mod message;
mod provider;
mod response_format;
mod stop;
mod stream_options;
mod tool_choice;
mod tools;

pub use message::*;
pub use provider::Provider;
pub use response_format::{JsonSchema, ResponseFormat};
pub use stop::Stop;
pub use stream_options::StreamOptions;
pub use tool_choice::ToolChoice;
pub use tools::{FunctionDefinition, ToolDefinition, ToolType};

/// Completed request data, independent of byte chunks and parser state.
#[derive(Clone, Debug, PartialEq)]
pub enum TextRequestComponent {
    /// Identifies the model requested by the caller.
    Model(String),
    /// One conversation message, including text, tool calls/results, or reasoning history.
    Message(Message),
    /// Declares the functions the model may request the caller to execute.
    Tools(Vec<ToolDefinition>),
    /// Controls whether the model may, must, or must not call a tool, or selects one function.
    ToolChoice(ToolChoice),
    /// Controls whether the model may request multiple tool calls in one turn.
    ParallelToolCalls(bool),
    /// Limits generated tokens, including reasoning tokens where the provider counts them.
    MaxCompletionTokens(u32),
    /// Nucleus sampling: considers tokens within this cumulative probability mass.
    TopP(f64),
    /// Limits sampling to the most likely K tokens; zero disables this filter.
    TopK(u32),
    /// Supplies text sequences at which generation should stop, or disables explicit stops.
    Stop(Stop),
    /// Requests ordinary text, a JSON object, or JSON constrained by a supplied schema.
    ResponseFormat(ResponseFormat),
    /// Configures provider routing requirements, such as support for requested parameters.
    Provider(Provider),
    /// Requests incremental response delivery instead of a single completed response.
    Stream(bool),
    /// Configures streamed response metadata, such as token usage; None represents explicit null.
    StreamOptions(Option<StreamOptions>),
}

use serde::{
    Deserialize, Deserializer,
    de::{DeserializeOwned, Error},
};
use serde_json::{Map, Value};

type Result<T> = std::result::Result<T, serde_json::Error>;
struct ComponentDeserializer;

impl ComponentDeserializer {
    fn invalid() -> serde_json::Error {
        serde_json::Error::custom("invalid request component")
    }
    fn object(value: Value) -> Result<Map<String, Value>> {
        match value {
            Value::Object(map) => Ok(map),
            _ => Err(Self::invalid()),
        }
    }
    fn required<T: DeserializeOwned>(map: &mut Map<String, Value>, key: &str) -> Result<T> {
        serde_json::from_value(map.remove(key).ok_or_else(Self::invalid)?)
    }
    fn optional<T: DeserializeOwned>(map: &mut Map<String, Value>, key: &str) -> Result<Option<T>> {
        map.remove(key)
            .map(serde_json::from_value::<Option<T>>)
            .transpose()
            .map(Option::flatten)
    }
    fn done(map: Map<String, Value>) -> Result<()> {
        if map.is_empty() {
            Ok(())
        } else {
            Err(Self::invalid())
        }
    }
    fn nonempty(text: String) -> Result<String> {
        if text.is_empty() {
            Err(Self::invalid())
        } else {
            Ok(text)
        }
    }
    fn schema(value: Value) -> Result<String> {
        match value {
            Value::String(text) => Ok(text),
            Value::Object(_) => serde_json::to_string(&value),
            _ => Err(Self::invalid()),
        }
    }
}

macro_rules! deserialize {
    ($ty:ty, $value:ident, $body:block) => {
        impl<'de> Deserialize<'de> for $ty {
            fn deserialize<D: Deserializer<'de>>(
                deserializer: D,
            ) -> std::result::Result<Self, D::Error> {
                let $value = Value::deserialize(deserializer)?;
                let parse = || -> Result<Self> { $body };
                parse().map_err(D::Error::custom)
            }
        }
    };
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
deserialize!(ToolType, value, {
    if value.as_str() == Some("function") {
        Ok(Self::Function)
    } else {
        Err(ComponentDeserializer::invalid())
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
deserialize!(FunctionDefinition, value, {
    let mut map = ComponentDeserializer::object(value)?;
    let name = ComponentDeserializer::nonempty(ComponentDeserializer::required(&mut map, "name")?)?;
    let description = ComponentDeserializer::optional(&mut map, "description")?;
    let strict = ComponentDeserializer::optional(&mut map, "strict")?;
    let parameters = map
        .remove("parameters")
        .filter(|value| !value.is_null())
        .map(ComponentDeserializer::schema)
        .transpose()?;
    ComponentDeserializer::done(map)?;
    Ok(Self {
        name,
        description,
        strict,
        parameters,
    })
});
deserialize!(ToolDefinition, value, {
    let mut map = ComponentDeserializer::object(value)?;
    let kind = ComponentDeserializer::required(&mut map, "type")?;
    let function = ComponentDeserializer::required(&mut map, "function")?;
    ComponentDeserializer::done(map)?;
    Ok(Self { kind, function })
});
deserialize!(ToolChoice, value, {
    if let Value::String(choice) = value {
        return match choice.as_str() {
            "auto" => Ok(Self::Auto),
            "none" => Ok(Self::None),
            "required" => Ok(Self::Required),
            _ => Err(ComponentDeserializer::invalid()),
        };
    }
    let mut map = ComponentDeserializer::object(value)?;
    let _: ToolType = ComponentDeserializer::required(&mut map, "type")?;
    let mut function =
        ComponentDeserializer::object(ComponentDeserializer::required(&mut map, "function")?)?;
    let name =
        ComponentDeserializer::nonempty(ComponentDeserializer::required(&mut function, "name")?)?;
    ComponentDeserializer::done(function)?;
    ComponentDeserializer::done(map)?;
    Ok(Self::Function(name))
});
deserialize!(StreamOptions, value, {
    let mut map = ComponentDeserializer::object(value)?;
    let include_usage = ComponentDeserializer::optional(&mut map, "include_usage")?;
    ComponentDeserializer::done(map)?;
    Ok(Self { include_usage })
});
deserialize!(Provider, value, {
    let mut map = ComponentDeserializer::object(value)?;
    if map.contains_key("require_parameters") {
        if map.contains_key("require_parameter") {
            return Err(ComponentDeserializer::invalid());
        }
        let value = map
            .remove("require_parameters")
            .ok_or_else(ComponentDeserializer::invalid)?;
        map.insert("require_parameter".into(), value);
    }
    let required = ComponentDeserializer::required(&mut map, "require_parameter")?;
    ComponentDeserializer::done(map)?;
    Ok(Self::RequireParameter(required))
});
deserialize!(Stop, value, {
    match value {
        Value::Null => Ok(Self::Null),
        Value::String(text) if !text.is_empty() && text.len() <= 4096 => Ok(Self::Text(text)),
        value => {
            let stops: Vec<String> = serde_json::from_value(value)?;
            if stops.len() > 4
                || stops.iter().any(String::is_empty)
                || stops.iter().map(String::len).sum::<usize>() > 4096
            {
                return Err(ComponentDeserializer::invalid());
            }
            Ok(Self::Array(stops))
        }
    }
});
deserialize!(JsonSchema, value, {
    let mut map = ComponentDeserializer::object(value)?;
    let name = ComponentDeserializer::nonempty(ComponentDeserializer::required(&mut map, "name")?)?;
    let description = ComponentDeserializer::optional(&mut map, "description")?;
    let strict = ComponentDeserializer::optional(&mut map, "strict")?;
    let schema =
        ComponentDeserializer::schema(ComponentDeserializer::required(&mut map, "schema")?)?;
    ComponentDeserializer::done(map)?;
    Ok(Self {
        name,
        description,
        strict,
        schema,
    })
});
deserialize!(ResponseFormat, value, {
    let mut map = ComponentDeserializer::object(value)?;
    let kind: String = ComponentDeserializer::required(&mut map, "type")?;
    let format = match kind.as_str() {
        "text" => Self::Text,
        "json_object" => Self::JsonObject,
        "json_schema" => {
            Self::JsonSchema(ComponentDeserializer::required(&mut map, "json_schema")?)
        }
        _ => return Err(ComponentDeserializer::invalid()),
    };
    ComponentDeserializer::done(map)?;
    Ok(format)
});
