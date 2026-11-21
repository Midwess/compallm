use crate::provider::capability::*;
use hero::distributed_id::{generator::DistributedIdError, try_gen_id};
use std::sync::LazyLock;

use cedar_policy::SchemaFragment;

use crate::authorization::{
    SystemPolicy,
    policy::{EntityType, Policy},
};

use crate::provider::{Capability, ModelLimits, ServiceKind};

/// An organization-owned logical model and its organization-facing prices.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Model {
    pub id: u64,
    pub organization_id: u64,
    pub name: String,
    pub service_kind: ServiceKind,
    pub enabled: bool,
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
}

impl Model {
    fn capability_fields(&self) -> CapabilityFields<'_> {
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

impl SystemPolicy for Model {
    /// Declares logical-model discovery, invocation, and administration separately.
    ///
    /// - `model.read`: list models and inspect organization-visible metadata, capabilities,
    ///   and user-facing prices. It excludes provider credentials and privileged routing.
    /// - `model.create`: create a logical model inside the destination Organization.
    ///   Grants cover all models in the active organization; per-model grants are deferred.
    /// - `model.use`: invoke an available logical model. The grant does not enable a
    ///   disabled model or bypass spending limits, admission, or routing availability.
    /// - `model.configuration.update`: change permitted metadata/availability; pricing and route changes
    ///   additionally require their respective actions.
    /// - `model.route.add`, `model.route.update`, `model.route.remove`: change one binding
    ///   on this Model using `context.providerModel`. Load both endpoints and verify
    ///   same-organization ownership plus upstream-model delegation/compatibility.
    /// - `model.pricing.update`: change organization-facing prices, not upstream costs.
    /// - `model.delete`: delete the logical model through its binding/usage-retention flow.
    ///
    /// Inspecting or managing a disabled model is valid. All checks apply to actual
    /// resources; schema validation cannot enforce availability, quotas, or consistency.
    fn policy() -> SchemaFragment {
        static DEFINITION: LazyLock<SchemaFragment> =
            LazyLock::new(|| Policy::new(EntityType::Model).into());
        DEFINITION.clone()
    }
}
