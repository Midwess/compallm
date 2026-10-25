use crate::{
    persistent::{DbContext, organization::OrganizationRepository},
    transaction,
    user::User,
};

use super::{
    Organization, OrganizationCreationError, OrganizationMember, OrganizationRole,
    OrganizationStatus,
};

pub struct CreationService {
    db: &'static DbContext,
    #[allow(clippy::borrowed_box)]
    organizations: &'static Box<dyn OrganizationRepository>,
}

impl CreationService {
    #[allow(clippy::borrowed_box)]
    pub fn new(
        db: &'static DbContext,
        organizations: &'static Box<dyn OrganizationRepository>,
    ) -> Self {
        Self { db, organizations }
    }

    /// Create the organization and its owner membership atomically.
    pub async fn create(
        &self,
        name: &str,
        user: &User,
    ) -> Result<Organization, OrganizationCreationError> {
        let name = name.trim();
        if !Organization::is_valid_name(name) {
            return Err(OrganizationCreationError::InvalidName);
        }
        transaction!(self.db, {
            let organization = self
                .organizations
                .create(Organization {
                    id: hero::distributed_id::try_gen_id().await?,
                    name: name.into(),
                    status: OrganizationStatus::Active,
                    created_by: user.id,
                })
                .await?;
            self.organizations
                .add_member(OrganizationMember {
                    organization_id: organization.id,
                    user_id: user.id,
                    role: OrganizationRole::Owner,
                })
                .await?;
            Ok(organization)
        })
    }

    pub async fn create_default(
        &self,
        user: &User,
    ) -> Result<Organization, OrganizationCreationError> {
        let base = user
            .display_name
            .split(|character: char| !character.is_ascii_alphanumeric())
            .filter(|part| !part.is_empty())
            .collect::<Vec<_>>()
            .join("-")
            .to_ascii_lowercase();
        let base = if base.is_empty() {
            format!("user-{}", user.id)
        } else {
            base
        };
        let suffix = "-workspace";
        let base: String = base
            .chars()
            .take(Organization::MAX_NAME_LENGTH - suffix.len())
            .collect();
        self.create(&format!("{}{suffix}", base.trim_end_matches('-')), user)
            .await
    }
}
