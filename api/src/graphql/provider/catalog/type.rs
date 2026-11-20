use super::super::input::{ProviderApiProtocol, ProviderModelCapability, TextModelProtocol};
use crate::{
    graphql::{base_type::Collection, error::Error},
    http::authorization::AuthorizationContext,
};
use compallm_core::provider_template as domain;
use compallm_postgres::catalog::repository::CatalogRepository;
use juniper::{GraphQLEnum, GraphQLObject, ID, graphql_object};

#[derive(Clone, Debug)]
pub struct ProviderTemplate(pub domain::ProviderTemplate);
impl From<domain::ProviderTemplate> for ProviderTemplate {
    fn from(value: domain::ProviderTemplate) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug)]
pub struct ModelTemplate(pub domain::ModelTemplate);
impl From<domain::ModelTemplate> for ModelTemplate {
    fn from(value: domain::ModelTemplate) -> Self {
        Self(value)
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
    pub fn compatible(value: &domain::ModelTemplate) -> bool {
        !value.connection.operations.is_empty()
            && value.connection.operations.iter().any(|(operation, path)| {
                let input = super::super::input::ProviderModelConfiguration {
                    name: value.upstream_model_id.clone(),
                    protocol: (*operation).into(),
                    path: path.clone(),
                    max_context_tokens: match value
                        .limits
                        .max_context_tokens
                        .and_then(|v| super::super::scalar::ContextTokens::try_from(v).ok())
                    {
                        Some(v) => v,
                        None => return false,
                    },
                    context_limit_enabled: true,
                    max_input_tokens: value
                        .limits
                        .max_input_tokens
                        .map(super::super::scalar::ContextTokens::try_from)
                        .transpose()
                        .ok()
                        .flatten(),
                    max_output_tokens: value
                        .limits
                        .max_output_tokens
                        .map(super::super::scalar::ContextTokens::try_from)
                        .transpose()
                        .ok()
                        .flatten(),
                    headers: vec![],
                    body_defaults: vec![],
                    enabled: true,
                    effort_mapping: Some(super::super::input::ProviderEffortMapping {
                        low: value
                            .thinking
                            .effort_mapping
                            .get(&domain::EffortLevel::Low)
                            .cloned(),
                        medium: value
                            .thinking
                            .effort_mapping
                            .get(&domain::EffortLevel::Medium)
                            .cloned(),
                        high: value
                            .thinking
                            .effort_mapping
                            .get(&domain::EffortLevel::High)
                            .cloned(),
                        xhigh: value
                            .thinking
                            .effort_mapping
                            .get(&domain::EffortLevel::Xhigh)
                            .cloned(),
                        max: value
                            .thinking
                            .effort_mapping
                            .get(&domain::EffortLevel::Max)
                            .cloned(),
                    }),
                    capabilities: vec![],
                    catalog_ref: None,
                    input_price: value
                        .pricing
                        .input_price
                        .map(|v| v.to_string())
                        .unwrap_or_default(),
                    output_price: value
                        .pricing
                        .output_price
                        .map(|v| v.to_string())
                        .unwrap_or_default(),
                    cached_price: value
                        .pricing
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
        match self.0.connection.protocol {
            domain::ProtocolFamily::OpenaiCompatible => ProviderApiProtocol::OpenaiCompatible,
            domain::ProtocolFamily::Anthropic => ProviderApiProtocol::Anthropic,
            domain::ProtocolFamily::ProviderSpecific => ProviderApiProtocol::ProviderSpecific,
        }
    }
    fn base_url(&self) -> Option<&str> {
        self.0.connection.base_url.as_deref()
    }
    fn headers(&self) -> Result<Vec<TemplateHeader>, Error> {
        let detail = &self.0;
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
    fn endpoint(&self) -> Result<TemplateEndpoint, Error> {
        let detail = &self.0;
        Ok(TemplateEndpoint {
            mode: detail.connection.endpoint_mode.into(),
            configuration: detail.connection.endpoint.clone().into(),
        })
    }
    fn authentication(&self) -> Result<Vec<TemplateAuthentication>, Error> {
        let detail = &self.0;
        Ok(detail
            .connection
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
        #[graphql(context)] repository: &CatalogRepository,
        #[graphql(context)] authority: &AuthorizationContext,
    ) -> Result<Collection<ModelTemplate>, Error> {
        let member = authority.member_permission().await?;
        let nodes: Vec<ModelTemplate> = repository
            .models(&self.0.id)
            .await
            .map_err(|_| Error::internal())?
            .into_iter()
            .filter(ModelTemplate::compatible)
            .map(Into::into)
            .collect();
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
    fn headers(&self) -> Vec<TemplateHeader> {
        self.0
            .headers
            .iter()
            .map(|(key, value)| TemplateHeader {
                key: key.clone(),
                value: value.clone(),
            })
            .collect()
    }
    fn operations(&self) -> Vec<TemplateOperation> {
        self.0
            .connection
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
    fn max_context_tokens(&self) -> Result<Option<super::super::scalar::ContextTokens>, Error> {
        let detail = &self.0;
        detail
            .limits
            .max_context_tokens
            .map(super::super::scalar::ContextTokens::try_from)
            .transpose()
            .map_err(|_| Error::internal())
    }
    fn max_input_tokens(&self) -> Result<Option<super::super::scalar::ContextTokens>, Error> {
        let detail = &self.0;
        detail
            .limits
            .max_input_tokens
            .map(super::super::scalar::ContextTokens::try_from)
            .transpose()
            .map_err(|_| Error::internal())
    }
    fn max_output_tokens(&self) -> Result<Option<super::super::scalar::ContextTokens>, Error> {
        let detail = &self.0;
        detail
            .limits
            .max_output_tokens
            .map(super::super::scalar::ContextTokens::try_from)
            .transpose()
            .map_err(|_| Error::internal())
    }
    fn effort_mapping(&self) -> Result<Vec<TemplateEffort>, Error> {
        let detail = &self.0;
        Ok(detail
            .thinking
            .effort_mapping
            .iter()
            .map(|(level, value)| TemplateEffort {
                level: (*level).into(),
                value: value.clone(),
            })
            .collect())
    }
    fn input_price(&self) -> Result<Option<String>, Error> {
        let detail = &self.0;
        Ok(detail.pricing.input_price.map(|v| v.to_string()))
    }
    fn output_price(&self) -> Result<Option<String>, Error> {
        let detail = &self.0;
        Ok(detail.pricing.output_price.map(|v| v.to_string()))
    }
    fn cached_price(&self) -> Result<Option<String>, Error> {
        let detail = &self.0;
        Ok(detail.pricing.cached_price.map(|v| v.to_string()))
    }
    fn billing_model(&self) -> Result<TemplateBillingModel, Error> {
        let detail = &self.0;
        Ok(detail.pricing.billing_model.into())
    }
    fn pricing_mode(&self) -> Result<TemplatePricingMode, Error> {
        let detail = &self.0;
        Ok(detail.pricing.mode.into())
    }
    fn cache_billing(&self) -> Result<TemplateCacheBilling, Error> {
        let detail = &self.0;
        Ok(detail.pricing.cache_billing.into())
    }
    fn readiness(&self) -> TemplateReadiness {
        self.0.readiness.into()
    }
    fn service_kind(&self) -> Result<TemplateServiceKind, Error> {
        Ok(self.0.service_kind.into())
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
    fn from(value: domain::ServiceKind) -> Self {
        match value {
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
