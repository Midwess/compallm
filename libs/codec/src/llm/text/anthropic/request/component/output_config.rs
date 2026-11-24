use crate::llm::text::component::ReasoningEffort;
use compallm_serde_rules::deserialize;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub(crate) struct OutputConfig<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effort: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<Format<'a>>,
}

#[derive(Serialize)]
pub(crate) struct Format<'a> {
    pub r#type: &'static str,
    #[serde(serialize_with = "compallm_serde_rules::serialize::serialize_raw")]
    pub schema: &'a str,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct OutputConfigInput {
    pub effort: Option<EffortInput>,
    pub format: Option<FormatInput>,
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
pub(crate) struct FormatInput {
    #[serde(rename = "type")]
    _kind: FormatKind,
    #[serde(deserialize_with = "deserialize::schema")]
    pub schema: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum FormatKind {
    JsonSchema,
}
