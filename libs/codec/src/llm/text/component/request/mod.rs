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

use serde::de::{DeserializeOwned, Error};
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
        impl<'de> serde::Deserialize<'de> for $ty {
            fn deserialize<D: serde::Deserializer<'de>>(
                deserializer: D,
            ) -> std::result::Result<Self, D::Error> {
                let $value = <serde_json::Value as serde::Deserialize>::deserialize(deserializer)?;
                let parse = || -> super::Result<Self> { $body };
                parse().map_err(<D::Error as serde::de::Error>::custom)
            }
        }
    };
}

pub(super) use deserialize;
