use crate::llm::text::component::*;
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::DeserializeAs;

#[derive(Deserialize)]
#[serde(untagged)]
pub(in crate::llm::text::openai::request::completions) enum StopInput {
    Null,
    Text(String),
    Array(Vec<String>),
}

impl TryFrom<StopInput> for Stop {
    type Error = &'static str;

    fn try_from(value: StopInput) -> Result<Self, Self::Error> {
        let valid = match &value {
            StopInput::Null => true,
            StopInput::Text(text) => !text.is_empty() && text.len() <= 4096,
            StopInput::Array(values) => {
                values.len() <= 4
                    && values.iter().all(|value| !value.is_empty())
                    && values.iter().map(String::len).sum::<usize>() <= 4096
            }
        };
        if !valid {
            return Err("invalid stop sequences");
        }
        Ok(match value {
            StopInput::Null => Self::Null,
            StopInput::Text(text) => Self::Text(text),
            StopInput::Array(values) => Self::Array(values),
        })
    }
}

impl<'de> DeserializeAs<'de, Stop> for StopInput {
    fn deserialize_as<D: Deserializer<'de>>(deserializer: D) -> Result<Stop, D::Error> {
        StopInput::deserialize(deserializer)?
            .try_into()
            .map_err(serde::de::Error::custom)
    }
}

/// Encoded Chat Completions stop sequences.
#[derive(Serialize)]
#[serde(untagged)]
pub(in crate::llm::text::openai::request::completions) enum StopOutput<'a> {
    Null,
    Text(&'a str),
    Array(&'a [String]),
}

impl<'a> From<&'a Stop> for StopOutput<'a> {
    fn from(stop: &'a Stop) -> Self {
        match stop {
            Stop::Null => Self::Null,
            Stop::Text(text) => Self::Text(text),
            Stop::Array(values) => Self::Array(values),
        }
    }
}
