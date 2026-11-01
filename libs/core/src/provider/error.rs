#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
#[error("provider API key must not be empty")]
pub struct ProviderApiKeyError;

#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
#[error("reasoning effort must be between 1 and 255")]
pub struct ReasoningEffortError;
