use crate::llm::text::component::{Reasoning, ReasoningEffort};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum Thinking {
    Adaptive,
    Disabled,
    Enabled { budget_tokens: u32 },
}

impl From<&Reasoning> for Thinking {
    fn from(reasoning: &Reasoning) -> Self {
        if reasoning.enabled == Some(false) {
            Self::Disabled
        } else if let Some(budget_tokens) = reasoning.max_tokens {
            Self::Enabled { budget_tokens }
        } else {
            Self::Adaptive
        }
    }
}

impl Thinking {
    pub(crate) fn effort(effort: ReasoningEffort) -> Option<&'static str> {
        match effort {
            ReasoningEffort::Low => Some("low"),
            ReasoningEffort::Medium => Some("medium"),
            ReasoningEffort::High => Some("high"),
            ReasoningEffort::Max => Some("max"),
            _ => None,
        }
    }
}
#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub(crate) enum ThinkingInput {
    Adaptive,
    Disabled,
    Enabled { budget_tokens: u32 },
    BetweenTools,
}

impl From<ThinkingInput> for Reasoning {
    fn from(value: ThinkingInput) -> Self {
        match value {
            ThinkingInput::Adaptive | ThinkingInput::BetweenTools => Self {
                enabled: Some(true),
                ..Self::default()
            },
            ThinkingInput::Disabled => Self {
                enabled: Some(false),
                ..Self::default()
            },
            ThinkingInput::Enabled { budget_tokens } => Self {
                enabled: Some(true),
                max_tokens: Some(budget_tokens),
                ..Self::default()
            },
        }
    }
}
