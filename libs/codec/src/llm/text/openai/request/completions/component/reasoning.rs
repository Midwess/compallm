use crate::llm::text::component::*;
use serde::Deserialize;

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::llm::text::openai::request::completions) enum EffortInput {
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

impl EffortInput {
    pub(in crate::llm::text::openai::request::completions) fn output(
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
