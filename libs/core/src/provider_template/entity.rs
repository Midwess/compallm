use super::{model_info::ModelInfoSource, semantics::*};
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
pub type Authentication = BTreeMap<AuthenticationMethod, AuthConfig>;
pub type Capabilities = BTreeMap<Capability, Option<bool>>;
pub type EffortMapping = BTreeMap<EffortLevel, String>;
pub type Headers = BTreeMap<String, String>;
pub type FieldMetadata = BTreeMap<String, FieldStatus>;
pub type ResolutionPlan = BTreeMap<String, FieldResolution>;
pub type UnresolvedFields = BTreeMap<String, UnresolvedField>;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceReference {
    pub kind: ReferenceKind,
    pub reference: String,
    pub revision: Option<String>,
    pub fields: Vec<String>,
    pub status: ReferenceStatus,
    pub note: Option<String>,
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReferenceKind {
    Repository,
    Web,
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReferenceStatus {
    Imported,
    Estimate,
    Verified,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FieldStatus {
    pub status: FieldStatusKind,
    pub reason: String,
    pub source_url: Option<String>,
    pub base_url: Option<String>,
    pub source_units: Option<serde_json::Value>,
    pub conditions: Option<serde_json::Value>,
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FieldStatusKind {
    Unknown,
    AccountSpecific,
    SubscriptionBased,
    Variable,
    UnsupportedPrecision,
    StandardInputRate,
    Conditional,
    RequiresAuthentication,
    Unconfigured,
    NotApplicable,
    Unpublished,
    NoPublicEndpoint,
    MigrationRequired,
    CreditBased,
    Deprecated,
    ClientRestricted,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TemplateResearch {
    pub metadata: FieldMetadata,
    pub original_order: Vec<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProviderTemplate {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub logo_path: String,
    pub protocol: ProtocolFamily,
    pub base_url: Option<String>,
    pub headers: Headers,
    pub source_refs: Vec<SourceReference>,
    pub authentication: Authentication,
    pub endpoint_config: EndpointConfig,
    pub endpoint_mode: EndpointMode,
    pub readiness: Readiness,
    pub field_metadata: FieldMetadata,
    pub model_info_sources: Vec<ModelInfoSource>,
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
    pub source_refs: Vec<SourceReference>,
    pub field_metadata: FieldMetadata,
    pub model_info_sources: Vec<ModelInfoSource>,
    pub service_kind: ServiceKind,
    pub billing_model: BillingModel,
    pub billing_unit: Option<BillingUnit>,
    pub pricing_mode: PricingMode,
    pub cache_billing: CacheBilling,
    pub specification_mode: SpecificationMode,
    pub pricing_config: PricingConfig,
    pub resolution_plan: ResolutionPlan,
    pub unresolved_fields: UnresolvedFields,
    pub readiness: Readiness,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelTemplateSummary {
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
    #[serde(with = "rust_decimal::serde::str_option")]
    pub input_price: Option<Decimal>,
    #[serde(with = "rust_decimal::serde::str_option")]
    pub output_price: Option<Decimal>,
    #[serde(with = "rust_decimal::serde::str_option")]
    pub cached_price: Option<Decimal>,
    pub effort_mapping: EffortMapping,
}
impl From<&ModelTemplate> for ModelTemplateSummary {
    fn from(v: &ModelTemplate) -> Self {
        Self {
            provider_template_id: v.provider_template_id.clone(),
            id: v.id.clone(),
            upstream_model_id: v.upstream_model_id.clone(),
            display_name: v.display_name.clone(),
            description: v.description.clone(),
            operations: v.operations.clone(),
            capabilities: v.capabilities.clone(),
            max_context_tokens: v.max_context_tokens,
            max_input_tokens: v.max_input_tokens,
            max_output_tokens: v.max_output_tokens,
            input_price: v.input_price,
            output_price: v.output_price,
            cached_price: v.cached_price,
            effort_mapping: v.effort_mapping.clone(),
        }
    }
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
