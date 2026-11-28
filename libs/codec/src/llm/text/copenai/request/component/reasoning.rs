use crate::llm::text::component::{Reasoning, ReasoningEffort};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_with::{DeserializeAs, SerializeAs, serde_as};

#[serde_as]
#[derive(Serialize, Deserialize)]
#[serde(remote = "Reasoning", deny_unknown_fields)]
pub(in crate::llm::text::copenai::request) struct ReasoningWire {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<bool>,
    #[serde_as(as = "Option<EffortWire>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    effort: Option<ReasoningEffort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude: Option<bool>,
}

#[serde_as]
#[derive(Serialize)]
pub(in crate::llm::text::copenai::request) struct ConfiguredReasoning {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde_as(as = "Option<EffortWire>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effort: Option<ReasoningEffort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude: Option<bool>,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "ReasoningEffort", rename_all = "lowercase")]
enum EffortWire {
    None,
    Minimal,
    Low,
    Medium,
    High,
    XHigh,
    Max,
}

impl SerializeAs<ReasoningEffort> for EffortWire {
    fn serialize_as<S: Serializer>(
        value: &ReasoningEffort,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        Self::serialize(value, serializer)
    }
}

impl<'de> DeserializeAs<'de, ReasoningEffort> for EffortWire {
    fn deserialize_as<D: Deserializer<'de>>(deserializer: D) -> Result<ReasoningEffort, D::Error> {
        Self::deserialize(deserializer)
    }
}

impl<'de> DeserializeAs<'de, Reasoning> for ReasoningWire {
    fn deserialize_as<D: Deserializer<'de>>(deserializer: D) -> Result<Reasoning, D::Error> {
        let value = Self::deserialize(deserializer)?;
        Self::validate(&value).map_err(serde::de::Error::custom)?;
        Ok(value)
    }
}

impl ReasoningWire {
    pub(in crate::llm::text::copenai::request) fn validate(
        value: &Reasoning,
    ) -> Result<(), &'static str> {
        if value.effort.is_some() && value.max_tokens.is_some() {
            return Err("reasoning effort and token budget are mutually exclusive");
        }
        Ok(())
    }
}
