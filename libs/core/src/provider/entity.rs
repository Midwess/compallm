use super::configuration::{
    Capability, Connection, Headers, ModelLimits, Pricing, Protocol, ServiceKind, Thinking,
};
use super::error::ProviderApiKeyError;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Provider {
    pub id: u64,
    pub organization_id: u64,
    pub name: String,
    pub connection: Connection,
    pub headers: Headers,
    pub models: Vec<ProviderModel>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProviderModel {
    pub id: u64,
    pub organization_id: u64,
    pub provider_id: u64,
    pub name: String,
    pub enabled: bool,
    pub service_kind: ServiceKind,
    pub protocol: Protocol,
    pub connection: Connection,
    pub headers: Headers,
    pub capabilities: Vec<Capability>,
    pub limits: ModelLimits,
    pub thinking: Option<Thinking>,
    pub pricing: Pricing,
}

/// Secret value persisted in the provider's `credentials` column.
///
/// This type deliberately does not implement `Debug`, `Display`, `Clone`, or
/// serialization. Callers must protect the value before it crosses a trust
/// boundary; repositories never return it from ordinary reads.
pub struct ProviderApiKey(String);

impl ProviderApiKey {
    pub fn new(value: impl Into<String>) -> Result<Self, ProviderApiKeyError> {
        let value = value.into();
        if value.is_empty() {
            return Err(ProviderApiKeyError);
        }
        Ok(Self(value))
    }

    pub fn into_storage_value(self) -> String {
        self.0
    }
}
