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

#[graphql_object(context=crate::graphql::GraphqlContext)]
impl Provider {
    fn logo_url(&self) -> Option<&str> {
        self.0.logo_url.as_deref()
    }
    fn description(&self) -> Option<&str> {
        self.0.description.as_deref()
    }
    fn connection(&self) -> super::configuration::ConnectionConfiguration {
        (&self.0.connection).into()
    }
    async fn models(
        &self,
        #[graphql(context)] authority: &crate::http::authorization::AuthorizationContext,
    ) -> Result<Vec<ProviderModel>, crate::graphql::error::Error> {
        use compallm_core::authorization::{condition::CheckPermission, policy::Action};
        let permission = authority.member_permission().await?;
        let mut models = Vec::new();
        for model in &self.0.models {
            Action::ProviderModelRead
                .check_permission(&permission)
                .map_err(compallm_core::authorization::error::AuthorizationError::from)?;
            models.push(ProviderModel::from(model.clone()));
        }
        Ok(models)
    }
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
    fn capabilities(&self) -> Vec<super::input::ProviderModelCapability> {
        self.0
            .supported_capabilities()
            .into_iter()
            .map(Into::into)
            .collect()
    }
    fn input_price(&self) -> Option<String> {
        super::pricing::TokenDefaults::from(&self.0.text).input
    }
    fn output_price(&self) -> Option<String> {
        super::pricing::TokenDefaults::from(&self.0.text).output
    }
    fn cached_price(&self) -> Option<String> {
        super::pricing::TokenDefaults::from(&self.0.text).cached
    }

    fn protocol(&self) -> super::configuration::UpstreamProtocol {
        self.0.protocol.into()
    }
    fn connection(&self) -> super::configuration::ConnectionConfiguration {
        (&self.0.connection).into()
    }
    fn text(&self) -> super::capability::TextCapabilities {
        (&self.0.text).into()
    }
    fn context_tokens(&self) -> Option<super::scalar::ContextTokens> {
        self.0.limits.context_tokens.and_then(|v| v.try_into().ok())
    }

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
