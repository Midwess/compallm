/// A non-negative USD price per million tokens with eight decimal places.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Price(u128);

impl Price {
    pub const SCALE: u32 = 8;

    /// Creates a price from units of 10^-8 USD.
    pub const fn from_scaled(value: u128) -> Self {
        Self(value)
    }

    /// Returns the price as units of 10^-8 USD.
    pub const fn scaled(self) -> u128 {
        self.0
    }
}

/// An organization-owned logical model and its organization-facing prices.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Model {
    pub id: u64,
    pub organization_id: u64,
    pub name: String,
    pub input_price_per_million_tokens: Price,
    pub output_price_per_million_tokens: Price,
    pub cached_price_per_million_tokens: Price,
    pub settings: serde_json::Value,
}
