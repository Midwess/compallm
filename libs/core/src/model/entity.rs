use rust_decimal::Decimal;

/// An organization-owned logical model and its organization-facing prices.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Model {
    pub id: u64,
    pub organization_id: u64,
    pub name: String,
    pub input_price_per_million_tokens: Decimal,
    pub output_price_per_million_tokens: Decimal,
    pub cached_price_per_million_tokens: Decimal,
    pub enabled: bool,
    pub hidden: bool,
    pub settings: serde_json::Value,
}
