//! Organization-owned LLM provider connections.

pub mod entity;
pub mod error;

pub use entity::{ApiProtocol, Provider, ProviderApiKey};
pub use error::ProviderApiKeyError;
