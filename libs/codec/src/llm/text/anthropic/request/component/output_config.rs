use crate::llm::text::component::ReasoningEffort;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub(crate) struct OutputConfig<'a, S: AsRef<str>> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effort: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<Format<'a, S>>,
}

#[derive(Serialize)]
pub(crate) struct Format<'a, S: AsRef<str>> {
    pub r#type: &'static str,
    pub schema: &'a crate::json::JsonValue<S>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct OutputConfigInput<'a> {
    pub effort: Option<EffortInput>,
    #[serde(borrow)]
    pub format: Option<FormatInput<'a>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum EffortInput {
    Low,
    Medium,
    High,
    Max,
}

impl From<EffortInput> for ReasoningEffort {
    fn from(value: EffortInput) -> Self {
        match value {
            EffortInput::Low => Self::Low,
            EffortInput::Medium => Self::Medium,
            EffortInput::High => Self::High,
            EffortInput::Max => Self::Max,
        }
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct FormatInput<'a> {
    #[serde(rename = "type")]
    _kind: FormatKind,
    #[serde(borrow)]
    pub schema: crate::json::JsonValue<&'a str>,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum FormatKind {
    JsonSchema,
}
