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

#[derive(Clone, Debug, validify::Validify)]
pub struct User {
    pub id: u64,
    #[modify(trim, custom(str::make_ascii_lowercase))]
    #[validate(length(max = 256), custom(function = Self::validate_email))]
    pub email: String,
    #[validate(length(min = 1, max = 256), custom(function = Self::validate_display_name_characters))]
    pub display_name: String,
    pub status: UserStatus,
}

impl User {
    /// Prepare a canonical user after the caller verifies ownership of the email.
    pub fn from_email(id: u64, email: &str) -> Result<Self, RegistrationError> {
        use validify::{Modify, Validate};
        let mut user = Self {
            id,
            email: email.into(),
            display_name: String::new(),
            status: UserStatus::Active,
        };
        user.modify();
        user.display_name = user
            .email
            .rsplit_once('@')
            .map(|(name, _)| name.to_owned())
            .unwrap_or_default();
        user.validate().map_err(|errors| {
            if errors
                .errors()
                .iter()
                .any(|error| error.field_name() == Some("email"))
            {
                RegistrationError::InvalidEmail
            } else {
                RegistrationError::InvalidDisplayName
            }
        })?;
        Ok(user)
    }

    pub fn validate_email(value: &str) -> Result<(), validify::ValidationError> {
        email_address::EmailAddress::is_valid(value)
            .then_some(())
            .ok_or_else(|| validify::ValidationError::new_field("email"))
    }

    /// Project-specific unquoted email local-part rules for display names.
    pub fn validate_display_name_characters(value: &str) -> Result<(), validify::ValidationError> {
        if compallm_utils::text::has_special_characters(value, ".!#$%&'*+-/=?^_`{|}~")
            || value.starts_with('.')
            || value.ends_with('.')
            || value.contains("..")
        {
            Err(validify::ValidationError::new_field("display_name"))
        } else {
            Ok(())
        }
    }
}
