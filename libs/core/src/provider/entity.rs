use crate::provider::capability::*;
use hero::distributed_id::{generator::DistributedIdError, try_gen_id};
use std::sync::LazyLock;

use cedar_policy::SchemaFragment;

use crate::authorization::{
    SystemPolicy,
    policy::{EntityType, Policy},
};

use super::ProviderApiKey;
use super::configuration::{
    BodyDefaults, Connection, Headers, ModelCatalogReference, ModelLimits, Protocol,
    RouteSelection, ServiceKind,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Provider {
    pub id: u64,
    pub organization_id: u64,
    pub name: String,
    pub description: Option<String>,
    pub logo_url: Option<String>,
    pub template_id: Option<String>,
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
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub template: Option<ModelCatalogReference>,
    pub enabled: bool,
    pub service_kind: ServiceKind,
    pub protocol: Protocol,
    pub connection: Connection,
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

impl Provider {
    /// Initialize a new aggregate identity and derive configured-model ownership.
    pub async fn initialize_ids(&mut self) -> Result<(), DistributedIdError> {
        self.id = try_gen_id().await?;
        for model in &mut self.models {
            model.initialize_id().await?;
            model.organization_id = self.organization_id;
            model.provider_id = self.id;
        }
        Ok(())
    }
}

impl ProviderModel {
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
    pub fn validate_capabilities(&self) -> Result<(), validify::ValidationErrors> {
        self.capability_fields().validate(self.billing.as_ref())
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

    pub async fn initialize_id(&mut self) -> Result<(), DistributedIdError> {
        self.id = try_gen_id().await?;
        Ok(())
    }
}

impl SystemPolicy for Provider {
    /// Declares connection management without exposing provider secrets.
    ///
    /// - `provider.read`: list providers and read their non-secret configuration;
    ///   omit API keys and stored secret-header values from every response.
    /// - `provider.create`: create a connection in the destination Organization; creation
    ///   neither performs inference nor requires a successful connection test.
    ///   This targets the container, not a nonexistent Provider; admission, timeout,
    ///   upstream billing, and network-safety contracts remain required before shipping.
    /// - `provider.configuration.update`: change permitted non-secret connection configuration.
    /// - `provider.credential.replace`: replace a key without revealing its predecessor;
    ///   generic update does not imply this action, and no credential-read action exists.
    /// - `provider.delete`: delete the connection through scoped retention/route cleanup.
    ///
    /// A full configuration replacement that changes models, prices, or credentials
    /// must check their additional actions. Concrete targets, organization isolation,
    /// safe field projection, and deletion dependencies are service responsibilities.
    fn policy() -> SchemaFragment {
        static DEFINITION: LazyLock<SchemaFragment> =
            LazyLock::new(|| Policy::new(EntityType::Provider).into());
        DEFINITION.clone()
    }
}

impl SystemPolicy for ProviderModel {
    /// Declares configured upstream models, distinct from logical Model resources.
    ///
    /// - `provider.model.read`: list configured models and inspect their non-secret configuration,
    ///   including configured costs; no API keys or secret header values are returned.
    /// - `provider.model.create`: configure an upstream model beneath that Provider;
    ///   it grants neither logical-model creation nor inference invocation.
    /// - `provider.model.configuration.update`: edit permitted routing/protocol/availability settings;
    ///   pricing and credential replacement require additional permissions.
    /// - `provider.model.pricing.update`: change upstream costs for this ProviderModel,
    ///   separate from organization-facing logical-model pricing.
    /// - `provider.model.delete`: remove this configuration with scoped binding cleanup;
    ///   logical models and usage retention follow their own deletion contracts.
    ///
    /// The provider and model must share their actual organization. A typed provider
    /// reference alone does not prove that consistency. Reads/updates can target disabled
    /// models; this fragment supplies no grants or inference permission.
    fn policy() -> SchemaFragment {
        static DEFINITION: LazyLock<SchemaFragment> =
            LazyLock::new(|| Policy::new(EntityType::ProviderModel).into());
        DEFINITION.clone()
    }
}
