use super::{
    MessageWire, ProviderWire, ReasoningWire, ResponseFormatWire, StopWire, StreamOptionsWire,
    ToolChoiceWire, ToolWire,
};
use crate::llm::text::component::*;
use serde::Serialize;
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
    MaxTokens(u32),
    Reasoning(#[serde(with = "ReasoningWire")] Reasoning),
    TopP(f64),
    TopK(u32),
    Stop(#[serde(with = "StopWire")] Stop),
    ResponseFormat(#[serde(with = "ResponseFormatWire")] ResponseFormat),
    Provider(#[serde(with = "ProviderWire")] Provider),
    Stream(bool),
    StreamOptions(#[serde_as(as = "Option<StreamOptionsWire>")] Option<StreamOptions>),
}
