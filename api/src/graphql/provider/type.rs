use compallm_core::provider::{Provider as CoreProvider, ProviderModel as CoreProviderModel};

/// Safe provider metadata returned after establishing the caller's permission.
///
/// Connections, headers, credentials, and upstream costs are never projected here.
pub struct Provider(pub(super) CoreProvider);

impl From<CoreProvider> for Provider {
    fn from(provider: CoreProvider) -> Self {
        Self(provider)
    }
}

/// Safe configured-model metadata returned after authorizing its owning resources.
pub struct ProviderModel(pub(super) CoreProviderModel);

impl From<CoreProviderModel> for ProviderModel {
    fn from(model: CoreProviderModel) -> Self {
        Self(model)
    }
}

use juniper::{ID, graphql_object};

#[graphql_object]
impl Provider {
    /// Stable provider identity, serialized as a string.
    fn id(&self) -> ID {
        ID::new(self.0.id.to_string())
    }

    /// Owning organization; a supplied identifier never grants access.
    fn organization_id(&self) -> ID {
        ID::new(self.0.organization_id.to_string())
    }

    /// Provider display name within its organization.
    fn name(&self) -> &str {
        &self.0.name
    }
}

#[graphql_object]
impl ProviderModel {
    /// Stable configured-model identity, serialized as a string.
    fn id(&self) -> ID {
        ID::new(self.0.id.to_string())
    }

    /// Owning organization identity.
    fn organization_id(&self) -> ID {
        ID::new(self.0.organization_id.to_string())
    }

    /// Owning provider identity.
    fn provider_id(&self) -> ID {
        ID::new(self.0.provider_id.to_string())
    }

    /// Configured upstream model name.
    fn name(&self) -> &str {
        &self.0.name
    }

    /// Configured availability; this flag alone does not authorize invocation.
    fn enabled(&self) -> bool {
        self.0.enabled
    }
}
