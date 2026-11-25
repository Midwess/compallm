use crate::llm::text::component::*;
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::DeserializeAs;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(in crate::llm::text::openai::request::completions) struct StreamOptionsInput {
    #[serde(default)]
    include_usage: Option<bool>,
}

impl From<StreamOptionsInput> for StreamOptions {
    fn from(value: StreamOptionsInput) -> Self {
        Self {
            include_usage: value.include_usage,
        }
    }
}

impl<'de> DeserializeAs<'de, StreamOptions> for StreamOptionsInput {
    fn deserialize_as<D: Deserializer<'de>>(deserializer: D) -> Result<StreamOptions, D::Error> {
        StreamOptionsInput::deserialize(deserializer).map(Into::into)
    }
}

/// Encoded Chat Completions streaming options.
#[derive(Serialize)]
pub(in crate::llm::text::openai::request::completions) struct StreamOptionsOutput {
    #[serde(skip_serializing_if = "Option::is_none")]
    include_usage: Option<bool>,
}

impl From<&StreamOptions> for StreamOptionsOutput {
    fn from(options: &StreamOptions) -> Self {
        Self {
            include_usage: options.include_usage,
        }
    }
}
