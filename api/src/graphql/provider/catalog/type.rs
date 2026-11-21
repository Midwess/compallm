use super::super::input::{ProviderModelCapability, TextModelProtocol};
use crate::{
    graphql::{base_type::Collection, error::Error},
    http::authorization::AuthorizationContext,
};
use compallm_core::{model::ModelTemplate as DomainModelTemplate, provider as domain};
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
pub struct ModelTemplate(pub DomainModelTemplate);
impl From<DomainModelTemplate> for ModelTemplate {
    fn from(value: DomainModelTemplate) -> Self {
        Self(value)
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
impl ModelTemplate {
    pub fn compatible(value: &DomainModelTemplate) -> bool {
        value.service_kind == Some(domain::ServiceKind::Llm)
            && value.connections.iter().any(|(protocol, connection)| {
                let protocol = match protocol {
                    domain::Protocol::Openai => TextModelProtocol::OpenaiChatCompletions,
                    domain::Protocol::OpenaiResponses => TextModelProtocol::OpenaiResponses,
                    domain::Protocol::Anthropic => TextModelProtocol::AnthropicMessages,
                    _ => return false,
                };
                let Some(path) = &connection.path else {
                    return false;
                };
                let Some(context) = value
                    .limits
                    .context_tokens
                    .and_then(|v| super::super::scalar::ContextTokens::try_from(v).ok())
                else {
                    return false;
                };
                let prices = super::output::TokenDefaults::from(&value.text);
                let (Some(input), Some(output), Some(cached)) =
                    (prices.input, prices.output, prices.cached)
                else {
                    return false;
                };
                let efforts = value
                    .text
                    .output
                    .configuration()
                    .and_then(|v| v.thinking.configuration())
                    .map(|v| &v.effort_mapping);
                let effort = |level| efforts.and_then(|v| v.get(&level)).cloned();
                let input = super::super::input::ProviderModelConfiguration {
                    name: value.name.clone(),
                    protocol,
                    path: path.clone(),
                    max_context_tokens: context,
                    context_limit_enabled: true,
                    max_input_tokens: value
                        .text
                        .input
                        .configuration()
                        .and_then(|v| v.max_tokens)
                        .and_then(|v| super::super::scalar::ContextTokens::try_from(v).ok()),
                    max_output_tokens: value
                        .text
                        .output
                        .configuration()
                        .and_then(|v| v.max_tokens)
                        .and_then(|v| super::super::scalar::ContextTokens::try_from(v).ok()),
                    headers: vec![],
                    body_defaults: vec![],
                    enabled: true,
                    effort_mapping: Some(super::super::input::ProviderEffortMapping {
                        low: effort(domain::EffortLevel::Low),
                        medium: effort(domain::EffortLevel::Medium),
                        high: effort(domain::EffortLevel::High),
                        xhigh: effort(domain::EffortLevel::Xhigh),
                        max: effort(domain::EffortLevel::Max),
                    }),
                    capabilities: vec![],
                    catalog_ref: None,
                    input_price: input,
                    output_price: output,
                    cached_price: cached,
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
    fn logo_url(&self) -> Option<&str> {
        self.0.logo_url.as_deref()
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
    fn endpoint(&self) -> Result<TemplateEndpoint, Error> {
        let detail = &self.0;
        Ok(TemplateEndpoint {
            mode: detail.connection.endpoint.mode.into(),
            configuration: detail.connection.endpoint.clone().into(),
        })
    }
    fn authentication(&self) -> Result<Vec<TemplateAuthentication>, Error> {
        let detail = &self.0;
        Ok(detail
            .authentication_options
            .iter()
            .cloned()
            .map(Into::into)
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
            |m: &ModelTemplate| vec![Some(m.0.name.clone()), Some(m.0.id.clone())],
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
        &self.0.name
    }
    fn display_name(&self) -> &str {
        self.0.display_name.as_deref().unwrap_or(&self.0.name)
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
            .connections
            .iter()
            .filter_map(|(protocol, connection)| {
                let operation = match protocol {
                    domain::Protocol::Openai => TextModelProtocol::OpenaiChatCompletions,
                    domain::Protocol::OpenaiResponses => TextModelProtocol::OpenaiResponses,
                    domain::Protocol::Anthropic => TextModelProtocol::AnthropicMessages,
                    _ => return None,
                };
                connection
                    .path
                    .clone()
                    .map(|path| TemplateOperation { operation, path })
            })
            .collect()
    }
    fn capabilities(&self) -> Vec<ProviderModelCapability> {
        self.0
            .supported_capabilities()
            .iter()
            .copied()
            .map(Into::into)
            .collect()
    }
    fn text(&self) -> super::capability::TextCapabilities {
        (&self.0.text).into()
    }
    fn vision(&self) -> super::capability::VisionSupport {
        (&self.0.vision).into()
    }
    fn pdf(&self) -> super::capability::PdfSupport {
        (&self.0.pdf).into()
    }
    fn audio(&self) -> super::capability::AudioCapabilities {
        (&self.0.audio).into()
    }
    fn video(&self) -> super::capability::VideoSupport {
        (&self.0.video).into()
    }
    fn image_generation(&self) -> super::capability::ImageGenerationSupport {
        (&self.0.image_generation).into()
    }
    fn thinking(&self) -> super::capability::ThinkingSupport {
        self.0
            .text
            .output
            .configuration()
            .map(|v| (&v.thinking).into())
            .unwrap_or_else(|| (&domain::CapabilitySupport::Unknown).into())
    }
    fn tools(&self) -> super::capability::ToolCapabilities {
        (&self.0.tools).into()
    }
    fn structured_output(&self) -> super::capability::StructuredOutputSupport {
        (&self.0.structured_output).into()
    }
    fn search(&self) -> super::capability::SearchSupport {
        (&self.0.search).into()
    }
    fn max_context_tokens(&self) -> Result<Option<super::super::scalar::ContextTokens>, Error> {
        let detail = &self.0;
        detail
            .limits
            .context_tokens
            .map(super::super::scalar::ContextTokens::try_from)
            .transpose()
            .map_err(|_| Error::internal())
    }
    fn max_input_tokens(&self) -> Result<Option<super::super::scalar::ContextTokens>, Error> {
        let detail = &self.0;
        detail
            .text
            .input
            .configuration()
            .and_then(|v| v.max_tokens)
            .map(super::super::scalar::ContextTokens::try_from)
            .transpose()
            .map_err(|_| Error::internal())
    }
    fn max_output_tokens(&self) -> Result<Option<super::super::scalar::ContextTokens>, Error> {
        let detail = &self.0;
        detail
            .text
            .output
            .configuration()
            .and_then(|v| v.max_tokens)
            .map(super::super::scalar::ContextTokens::try_from)
            .transpose()
            .map_err(|_| Error::internal())
    }
    fn effort_mapping(&self) -> Result<Vec<TemplateEffort>, Error> {
        let detail = &self.0;
        Ok(detail
            .text
            .output
            .configuration()
            .map(|v| &v.thinking)
            .and_then(|v| v.configuration())
            .into_iter()
            .flat_map(|v| v.effort_mapping.iter())
            .map(|(level, value)| TemplateEffort {
                level: (*level).into(),
                value: value.clone(),
            })
            .collect())
    }
    fn input_price(&self) -> Option<String> {
        super::output::TokenDefaults::from(&self.0.text).input
    }
    fn output_price(&self) -> Option<String> {
        super::output::TokenDefaults::from(&self.0.text).output
    }
    fn cached_price(&self) -> Option<String> {
        super::output::TokenDefaults::from(&self.0.text).cached
    }
    fn billing(&self) -> Option<super::pricing::ModelBilling> {
        self.0.billing.as_ref().map(Into::into)
    }
    fn billing_model(&self) -> TemplateBillingModel {
        match &self.0.billing {
            Some(domain::ModelBilling::Subscription(_)) => TemplateBillingModel::Subscription,
            Some(domain::ModelBilling::Free(_)) => TemplateBillingModel::Free,
            _ => {
                if self.0.text.input.pricing().is_some() || self.0.text.output.pricing().is_some() {
                    TemplateBillingModel::Metered
                } else {
                    TemplateBillingModel::Unknown
                }
            }
        }
    }
    fn pricing_mode(&self) -> TemplatePricingMode {
        match self.0.text.input.pricing() {
            Some(domain::CapabilityPricing::Usage(domain::CapabilityUsagePricing::Fixed(_))) => {
                TemplatePricingMode::Fixed
            }
            Some(domain::CapabilityPricing::Usage(
                domain::CapabilityUsagePricing::TieredToken(_),
            )) => TemplatePricingMode::Tiered,
            Some(domain::CapabilityPricing::Usage(domain::CapabilityUsagePricing::Scheduled(
                _,
            ))) => TemplatePricingMode::Scheduled,
            Some(_) => TemplatePricingMode::NotApplicable,
            None => TemplatePricingMode::Unknown,
        }
    }
    fn cache_billing(&self) -> TemplateCacheBilling {
        if self.0.route_selection.is_some() {
            return TemplateCacheBilling::PerRoute;
        }
        match self.0.text.input.pricing() {
            Some(domain::CapabilityPricing::Usage(domain::CapabilityUsagePricing::Fixed(
                domain::CapabilityRates::Token(v),
            ))) => match &v.cache {
                Some(domain::CachePricing::SeparateRate(_)) => TemplateCacheBilling::SeparateRate,
                Some(domain::CachePricing::StandardInputRate) => {
                    TemplateCacheBilling::StandardInputRate
                }
                Some(domain::CachePricing::Included) => TemplateCacheBilling::Included,
                Some(domain::CachePricing::Unsupported) => TemplateCacheBilling::Unsupported,
                None => TemplateCacheBilling::Unknown,
            },
            _ => TemplateCacheBilling::Unknown,
        }
    }
    fn service_kind(&self) -> Option<TemplateServiceKind> {
        self.0.service_kind.map(Into::into)
    }
}

#[derive(GraphQLEnum)]
pub enum TemplateAuthenticationMethod {
    ApiKey,
    Oauth,
    Custom,
    None,
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

#[derive(GraphQLEnum)]
pub enum TemplateServiceKind {
    Text,
    Embedding,
    Image,
    Audio,
    Video,
    Rerank,
    Tts,
    Stt,
}
impl From<domain::ServiceKind> for TemplateServiceKind {
    fn from(value: domain::ServiceKind) -> Self {
        match value {
            domain::ServiceKind::Llm => Self::Text,
            domain::ServiceKind::Embedding => Self::Embedding,
            domain::ServiceKind::Image => Self::Image,
            domain::ServiceKind::Audio => Self::Audio,
            domain::ServiceKind::Video => Self::Video,
            domain::ServiceKind::Rerank => Self::Rerank,
            domain::ServiceKind::Tts => Self::Tts,
            domain::ServiceKind::Stt => Self::Stt,
        }
    }
}
