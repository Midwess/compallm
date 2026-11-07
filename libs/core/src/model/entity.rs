use std::sync::LazyLock;

use cedar_policy::SchemaFragment;

use crate::authorization::SystemPolicy;

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
        static DEFINITION: LazyLock<SchemaFragment> = LazyLock::new(|| {
            let (fragment, warnings) = SchemaFragment::from_cedarschema_str(
                r#"
namespace Compallm {
    entity Model in [Organization] { organization: Organization, enabled: Bool };
    action "model.list" appliesTo { principal: User, resource: Organization };
    action "model.create" appliesTo { principal: User, resource: Organization };
    action "model.metadata.read" appliesTo { principal: User, resource: Model };
    action "model.configuration.update" appliesTo { principal: User, resource: Model };
    action "model.delete" appliesTo { principal: User, resource: Model };
    action "model.use" appliesTo { principal: User, resource: Model };
    action "model.route.add" appliesTo { principal: User, resource: Model, context: { providerModel: ProviderModel } };
    action "model.route.update" appliesTo { principal: User, resource: Model, context: { providerModel: ProviderModel } };
    action "model.route.remove" appliesTo { principal: User, resource: Model, context: { providerModel: ProviderModel } };
    action "model.pricing.update" appliesTo { principal: User, resource: Model };
}
"#,
            )
            .expect("Model SystemPolicy must be a valid static Cedar schema fragment");
            assert!(
                warnings.count() == 0,
                "Model SystemPolicy must not contain schema warnings"
            );
            fragment
        });
        DEFINITION.clone()
    }
}
