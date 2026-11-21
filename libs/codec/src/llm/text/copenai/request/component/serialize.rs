use super::{
    MessageWire, ProviderWire, ResponseFormatWire, StopWire, StreamOptionsWire, ToolChoiceWire,
    ToolWire,
};
use crate::llm::text::component::*;
use serde::{Serialize, Serializer};
use serde_json::value::RawValue;
use serde_with::serde_as;

#[derive(Serialize)]
pub(in crate::llm::text::copenai::request) struct Request<'a>(
    #[serde(with = "RequestWire")]
    pub(in crate::llm::text::copenai::request)  &'a TextRequestComponent,
);

#[serde_as]
#[derive(Serialize)]
#[serde(remote = "TextRequestComponent", untagged)]
enum RequestWire {
    Model(String),
    Message(#[serde(with = "MessageWire")] Message),
    Tools(#[serde_as(as = "Vec<ToolWire>")] Vec<ToolDefinition>),
    ToolChoice(#[serde_as(as = "ToolChoiceWire")] ToolChoice),
    ParallelToolCalls(bool),
    MaxCompletionTokens(u32),
    TopP(f64),
    TopK(u32),
    Stop(#[serde(with = "StopWire")] Stop),
    ResponseFormat(#[serde(with = "ResponseFormatWire")] ResponseFormat),
    Provider(#[serde(with = "ProviderWire")] Provider),
    Stream(bool),
    StreamOptions(#[serde_as(as = "Option<StreamOptionsWire>")] Option<StreamOptions>),
}

pub(super) fn serialize_raw<T: AsRef<str>, S: Serializer>(
    value: &T,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let raw: &RawValue = serde_json::from_str(value.as_ref()).map_err(serde::ser::Error::custom)?;
    raw.serialize(serializer)
}

pub(super) fn optional_raw<T: AsRef<str>, S: Serializer>(
    value: &Option<T>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    value
        .as_ref()
        .map(|value| serde_json::from_str::<&RawValue>(value.as_ref()))
        .transpose()
        .map_err(serde::ser::Error::custom)?
        .serialize(serializer)
}
