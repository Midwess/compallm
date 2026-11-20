use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequiredInput {
    pub name: String,
    #[serde(rename = "type")]
    pub kind: InputKind,
    pub required: bool,
    pub allowed_values: Option<Vec<String>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InputKind {
    String,
    Url,
    Object,
    PositiveInteger,
    Price,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthConfig {
    pub status: AuthStatus,
    pub placements: Option<Vec<CredentialPlacement>>,
    pub flow: Option<OAuthFlow>,
    pub authorization_url: Option<String>,
    pub token_url: Option<String>,
    pub device_authorization_url: Option<String>,
    pub scopes: Option<Vec<String>>,
    pub required_inputs: Option<Vec<RequiredInput>>,
    pub reason: Option<String>,
    pub source_urls: Option<Vec<String>>,
    pub adapter: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthStatus {
    Configured,
    Unknown,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OAuthFlow {
    AuthorizationCode,
    DeviceCode,
    ClientCredentials,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CredentialPlacement {
    pub operations: Vec<String>,
    pub location: CredentialLocation,
    pub name: String,
    pub prefix: String,
    pub adapter_requirements: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CredentialLocation {
    Header,
    Query,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EndpointConfig {
    pub required_inputs: Option<Vec<RequiredInput>>,
    pub source_urls: Option<Vec<String>>,
    pub reason: Option<String>,
    pub source_index: Option<usize>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "method", rename_all = "snake_case", deny_unknown_fields)]
pub enum FieldResolution {
    Api { source_index: usize },
    Account { required_inputs: Vec<RequiredInput> },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PricingConfig {
    pub currency: Option<String>,
    pub unit_quantity: Option<u64>,
    pub tiers: Option<Vec<TokenTier>>,
    pub schedule: Option<PricingSchedule>,
    pub subscription: Option<Subscription>,
    pub required_inputs: Option<Vec<RequiredInput>>,
    pub reason: Option<String>,
    pub native_reference: Option<serde_json::Value>,
    pub rates: Option<Vec<UnitRate>>,
    pub conversion: Option<CreditConversion>,
    pub source_index: Option<usize>,
    pub free_evidence: Option<FreeEvidence>,
    pub cache_evidence: Option<SourceEvidence>,
    pub zero_price_evidence: Option<BTreeMap<String, SourceEvidence>>,
    pub fallback: Option<FallbackPricing>,
    pub cost_proxy: Option<FallbackPricing>,
    pub route_options: Option<Vec<RoutePricing>>,
    pub route_policy: Option<RoutePolicy>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FreeEvidence {
    pub source_index: usize,
    pub reason: String,
    pub restrictions: BTreeMap<String, serde_json::Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceEvidence {
    pub source_index: usize,
    pub reason: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TokenTier {
    pub from_tokens: u64,
    pub to_tokens: Option<u64>,
    pub rates: TokenRates,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TokenRates {
    pub input_price: String,
    pub output_price: String,
    pub cached_price: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Subscription {
    pub product: String,
    pub plan_mode: SubscriptionPlanMode,
    pub required_inputs: Option<Vec<RequiredInput>>,
    pub source_urls: Vec<String>,
    pub plan: Option<String>,
    pub entitlement: Option<Entitlement>,
    pub fee: Option<SubscriptionFee>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionPlanMode {
    Stored,
    AccountSpecific,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SubscriptionFee {
    pub amount: String,
    pub currency: String,
    pub period: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Entitlement {
    pub unit: String,
    pub quota: u64,
    pub period: String,
    pub overage_policy: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UnitRate {
    pub amount: String,
    pub quantity: u64,
    pub conditions: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreditConversion {
    pub usd_per_credit: String,
    pub currency: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UnresolvedField {
    pub status: MissingStatus,
    pub reason: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MissingStatus {
    Unknown,
    Unsupported,
    Unpublished,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RoutePricing {
    pub model_id: String,
    pub tag: String,
    pub provider: String,
    pub context_tokens: u32,
    pub max_input_tokens: Option<u32>,
    pub max_output_tokens: Option<u32>,
    pub implicit_caching: Option<bool>,
    pub rates: TokenRates,
    pub tiers: Vec<TokenTier>,
    pub source_index: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RoutePolicy {
    pub allow_fallbacks: bool,
    pub exact_provider_selection: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PricingSchedule {
    pub timezone: String,
    pub slots: Vec<TimePriceSlot>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TimePriceSlot {
    pub from_hour: u8,
    pub to_hour: u8,
    pub rates: TokenRates,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FallbackPricing {
    pub currency: String,
    pub unit_quantity: u64,
    pub input_price: Option<String>,
    pub output_price: Option<String>,
    pub cached_price: Option<String>,
    pub basis: FallbackBasis,
    pub reference_provider: String,
    pub reference_model: String,
    pub source_urls: Vec<String>,
    pub confidence: EstimateConfidence,
    pub notes: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FallbackBasis {
    SameModelOtherProvider,
    RelatedModel,
    TargetProviderEstimate,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EstimateConfidence {
    High,
    Medium,
    Low,
}
