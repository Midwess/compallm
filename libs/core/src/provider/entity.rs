use std::fmt;

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

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ProviderApiKeyError;

impl fmt::Debug for ProviderApiKeyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("ProviderApiKeyError")
    }
}

impl fmt::Display for ProviderApiKeyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("provider API key must not be empty")
    }
}

impl std::error::Error for ProviderApiKeyError {}

pub struct NewProvider {
    pub id: u64,
    pub organization_id: u64,
    pub name: String,
    pub url: String,
    pub api_key: ProviderApiKey,
    pub api_protocol: ApiProtocol,
}

pub struct ProviderUpdate {
    pub name: String,
    pub url: String,
    /// Preserve the current credential when omitted.
    pub api_key: Option<ProviderApiKey>,
    pub api_protocol: ApiProtocol,
}
