#[path = "config-service.rs"]
pub mod config_service;
#[path = "creation-service.rs"]
pub mod creation_service;
pub mod entity;
pub mod error;

pub use config_service::ConfigService;
pub use creation_service::CreationService;
pub use entity::{Organization, OrganizationMember, OrganizationStatus};
pub use error::{OrganizationConfigError, OrganizationCreationError};

pub mod group;
pub mod role;
pub use group::Group;
pub use role::Role;

mod cedar;

#[path = "role-service.rs"]
pub mod role_service;
pub use role_service::{RoleError, RoleService};
