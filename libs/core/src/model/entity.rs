use hero::distributed_id::{generator::DistributedIdError, try_gen_id};
use std::sync::LazyLock;

use cedar_policy::SchemaFragment;

use crate::authorization::{
    SystemPolicy,
    policy::{EntityType, Policy},
};

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

impl Model {
    pub async fn initialize_id(&mut self) -> Result<(), DistributedIdError> {
        self.id = try_gen_id().await?;
        Ok(())
    }
}

impl SystemPolicy for Model {
    /// Declares logical-model discovery, invocation, and administration separately.
    ///
    /// - `model.list`: discover permitted model summaries in an Organization; it does
    ///   not expose provider credentials or privileged routing configuration.
    /// - `model.create`: create a logical model inside the destination Organization.
    /// - `model.metadata.read`: inspect permitted organization-visible model metadata such as capability
    ///   and configured user-facing prices. First-release grants cover all models in
    ///   the active organization; per-model grant selection is deferred.
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
