//! Organization-owned LLM provider connections.

pub mod entity;
pub mod error;

pub use entity::{
    ApiProtocol, Provider, ProviderApiKey, ProviderModel, ReasoningCapabilities, ReasoningEffort,
    ReasoningSupport,
};
pub use error::{ProviderApiKeyError, ReasoningEffortError};
