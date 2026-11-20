use super::super::input::{ProviderApiProtocol, ProviderModelCapability, TextModelProtocol};
use crate::{
    graphql::{base_type::Collection, error::Error},
    http::authorization::AuthorizationContext,
};
use compallm_core::provider_template as domain;
use compallm_postgres::catalog::repository::CatalogRepository;
use juniper::{GraphQLEnum, GraphQLObject, ID, graphql_object};

#[derive(Clone, Debug)]
pub struct ProviderTemplate(
    pub domain::ProviderTemplateSummary,
    std::sync::Arc<tokio::sync::OnceCell<domain::ProviderTemplate>>,
);
impl From<domain::ProviderTemplateSummary> for ProviderTemplate {
    fn from(v: domain::ProviderTemplateSummary) -> Self {
        Self(v, Default::default())
    }
}
impl From<domain::ProviderTemplate> for ProviderTemplate {
    fn from(value: domain::ProviderTemplate) -> Self {
        Self(
            domain::ProviderTemplateSummary::from(&value),
            std::sync::Arc::new(tokio::sync::OnceCell::new_with(Some(value))),
        )
    }
}
impl ProviderTemplate {
    async fn detail(
        &self,
        repository: &CatalogRepository,
        authority: &AuthorizationContext,
    ) -> Result<&domain::ProviderTemplate, Error> {
        authority.member_permission().await?;
        self.1
            .get_or_try_init(|| async {
                repository
                    .provider(&self.0.id)
                    .await
                    .map_err(|_| Error::internal())?
                    .ok_or_else(Error::internal)
            })
            .await
    }
}
#[derive(Clone, Debug)]
pub struct ModelTemplate(
    pub domain::ModelTemplateSummary,
    std::sync::Arc<tokio::sync::OnceCell<domain::ModelTemplate>>,
);
impl From<domain::ModelTemplateSummary> for ModelTemplate {
    fn from(v: domain::ModelTemplateSummary) -> Self {
        Self(v, Default::default())
    }
}
impl From<domain::ModelTemplate> for ModelTemplate {
    fn from(value: domain::ModelTemplate) -> Self {
        Self(
            domain::ModelTemplateSummary::from(&value),
            std::sync::Arc::new(tokio::sync::OnceCell::new_with(Some(value))),
        )
    }
}

#[derive(Clone, Copy, GraphQLEnum)]
pub enum TemplateReadiness {
    Ready,
    NeedsFetch,
    NeedsConfiguration,
    Incomplete,
}
impl From<domain::Readiness> for TemplateReadiness {
    fn from(value: domain::Readiness) -> Self {
        match value {
            domain::Readiness::Ready => Self::Ready,
            domain::Readiness::NeedsFetch => Self::NeedsFetch,
            domain::Readiness::NeedsConfiguration => Self::NeedsConfiguration,
            domain::Readiness::Incomplete => Self::Incomplete,
        }
    }
}
#[derive(GraphQLObject)]
pub struct TemplateHeader {
    pub key: String,
    pub value: String,
}
#[derive(GraphQLObject)]
pub struct TemplateOperation {
    pub operation: TextModelProtocol,
    pub path: String,
}
#[derive(GraphQLObject)]
pub struct TemplateEffort {
    pub level: TemplateEffortLevel,
    pub value: String,
}
#[derive(GraphQLObject)]
pub struct TemplateAuthentication {
    pub method: TemplateAuthenticationMethod,
    pub configuration: super::output::TemplateAuthConfig,
}
#[derive(GraphQLObject)]
pub struct TemplateEndpoint {
    pub mode: TemplateEndpointMode,
    pub configuration: super::output::TemplateEndpointConfig,
}
#[derive(GraphQLObject)]
pub struct TemplateUnresolved {
    pub field: String,
    pub requirement: super::output::TemplateUnresolvedField,
}

#[derive(GraphQLEnum)]
pub enum TemplateEffortLevel {
    Low,
    Medium,
    High,
    Xhigh,
    Max,
}
impl From<domain::EffortLevel> for TemplateEffortLevel {
    fn from(value: domain::EffortLevel) -> Self {
        match value {
            domain::EffortLevel::Low => Self::Low,
            domain::EffortLevel::Medium => Self::Medium,
            domain::EffortLevel::High => Self::High,
            domain::EffortLevel::Xhigh => Self::Xhigh,
            domain::EffortLevel::Max => Self::Max,
        }
    }
}
impl From<domain::TextOperation> for TextModelProtocol {
    fn from(v: domain::TextOperation) -> Self {
        match v {
            domain::TextOperation::OpenaiChatCompletions => Self::OpenaiChatCompletions,
            domain::TextOperation::OpenaiResponses => Self::OpenaiResponses,
            domain::TextOperation::AnthropicMessages => Self::AnthropicMessages,
        }
    }
}
impl ModelTemplate {
    async fn detail(
        &self,
        repository: &CatalogRepository,
        authority: &AuthorizationContext,
    ) -> Result<&domain::ModelTemplate, Error> {
        authority.member_permission().await?;
        self.1
            .get_or_try_init(|| async {
                repository
                    .model(&self.0.provider_template_id, &self.0.id)
                    .await
                    .map_err(|_| Error::internal())?
                    .ok_or_else(Error::internal)
            })
            .await
    }
    pub fn compatible(value: &domain::ModelTemplateSummary) -> bool {
        !value.operations.is_empty()
            && value.operations.iter().any(|(operation, path)| {
                let input = super::super::input::ProviderModelConfiguration {
                    name: value.upstream_model_id.clone(),
                    protocol: (*operation).into(),
                    path: path.clone(),
                    max_context_tokens: match value
                        .max_context_tokens
                        .and_then(|v| super::super::scalar::ContextTokens::try_from(v).ok())
                    {
                        Some(v) => v,
                        None => return false,
                    },
                    context_limit_enabled: true,
                    max_input_tokens: value
                        .max_input_tokens
                        .map(super::super::scalar::ContextTokens::try_from)
                        .transpose()
                        .ok()
                        .flatten(),
                    max_output_tokens: value
                        .max_output_tokens
                        .map(super::super::scalar::ContextTokens::try_from)
                        .transpose()
                        .ok()
                        .flatten(),
                    headers: vec![],
                    body_defaults: vec![],
                    enabled: true,
                    effort_mapping: Some(super::super::input::ProviderEffortMapping {
                        low: value.effort_mapping.get(&domain::EffortLevel::Low).cloned(),
                        medium: value
                            .effort_mapping
                            .get(&domain::EffortLevel::Medium)
                            .cloned(),
                        high: value
                            .effort_mapping
                            .get(&domain::EffortLevel::High)
                            .cloned(),
                        xhigh: value
                            .effort_mapping
                            .get(&domain::EffortLevel::Xhigh)
                            .cloned(),
                        max: value.effort_mapping.get(&domain::EffortLevel::Max).cloned(),
                    }),
                    capabilities: vec![],
                    catalog_ref: None,
                    input_price: value.input_price.map(|v| v.to_string()).unwrap_or_default(),
                    output_price: value
                        .output_price
                        .map(|v| v.to_string())
                        .unwrap_or_default(),
                    cached_price: value
                        .cached_price
                        .map(|v| v.to_string())
                        .unwrap_or_default(),
                    input_price_enabled: true,
                    output_price_enabled: true,
                    cached_price_enabled: true,
                };
                validify::Validate::validate(&input).is_ok()
            })
    }
}

#[graphql_object(context=crate::graphql::context::GraphqlContext, scalar=juniper::DefaultScalarValue)]
impl ProviderTemplate {
    fn id(&self) -> ID {
        ID::new(&self.0.id)
    }
    fn name(&self) -> &str {
        &self.0.name
    }
    fn description(&self) -> Option<&str> {
        self.0.description.as_deref()
    }
    fn logo_path(&self) -> &str {
        &self.0.logo_path
    }
    fn protocol(&self) -> ProviderApiProtocol {
        match self.0.protocol {
            domain::ProtocolFamily::OpenaiCompatible => ProviderApiProtocol::OpenaiCompatible,
            domain::ProtocolFamily::Anthropic => ProviderApiProtocol::Anthropic,
            domain::ProtocolFamily::ProviderSpecific => ProviderApiProtocol::ProviderSpecific,
        }
    }
    fn base_url(&self) -> Option<&str> {
        self.0.base_url.as_deref()
    }
    async fn headers(
        &self,
        #[graphql(context)] repository: &CatalogRepository,
        #[graphql(context)] authority: &AuthorizationContext,
    ) -> Result<Vec<TemplateHeader>, Error> {
        let detail = self.detail(repository, authority).await?;
        Ok(detail
            .headers
            .iter()
            .map(|(key, value)| TemplateHeader {
                key: key.clone(),
                value: value.clone(),
            })
            .collect())
    }
    fn readiness(&self) -> TemplateReadiness {
        self.0.readiness.into()
    }
    async fn endpoint(
        &self,
        #[graphql(context)] repository: &CatalogRepository,
        #[graphql(context)] authority: &AuthorizationContext,
    ) -> Result<TemplateEndpoint, Error> {
        let detail = self.detail(repository, authority).await?;
        Ok(TemplateEndpoint {
            mode: detail.endpoint_mode.into(),
            configuration: detail.endpoint_config.clone().into(),
        })
    }
    async fn authentication(
        &self,
        #[graphql(context)] repository: &CatalogRepository,
        #[graphql(context)] authority: &AuthorizationContext,
    ) -> Result<Vec<TemplateAuthentication>, Error> {
        let detail = self.detail(repository, authority).await?;
        Ok(detail
            .authentication
            .iter()
            .map(|(method, v)| TemplateAuthentication {
                method: (*method).into(),
                configuration: v.clone().into(),
            })
            .collect())
    }
    async fn models(
        &self,
        executor: &juniper::Executor<'_, '_, crate::graphql::context::GraphqlContext>,
        #[graphql(context)] repository: &CatalogRepository,
        #[graphql(context)] authority: &AuthorizationContext,
    ) -> Result<Collection<ModelTemplate>, Error> {
        let member = authority.member_permission().await?;
        let mut nodes: Vec<ModelTemplate> = repository
            .models(&self.0.id)
            .await
            .map_err(|_| Error::internal())?
            .into_iter()
            .filter(ModelTemplate::compatible)
            .map(Into::into)
            .collect();
        let selection = executor.look_ahead();
        let detailed = selection
            .children()
            .iter()
            .filter(|field| field.field_original_name() == "edges")
            .any(|edge| {
                edge.children()
                    .iter()
                    .filter(|field| field.field_original_name() == "node")
                    .any(|node| {
                        node.children().iter().any(|field| {
                            !matches!(
                                field.field_original_name(),
                                "id" | "providerTemplateId"
                                    | "upstreamModelId"
                                    | "displayName"
                                    | "description"
                                    | "operations"
                                    | "capabilities"
                                    | "capabilitySupport"
                                    | "readiness"
                                    | "__typename"
                            )
                        })
                    })
            });
        if detailed {
            let details = repository
                .model_details(
                    &self.0.id,
                    nodes.iter().map(|node| node.0.id.clone()).collect(),
                )
                .await
                .map_err(|_| Error::internal())?;
            let mut details: std::collections::BTreeMap<_, _> = details
                .into_iter()
                .map(|model| (model.id.clone(), model))
                .collect();
            for node in &mut nodes {
                if let Some(detail) = details.remove(&node.0.id) {
                    node.1 = std::sync::Arc::new(tokio::sync::OnceCell::new_with(Some(detail)));
                }
            }
        }
        Collection::complete(
            nodes,
            &format!(
                "template-models:{}:{}:{}",
                member.organization_id(),
                member.user_id(),
                self.0.id
            ),
            |m: &ModelTemplate| vec![Some(m.0.upstream_model_id.clone()), Some(m.0.id.clone())],
        )
        .map_err(|_| Error::internal())
    }
}
#[graphql_object(context=crate::graphql::context::GraphqlContext, scalar=juniper::DefaultScalarValue)]
impl ModelTemplate {
    fn id(&self) -> ID {
        ID::new(&self.0.id)
    }
    fn provider_template_id(&self) -> ID {
        ID::new(&self.0.provider_template_id)
    }
    fn upstream_model_id(&self) -> &str {
        &self.0.upstream_model_id
    }
    fn display_name(&self) -> &str {
        self.0
            .display_name
            .as_deref()
            .unwrap_or(&self.0.upstream_model_id)
    }
    fn description(&self) -> Option<&str> {
        self.0.description.as_deref()
    }
    fn operations(&self) -> Vec<TemplateOperation> {
        self.0
            .operations
            .iter()
            .map(|(op, path)| TemplateOperation {
                operation: (*op).into(),
                path: path.clone(),
            })
            .collect()
    }
    fn capabilities(&self) -> Vec<ProviderModelCapability> {
        self.0
            .capabilities
            .iter()
            .filter(|(_, value)| **value == Some(true))
            .map(|(capability, _)| (*capability).into())
            .collect()
    }
    fn capability_support(&self) -> Vec<TemplateCapabilitySupport> {
        self.0
            .capabilities
            .iter()
            .map(|(capability, supported)| TemplateCapabilitySupport {
                capability: (*capability).into(),
                supported: *supported,
            })
            .collect()
    }
    async fn max_context_tokens(
        &self,
        #[graphql(context)] repository: &CatalogRepository,
        #[graphql(context)] authority: &AuthorizationContext,
    ) -> Result<Option<super::super::scalar::ContextTokens>, Error> {
        let detail = self.detail(repository, authority).await?;
        detail
            .max_context_tokens
            .map(super::super::scalar::ContextTokens::try_from)
            .transpose()
            .map_err(|_| Error::internal())
    }
    async fn max_input_tokens(
        &self,
        #[graphql(context)] repository: &CatalogRepository,
        #[graphql(context)] authority: &AuthorizationContext,
    ) -> Result<Option<super::super::scalar::ContextTokens>, Error> {
        let detail = self.detail(repository, authority).await?;
        detail
            .max_input_tokens
            .map(super::super::scalar::ContextTokens::try_from)
            .transpose()
            .map_err(|_| Error::internal())
    }
    async fn max_output_tokens(
        &self,
        #[graphql(context)] repository: &CatalogRepository,
        #[graphql(context)] authority: &AuthorizationContext,
    ) -> Result<Option<super::super::scalar::ContextTokens>, Error> {
        let detail = self.detail(repository, authority).await?;
        detail
            .max_output_tokens
            .map(super::super::scalar::ContextTokens::try_from)
            .transpose()
            .map_err(|_| Error::internal())
    }
    async fn effort_mapping(
        &self,
        #[graphql(context)] repository: &CatalogRepository,
        #[graphql(context)] authority: &AuthorizationContext,
    ) -> Result<Vec<TemplateEffort>, Error> {
        let detail = self.detail(repository, authority).await?;
        Ok(detail
            .effort_mapping
            .iter()
            .map(|(level, value)| TemplateEffort {
                level: (*level).into(),
                value: value.clone(),
            })
            .collect())
    }
    async fn input_price(
        &self,
        #[graphql(context)] repository: &CatalogRepository,
        #[graphql(context)] authority: &AuthorizationContext,
    ) -> Result<Option<String>, Error> {
        let detail = self.detail(repository, authority).await?;
        Ok(detail.input_price.map(|v| v.to_string()))
    }
    async fn output_price(
        &self,
        #[graphql(context)] repository: &CatalogRepository,
        #[graphql(context)] authority: &AuthorizationContext,
    ) -> Result<Option<String>, Error> {
        let detail = self.detail(repository, authority).await?;
        Ok(detail.output_price.map(|v| v.to_string()))
    }
    async fn cached_price(
        &self,
        #[graphql(context)] repository: &CatalogRepository,
        #[graphql(context)] authority: &AuthorizationContext,
    ) -> Result<Option<String>, Error> {
        let detail = self.detail(repository, authority).await?;
        Ok(detail.cached_price.map(|v| v.to_string()))
    }
    async fn billing_model(
        &self,
        #[graphql(context)] repository: &CatalogRepository,
        #[graphql(context)] authority: &AuthorizationContext,
    ) -> Result<TemplateBillingModel, Error> {
        let detail = self.detail(repository, authority).await?;
        Ok(detail.billing_model.into())
    }
    async fn pricing_mode(
        &self,
        #[graphql(context)] repository: &CatalogRepository,
        #[graphql(context)] authority: &AuthorizationContext,
    ) -> Result<TemplatePricingMode, Error> {
        let detail = self.detail(repository, authority).await?;
        Ok(detail.pricing_mode.into())
    }
    async fn cache_billing(
        &self,
        #[graphql(context)] repository: &CatalogRepository,
        #[graphql(context)] authority: &AuthorizationContext,
    ) -> Result<TemplateCacheBilling, Error> {
        let detail = self.detail(repository, authority).await?;
        Ok(detail.cache_billing.into())
    }
    fn readiness(&self) -> TemplateReadiness {
        TemplateReadiness::Ready
    }
    async fn service_kind(
        &self,
        #[graphql(context)] repository: &CatalogRepository,
        #[graphql(context)] authority: &AuthorizationContext,
    ) -> Result<TemplateServiceKind, Error> {
        Ok(self
            .detail(repository, authority)
            .await?
            .service_kind
            .into())
    }
    async fn specification_mode(
        &self,
        #[graphql(context)] repository: &CatalogRepository,
        #[graphql(context)] authority: &AuthorizationContext,
    ) -> Result<TemplateSpecificationMode, Error> {
        Ok(self
            .detail(repository, authority)
            .await?
            .specification_mode
            .into())
    }
    async fn unresolved_fields(
        &self,
        #[graphql(context)] repository: &CatalogRepository,
        #[graphql(context)] authority: &AuthorizationContext,
    ) -> Result<Vec<TemplateUnresolved>, Error> {
        Ok(self
            .detail(repository, authority)
            .await?
            .unresolved_fields
            .iter()
            .map(|(field, v)| TemplateUnresolved {
                field: field.clone(),
                requirement: v.clone().into(),
            })
            .collect())
    }
}

#[derive(GraphQLEnum)]
pub enum TemplateAuthenticationMethod {
    ApiKey,
    Oauth,
    Custom,
    None,
}
impl From<domain::AuthenticationMethod> for TemplateAuthenticationMethod {
    fn from(v: domain::AuthenticationMethod) -> Self {
        match v {
            domain::AuthenticationMethod::ApiKey => Self::ApiKey,
            domain::AuthenticationMethod::Oauth => Self::Oauth,
            domain::AuthenticationMethod::Custom => Self::Custom,
            domain::AuthenticationMethod::None => Self::None,
        }
    }
}

#[derive(GraphQLEnum)]
pub enum TemplateEndpointMode {
    Fixed,
    AccountSpecific,
    Discovered,
    Unknown,
}
impl From<domain::EndpointMode> for TemplateEndpointMode {
    fn from(v: domain::EndpointMode) -> Self {
        match v {
            domain::EndpointMode::Fixed => Self::Fixed,
            domain::EndpointMode::AccountSpecific => Self::AccountSpecific,
            domain::EndpointMode::Discovered => Self::Discovered,
            domain::EndpointMode::Unknown => Self::Unknown,
        }
    }
}

#[derive(GraphQLEnum)]
pub enum TemplateBillingModel {
    Metered,
    Subscription,
    SubscriptionWithOverage,
    Free,
    Unknown,
}
impl From<domain::BillingModel> for TemplateBillingModel {
    fn from(v: domain::BillingModel) -> Self {
        match v {
            domain::BillingModel::Metered => Self::Metered,
            domain::BillingModel::Subscription => Self::Subscription,
            domain::BillingModel::SubscriptionWithOverage => Self::SubscriptionWithOverage,
            domain::BillingModel::Free => Self::Free,
            domain::BillingModel::Unknown => Self::Unknown,
        }
    }
}

#[derive(GraphQLEnum)]
pub enum TemplatePricingMode {
    Fixed,
    Tiered,
    Api,
    AccountSpecific,
    NotApplicable,
    Unpublished,
    Unknown,
    Scheduled,
}
impl From<domain::PricingMode> for TemplatePricingMode {
    fn from(v: domain::PricingMode) -> Self {
        match v {
            domain::PricingMode::Fixed => Self::Fixed,
            domain::PricingMode::Tiered => Self::Tiered,
            domain::PricingMode::Api => Self::Api,
            domain::PricingMode::AccountSpecific => Self::AccountSpecific,
            domain::PricingMode::NotApplicable => Self::NotApplicable,
            domain::PricingMode::Unpublished => Self::Unpublished,
            domain::PricingMode::Unknown => Self::Unknown,
            domain::PricingMode::Scheduled => Self::Scheduled,
        }
    }
}

#[derive(GraphQLEnum)]
pub enum TemplateCacheBilling {
    SeparateRate,
    StandardInputRate,
    Unsupported,
    Included,
    NotApplicable,
    PerRoute,
    Unknown,
}
impl From<domain::CacheBilling> for TemplateCacheBilling {
    fn from(v: domain::CacheBilling) -> Self {
        match v {
            domain::CacheBilling::SeparateRate => Self::SeparateRate,
            domain::CacheBilling::StandardInputRate => Self::StandardInputRate,
            domain::CacheBilling::Unsupported => Self::Unsupported,
            domain::CacheBilling::Included => Self::Included,
            domain::CacheBilling::NotApplicable => Self::NotApplicable,
            domain::CacheBilling::PerRoute => Self::PerRoute,
            domain::CacheBilling::Unknown => Self::Unknown,
        }
    }
}

#[derive(GraphQLEnum)]
pub enum TemplateSpecificationMode {
    Stored,
    Api,
    AccountSpecific,
    NotApplicable,
    Unknown,
}
impl From<domain::SpecificationMode> for TemplateSpecificationMode {
    fn from(v: domain::SpecificationMode) -> Self {
        match v {
            domain::SpecificationMode::Stored => Self::Stored,
            domain::SpecificationMode::Api => Self::Api,
            domain::SpecificationMode::AccountSpecific => Self::AccountSpecific,
            domain::SpecificationMode::NotApplicable => Self::NotApplicable,
            domain::SpecificationMode::Unknown => Self::Unknown,
        }
    }
}

#[derive(GraphQLEnum)]
pub enum TemplateServiceKind {
    Text,
    Embedding,
    Image,
    Audio,
    Video,
    Rerank,
    Unknown,
}
impl From<domain::ServiceKind> for TemplateServiceKind {
    fn from(v: domain::ServiceKind) -> Self {
        match v {
            domain::ServiceKind::Text => Self::Text,
            domain::ServiceKind::Embedding => Self::Embedding,
            domain::ServiceKind::Image => Self::Image,
            domain::ServiceKind::Audio => Self::Audio,
            domain::ServiceKind::Video => Self::Video,
            domain::ServiceKind::Rerank => Self::Rerank,
            domain::ServiceKind::Unknown => Self::Unknown,
        }
    }
}

impl From<domain::Capability> for ProviderModelCapability {
    fn from(v: domain::Capability) -> Self {
        match v {
            domain::Capability::Text => Self::Text,
            domain::Capability::Vision => Self::Vision,
            domain::Capability::Pdf => Self::Pdf,
            domain::Capability::Audio => Self::Audio,
            domain::Capability::Video => Self::Video,
            domain::Capability::ImageGeneration => Self::ImageGeneration,
            domain::Capability::AudioGeneration => Self::AudioGeneration,
            domain::Capability::Search => Self::Search,
            domain::Capability::Reasoning => Self::Reasoning,
            domain::Capability::ToolCall => Self::ToolCall,
            domain::Capability::ForcedToolChoice => Self::ForcedToolChoice,
            domain::Capability::StructuredOutput => Self::StructuredOutput,
        }
    }
}

#[derive(GraphQLObject)]
pub struct TemplateCapabilitySupport {
    pub capability: ProviderModelCapability,
    pub supported: Option<bool>,
}
