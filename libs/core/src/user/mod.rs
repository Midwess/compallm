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
#[path = "profile-service.rs"]
pub mod profile_service;
pub use profile_service::{AvatarMediaType, ProfileError, ProfilePatch, ProfileService};
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
    #[validate(length(max = 256))]
    pub display_name: String,
    #[modify(trim, custom(str::make_ascii_lowercase))]
    #[validate(length(min = 1, max = 256), custom(function = Self::validate_username_characters))]
    pub username: String,
    pub status: UserStatus,
    pub avatar_path: Option<String>,
}

impl User {
    /// Try the email local part, then its domain label, then numbered domain suffixes.
    pub fn username_candidate(email: &str, attempt: u64) -> String {
        let (local, domain) = email.rsplit_once('@').unwrap_or((email, "user"));
        let suffix = match attempt {
            0 => String::new(),
            1 => format!("-{}", domain.split('.').next().unwrap_or("user")),
            n => format!("-{}-{n}", domain.split('.').next().unwrap_or("user")),
        };
        let base: String = local
            .chars()
            .take(256_usize.saturating_sub(suffix.len()))
            .collect();
        format!("{base}{suffix}").to_ascii_lowercase()
    }

    /// Prepare a canonical user after the caller verifies ownership of the email.
    pub async fn from_email(email: &str) -> Result<Self, RegistrationError> {
        use validify::{Modify, Validate};
        let mut user = Self {
            id: 0,
            email: email.into(),
            display_name: String::new(),
            username: String::new(),
            status: UserStatus::Active,
            avatar_path: None,
        };
        user.modify();
        user.username = user
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
                RegistrationError::InvalidUsername
            }
        })?;
        user.id = hero::distributed_id::try_gen_id().await?;
        Ok(user)
    }

    /// Project-specific unquoted email local-part rules for usernames.
    pub fn validate_username_characters(value: &str) -> Result<(), validify::ValidationError> {
        if compallm_utils::text::has_special_characters(value, ".!#$%&'*+-/=?^_`{|}~")
            || value.starts_with('.')
            || value.ends_with('.')
            || value.contains("..")
        {
            Err(validify::ValidationError::new_field("username"))
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
