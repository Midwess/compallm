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
impl From<Option<&domain::Pricing>> for TokenDefaults {
    fn from(value: Option<&domain::Pricing>) -> Self {
        fn amount(rate: &domain::Rate) -> Option<String> {
            (rate.currency.as_deref() == Some("USD") && rate.quantity == Some(1_000_000))
                .then(|| rate.amount.map(|v| v.to_string()))
                .flatten()
        }
        match value {
            Some(domain::Pricing::Free(_)) => Self {
                input: Some("0".into()),
                output: Some("0".into()),
                cached: None,
            },
            Some(domain::Pricing::Usage(domain::UsagePricing::Fixed(
                domain::UsageRates::Token(value),
            ))) => {
                let input = amount(&value.input);
                let cached = match &value.cache {
                    Some(domain::CachePricing::SeparateRate(value)) => amount(&value.rate),
                    Some(domain::CachePricing::StandardInputRate) => input.clone(),
                    Some(domain::CachePricing::Included) => Some("0".into()),
                    _ => None,
                };
                Self {
                    input,
                    output: amount(&value.output),
                    cached,
                }
            }
            _ => Self::default(),
        }
    }
}
