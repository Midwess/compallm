use crate::llm::text::component::{FunctionDefinition, ToolDefinition, ToolType};
use compallm_serde_rules::borrowed::{nonempty, optional_text};
use serde::{Deserialize, Serialize, Serializer};
use serde_with::SerializeAs;

#[derive(Serialize, Deserialize)]
#[serde(remote = "ToolType", rename_all = "snake_case")]
pub(super) enum ToolTypeWire {
    Function,
}

#[derive(Serialize)]
#[serde(
    remote = "ToolDefinition",
    deny_unknown_fields,
    bound(serialize = "T: Serialize + AsRef<str>")
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

#[derive(Serialize)]
#[serde(
    remote = "FunctionDefinition",
    deny_unknown_fields,
    bound(serialize = "T: Serialize + AsRef<str>")
)]
struct FunctionDefinitionWire<T = String> {
    name: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<T>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    parameters: Option<crate::json::JsonValue<T>>,
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
    name: &'a str,
    #[serde(default, borrow, deserialize_with = "optional_text")]
    description: Option<&'a str>,
    #[serde(default, borrow)]
    parameters: Option<crate::json::JsonValue<&'a str>>,
    strict: Option<bool>,
}

impl<'a> From<ChatTool<'a>> for ToolDefinition<&'a str> {
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
