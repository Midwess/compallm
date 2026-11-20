use super::semantics::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TextOperation {
    OpenaiChatCompletions,
    OpenaiResponses,
    AnthropicMessages,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthenticationMethod {
    ApiKey,
    Oauth,
    Custom,
    None,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Readiness {
    Ready,
    NeedsFetch,
    NeedsConfiguration,
    Incomplete,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EndpointMode {
    Fixed,
    AccountSpecific,
    Discovered,
    Unknown,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProtocolFamily {
    OpenaiCompatible,
    Anthropic,
    ProviderSpecific,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ServiceKind {
    Text,
    Embedding,
    Image,
    Audio,
    Video,
    Rerank,
    Unknown,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BillingModel {
    Metered,
    Subscription,
    SubscriptionWithOverage,
    Free,
    Unknown,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BillingUnit {
    Tokens,
    Requests,
    Images,
    Seconds,
    Characters,
    Credits,
    Videos,
    Utf8Bytes,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PricingMode {
    Fixed,
    Tiered,
    Api,
    AccountSpecific,
    NotApplicable,
    Unpublished,
    Unknown,
    Scheduled,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CacheBilling {
    SeparateRate,
    StandardInputRate,
    Unsupported,
    Included,
    NotApplicable,
    PerRoute,
    Unknown,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SpecificationMode {
    Stored,
    Api,
    AccountSpecific,
    NotApplicable,
    Unknown,
}
pub use crate::provider::Capability;
pub use crate::provider::EffortLevel;

pub type Operations = BTreeMap<TextOperation, String>;
pub type Authentication = BTreeMap<AuthenticationMethod, AuthenticationDefaults>;
pub type Capabilities = BTreeMap<Capability, Option<bool>>;
pub type EffortMapping = BTreeMap<EffortLevel, String>;
pub type Headers = BTreeMap<String, String>;
pub type UnresolvedFields = BTreeMap<String, UnresolvedField>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProviderTemplate {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub logo_path: String,
    pub protocol: ProtocolFamily,
    pub base_url: Option<String>,
    pub headers: Headers,
    pub authentication: Authentication,
    pub endpoint_config: EndpointDefaults,
    pub endpoint_mode: EndpointMode,
    pub readiness: Readiness,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelTemplate {
    pub provider_template_id: String,
    pub id: String,
    pub upstream_model_id: String,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub operations: Operations,
    pub capabilities: Capabilities,
    pub max_context_tokens: Option<u32>,
    pub max_input_tokens: Option<u32>,
    pub max_output_tokens: Option<u32>,
    pub effort_mapping: EffortMapping,
    #[serde(with = "rust_decimal::serde::str_option")]
    pub input_price: Option<Decimal>,
    #[serde(with = "rust_decimal::serde::str_option")]
    pub output_price: Option<Decimal>,
    #[serde(with = "rust_decimal::serde::str_option")]
    pub cached_price: Option<Decimal>,
    pub service_kind: ServiceKind,
    pub billing_model: BillingModel,
    pub billing_unit: Option<BillingUnit>,
    pub pricing_mode: PricingMode,
    pub cache_billing: CacheBilling,
    pub specification_mode: SpecificationMode,
    pub unresolved_fields: UnresolvedFields,
    pub readiness: Readiness,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProviderTemplateSummary {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub logo_path: String,
    pub protocol: ProtocolFamily,
    pub base_url: Option<String>,
    pub readiness: Readiness,
}
impl From<&ProviderTemplate> for ProviderTemplateSummary {
    fn from(v: &ProviderTemplate) -> Self {
        Self {
            id: v.id.clone(),
            name: v.name.clone(),
            description: v.description.clone(),
            logo_path: v.logo_path.clone(),
            protocol: v.protocol,
            base_url: v.base_url.clone(),
            readiness: v.readiness,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthenticationDefaults {
    pub status: AuthStatus,
    pub placements: Option<Vec<CredentialPlacement>>,
    pub flow: Option<OAuthFlow>,
    pub authorization_url: Option<String>,
    pub token_url: Option<String>,
    pub device_authorization_url: Option<String>,
    pub scopes: Option<Vec<String>>,
    pub required_inputs: Option<Vec<RequiredInput>>,
    pub reason: Option<String>,
    pub adapter: Option<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EndpointDefaults {
    pub required_inputs: Option<Vec<RequiredInput>>,
    pub reason: Option<String>,
}
