use super::super::input::{ProviderModelCapability, TextModelProtocol};
use super::super::{
    capability,
    configuration::*,
    pricing::{self, BillingModel, CacheBilling, PricingMode},
    scalar,
};
use crate::graphql::{base_type::Collection, error::Error};
use compallm_core::persistent::db_context;
use compallm_core::{model::ModelCatalog as DomainModelCatalog, provider as domain};
use compallm_postgres::catalog::repository::CatalogRepository;
use juniper::{ID, graphql_object};

#[derive(Clone, Debug)]
pub struct ProviderCatalog(
    pub domain::ProviderCatalog,
    pub Option<compallm_postgres::catalog::repository::CatalogScope>,
);
impl From<domain::ProviderCatalog> for ProviderCatalog {
    fn from(value: domain::ProviderCatalog) -> Self {
        Self(value, None)
    }
}
#[derive(Clone, Debug)]
pub struct ModelCatalog(pub DomainModelCatalog);
impl From<DomainModelCatalog> for ModelCatalog {
    fn from(value: DomainModelCatalog) -> Self {
        Self(value)
    }
}

#[graphql_object(context=crate::graphql::context::GraphqlContext, scalar=juniper::DefaultScalarValue)]
impl ProviderCatalog {
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
    fn connection(&self) -> ConnectionConfiguration {
        (&self.0.connection).into()
    }
    fn base_url(&self) -> Option<&str> {
        self.0.connection.base_url.as_deref()
    }
    fn headers(&self) -> Result<Vec<ConfigurationEntry>, Error> {
        let detail = &self.0;
        Ok(detail
            .headers
            .iter()
            .map(|(key, value)| ConfigurationEntry {
                key: key.clone(),
                value: value.clone(),
            })
            .collect())
    }
    fn endpoint(&self) -> Result<EndpointRequirements, Error> {
        let detail = &self.0;
        Ok(EndpointRequirements {
            mode: detail.connection.endpoint.mode.into(),
            configuration: detail.connection.endpoint.clone().into(),
        })
    }
    fn authentication(&self) -> Result<Vec<AuthenticationConfiguration>, Error> {
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
        filter: Option<super::input::ModelCatalogFilterInput>,
    ) -> Result<Collection<ModelCatalog>, Error> {
        let nodes: Vec<ModelCatalog> = if let Some(scope) = self.1 {
            repository
                .models(
                    &self.0.id,
                    Box::new(filter.unwrap_or_default()),
                    Some(scope),
                )
                .await
                .map_err(|_| Error::internal())?
                .into_iter()
                .map(Into::into)
                .collect()
        } else {
            public_models(repository, &self.0.id, filter).await?
        };
        Collection::complete(
            nodes,
            &format!("template-models:{}", self.0.id),
            |m: &ModelCatalog| vec![Some(m.0.name.clone()), Some(m.0.id.clone())],
        )
        .map_err(|_| Error::internal())
    }
}
#[graphql_object(context=crate::graphql::context::GraphqlContext, scalar=juniper::DefaultScalarValue)]
impl ModelCatalog {
    fn custom_fields(&self) -> Vec<&str> {
        self.0.custom_fields.iter().map(String::as_str).collect()
    }
    fn origin(&self) -> ModelCatalogOrigin {
        if self.0.research_id.is_some() {
            ModelCatalogOrigin::Global
        } else if self.0.id.starts_with("custom:") {
            ModelCatalogOrigin::Custom
        } else {
            ModelCatalogOrigin::Override
        }
    }
    fn id(&self) -> ID {
        ID::new(&self.0.id)
    }
    fn provider_catalog_id(&self) -> ID {
        ID::new(&self.0.provider_catalog_id)
    }
    fn upstream_model_id(&self) -> &str {
        &self.0.name
    }
    fn name(&self) -> &str {
        &self.0.name
    }
    fn connections(&self) -> Vec<ModelConnection> {
        self.0
            .connections
            .iter()
            .map(|(protocol, connection)| ModelConnection {
                protocol: (*protocol).into(),
                configuration: connection.into(),
            })
            .collect()
    }
    fn body_defaults(&self) -> Vec<BodyDefaultEntry> {
        self.0
            .body_defaults
            .iter()
            .map(|(key, value)| BodyDefaultEntry {
                key: key.clone(),
                value: value.to_string(),
            })
            .collect()
    }
    fn route_selection(&self) -> Option<RouteSelection> {
        self.0.route_selection.as_ref().map(Into::into)
    }
    fn display_name(&self) -> &str {
        self.0.display_name.as_deref().unwrap_or(&self.0.name)
    }
    fn description(&self) -> Option<&str> {
        self.0.description.as_deref()
    }
    fn headers(&self) -> Vec<ConfigurationEntry> {
        self.0
            .headers
            .iter()
            .map(|(key, value)| ConfigurationEntry {
                key: key.clone(),
                value: value.clone(),
            })
            .collect()
    }
    fn operations(&self) -> Vec<ModelOperation> {
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
                    .map(|path| ModelOperation { operation, path })
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
    fn text(&self) -> capability::TextCapabilities {
        (&self.0.text).into()
    }
    fn vision(&self) -> capability::VisionSupport {
        (&self.0.vision).into()
    }
    fn pdf(&self) -> capability::PdfSupport {
        (&self.0.pdf).into()
    }
    fn audio(&self) -> capability::AudioCapabilities {
        (&self.0.audio).into()
    }
    fn video(&self) -> capability::VideoSupport {
        (&self.0.video).into()
    }
    fn image_generation(&self) -> capability::ImageGenerationSupport {
        (&self.0.image_generation).into()
    }
    fn thinking(&self) -> capability::ThinkingSupport {
        self.0
            .text
            .output
            .configuration()
            .map(|v| (&v.thinking).into())
            .unwrap_or_else(|| (&domain::CapabilitySupport::Unknown).into())
    }
    fn tools(&self) -> capability::ToolCapabilities {
        (&self.0.tools).into()
    }
    fn structured_output(&self) -> capability::StructuredOutputSupport {
        (&self.0.structured_output).into()
    }
    fn search(&self) -> capability::SearchSupport {
        (&self.0.search).into()
    }
    fn max_context_tokens(&self) -> Result<Option<scalar::ContextTokens>, Error> {
        let detail = &self.0;
        detail
            .limits
            .context_tokens
            .map(scalar::ContextTokens::try_from)
            .transpose()
            .map_err(|_| Error::internal())
    }
    fn max_input_tokens(&self) -> Result<Option<scalar::ContextTokens>, Error> {
        let detail = &self.0;
        detail
            .text
            .input
            .configuration()
            .and_then(|v| v.max_tokens)
            .map(scalar::ContextTokens::try_from)
            .transpose()
            .map_err(|_| Error::internal())
    }
    fn max_output_tokens(&self) -> Result<Option<scalar::ContextTokens>, Error> {
        let detail = &self.0;
        detail
            .text
            .output
            .configuration()
            .and_then(|v| v.max_tokens)
            .map(scalar::ContextTokens::try_from)
            .transpose()
            .map_err(|_| Error::internal())
    }
    fn effort_mapping(&self) -> Result<Vec<ThinkingEffort>, Error> {
        let detail = &self.0;
        Ok(detail
            .text
            .output
            .configuration()
            .map(|v| &v.thinking)
            .and_then(|v| v.configuration())
            .into_iter()
            .flat_map(|v| v.effort_mapping.iter())
            .map(|(level, value)| ThinkingEffort {
                level: (*level).into(),
                value: value.clone(),
            })
            .collect())
    }
    fn input_price(&self) -> Option<String> {
        pricing::TokenDefaults::from(&self.0.text).input
    }
    fn output_price(&self) -> Option<String> {
        pricing::TokenDefaults::from(&self.0.text).output
    }
    fn cached_price(&self) -> Option<String> {
        pricing::TokenDefaults::from(&self.0.text).cached
    }
    fn billing(&self) -> Option<pricing::ModelBilling> {
        self.0.billing.as_ref().map(Into::into)
    }
    fn billing_model(&self) -> BillingModel {
        match &self.0.billing {
            Some(domain::ModelBilling::Subscription(_)) => BillingModel::Subscription,
            Some(domain::ModelBilling::Free(_)) => BillingModel::Free,
            _ => {
                if self.0.text.input.pricing().is_some() || self.0.text.output.pricing().is_some() {
                    BillingModel::Metered
                } else {
                    BillingModel::Unknown
                }
            }
        }
    }
    fn pricing_mode(&self) -> PricingMode {
        match self.0.text.input.pricing() {
            Some(domain::CapabilityPricing::Usage(domain::CapabilityUsagePricing::Fixed(_))) => {
                PricingMode::Fixed
            }
            Some(domain::CapabilityPricing::Usage(
                domain::CapabilityUsagePricing::TieredToken(_),
            )) => PricingMode::Tiered,
            Some(domain::CapabilityPricing::Usage(domain::CapabilityUsagePricing::Scheduled(
                _,
            ))) => PricingMode::Scheduled,
            Some(_) => PricingMode::NotApplicable,
            None => PricingMode::Unknown,
        }
    }
    fn cache_billing(&self) -> CacheBilling {
        if self.0.route_selection.is_some() {
            return CacheBilling::PerRoute;
        }
        match self.0.text.input.pricing() {
            Some(domain::CapabilityPricing::Usage(domain::CapabilityUsagePricing::Fixed(
                domain::CapabilityRates::Token(v),
            ))) => match &v.cache {
                Some(domain::CachePricing::SeparateRate(_)) => CacheBilling::SeparateRate,
                Some(domain::CachePricing::StandardInputRate) => CacheBilling::StandardInputRate,
                Some(domain::CachePricing::Included) => CacheBilling::Included,
                Some(domain::CachePricing::Unsupported) => CacheBilling::Unsupported,
                Some(domain::CachePricing::EstimatedRate(_)) | None => CacheBilling::Unknown,
            },
            _ => CacheBilling::Unknown,
        }
    }
    fn service_kind(&self) -> Option<ModelServiceKind> {
        self.0.service_kind.map(Into::into)
    }
}

#[db_context::root]
async fn public_models(
    repository: &CatalogRepository,
    id: &str,
    filter: Option<super::input::ModelCatalogFilterInput>,
) -> Result<Vec<ModelCatalog>, Error> {
    Ok(repository
        .models(id, Box::new(filter.unwrap_or_default()), None)
        .await
        .map_err(|_| Error::internal())?
        .into_iter()
        .map(Into::into)
        .collect())
}
#[derive(Clone, Copy, juniper::GraphQLEnum)]
pub enum ModelCatalogOrigin {
    Global,
    Override,
    Custom,
}
