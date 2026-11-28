use super::{
    CollectionWire, MessageWire, ResponseFormatWire, StopWire, StreamOptionsWire, ToolChoiceWire,
    ToolWire,
};
use crate::llm::text::component::*;
use serde::Serialize;
use serde_with::serde_as;

#[derive(Serialize)]
pub(in crate::llm::text::openai_completion::request) struct Request<'a, S: Serialize + AsRef<str>>(
    #[serde(with = "RequestWire")]
    pub(in crate::llm::text::openai_completion::request)  &'a TextRequestComponent<S>,
);

#[serde_as]
#[derive(Serialize)]
#[serde(remote = "TextRequestComponent", untagged)]
enum RequestWire<S: AsRef<str>> {
    Model(S),
    Message(#[serde(with = "MessageWire")] Message<S>),
    Tools(#[serde_as(as = "CollectionWire<ToolWire<S>>")] Collection<ToolDefinition<S>, S>),
    ToolChoice(#[serde_as(as = "ToolChoiceWire")] ToolChoice<S>),
    ParallelToolCalls(bool),
    MaxTokens(u32),
    #[serde(skip)]
    Reasoning(Reasoning),
    TopP(f64),
    TopK(u32),
    Stop(#[serde(with = "StopWire")] Stop<S>),
    ResponseFormat(#[serde(with = "ResponseFormatWire")] ResponseFormat<S>),
    #[serde(skip)]
    Provider(Provider),
    Stream(bool),
    StreamOptions(#[serde_as(as = "Option<StreamOptionsWire>")] Option<StreamOptions>),
}
