//! User domain identity and state used by services and repository contracts.
//! Persistence adapters map their storage models into these types.

use std::sync::LazyLock;

use cedar_policy::SchemaFragment;
use compallm_utils::email;

use crate::authorization::{
    SystemPolicy,
    policy::{EntityType, Policy},
};

pub mod error;
pub mod preferences;
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
    #[validate(length(max = 256), custom(function = email::validate_email))]
    pub email: String,
    #[validate(length(min = 1, max = 256), custom(function = Self::validate_display_name_characters))]
    pub display_name: String,
    pub status: UserStatus,
}

impl User {
    /// Prepare a canonical user after the caller verifies ownership of the email.
    pub async fn from_email(email: &str) -> Result<Self, RegistrationError> {
        use validify::{Modify, Validate};
        let mut user = Self {
            id: 0,
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
        user.id = hero::distributed_id::try_gen_id().await?;
        Ok(user)
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

impl SystemPolicy for User {
    /// Declare the user principal and its organization role/group relationships.
    /// Personal profile and account operations use their own ownership checks.
    fn policy() -> SchemaFragment {
        static DEFINITION: LazyLock<SchemaFragment> =
            LazyLock::new(|| Policy::new(EntityType::User).into());
        DEFINITION.clone()
    }
}
