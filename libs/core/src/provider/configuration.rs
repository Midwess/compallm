use std::collections::BTreeMap;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

pub type Headers = BTreeMap<String, String>;
pub type BodyDefaults = BTreeMap<String, serde_json::Value>;
pub type EffortMapping = BTreeMap<EffortLevel, String>;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ModelTemplateReference {
    pub provider_id: String,
    pub model_id: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ServiceKind {
    Llm,
    Image,
    Video,
    Embedding,
    Tts,
    Stt,
    Audio,
    Rerank,
}

impl From<ServiceKind> for String {
    fn from(value: ServiceKind) -> Self {
        match value {
            ServiceKind::Llm => "llm",
            ServiceKind::Image => "image",
            ServiceKind::Video => "video",
            ServiceKind::Embedding => "embedding",
            ServiceKind::Tts => "tts",
            ServiceKind::Stt => "stt",
            ServiceKind::Audio => "audio",
            ServiceKind::Rerank => "rerank",
        }
        .into()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Protocol {
    Openai,
    OpenaiResponses,
    Anthropic,
    Gemini,
    Minimax,
    Openrouter,
    Xai,
}

impl Protocol {
    pub const fn supports(self, service: ServiceKind) -> bool {
        matches!(
            (service, self),
            (
                ServiceKind::Llm,
                Self::Openai | Self::OpenaiResponses | Self::Anthropic | Self::Gemini
            ) | (ServiceKind::Image, Self::Openai | Self::Gemini)
                | (ServiceKind::Embedding | ServiceKind::Stt, Self::Openai)
                | (
                    ServiceKind::Tts,
                    Self::Openai | Self::Minimax | Self::Openrouter
                )
                | (ServiceKind::Video, Self::Xai)
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Connection {
    pub base_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    pub timeout_ms: Option<u32>,
    #[serde(default)]
    pub query: BTreeMap<String, String>,
    pub authentication: Option<Authentication>,
    pub endpoint: EndpointRequirements,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "configuration", rename_all = "snake_case")]
pub enum Authentication {
    ApiKey(ApiKeyConfiguration),
    OAuth(OAuthAuthentication),
    None,
    Custom(CustomAuthentication),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct OAuthAuthentication {
    pub flow: Option<OAuthFlow>,
    pub authorization_url: Option<String>,
    pub token_url: Option<String>,
    pub device_authorization_url: Option<String>,
    pub scopes: Vec<String>,
    pub token_placement: Option<ApiKeyAuthentication>,
    pub required_inputs: Vec<ConfigurationInput>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OAuthFlow {
    AuthorizationCode,
    DeviceCode,
    ClientCredentials,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CustomAuthentication {
    pub adapter: Option<String>,
    pub parameters: BTreeMap<String, String>,
    pub required_inputs: Vec<ConfigurationInput>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ApiKeyConfiguration {
    pub placement: Option<ApiKeyAuthentication>,
    pub required_inputs: Vec<ConfigurationInput>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct EndpointRequirements {
    pub mode: EndpointMode,
    pub url_template: Option<String>,
    pub required_inputs: Vec<ConfigurationInput>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EndpointMode {
    Fixed,
    AccountSpecific,
    Discovered,
    Unknown,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ConfigurationInput {
    pub name: String,
    pub kind: ConfigurationInputKind,
    pub required: bool,
    pub allowed_values: Option<Vec<String>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfigurationInputKind {
    String,
    Url,
    Object,
    PositiveInteger,
    Price,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged, deny_unknown_fields, rename_all_fields = "camelCase")]
pub enum ApiKeyAuthentication {
    Header { api_key_header: ApiKeyHeader },
    Query { api_key_query: String },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ApiKeyHeader {
    pub name: String,
    pub prefix: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Capability {
    Text,
    Vision,
    Pdf,
    Audio,
    Video,
    ImageGeneration,
    AudioGeneration,
    Search,
    Reasoning,
    ToolCall,
    ForcedToolChoice,
    StructuredOutput,
}

pub static ALL_CAPABILITIES: [Capability; 12] = [
    Capability::Text,
    Capability::Vision,
    Capability::Pdf,
    Capability::Audio,
    Capability::Video,
    Capability::ImageGeneration,
    Capability::AudioGeneration,
    Capability::Search,
    Capability::Reasoning,
    Capability::ToolCall,
    Capability::ForcedToolChoice,
    Capability::StructuredOutput,
];

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ModelLimits {
    pub context_tokens: Option<u32>,
    pub max_input_tokens: Option<u32>,
    pub max_output_tokens: Option<u32>,
    pub max_images_per_request: Option<u32>,
    pub max_duration_ms: Option<u32>,
    pub max_audio_bytes: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Thinking {
    pub effort_mapping: EffortMapping,
    pub budget: Option<ThinkingBudget>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EffortLevel {
    Low,
    Medium,
    High,
    Xhigh,
    Max,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ThinkingBudget {
    pub min_tokens: u32,
    pub max_tokens: u32,
    pub default_tokens: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Pricing {
    Usage(UsagePricing),
    Subscription(SubscriptionPricing),
    Free(FreePricing),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UsagePricing {
    Fixed(UsageRates),
    TieredToken(TieredTokenPricing),
    Scheduled(ScheduledPricing),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UsageRates {
    Token(TokenPricing),
    Audio(AudioPricing),
    Image(ImagePricing),
    Video(VideoPricing),
    Request(RequestPricing),
    Credit(CreditPricing),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct TokenPricing {
    pub input: Rate,
    pub output: Rate,
    pub cache: Option<CachePricing>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "configuration", rename_all = "snake_case")]
pub enum AudioPricing {
    Duration(AudioDurationPricing),
    Character(AudioCharacterPricing),
    Utf8Bytes(AudioBytePricing),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct AudioDurationPricing {
    pub rate: Rate,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct AudioCharacterPricing {
    pub rate: Rate,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ImagePricing {
    pub rates: Vec<ImageRate>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ImageRate {
    pub rate: Rate,
    #[serde(default)]
    pub conditions: RateConditions,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct VideoPricing {
    pub rates: Vec<VideoRate>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct VideoRate {
    pub unit: VideoBillingUnit,
    pub rate: Rate,
    #[serde(default)]
    pub conditions: RateConditions,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct RequestPricing {
    pub rate: Rate,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct RateConditions {
    #[serde(default)]
    pub parameters: BTreeMap<String, String>,
    pub resolution: Option<String>,
    pub quality: Option<String>,
    pub audio: Option<bool>,
    pub service_tier: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "configuration", rename_all = "snake_case")]
pub enum CachePricing {
    SeparateRate(CacheTokenRate),
    StandardInputRate,
    Included,
    Unsupported,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CacheTokenRate {
    pub rate: Rate,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct TieredTokenPricing {
    pub measured_by: TokenTierBasis,
    pub tiers: Vec<TokenPriceTier>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TokenTierBasis {
    InputTokens,
    TotalTokens,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct TokenPriceTier {
    pub from_tokens: u64,
    pub to_tokens: Option<u64>,
    pub rates: TokenPricing,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ScheduledPricing {
    pub timezone: String,
    pub slots: Vec<PriceSlot>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PriceSlot {
    pub from_hour: u8,
    pub to_hour: u8,
    pub pricing: ScheduledRates,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScheduledRates {
    Fixed(UsageRates),
    TieredToken(TieredTokenPricing),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SubscriptionPricing {
    pub product: String,
    pub plan: Option<String>,
    pub fee: Option<SubscriptionFee>,
    pub entitlements: Vec<UsageAllowance>,
    pub overage: Option<UsagePricing>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SubscriptionFee {
    #[serde(with = "rust_decimal::serde::str_option")]
    pub amount: Option<Decimal>,
    pub currency: Option<String>,
    pub period: Option<BillingPeriod>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BillingPeriod {
    Minute,
    Hour,
    Day,
    Week,
    Month,
    Year,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BillingUnit {
    Tokens,
    Requests,
    Images,
    Seconds,
    Characters,
    Utf8Bytes,
    Credits,
    Videos,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct UsageAllowance {
    pub unit: BillingUnit,
    pub quota: Option<u64>,
    pub reset_period: Option<BillingPeriod>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct FreePricing {
    pub description: Option<String>,
    pub allowances: Vec<UsageAllowance>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Rate {
    #[serde(with = "rust_decimal::serde::str_option")]
    pub amount: Option<Decimal>,
    pub currency: Option<String>,
    pub quantity: Option<u64>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VideoBillingUnit {
    Seconds,
    Videos,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct AudioBytePricing {
    pub rate: Rate,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CreditPricing {
    pub rates: Vec<CreditRate>,
    pub conversion: Option<CreditConversion>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CreditRate {
    pub unit: BillingUnit,
    pub direction: Option<UsageDirection>,
    pub quantity: Option<u64>,
    #[serde(with = "rust_decimal::serde::str_option")]
    pub credits: Option<Decimal>,
    #[serde(default)]
    pub conditions: RateConditions,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UsageDirection {
    Input,
    Output,
    CacheRead,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CreditConversion {
    #[serde(with = "rust_decimal::serde::str_option")]
    pub amount_per_credit: Option<Decimal>,
    pub currency: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ModelRouteOption {
    pub id: String,
    pub provider: String,
    pub upstream_model_id: String,
    pub limits: ModelLimits,
    pub pricing: Option<Pricing>,
    pub implicit_caching: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct RouteSelection {
    pub options: Vec<ModelRouteOption>,
    pub selected: Option<String>,
    pub allow_fallbacks: bool,
    pub exact_provider_selection: bool,
}
