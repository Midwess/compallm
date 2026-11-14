use hero::distributed_id::{generator::DistributedIdError, try_gen_id};
use std::sync::LazyLock;

use cedar_policy::SchemaFragment;

use crate::authorization::{
    SystemPolicy,
    policy::{EntityType, Policy},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OrganizationStatus {
    Active,
    Disabled,
}

#[derive(Clone, Debug)]
pub struct Organization {
    pub id: u64,
    pub name: String,
    pub avatar_path: Option<String>,
    pub status: OrganizationStatus,
}

impl Organization {
    /// Create a new identity; persistence restoration uses existing record fields instead.
    pub async fn new(name: impl Into<String>) -> Result<Self, DistributedIdError> {
        Ok(Self {
            id: try_gen_id().await?,
            name: name.into(),
            avatar_path: None,
            status: OrganizationStatus::Active,
        })
    }

    pub const MAX_NAME_LENGTH: usize = 63;

    pub fn normalize_name(name: &str) -> Result<String, validify::ValidationErrors> {
        use validify::Validify;
        let mut name = OrganizationName { value: name.into() };
        name.validify()?;
        Ok(name.value)
    }

    fn validate_name_characters(value: &str) -> Result<(), validify::ValidationError> {
        if compallm_utils::text::has_special_characters(value, "-") {
            Err(validify::ValidationError::new_field("organization_name"))
        } else {
            Ok(())
        }
    }
}

#[derive(validify::Validify)]
struct OrganizationName {
    #[modify(trim)]
    #[validate(length(min = 1, max = 63), custom(function = Organization::validate_name_characters))]
    value: String,
}

impl SystemPolicy for Organization {
    /// Declares private organization settings and destructive administration.
    ///
    /// - `organization.settings.read`: inspect non-public organization configuration.
    ///   Public profile/name visibility does not grant access to these settings.
    /// - `organization.settings.update`: change permitted organization configuration;
    ///   public profile reading and settings reading grant no mutation authority.
    /// - `organization.delete`: delete the entire organization through its protected
    ///   ownership, confirmation, retention, and concurrent last-owner safeguards.
    ///
    /// Organization profiles are readable by everyone without RBAC; no profile-read
    /// action exists. Public projections omit settings and membership administration.
    /// Protected targets remain in the verified active organization; an ID or a
    /// role held elsewhere is insufficient. Schema declarations do not grant access.
    fn policy() -> SchemaFragment {
        static DEFINITION: LazyLock<SchemaFragment> =
            LazyLock::new(|| Policy::new(EntityType::Organization).into());
        DEFINITION.clone()
    }
}
