use compallm_core::provider as domain;
use juniper::{GraphQLEnum, GraphQLObject};

#[derive(GraphQLEnum)]
pub enum CapabilityPricingKind {
    Usage,
    Free,
    Included,
    ModelBilling,
}
#[derive(GraphQLObject)]
pub struct CapabilityPricing {
    pub kind: CapabilityPricingKind,
    pub usage: Option<CapabilityUsage>,
}
impl From<&domain::CapabilityPricing> for CapabilityPricing {
    fn from(v: &domain::CapabilityPricing) -> Self {
        match v {
            domain::CapabilityPricing::Usage(v) => Self {
                kind: CapabilityPricingKind::Usage,
                usage: Some(v.into()),
            },
            domain::CapabilityPricing::Free => Self {
                kind: CapabilityPricingKind::Free,
                usage: None,
            },
            domain::CapabilityPricing::Included => Self {
                kind: CapabilityPricingKind::Included,
                usage: None,
            },
            domain::CapabilityPricing::ModelBilling => Self {
                kind: CapabilityPricingKind::ModelBilling,
                usage: None,
            },
        }
    }
}
#[derive(GraphQLEnum)]
pub enum CapabilityPriceMode {
    Fixed,
    TieredToken,
    Scheduled,
}
#[derive(GraphQLEnum)]
pub enum PriceUnit {
    Tokens,
    Requests,
    Images,
    Seconds,
    Characters,
    Utf8Bytes,
    Credits,
    Videos,
}
impl From<domain::BillingUnit> for PriceUnit {
    fn from(v: domain::BillingUnit) -> Self {
        match v {
            domain::BillingUnit::Tokens => Self::Tokens,
            domain::BillingUnit::Requests => Self::Requests,
            domain::BillingUnit::Images => Self::Images,
            domain::BillingUnit::Seconds => Self::Seconds,
            domain::BillingUnit::Characters => Self::Characters,
            domain::BillingUnit::Utf8Bytes => Self::Utf8Bytes,
            domain::BillingUnit::Credits => Self::Credits,
            domain::BillingUnit::Videos => Self::Videos,
        }
    }
}
#[derive(GraphQLObject)]
pub struct CapabilityUsage {
    pub mode: CapabilityPriceMode,
    pub rates: Vec<CapabilityRate>,
    pub tier_basis: Option<PriceTierBasis>,
    pub tiers: Vec<CapabilityTier>,
    pub timezone: Option<String>,
    pub slots: Vec<CapabilitySlot>,
}
#[derive(GraphQLEnum)]
pub enum PriceTierBasis {
    InputTokens,
    TotalTokens,
}
#[derive(GraphQLObject)]
pub struct CapabilityTier {
    pub from_tokens: String,
    pub to_tokens: Option<String>,
    pub rate: CapabilityRate,
}
#[derive(GraphQLObject)]
pub struct CapabilitySlot {
    pub from_hour: i32,
    pub to_hour: i32,
    pub pricing: CapabilityUsage,
}
#[derive(GraphQLObject)]
pub struct CapabilityRate {
    pub unit: PriceUnit,
    pub amount: Option<String>,
    pub currency: Option<String>,
    pub quantity: Option<String>,
    pub credits: Option<String>,
    pub direction: Option<CreditUsageDirection>,
    pub cache: Option<CapabilityCachePrice>,
    pub conditions: Vec<super::configuration::ConfigurationEntry>,
}
#[derive(GraphQLObject)]
pub struct CapabilityCachePrice {
    pub mode: CapabilityCacheMode,
    pub amount: Option<String>,
    pub currency: Option<String>,
    pub quantity: Option<String>,
}
#[derive(GraphQLEnum)]
pub enum CapabilityCacheMode {
    SeparateRate,
    StandardInputRate,
    Included,
    Unsupported,
}
impl From<&domain::CachePricing> for CapabilityCachePrice {
    fn from(v: &domain::CachePricing) -> Self {
        let (mode, rate) = match v {
            domain::CachePricing::SeparateRate(v) => {
                (CapabilityCacheMode::SeparateRate, Some(&v.rate))
            }
            domain::CachePricing::StandardInputRate => {
                (CapabilityCacheMode::StandardInputRate, None)
            }
            domain::CachePricing::Included => (CapabilityCacheMode::Included, None),
            domain::CachePricing::Unsupported => (CapabilityCacheMode::Unsupported, None),
        };
        Self {
            mode,
            amount: rate.and_then(|v| v.amount.map(|v| v.to_string())),
            currency: rate.and_then(|v| v.currency.clone()),
            quantity: rate.and_then(|v| v.quantity.map(|v| v.to_string())),
        }
    }
}
impl From<(&domain::Rate, domain::BillingUnit)> for CapabilityRate {
    fn from((v, unit): (&domain::Rate, domain::BillingUnit)) -> Self {
        Self {
            unit: unit.into(),
            amount: v.amount.map(|v| v.to_string()),
            currency: v.currency.clone(),
            quantity: v.quantity.map(|v| v.to_string()),
            credits: None,
            direction: None,
            cache: None,
            conditions: vec![],
        }
    }
}
impl From<&domain::CapabilityTokenRate> for CapabilityRate {
    fn from(v: &domain::CapabilityTokenRate) -> Self {
        let mut result = Self::from((&v.rate, domain::BillingUnit::Tokens));
        result.cache = v.cache.as_ref().map(Into::into);
        result
    }
}
struct Conditions<'a>(&'a domain::RateConditions);
impl From<Conditions<'_>> for Vec<super::configuration::ConfigurationEntry> {
    fn from(Conditions(v): Conditions<'_>) -> Self {
        let mut fields = v.parameters.clone();
        for (k, value) in [
            ("resolution", v.resolution.clone()),
            ("quality", v.quality.clone()),
            ("serviceTier", v.service_tier.clone()),
            ("audio", v.audio.map(|v| v.to_string())),
        ] {
            if let Some(value) = value {
                fields.insert(k.into(), value);
            }
        }
        fields
            .into_iter()
            .map(|(key, value)| super::configuration::ConfigurationEntry { key, value })
            .collect()
    }
}
struct Rates<'a>(&'a domain::CapabilityRates);
impl From<Rates<'_>> for Vec<CapabilityRate> {
    fn from(Rates(v): Rates<'_>) -> Self {
        match v {
            domain::CapabilityRates::Token(v) => vec![v.into()],
            domain::CapabilityRates::Audio(v) => {
                let (rate, unit) = match v {
                    domain::AudioPricing::Duration(v) => (&v.rate, domain::BillingUnit::Seconds),
                    domain::AudioPricing::Character(v) => {
                        (&v.rate, domain::BillingUnit::Characters)
                    }
                    domain::AudioPricing::Utf8Bytes(v) => (&v.rate, domain::BillingUnit::Utf8Bytes),
                };
                vec![(rate, unit).into()]
            }
            domain::CapabilityRates::Image(v) => v
                .rates
                .iter()
                .map(|v| {
                    let mut r = CapabilityRate::from((&v.rate, domain::BillingUnit::Images));
                    r.conditions = Conditions(&v.conditions).into();
                    r
                })
                .collect(),
            domain::CapabilityRates::Video(v) => v
                .rates
                .iter()
                .map(|v| {
                    let unit = match v.unit {
                        domain::VideoBillingUnit::Seconds => domain::BillingUnit::Seconds,
                        domain::VideoBillingUnit::Videos => domain::BillingUnit::Videos,
                    };
                    let mut r = CapabilityRate::from((&v.rate, unit));
                    r.conditions = Conditions(&v.conditions).into();
                    r
                })
                .collect(),
            domain::CapabilityRates::Request(v) => {
                vec![(&v.rate, domain::BillingUnit::Requests).into()]
            }
            domain::CapabilityRates::Credit(v) => v
                .rates
                .iter()
                .map(|r| CapabilityRate {
                    unit: r.unit.into(),
                    amount: None,
                    currency: None,
                    quantity: r.quantity.map(|v| v.to_string()),
                    credits: r.credits.map(|v| v.to_string()),
                    direction: r.direction.map(Into::into),
                    cache: None,
                    conditions: Conditions(&r.conditions).into(),
                })
                .collect(),
        }
    }
}
impl From<&domain::CapabilityUsagePricing> for CapabilityUsage {
    fn from(v: &domain::CapabilityUsagePricing) -> Self {
        let mut result = Self {
            mode: CapabilityPriceMode::Fixed,
            rates: vec![],
            tier_basis: None,
            tiers: vec![],
            timezone: None,
            slots: vec![],
        };
        match v {
            domain::CapabilityUsagePricing::Fixed(v) => result.rates = Rates(v).into(),
            domain::CapabilityUsagePricing::TieredToken(v) => {
                result.mode = CapabilityPriceMode::TieredToken;
                result.tier_basis = Some(match v.measured_by {
                    domain::TokenTierBasis::InputTokens => PriceTierBasis::InputTokens,
                    domain::TokenTierBasis::TotalTokens => PriceTierBasis::TotalTokens,
                });
                result.tiers = v
                    .tiers
                    .iter()
                    .map(|v| CapabilityTier {
                        from_tokens: v.from_tokens.to_string(),
                        to_tokens: v.to_tokens.map(|v| v.to_string()),
                        rate: (&v.rate).into(),
                    })
                    .collect();
            }
            domain::CapabilityUsagePricing::Scheduled(v) => {
                result.mode = CapabilityPriceMode::Scheduled;
                result.timezone = Some(v.timezone.clone());
                result.slots = v
                    .slots
                    .iter()
                    .map(|v| CapabilitySlot {
                        from_hour: v.from_hour.into(),
                        to_hour: v.to_hour.into(),
                        pricing: match &v.pricing {
                            domain::CapabilityScheduledRates::Fixed(v) => {
                                (&domain::CapabilityUsagePricing::Fixed(v.clone())).into()
                            }
                            domain::CapabilityScheduledRates::TieredToken(v) => {
                                (&domain::CapabilityUsagePricing::TieredToken(v.clone())).into()
                            }
                        },
                    })
                    .collect();
            }
        }
        result
    }
}

#[derive(GraphQLEnum)]
pub enum ModelBillingKind {
    Subscription,
    Free,
    Request,
    Credit,
}
#[derive(GraphQLObject)]
pub struct ModelBilling {
    pub kind: ModelBillingKind,
    pub product: Option<String>,
    pub plan: Option<String>,
    pub fee: Option<BillingFee>,
    pub entitlements: Vec<BillingAllowance>,
    pub request_rate: Option<CapabilityRate>,
    pub amount_per_credit: Option<String>,
    pub currency: Option<String>,
    pub description: Option<String>,
}
#[derive(GraphQLObject)]
pub struct BillingFee {
    pub amount: Option<String>,
    pub currency: Option<String>,
    pub period: Option<PriceBillingPeriod>,
}
#[derive(GraphQLObject)]
pub struct BillingAllowance {
    pub unit: PriceUnit,
    pub quota: Option<String>,
    pub reset_period: Option<PriceBillingPeriod>,
}
#[derive(GraphQLEnum)]
pub enum PriceBillingPeriod {
    Minute,
    Hour,
    Day,
    Week,
    Month,
    Year,
}
impl From<domain::BillingPeriod> for PriceBillingPeriod {
    fn from(v: domain::BillingPeriod) -> Self {
        match v {
            domain::BillingPeriod::Minute => Self::Minute,
            domain::BillingPeriod::Hour => Self::Hour,
            domain::BillingPeriod::Day => Self::Day,
            domain::BillingPeriod::Week => Self::Week,
            domain::BillingPeriod::Month => Self::Month,
            domain::BillingPeriod::Year => Self::Year,
        }
    }
}
impl From<&domain::UsageAllowance> for BillingAllowance {
    fn from(v: &domain::UsageAllowance) -> Self {
        Self {
            unit: v.unit.into(),
            quota: v.quota.map(|v| v.to_string()),
            reset_period: v.reset_period.map(Into::into),
        }
    }
}
impl From<&domain::ModelBilling> for ModelBilling {
    fn from(v: &domain::ModelBilling) -> Self {
        let mut result = Self {
            kind: ModelBillingKind::Free,
            product: None,
            plan: None,
            fee: None,
            entitlements: vec![],
            request_rate: None,
            amount_per_credit: None,
            currency: None,
            description: None,
        };
        match v {
            domain::ModelBilling::Subscription(v) => {
                result.kind = ModelBillingKind::Subscription;
                result.product = Some(v.product.clone());
                result.plan = v.plan.clone();
                result.fee = v.fee.as_ref().map(|v| BillingFee {
                    amount: v.amount.map(|v| v.to_string()),
                    currency: v.currency.clone(),
                    period: v.period.map(Into::into),
                });
                result.entitlements = v.entitlements.iter().map(Into::into).collect();
            }
            domain::ModelBilling::Free(v) => {
                result.entitlements = v.allowances.iter().map(Into::into).collect();
                result.description = v.description.clone();
            }
            domain::ModelBilling::Request(v) => {
                result.kind = ModelBillingKind::Request;
                result.request_rate = Some((&v.rate, domain::BillingUnit::Requests).into());
            }
            domain::ModelBilling::Credit(v) => {
                result.kind = ModelBillingKind::Credit;
                result.amount_per_credit = v.amount_per_credit.map(|v| v.to_string());
                result.currency = v.currency.clone();
            }
        }
        result
    }
}

#[derive(GraphQLEnum)]
pub enum CreditUsageDirection {
    Input,
    Output,
    CacheRead,
}
impl From<domain::UsageDirection> for CreditUsageDirection {
    fn from(v: domain::UsageDirection) -> Self {
        match v {
            domain::UsageDirection::Input => Self::Input,
            domain::UsageDirection::Output => Self::Output,
            domain::UsageDirection::CacheRead => Self::CacheRead,
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

#[derive(GraphQLEnum)]
pub enum BillingModel {
    Metered,
    Subscription,
    SubscriptionWithOverage,
    Free,
    Unknown,
}
#[derive(GraphQLEnum)]
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
#[derive(GraphQLEnum)]
pub enum CacheBilling {
    SeparateRate,
    StandardInputRate,
    Unsupported,
    Included,
    NotApplicable,
    PerRoute,
    Unknown,
}
