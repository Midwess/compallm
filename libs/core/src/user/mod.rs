//! User domain identity and state used by services and repository contracts.
//! Persistence adapters map their storage models into these types.

use std::sync::LazyLock;

use cedar_policy::SchemaFragment;
use compallm_utils::email;

use crate::authorization::SystemPolicy;

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
    /// Declares protected profile mutation and account deletion, not profile reads.
    ///
    /// - `user.profile.update`: change one's own allowed profile fields. Public
    ///   visibility of that profile never permits somebody else to edit it.
    ///   Email verification and authentication changes are separate workflows.
    /// - `user.account.delete`: delete one's own entire account through its
    ///   confirmation, reauthentication, last-owner, and retention safeguards.
    ///   Updating a profile does not imply account deletion.
    ///
    /// User profiles are readable by everyone without RBAC; no profile-read action
    /// or permit belongs here. Public projections exclude private authentication
    /// and membership-administration data. Knowing an ID grants no mutation rights.
    /// UserAuth defines authentication operations separately. Role/Group types
    /// describe membership only; baseline account decisions exclude custom forbids.
    fn policy() -> SchemaFragment {
        static DEFINITION: LazyLock<SchemaFragment> = LazyLock::new(|| {
            let (fragment, warnings) = SchemaFragment::from_cedarschema_str(
                r#"
namespace Compallm {
    entity User in [Role, Group, Organization];
    action "user.profile.update" appliesTo { principal: User, resource: User };
    action "user.account.delete" appliesTo { principal: User, resource: User };
}
"#,
            )
            .expect("User SystemPolicy must be a valid static Cedar schema fragment");
            assert!(
                warnings.count() == 0,
                "User SystemPolicy must not contain schema warnings"
            );
            fragment
        });
        DEFINITION.clone()
    }
}
