use crate::llm::text::component::*;
use serde::Serialize;

/// Encoded Chat Completions stop sequences.
#[derive(Serialize)]
#[serde(untagged)]
pub(in crate::llm::text::openai::request::completions) enum StopOutput<'a, S> {
    Null,
    Text(&'a S),
    Array(&'a [S]),
}

impl<'a, S> From<&'a Stop<S>> for StopOutput<'a, S> {
    fn from(stop: &'a Stop<S>) -> Self {
        match stop {
            Stop::Null => Self::Null,
            Stop::Text(text) => Self::Text(text),
            Stop::Array(values) => Self::Array(values),
        }
    }
}
