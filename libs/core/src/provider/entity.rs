use super::ProviderApiKey;
use super::configuration::{
    Capability, Connection, Headers, ModelLimits, Pricing, Protocol, ServiceKind, Thinking,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Provider {
    pub id: u64,
    pub organization_id: u64,
    pub name: String,
    pub connection: Connection,
    pub headers: Headers,
    pub api_key: ProviderApiKey,
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
