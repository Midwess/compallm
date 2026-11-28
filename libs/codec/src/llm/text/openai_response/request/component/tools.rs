use super::CollectionWire;
use crate::llm::text::component::{Collection, FunctionDefinition, ToolDefinition, ToolType};
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
pub(in crate::llm::text::openai_response::request) struct ToolWire<T = String> {
    #[serde(rename = "type", with = "ToolTypeWire")]
    kind: ToolType,
    #[serde(flatten, with = "FunctionDefinitionWire")]
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
    parameters: Option<crate::json::JsonView<T>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    strict: Option<bool>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(in crate::llm::text::openai_response::request) struct ResponseTool<'a> {
    #[serde(rename = "type")]
    kind: FunctionType,
    #[serde(borrow, deserialize_with = "nonempty")]
    name: &'a str,
    #[serde(default, borrow, deserialize_with = "optional_text")]
    description: Option<&'a str>,
    #[serde(default, borrow)]
    parameters: Option<crate::json::JsonView<&'a str>>,
    strict: Option<bool>,
}
#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum FunctionType {
    Function,
}
impl<'a> From<ResponseTool<'a>> for ToolDefinition<&'a str> {
    fn from(value: ResponseTool<'a>) -> Self {
        let FunctionType::Function = value.kind;
        Self {
            kind: ToolType::Function,
            function: FunctionDefinition {
                name: value.name,
                description: value.description,
                parameters: value.parameters,
                strict: value.strict,
            },
        }
    }
}
#[serde_with::serde_as]
#[derive(Deserialize)]
#[serde(transparent)]
pub(in crate::llm::text::openai_response::request) struct ResponseTools<'a>(
    #[serde(borrow)]
    #[serde_as(as = "CollectionWire<serde_with::FromInto<ResponseTool<'a>>>")]
    pub Collection<ToolDefinition<&'a str>, &'a str>,
);

pub(in crate::llm::text::openai_response::request) struct ToolsOutput<'a, S: Serialize + AsRef<str>>(
    pub &'a Collection<ToolDefinition<S>, S>,
);
impl<T: Serialize + AsRef<str>> Serialize for ToolsOutput<'_, T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeSeq;
        let mut seq = serializer.serialize_seq(None)?;
        match self.0 {
            Collection::Single(value) => seq.serialize_element(
                &serde_with::ser::SerializeAsWrap::<_, ToolWire<T>>::new(value),
            )?,
            Collection::Owned(values) => {
                for value in values {
                    seq.serialize_element(
                        &serde_with::ser::SerializeAsWrap::<_, ToolWire<T>>::new(value),
                    )?;
                }
            }
            Collection::View(view) => {
                for item in view
                    .borrowed()
                    .items()
                    .ok_or_else(|| serde::ser::Error::custom("expected tools array"))?
                {
                    seq.serialize_element(&super::serialize::FlatTool(item))?;
                }
            }
        }
        seq.end()
    }
}
