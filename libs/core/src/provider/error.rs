#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
#[error("provider API key must not be empty")]
pub struct ProviderApiKeyError;
