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
pub struct PricingConfig {
    pub currency: Option<String>,
    pub unit_quantity: Option<u64>,
    pub tiers: Option<Vec<TokenTier>>,
    pub schedule: Option<PricingSchedule>,
    pub subscription: Option<Subscription>,
    pub required_inputs: Option<Vec<RequiredInput>>,
    pub reason: Option<String>,
    pub rates: Option<Vec<UnitRate>>,
    pub conversion: Option<CreditConversion>,
    pub route_options: Option<Vec<RoutePricing>>,
    pub route_policy: Option<RoutePolicy>,
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
