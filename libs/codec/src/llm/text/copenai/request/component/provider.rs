use super::Decoded;
use crate::llm::text::component::Provider;
use serde::Deserialize;

impl<'de> Deserialize<'de> for Decoded<Provider> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        ProviderFields::deserialize(deserializer).map(|value| Self(value.into()))
    }
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct ProviderFields {
    #[serde(alias = "require_parameters")]
    require_parameter: bool,
}

impl From<ProviderFields> for Provider {
    fn from(value: ProviderFields) -> Self {
        Self::RequireParameter(value.require_parameter)
    }
}
