//! Organization-owned LLM provider connections.

pub mod entity;
pub mod error;
#[path = "organization-provider-service.rs"]
pub mod organization_provider_service;

pub use entity::{ApiProtocol, Provider, ProviderApiKey};
pub use error::{ProviderApiKeyError, ProviderManagementError};
pub use organization_provider_service::OrganizationProviderService;
