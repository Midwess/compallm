use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::provider::{
    BodyDefaults, Capability, Connection, Headers, ModelLimits, Pricing, Protocol, RouteSelection,
    ServiceKind, Thinking,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ModelTemplate {
    pub id: String,
    pub provider_template_id: String,
    pub name: String,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub service_kind: Option<ServiceKind>,
    pub connections: BTreeMap<Protocol, Connection>,
    pub headers: Headers,
    pub body_defaults: BodyDefaults,
    pub capabilities: Vec<Capability>,
    pub limits: ModelLimits,
    pub thinking: Option<Thinking>,
    pub pricing: Option<Pricing>,
    pub route_selection: Option<RouteSelection>,
}

impl ModelTemplate {
    pub fn unsupported_capabilities(&self) -> Vec<Capability> {
        crate::provider::ALL_CAPABILITIES
            .iter()
            .copied()
            .filter(|capability| !self.capabilities.contains(capability))
            .collect()
    }
}
