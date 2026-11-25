use super::{
    MessageWire, ProviderWire, ReasoningWire, ResponseFormatWire, StopWire, StreamOptionsWire,
    ToolChoiceWire, ToolWire,
};
use crate::llm::text::component::*;
use serde::Serialize;
use serde_with::serde_as;

#[derive(Serialize)]
pub(in crate::llm::text::copenai::request) struct Request<'a, S: Serialize + AsRef<str>>(
    #[serde(with = "RequestWire")]
    pub(in crate::llm::text::copenai::request)  &'a TextRequestComponent<S>,
);

#[serde_as]
#[derive(Serialize)]
#[serde(remote = "TextRequestComponent", untagged)]
enum RequestWire<S: AsRef<str>> {
    Model(S),
    Message(#[serde(with = "MessageWire")] Message<S>),
    ToolsStarted,
    Tool(#[serde_as(as = "ToolWire<S>")] ToolDefinition<S>),
    ToolsEnded,
    ToolChoice(#[serde_as(as = "ToolChoiceWire")] ToolChoice<S>),
    ParallelToolCalls(bool),
    MaxTokens(u32),
    Reasoning(#[serde(with = "ReasoningWire")] Reasoning),
    TopP(f64),
    TopK(u32),
    Stop(#[serde(with = "StopWire")] Stop<S>),
    ResponseFormat(#[serde(with = "ResponseFormatWire")] ResponseFormat<S>),
    Provider(#[serde(with = "ProviderWire")] Provider),
    Stream(bool),
    StreamOptions(#[serde_as(as = "Option<StreamOptionsWire>")] Option<StreamOptions>),
}
