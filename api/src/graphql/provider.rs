use async_graphql::{ID, Object};
use compallm_core::provider::{Provider as CoreProvider, ProviderModel as CoreProviderModel};

/// Safe provider metadata returned after establishing the caller's permission.
///
/// Connections, headers, credentials, and upstream costs are never projected here.
pub struct Provider(CoreProvider);

impl From<CoreProvider> for Provider {
    fn from(provider: CoreProvider) -> Self {
        Self(provider)
    }
}

#[Object]
impl Provider {
    /// Stable provider identity, serialized as a string.
    async fn id(&self) -> ID {
        ID(self.0.id.to_string())
    }

    /// Owning organization; a supplied identifier never grants access.
    async fn organization_id(&self) -> ID {
        ID(self.0.organization_id.to_string())
    }

    /// Provider display name within its organization.
    async fn name(&self) -> &str {
        &self.0.name
    }
}

/// Safe configured-model metadata returned after authorizing its owning resources.
pub struct ProviderModel(CoreProviderModel);

impl From<CoreProviderModel> for ProviderModel {
    fn from(model: CoreProviderModel) -> Self {
        Self(model)
    }
}

#[Object]
impl ProviderModel {
    /// Stable configured-model identity, serialized as a string.
    async fn id(&self) -> ID {
        ID(self.0.id.to_string())
    }

    /// Owning organization identity.
    async fn organization_id(&self) -> ID {
        ID(self.0.organization_id.to_string())
    }

    /// Owning provider identity.
    async fn provider_id(&self) -> ID {
        ID(self.0.provider_id.to_string())
    }

    /// Configured upstream model name.
    async fn name(&self) -> &str {
        &self.0.name
    }

    /// Configured availability; this flag alone does not authorize invocation.
    async fn enabled(&self) -> bool {
        self.0.enabled
    }
}
