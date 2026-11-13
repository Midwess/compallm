//! Typed preferences and their reusable values.

pub mod accessibility;
pub mod calendar;
pub mod code_appearance;
pub mod member_display;
pub mod message_composition;
pub mod navigation;
pub mod sidebar;
pub mod theme;
pub mod typography;
pub mod user;
pub mod values;

pub use user::UserPreferences;

pub mod patch;
#[path = "user-service.rs"]
pub mod user_service;
pub use user_service::{UserPreferenceError, UserPreferenceService};
