use crate::llm::text::component::Provider;
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::DeserializeAs;

#[derive(Serialize, Deserialize)]
#[serde(remote = "Provider", deny_unknown_fields)]
pub(in crate::llm::text::copenai::request) enum ProviderWire {
    #[serde(rename = "require_parameters", alias = "require_parameter")]
    RequireParameter(bool),
}

impl<'de> DeserializeAs<'de, Provider> for ProviderWire {
    fn deserialize_as<D: Deserializer<'de>>(deserializer: D) -> Result<Provider, D::Error> {
        Self::deserialize(deserializer)
    }
}
