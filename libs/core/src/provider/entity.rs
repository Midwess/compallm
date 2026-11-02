use rust_decimal::Decimal;

use super::error::ProviderApiKeyError;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ApiProtocol {
    Anthropic,
    Openai,
    OpenaiCompatible,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Provider {
    pub id: u64,
    pub organization_id: u64,
    pub name: String,
    pub url: String,
    pub api_protocol: ApiProtocol,
    pub enabled: bool,
    pub models: Vec<ProviderModel>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProviderModel {
    pub id: u64,
    pub organization_id: u64,
    pub provider_id: u64,
    pub name: String,
    pub enabled: bool,
    pub input_price_per_million_tokens: Decimal,
    pub output_price_per_million_tokens: Decimal,
    pub cached_price_per_million_tokens: Decimal,
    pub reasoning_effort_values: Option<Vec<String>>,
    pub supports_reasoning_budget: bool,
    pub supports_adaptive_reasoning: bool,
    pub minimum_reasoning_budget_tokens: Option<u32>,
    pub maximum_reasoning_budget_tokens: Option<u32>,
}

/// Secret value persisted in the provider's `api_key` column.
///
/// This type deliberately does not implement `Debug`, `Display`, `Clone`, or
/// serialization. Callers must protect the value before it crosses a trust
/// boundary; repositories never return it from ordinary reads.
pub struct ProviderApiKey(String);

impl ProviderApiKey {
    pub fn new(value: impl Into<String>) -> Result<Self, ProviderApiKeyError> {
        let value = value.into();
        if value.is_empty() {
            return Err(ProviderApiKeyError);
        }
        Ok(Self(value))
    }

    pub fn into_storage_value(self) -> String {
        self.0
    }
}
