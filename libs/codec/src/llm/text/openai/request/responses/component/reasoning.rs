use crate::llm::text::component::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(in crate::llm::text::openai::request::responses) struct ReasoningInput {
    #[serde(default)]
    effort: Option<EffortInput>,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::llm::text::openai::request::responses) enum EffortInput {
    Minimal,
    Low,
    Medium,
    High,
}

impl From<EffortInput> for ReasoningEffort {
    fn from(value: EffortInput) -> Self {
        match value {
            EffortInput::Minimal => Self::Minimal,
            EffortInput::Low => Self::Low,
            EffortInput::Medium => Self::Medium,
            EffortInput::High => Self::High,
        }
    }
}

impl From<ReasoningInput> for Reasoning {
    fn from(value: ReasoningInput) -> Self {
        Self {
            effort: value.effort.map(Into::into),
            ..Self::default()
        }
    }
}

impl ReasoningOutput {
    pub(in crate::llm::text::openai::request::responses) fn effort(
        value: ReasoningEffort,
    ) -> Option<&'static str> {
        match value {
            ReasoningEffort::Minimal => Some("minimal"),
            ReasoningEffort::Low => Some("low"),
            ReasoningEffort::Medium => Some("medium"),
            ReasoningEffort::High => Some("high"),
            _ => None,
        }
    }
}

/// Encoded Responses reasoning configuration.
#[derive(Serialize)]
pub(in crate::llm::text::openai::request::responses) struct ReasoningOutput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(in crate::llm::text::openai::request::responses) effort: Option<&'static str>,
}
