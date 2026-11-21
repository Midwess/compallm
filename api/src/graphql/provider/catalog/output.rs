use super::r#type::{TemplateAuthentication, TemplateAuthenticationMethod};
use compallm_core::provider as domain;

#[derive(juniper::GraphQLObject)]
pub struct CatalogRequiredInput {
    pub name: String,
    pub kind: TemplateInputKind,
    pub required: bool,
    pub allowed_values: Option<Vec<String>>,
}
impl From<domain::ConfigurationInput> for CatalogRequiredInput {
    fn from(value: domain::ConfigurationInput) -> Self {
        Self {
            name: value.name,
            kind: value.kind.into(),
            required: value.required,
            allowed_values: value.allowed_values,
        }
    }
}

#[derive(juniper::GraphQLObject, Default)]
pub struct TemplateAuthConfig {
    pub placement: Option<TemplateCredentialPlacement>,
    pub flow: Option<TemplateOAuthFlow>,
    pub authorization_url: Option<String>,
    pub token_url: Option<String>,
    pub device_authorization_url: Option<String>,
    pub scopes: Vec<String>,
    pub required_inputs: Vec<CatalogRequiredInput>,
    pub adapter: Option<String>,
    pub parameters: Vec<super::r#type::TemplateHeader>,
}
impl From<domain::Authentication> for TemplateAuthentication {
    fn from(value: domain::Authentication) -> Self {
        let (method, configuration) = match value {
            domain::Authentication::ApiKey(value) => (
                TemplateAuthenticationMethod::ApiKey,
                TemplateAuthConfig {
                    placement: value.placement.map(Into::into),
                    required_inputs: value.required_inputs.into_iter().map(Into::into).collect(),
                    ..Default::default()
                },
            ),
            domain::Authentication::OAuth(value) => (
                TemplateAuthenticationMethod::Oauth,
                TemplateAuthConfig {
                    placement: value.token_placement.map(Into::into),
                    flow: value.flow.map(Into::into),
                    authorization_url: value.authorization_url,
                    token_url: value.token_url,
                    device_authorization_url: value.device_authorization_url,
                    scopes: value.scopes,
                    required_inputs: value.required_inputs.into_iter().map(Into::into).collect(),
                    ..Default::default()
                },
            ),
            domain::Authentication::Custom(value) => (
                TemplateAuthenticationMethod::Custom,
                TemplateAuthConfig {
                    adapter: value.adapter,
                    parameters: value
                        .parameters
                        .into_iter()
                        .map(|(key, value)| super::r#type::TemplateHeader { key, value })
                        .collect(),
                    required_inputs: value.required_inputs.into_iter().map(Into::into).collect(),
                    ..Default::default()
                },
            ),
            domain::Authentication::None => (
                TemplateAuthenticationMethod::None,
                TemplateAuthConfig::default(),
            ),
        };
        Self {
            method,
            configuration,
        }
    }
}

#[derive(juniper::GraphQLObject)]
pub struct TemplateCredentialPlacement {
    pub location: TemplateCredentialLocation,
    pub name: String,
    pub prefix: Option<String>,
}
impl From<domain::ApiKeyAuthentication> for TemplateCredentialPlacement {
    fn from(value: domain::ApiKeyAuthentication) -> Self {
        match value {
            domain::ApiKeyAuthentication::Header { api_key_header } => Self {
                location: TemplateCredentialLocation::Header,
                name: api_key_header.name,
                prefix: Some(api_key_header.prefix),
            },
            domain::ApiKeyAuthentication::Query { api_key_query } => Self {
                location: TemplateCredentialLocation::Query,
                name: api_key_query,
                prefix: None,
            },
        }
    }
}

#[derive(juniper::GraphQLObject)]
pub struct TemplateEndpointConfig {
    pub url_template: Option<String>,
    pub required_inputs: Vec<CatalogRequiredInput>,
}
impl From<domain::EndpointRequirements> for TemplateEndpointConfig {
    fn from(value: domain::EndpointRequirements) -> Self {
        Self {
            url_template: value.url_template,
            required_inputs: value.required_inputs.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(juniper::GraphQLEnum)]
pub enum TemplateInputKind {
    String,
    Url,
    Object,
    PositiveInteger,
    Price,
}
impl From<domain::ConfigurationInputKind> for TemplateInputKind {
    fn from(value: domain::ConfigurationInputKind) -> Self {
        match value {
            domain::ConfigurationInputKind::String => Self::String,
            domain::ConfigurationInputKind::Url => Self::Url,
            domain::ConfigurationInputKind::Object => Self::Object,
            domain::ConfigurationInputKind::PositiveInteger => Self::PositiveInteger,
            domain::ConfigurationInputKind::Price => Self::Price,
        }
    }
}

#[derive(juniper::GraphQLEnum)]
pub enum TemplateCredentialLocation {
    Header,
    Query,
}

#[derive(juniper::GraphQLEnum)]
pub enum TemplateOAuthFlow {
    AuthorizationCode,
    DeviceCode,
    ClientCredentials,
}
impl From<domain::OAuthFlow> for TemplateOAuthFlow {
    fn from(value: domain::OAuthFlow) -> Self {
        match value {
            domain::OAuthFlow::AuthorizationCode => Self::AuthorizationCode,
            domain::OAuthFlow::DeviceCode => Self::DeviceCode,
            domain::OAuthFlow::ClientCredentials => Self::ClientCredentials,
        }
    }
}

#[derive(Default)]
pub(super) struct TokenDefaults {
    pub input: Option<String>,
    pub output: Option<String>,
    pub cached: Option<String>,
}
impl From<&domain::TextCapabilities> for TokenDefaults {
    fn from(value: &domain::TextCapabilities) -> Self {
        fn rate(value: Option<&domain::CapabilityPricing>) -> Option<&domain::CapabilityTokenRate> {
            match value {
                Some(domain::CapabilityPricing::Usage(domain::CapabilityUsagePricing::Fixed(
                    domain::CapabilityRates::Token(v),
                ))) => Some(v),
                _ => None,
            }
        }
        fn amount(rate: &domain::Rate) -> Option<String> {
            if rate.currency.as_deref() == Some("USD") && rate.quantity == Some(1000000) {
                rate.amount.map(|v| v.to_string())
            } else {
                None
            }
        }
        fn scalar(value: Option<&domain::CapabilityPricing>) -> Option<String> {
            match value {
                Some(domain::CapabilityPricing::Free | domain::CapabilityPricing::Included) => {
                    Some("0".into())
                }
                _ => rate(value).and_then(|v| amount(&v.rate)),
            }
        }
        let input = scalar(value.input.pricing());
        let cached = rate(value.input.pricing()).and_then(|v| match &v.cache {
            Some(domain::CachePricing::SeparateRate(v)) => amount(&v.rate),
            Some(domain::CachePricing::StandardInputRate) => input.clone(),
            Some(domain::CachePricing::Included) => Some("0".into()),
            _ => None,
        });
        Self {
            input,
            output: scalar(value.output.pricing()),
            cached,
        }
    }
}

#[derive(juniper::GraphQLObject)]
pub struct TemplateConnection {
    pub base_url: Option<String>,
    pub path: Option<String>,
    pub timeout_ms: Option<f64>,
    pub query: Vec<super::r#type::TemplateHeader>,
    pub authentication: Option<TemplateAuthentication>,
    pub endpoint: super::r#type::TemplateEndpoint,
}
impl From<&domain::Connection> for TemplateConnection {
    fn from(value: &domain::Connection) -> Self {
        Self {
            base_url: value.base_url.clone(),
            path: value.path.clone(),
            timeout_ms: value.timeout_ms.map(f64::from),
            query: value
                .query
                .iter()
                .map(|(key, value)| super::r#type::TemplateHeader {
                    key: key.clone(),
                    value: value.clone(),
                })
                .collect(),
            authentication: value.authentication.clone().map(Into::into),
            endpoint: super::r#type::TemplateEndpoint {
                mode: value.endpoint.mode.into(),
                configuration: value.endpoint.clone().into(),
            },
        }
    }
}

#[derive(juniper::GraphQLObject)]
pub struct TemplateModelConnection {
    pub protocol: TemplateProtocol,
    pub configuration: TemplateConnection,
}
#[derive(juniper::GraphQLEnum)]
pub enum TemplateProtocol {
    Openai,
    OpenaiResponses,
    Anthropic,
    Gemini,
    Minimax,
    Openrouter,
    Xai,
}
impl From<domain::Protocol> for TemplateProtocol {
    fn from(value: domain::Protocol) -> Self {
        match value {
            domain::Protocol::Openai => Self::Openai,
            domain::Protocol::OpenaiResponses => Self::OpenaiResponses,
            domain::Protocol::Anthropic => Self::Anthropic,
            domain::Protocol::Gemini => Self::Gemini,
            domain::Protocol::Minimax => Self::Minimax,
            domain::Protocol::Openrouter => Self::Openrouter,
            domain::Protocol::Xai => Self::Xai,
        }
    }
}
#[derive(juniper::GraphQLObject)]
/// Public body default. Value is JSON text for the configuration editor.
pub struct TemplateBodyDefault {
    pub key: String,
    pub value: String,
}
#[derive(juniper::GraphQLObject)]
pub struct TemplateRouteSelection {
    pub options: Vec<TemplateRouteOption>,
    pub selected: Option<String>,
    pub allow_fallbacks: bool,
    pub exact_provider_selection: bool,
}
#[derive(juniper::GraphQLObject)]
pub struct TemplateRouteOption {
    pub id: String,
    pub provider: String,
    pub upstream_model_id: String,
    pub context_tokens: Option<super::super::scalar::ContextTokens>,
    pub text: super::capability::TextCapabilities,
    pub billing: Option<super::pricing::ModelBilling>,
    pub implicit_caching: Option<bool>,
}
impl From<&domain::RouteSelection> for TemplateRouteSelection {
    fn from(v: &domain::RouteSelection) -> Self {
        Self {
            options: v.options.iter().map(Into::into).collect(),
            selected: v.selected.clone(),
            allow_fallbacks: v.allow_fallbacks,
            exact_provider_selection: v.exact_provider_selection,
        }
    }
}
impl From<&domain::ModelRouteOption> for TemplateRouteOption {
    fn from(v: &domain::ModelRouteOption) -> Self {
        Self {
            id: v.id.clone(),
            provider: v.provider.clone(),
            upstream_model_id: v.upstream_model_id.clone(),
            context_tokens: v
                .limits
                .context_tokens
                .and_then(|v| super::super::scalar::ContextTokens::try_from(v).ok()),
            text: (&v.text).into(),
            billing: v.billing.as_ref().map(Into::into),
            implicit_caching: v.implicit_caching,
        }
    }
}
