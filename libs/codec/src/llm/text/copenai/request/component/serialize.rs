use crate::llm::text::component::*;
use serde::ser::{SerializeMap, SerializeSeq};
use serde::{Serialize, Serializer};
use serde_json::value::RawValue;

pub(crate) struct Encoded<'a, T: ?Sized>(pub(crate) &'a T);

impl<'a, T: ?Sized> From<&'a T> for Encoded<'a, T> {
    fn from(value: &'a T) -> Self {
        Self(value)
    }
}

impl<T> Serialize for Encoded<'_, [T]>
where
    for<'a> Encoded<'a, T>: Serialize,
{
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut sequence = serializer.serialize_seq(Some(self.0.len()))?;
        for value in self.0 {
            sequence.serialize_element(&Encoded::from(value))?;
        }
        sequence.end()
    }
}

impl Serialize for Encoded<'_, Message> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let value = self.0;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry(
            "role",
            match value.role {
                Role::System => "system",
                Role::Developer => "developer",
                Role::User => "user",
                Role::Assistant => "assistant",
                Role::Tool => "tool",
            },
        )?;
        if let Some(content) = &value.content {
            map.serialize_entry("content", &Encoded::from(content))?;
        }
        if let Some(calls) = &value.tool_calls {
            map.serialize_entry("tool_calls", &Encoded::from(calls.as_slice()))?;
        }
        if let Some(id) = &value.tool_call_id {
            map.serialize_entry("tool_call_id", id)?;
        }
        if let Some(text) = &value.reasoning {
            map.serialize_entry("reasoning", text)?;
        }
        if let Some(text) = &value.reasoning_content {
            map.serialize_entry("reasoning_content", text)?;
        }
        if let Some(details) = &value.reasoning_details {
            map.serialize_entry("reasoning_details", &Encoded::from(details.as_slice()))?;
        }
        map.end()
    }
}

impl Serialize for Encoded<'_, Content> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self.0 {
            Content::Null => serializer.serialize_none(),
            Content::Text(value) => value.serialize(serializer),
            Content::Object(value) => Encoded::from(value).serialize(serializer),
            Content::Array(values) => Encoded::from(values.as_slice()).serialize(serializer),
        }
    }
}

impl Serialize for Encoded<'_, ObjectMessage> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(2))?;
        match self.0 {
            ObjectMessage::Text { text } => {
                map.serialize_entry("type", "text")?;
                map.serialize_entry("text", text)?;
            }
        }
        map.end()
    }
}

#[derive(Serialize)]
struct FunctionName<'a> {
    name: &'a str,
}

impl Serialize for Encoded<'_, ToolCall> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct Function<'a> {
            name: &'a str,
            arguments: &'a str,
        }
        let mut map = serializer.serialize_map(Some(3))?;
        map.serialize_entry("id", &self.0.id)?;
        map.serialize_entry("type", "function")?;
        map.serialize_entry(
            "function",
            &Function {
                name: &self.0.function.name,
                arguments: &self.0.function.arguments,
            },
        )?;
        map.end()
    }
}

impl Serialize for Encoded<'_, ReasoningDetail> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(None)?;
        match &self.0.content {
            ReasoningDetailContent::Summary { summary } => {
                map.serialize_entry("type", "reasoning.summary")?;
                map.serialize_entry("summary", summary)?;
            }
            ReasoningDetailContent::Text { text, signature } => {
                map.serialize_entry("type", "reasoning.text")?;
                map.serialize_entry("text", text)?;
                if let Some(signature) = signature {
                    map.serialize_entry("signature", signature)?;
                }
            }
            ReasoningDetailContent::Encrypted { data } => {
                map.serialize_entry("type", "reasoning.encrypted")?;
                map.serialize_entry("data", data)?;
            }
        }
        if let Some(id) = &self.0.id {
            map.serialize_entry("id", id)?;
        }
        if let Some(format) = &self.0.format {
            map.serialize_entry("format", format)?;
        }
        if let Some(index) = self.0.index {
            map.serialize_entry("index", &index)?;
        }
        map.end()
    }
}

fn raw<E: serde::ser::Error>(value: &str) -> Result<&RawValue, E> {
    serde_json::from_str(value).map_err(E::custom)
}

impl Serialize for Encoded<'_, ToolDefinition> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("type", "function")?;
        map.serialize_entry("function", &Encoded::from(&self.0.function))?;
        map.end()
    }
}

impl Serialize for Encoded<'_, FunctionDefinition> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("name", &self.0.name)?;
        if let Some(description) = &self.0.description {
            map.serialize_entry("description", description)?;
        }
        if let Some(parameters) = &self.0.parameters {
            map.serialize_entry("parameters", raw::<S::Error>(parameters)?)?;
        }
        if let Some(strict) = self.0.strict {
            map.serialize_entry("strict", &strict)?;
        }
        map.end()
    }
}

impl Serialize for Encoded<'_, ToolChoice> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self.0 {
            ToolChoice::Auto => serializer.serialize_str("auto"),
            ToolChoice::None => serializer.serialize_str("none"),
            ToolChoice::Required => serializer.serialize_str("required"),
            ToolChoice::Function(name) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "function")?;
                map.serialize_entry("function", &FunctionName { name })?;
                map.end()
            }
        }
    }
}

impl Serialize for Encoded<'_, ResponseFormat> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry(
            "type",
            match self.0 {
                ResponseFormat::Text => "text",
                ResponseFormat::JsonObject => "json_object",
                ResponseFormat::JsonSchema(_) => "json_schema",
            },
        )?;
        if let ResponseFormat::JsonSchema(schema) = self.0 {
            map.serialize_entry("json_schema", &Encoded::from(schema))?;
        }
        map.end()
    }
}

impl Serialize for Encoded<'_, JsonSchema> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("name", &self.0.name)?;
        if let Some(description) = &self.0.description {
            map.serialize_entry("description", description)?;
        }
        if let Some(strict) = self.0.strict {
            map.serialize_entry("strict", &strict)?;
        }
        map.serialize_entry("schema", raw::<S::Error>(&self.0.schema)?)?;
        map.end()
    }
}

impl Serialize for Encoded<'_, Stop> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self.0 {
            Stop::Null => serializer.serialize_none(),
            Stop::Text(value) => value.serialize(serializer),
            Stop::Array(values) => values.serialize(serializer),
        }
    }
}

impl Serialize for Encoded<'_, Provider> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(1))?;
        let Provider::RequireParameter(value) = self.0;
        map.serialize_entry("require_parameters", value)?;
        map.end()
    }
}

impl Serialize for Encoded<'_, StreamOptions> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(None)?;
        if let Some(value) = self.0.include_usage {
            map.serialize_entry("include_usage", &value)?;
        }
        map.end()
    }
}

impl Serialize for Encoded<'_, TextRequestComponent> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self.0 {
            TextRequestComponent::Model(value) => value.serialize(serializer),
            TextRequestComponent::Message(value) => Encoded::from(value).serialize(serializer),
            TextRequestComponent::Tools(values) => {
                Encoded::from(values.as_slice()).serialize(serializer)
            }
            TextRequestComponent::ToolChoice(value) => Encoded::from(value).serialize(serializer),
            TextRequestComponent::ParallelToolCalls(value)
            | TextRequestComponent::Stream(value) => value.serialize(serializer),
            TextRequestComponent::MaxCompletionTokens(value)
            | TextRequestComponent::TopK(value) => value.serialize(serializer),
            TextRequestComponent::TopP(value) => value.serialize(serializer),
            TextRequestComponent::Stop(value) => Encoded::from(value).serialize(serializer),
            TextRequestComponent::ResponseFormat(value) => {
                Encoded::from(value).serialize(serializer)
            }
            TextRequestComponent::Provider(value) => Encoded::from(value).serialize(serializer),
            TextRequestComponent::StreamOptions(Some(value)) => {
                Encoded::from(value).serialize(serializer)
            }
            TextRequestComponent::StreamOptions(None) => serializer.serialize_none(),
        }
    }
}
