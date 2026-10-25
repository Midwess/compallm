//! User domain identity and state used by services and repository contracts.
//! Persistence adapters map their storage models into these types.

pub mod error;
#[path = "registration-service.rs"]
pub mod registration_service;

pub use error::RegistrationError;
pub use registration_service::RegistrationService;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UserStatus {
    Active,
    Disabled,
}

#[derive(Clone, Debug)]
pub struct User {
    pub id: u64,
    pub email: String,
    pub display_name: String,
    pub status: UserStatus,
}
