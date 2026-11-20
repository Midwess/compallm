#[derive(juniper::GraphQLObject)]
pub struct CatalogRequiredInput {
    pub name: String,
    pub kind: TemplateInputKind,
    pub required: bool,
    pub allowed_values: Option<Vec<String>>,
}
impl From<compallm_core::provider_template::semantics::RequiredInput> for CatalogRequiredInput {
    fn from(value: compallm_core::provider_template::semantics::RequiredInput) -> Self {
        Self {
            name: value.name,
            kind: value.kind.into(),
            required: value.required,
            allowed_values: value.allowed_values.map(|v| v.into_iter().collect()),
        }
    }
}

#[derive(juniper::GraphQLObject)]
pub struct TemplateAuthConfig {
    pub status: TemplateAuthStatus,
    pub placements: Option<Vec<TemplateCredentialPlacement>>,
    pub flow: Option<String>,
    pub authorization_url: Option<String>,
    pub token_url: Option<String>,
    pub device_authorization_url: Option<String>,
    pub scopes: Option<Vec<String>>,
    pub required_inputs: Option<Vec<CatalogRequiredInput>>,
    pub reason: Option<String>,
    pub adapter: Option<String>,
}
impl From<compallm_core::provider_template::AuthenticationDefaults> for TemplateAuthConfig {
    fn from(value: compallm_core::provider_template::AuthenticationDefaults) -> Self {
        Self {
            status: value.status.into(),
            placements: value
                .placements
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            flow: value.flow.map(|v| {
                serde_json::to_value(v)
                    .expect("serializable catalog enum")
                    .as_str()
                    .expect("string catalog enum")
                    .to_owned()
            }),
            authorization_url: value.authorization_url,
            token_url: value.token_url,
            device_authorization_url: value.device_authorization_url,
            scopes: value.scopes.map(|v| v.into_iter().collect()),
            required_inputs: value
                .required_inputs
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            reason: value.reason,
            adapter: value.adapter,
        }
    }
}

#[derive(juniper::GraphQLObject)]
pub struct TemplateCredentialPlacement {
    pub operations: Vec<String>,
    pub location: TemplateCredentialLocation,
    pub name: String,
    pub prefix: String,
    pub adapter_requirements: Vec<String>,
}
impl From<compallm_core::provider_template::semantics::CredentialPlacement>
    for TemplateCredentialPlacement
{
    fn from(value: compallm_core::provider_template::semantics::CredentialPlacement) -> Self {
        Self {
            operations: value.operations.into_iter().collect(),
            location: value.location.into(),
            name: value.name,
            prefix: value.prefix,
            adapter_requirements: value.adapter_requirements.into_iter().collect(),
        }
    }
}

#[derive(juniper::GraphQLObject)]
pub struct TemplateEndpointConfig {
    pub required_inputs: Option<Vec<CatalogRequiredInput>>,
    pub reason: Option<String>,
}
impl From<compallm_core::provider_template::EndpointDefaults> for TemplateEndpointConfig {
    fn from(value: compallm_core::provider_template::EndpointDefaults) -> Self {
        Self {
            required_inputs: value
                .required_inputs
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            reason: value.reason,
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
impl From<compallm_core::provider_template::semantics::InputKind> for TemplateInputKind {
    fn from(v: compallm_core::provider_template::semantics::InputKind) -> Self {
        match v {
            compallm_core::provider_template::semantics::InputKind::String => Self::String,
            compallm_core::provider_template::semantics::InputKind::Url => Self::Url,
            compallm_core::provider_template::semantics::InputKind::Object => Self::Object,
            compallm_core::provider_template::semantics::InputKind::PositiveInteger => {
                Self::PositiveInteger
            }
            compallm_core::provider_template::semantics::InputKind::Price => Self::Price,
        }
    }
}

#[derive(juniper::GraphQLEnum)]
pub enum TemplateAuthStatus {
    Configured,
    Unknown,
}
impl From<compallm_core::provider_template::semantics::AuthStatus> for TemplateAuthStatus {
    fn from(v: compallm_core::provider_template::semantics::AuthStatus) -> Self {
        match v {
            compallm_core::provider_template::semantics::AuthStatus::Configured => Self::Configured,
            compallm_core::provider_template::semantics::AuthStatus::Unknown => Self::Unknown,
        }
    }
}

#[derive(juniper::GraphQLEnum)]
pub enum TemplateCredentialLocation {
    Header,
    Query,
}
impl From<compallm_core::provider_template::semantics::CredentialLocation>
    for TemplateCredentialLocation
{
    fn from(v: compallm_core::provider_template::semantics::CredentialLocation) -> Self {
        match v {
            compallm_core::provider_template::semantics::CredentialLocation::Header => Self::Header,
            compallm_core::provider_template::semantics::CredentialLocation::Query => Self::Query,
        }
    }
}
