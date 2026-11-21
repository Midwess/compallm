use super::input::TextModelProtocol;
use compallm_core::provider as domain;
use juniper::{GraphQLEnum, GraphQLObject};

#[derive(juniper::GraphQLObject)]
pub struct ConfigurationInput {
    pub name: String,
    pub kind: ConfigurationInputKind,
    pub required: bool,
    pub allowed_values: Option<Vec<String>>,
}
impl From<domain::ConfigurationInput> for ConfigurationInput {
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
pub struct AuthenticationParameters {
    pub placement: Option<CredentialPlacement>,
    pub flow: Option<OAuthFlow>,
    pub authorization_url: Option<String>,
    pub token_url: Option<String>,
    pub device_authorization_url: Option<String>,
    pub scopes: Vec<String>,
    pub required_inputs: Vec<ConfigurationInput>,
    pub adapter: Option<String>,
    pub parameters: Vec<ConfigurationEntry>,
}
impl From<domain::Authentication> for AuthenticationConfiguration {
    fn from(value: domain::Authentication) -> Self {
        let (method, configuration) = match value {
            domain::Authentication::ApiKey(value) => (
                AuthenticationMethod::ApiKey,
                AuthenticationParameters {
                    placement: value.placement.map(Into::into),
                    required_inputs: value.required_inputs.into_iter().map(Into::into).collect(),
                    ..Default::default()
                },
            ),
            domain::Authentication::OAuth(value) => (
                AuthenticationMethod::Oauth,
                AuthenticationParameters {
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
                AuthenticationMethod::Custom,
                AuthenticationParameters {
                    adapter: value.adapter,
                    parameters: value
                        .parameters
                        .into_iter()
                        .map(|(key, value)| ConfigurationEntry { key, value })
                        .collect(),
                    required_inputs: value.required_inputs.into_iter().map(Into::into).collect(),
                    ..Default::default()
                },
            ),
            domain::Authentication::None => (
                AuthenticationMethod::None,
                AuthenticationParameters::default(),
            ),
        };
        Self {
            method,
            configuration,
        }
    }
}

#[derive(juniper::GraphQLObject)]
pub struct CredentialPlacement {
    pub location: CredentialLocation,
    pub name: String,
    pub prefix: Option<String>,
}
impl From<domain::ApiKeyAuthentication> for CredentialPlacement {
    fn from(value: domain::ApiKeyAuthentication) -> Self {
        match value {
            domain::ApiKeyAuthentication::Header { api_key_header } => Self {
                location: CredentialLocation::Header,
                name: api_key_header.name,
                prefix: Some(api_key_header.prefix),
            },
            domain::ApiKeyAuthentication::Query { api_key_query } => Self {
                location: CredentialLocation::Query,
                name: api_key_query,
                prefix: None,
            },
        }
    }
}

#[derive(juniper::GraphQLObject)]
pub struct EndpointConfiguration {
    pub url_template: Option<String>,
    pub required_inputs: Vec<ConfigurationInput>,
}
impl From<domain::EndpointRequirements> for EndpointConfiguration {
    fn from(value: domain::EndpointRequirements) -> Self {
        Self {
            url_template: value.url_template,
            required_inputs: value.required_inputs.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(juniper::GraphQLEnum)]
pub enum ConfigurationInputKind {
    String,
    Url,
    Object,
    PositiveInteger,
    Price,
}
impl From<domain::ConfigurationInputKind> for ConfigurationInputKind {
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
pub enum CredentialLocation {
    Header,
    Query,
}

#[derive(juniper::GraphQLEnum)]
pub enum OAuthFlow {
    AuthorizationCode,
    DeviceCode,
    ClientCredentials,
}
impl From<domain::OAuthFlow> for OAuthFlow {
    fn from(value: domain::OAuthFlow) -> Self {
        match value {
            domain::OAuthFlow::AuthorizationCode => Self::AuthorizationCode,
            domain::OAuthFlow::DeviceCode => Self::DeviceCode,
            domain::OAuthFlow::ClientCredentials => Self::ClientCredentials,
        }
    }
}

#[derive(juniper::GraphQLObject)]
pub struct ConnectionConfiguration {
    pub base_url: Option<String>,
    pub path: Option<String>,
    pub timeout_ms: Option<f64>,
    pub query: Vec<ConfigurationEntry>,
    pub authentication: Option<AuthenticationConfiguration>,
    pub endpoint: EndpointRequirements,
}
impl From<&domain::Connection> for ConnectionConfiguration {
    fn from(value: &domain::Connection) -> Self {
        Self {
            base_url: value.base_url.clone(),
            path: value.path.clone(),
            timeout_ms: value.timeout_ms.map(f64::from),
            query: value
                .query
                .iter()
                .map(|(key, value)| ConfigurationEntry {
                    key: key.clone(),
                    value: value.clone(),
                })
                .collect(),
            authentication: value.authentication.clone().map(Into::into),
            endpoint: EndpointRequirements {
                mode: value.endpoint.mode.into(),
                configuration: value.endpoint.clone().into(),
            },
        }
    }
}

#[derive(juniper::GraphQLObject)]
pub struct ModelConnection {
    pub protocol: UpstreamProtocol,
    pub configuration: ConnectionConfiguration,
}
#[derive(juniper::GraphQLEnum)]
pub enum UpstreamProtocol {
    Openai,
    OpenaiResponses,
    Anthropic,
    Gemini,
    Minimax,
    Openrouter,
    Xai,
}
impl From<domain::Protocol> for UpstreamProtocol {
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
pub struct BodyDefaultEntry {
    pub key: String,
    pub value: String,
}
#[derive(juniper::GraphQLObject)]
pub struct RouteSelection {
    pub options: Vec<RouteOption>,
    pub selected: Option<String>,
    pub allow_fallbacks: bool,
    pub exact_provider_selection: bool,
}
#[derive(juniper::GraphQLObject)]
pub struct RouteOption {
    pub id: String,
    pub provider: String,
    pub upstream_model_id: String,
    pub context_tokens: Option<super::scalar::ContextTokens>,
    pub text: super::capability::TextCapabilities,
    pub billing: Option<super::pricing::ModelBilling>,
    pub implicit_caching: Option<bool>,
}
impl From<&domain::RouteSelection> for RouteSelection {
    fn from(v: &domain::RouteSelection) -> Self {
        Self {
            options: v.options.iter().map(Into::into).collect(),
            selected: v.selected.clone(),
            allow_fallbacks: v.allow_fallbacks,
            exact_provider_selection: v.exact_provider_selection,
        }
    }
}
impl From<&domain::ModelRouteOption> for RouteOption {
    fn from(v: &domain::ModelRouteOption) -> Self {
        Self {
            id: v.id.clone(),
            provider: v.provider.clone(),
            upstream_model_id: v.upstream_model_id.clone(),
            context_tokens: v
                .limits
                .context_tokens
                .and_then(|v| super::scalar::ContextTokens::try_from(v).ok()),
            text: (&v.text).into(),
            billing: v.billing.as_ref().map(Into::into),
            implicit_caching: v.implicit_caching,
        }
    }
}

#[derive(GraphQLObject)]
pub struct ConfigurationEntry {
    pub key: String,
    pub value: String,
}
#[derive(GraphQLObject)]
pub struct ModelOperation {
    pub operation: TextModelProtocol,
    pub path: String,
}
#[derive(GraphQLObject)]
pub struct ThinkingEffort {
    pub level: ThinkingEffortLevel,
    pub value: String,
}
#[derive(GraphQLObject)]
pub struct AuthenticationConfiguration {
    pub method: AuthenticationMethod,
    pub configuration: AuthenticationParameters,
}
#[derive(GraphQLObject)]
pub struct EndpointRequirements {
    pub mode: EndpointMode,
    pub configuration: EndpointConfiguration,
}

#[derive(GraphQLEnum)]
pub enum ThinkingEffortLevel {
    Low,
    Medium,
    High,
    Xhigh,
    Max,
}
impl From<domain::EffortLevel> for ThinkingEffortLevel {
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

#[derive(GraphQLEnum)]
pub enum AuthenticationMethod {
    ApiKey,
    Oauth,
    Custom,
    None,
}
#[derive(GraphQLEnum)]
pub enum EndpointMode {
    Fixed,
    AccountSpecific,
    Discovered,
    Unknown,
}
impl From<domain::EndpointMode> for EndpointMode {
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
pub enum ModelServiceKind {
    Text,
    Embedding,
    Image,
    Audio,
    Video,
    Rerank,
    Tts,
    Stt,
}
impl From<domain::ServiceKind> for ModelServiceKind {
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
