use crate::provider::capability::*;
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
    pub research_id: String,
    pub provider_template_id: String,
    pub name: String,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub service_kind: Option<ServiceKind>,
    pub connections: BTreeMap<Protocol, Connection>,
    pub headers: Headers,
    pub body_defaults: BodyDefaults,
    pub text: FeatureSupport,
    pub vision: CapabilitySupport<VisionCapability>,
    pub pdf: CapabilitySupport<PdfCapability>,
    pub audio: AudioCapabilities,
    pub video: CapabilitySupport<VideoCapability>,
    pub image_generation: CapabilitySupport<ImageGenerationCapability>,
    pub thinking: CapabilitySupport<Thinking>,
    pub tools: ToolCapabilities,
    pub structured_output: CapabilitySupport<StructuredOutputCapability>,
    pub search: CapabilitySupport<SearchCapability>,
    pub limits: ModelLimits,
    pub pricing: Option<Pricing>,
    pub route_selection: Option<RouteSelection>,
}

impl ModelTemplate {
    fn capability_fields(&self) -> CapabilityFields<'_> {
        CapabilityFields {
            text: &self.text,
            vision: &self.vision,
            pdf: &self.pdf,
            audio: &self.audio,
            video: &self.video,
            image_generation: &self.image_generation,
            thinking: &self.thinking,
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
