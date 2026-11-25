use crate::llm::text::component::{FunctionDefinition, ToolDefinition, ToolType};
use compallm_serde_rules::borrowed::{nonempty, optional_schema, optional_text};
use compallm_serde_rules::{deserialize, serialize};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_with::{DeserializeAs, SerializeAs};
use std::borrow::Cow;

#[derive(Serialize, Deserialize)]
#[serde(remote = "ToolType", rename_all = "snake_case")]
pub(super) enum ToolTypeWire {
    Function,
}

#[derive(Serialize, Deserialize)]
#[serde(
    remote = "ToolDefinition",
    deny_unknown_fields,
    bound(
        deserialize = "T: Deserialize<'de> + AsRef<str> + From<String>",
        serialize = "T: Serialize + AsRef<str>"
    )
)]
pub(crate) struct ToolWire<T = String> {
    #[serde(rename = "type", with = "ToolTypeWire")]
    kind: ToolType,
    #[serde(with = "FunctionDefinitionWire")]
    function: FunctionDefinition<T>,
}

impl<T: Serialize + AsRef<str>> SerializeAs<ToolDefinition<T>> for ToolWire<T> {
    fn serialize_as<S: Serializer>(
        value: &ToolDefinition<T>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        Self::serialize(value, serializer)
    }
}

impl<'de> DeserializeAs<'de, ToolDefinition> for ToolWire {
    fn deserialize_as<D: Deserializer<'de>>(deserializer: D) -> Result<ToolDefinition, D::Error> {
        Self::deserialize(deserializer)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(
    remote = "FunctionDefinition",
    deny_unknown_fields,
    bound(
        deserialize = "T: Deserialize<'de> + AsRef<str> + From<String>",
        serialize = "T: Serialize + AsRef<str>"
    )
)]
struct FunctionDefinitionWire<T = String> {
    #[serde(deserialize_with = "deserialize::nonempty")]
    name: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<T>,
    #[serde(
        default,
        deserialize_with = "deserialize::optional_schema",
        serialize_with = "serialize::optional_raw",
        skip_serializing_if = "Option::is_none"
    )]
    parameters: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    strict: Option<bool>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ChatTool<'a> {
    #[serde(rename = "type")]
    kind: FunctionType,
    #[serde(borrow)]
    function: Function<'a>,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum FunctionType {
    Function,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Function<'a> {
    #[serde(borrow, deserialize_with = "nonempty")]
    name: Cow<'a, str>,
    #[serde(default, borrow, deserialize_with = "optional_text")]
    description: Option<Cow<'a, str>>,
    #[serde(default, borrow, deserialize_with = "optional_schema")]
    parameters: Option<Cow<'a, str>>,
    strict: Option<bool>,
}

impl<'a> From<ChatTool<'a>> for ToolDefinition<Cow<'a, str>> {
    fn from(value: ChatTool<'a>) -> Self {
        let FunctionType::Function = value.kind;
        Self {
            kind: ToolType::Function,
            function: FunctionDefinition {
                name: value.function.name,
                description: value.function.description,
                parameters: value.function.parameters,
                strict: value.function.strict,
            },
        }
    }
}
