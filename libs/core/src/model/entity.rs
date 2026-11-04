use crate::provider::{Capability, ModelLimits, Pricing, ServiceKind, Thinking};

/// An organization-owned logical model and its organization-facing prices.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Model {
    pub id: u64,
    pub organization_id: u64,
    pub name: String,
    pub service_kind: ServiceKind,
    pub enabled: bool,
    pub capabilities: Vec<Capability>,
    pub limits: ModelLimits,
    pub thinking: Option<Thinking>,
    pub pricing: Pricing,
}
