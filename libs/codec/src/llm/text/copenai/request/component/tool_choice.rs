use super::tools::ToolTypeWire;
use crate::llm::text::component::{ToolChoice, ToolType};
use compallm_serde_rules::borrowed;
use serde::{Deserialize, Serialize, Serializer};
use serde_with::SerializeAs;

pub(crate) struct ToolChoiceWire;
impl<T: AsRef<str>> SerializeAs<ToolChoice<T>> for ToolChoiceWire {
    fn serialize_as<S: Serializer>(
        value: &ToolChoice<T>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        ChoiceOutput::from(value).serialize(serializer)
    }
}
#[derive(Serialize)]
#[serde(untagged)]
enum ChoiceOutput<'a> {
    Name(&'static str),
    Function {
        #[serde(rename = "type", with = "ToolTypeWire")]
        kind: ToolType,
        function: FunctionNameOutput<'a>,
    },
}
#[derive(Serialize)]
struct FunctionNameOutput<'a> {
    name: &'a str,
}
impl<'a, T: AsRef<str>> From<&'a ToolChoice<T>> for ChoiceOutput<'a> {
    fn from(v: &'a ToolChoice<T>) -> Self {
        match v {
            ToolChoice::Auto => Self::Name("auto"),
            ToolChoice::None => Self::Name("none"),
            ToolChoice::Required => Self::Name("required"),
            ToolChoice::Function(name) => Self::Function {
                kind: ToolType::Function,
                function: FunctionNameOutput {
                    name: name.as_ref(),
                },
            },
        }
    }
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct FunctionChoice<'a> {
    #[serde(rename = "type", with = "ToolTypeWire")]
    kind: ToolType,
    #[serde(borrow)]
    function: FunctionName<'a>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct FunctionName<'a> {
    #[serde(borrow, deserialize_with = "borrowed::nonempty")]
    name: &'a str,
}
impl<'a> From<FunctionChoice<'a>> for ToolChoice<&'a str> {
    fn from(v: FunctionChoice<'a>) -> Self {
        let ToolType::Function = v.kind;
        Self::Function(v.function.name)
    }
}
