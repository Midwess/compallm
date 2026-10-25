//! Organization-owned LLM provider connections.

pub mod entity;
pub mod error;
#[path = "management-service.rs"]
pub mod management_service;

pub use entity::{
    ApiProtocol, NewProvider, Provider, ProviderApiKey, ProviderApiKeyError, ProviderUpdate,
};
pub use error::ProviderManagementError;
pub use management_service::ManagementService;
