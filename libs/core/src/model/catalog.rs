use crate::provider::capability::*;
use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::provider::{
    BodyDefaults, Capability, Connection, Headers, ModelLimits, Protocol, RouteSelection,
    ServiceKind,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ModelCatalog {
    pub id: String,
    #[serde(default)]
    pub custom_fields: std::collections::BTreeSet<String>,
    pub research_id: Option<String>,
    pub provider_catalog_id: String,
    pub name: String,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub service_kind: Option<ServiceKind>,
    pub connections: BTreeMap<Protocol, Connection>,
    pub headers: Headers,
    pub body_defaults: BodyDefaults,
    pub text: TextCapabilities,
    pub vision: CapabilitySupport<VisionCapability>,
    pub pdf: CapabilitySupport<PdfCapability>,
    pub audio: AudioCapabilities,
    pub video: CapabilitySupport<VideoCapability>,
    pub image_generation: CapabilitySupport<ImageGenerationCapability>,
    pub tools: ToolCapabilities,
    pub structured_output: CapabilitySupport<StructuredOutputCapability>,
    pub search: CapabilitySupport<SearchCapability>,
    pub limits: ModelLimits,
    pub billing: Option<crate::provider::ModelBilling>,
    pub route_selection: Option<RouteSelection>,
}

impl ModelCatalog {
    pub(crate) fn capability_fields(&self) -> CapabilityFields<'_> {
        CapabilityFields {
            text: &self.text,
            vision: &self.vision,
            pdf: &self.pdf,
            audio: &self.audio,
            video: &self.video,
            image_generation: &self.image_generation,
            tools: &self.tools,
            structured_output: &self.structured_output,
            search: &self.search,
        }
    }
    pub fn supported_capabilities(&self) -> Vec<Capability> {
        self.capability_fields().supported()
    }
    pub fn unknown_capabilities(&self) -> Vec<Capability> {
        self.capability_fields().unknown()
    }
    pub fn unsupported_capabilities(&self) -> Vec<Capability> {
        self.capability_fields().unsupported()
    }
}
