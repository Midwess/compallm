use super::{
    AudioPricing, CachePricing, CreditConversion, CreditRate, FreePricing, ImagePricing, Rate,
    RequestPricing, SubscriptionFee, TokenTierBasis, UsageAllowance, VideoPricing,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    tag = "type",
    content = "configuration",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum CapabilityPricing {
    Usage(CapabilityUsagePricing),
    Free,
    Included,
    ModelBilling,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityUsagePricing {
    Fixed(CapabilityRates),
    TieredToken(CapabilityTokenTiers),
    Scheduled(CapabilitySchedule),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityRates {
    Token(CapabilityTokenRate),
    Audio(AudioPricing),
    Image(ImagePricing),
    Video(VideoPricing),
    Request(RequestPricing),
    Credit(CapabilityCreditRates),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CapabilityTokenRate {
    pub rate: Rate,
    pub cache: Option<CachePricing>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CapabilityTokenTiers {
    pub measured_by: TokenTierBasis,
    pub tiers: Vec<CapabilityTokenTier>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CapabilityTokenTier {
    pub from_tokens: u64,
    pub to_tokens: Option<u64>,
    pub rate: CapabilityTokenRate,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CapabilitySchedule {
    pub timezone: String,
    pub slots: Vec<CapabilityPriceSlot>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CapabilityPriceSlot {
    pub from_hour: u8,
    pub to_hour: u8,
    pub pricing: CapabilityScheduledRates,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityScheduledRates {
    Fixed(CapabilityRates),
    TieredToken(CapabilityTokenTiers),
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CapabilityCreditRates {
    pub rates: Vec<CreditRate>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModelBilling {
    Subscription(SubscriptionBilling),
    Free(FreePricing),
    Request(RequestPricing),
    Credit(CreditConversion),
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SubscriptionBilling {
    pub product: String,
    pub plan: Option<String>,
    pub fee: Option<SubscriptionFee>,
    pub entitlements: Vec<UsageAllowance>,
}
