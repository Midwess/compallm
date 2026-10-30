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
    organizations: &'static Box<dyn OrganizationRepository>,
}

impl CreationService {
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
        let name = Organization::normalize_name(name)
            .map_err(|_| OrganizationCreationError::InvalidName)?;
        self.persist(&name, user).await
    }

    async fn persist(
        &self,
        name: &str,
        user: &User,
    ) -> Result<Organization, OrganizationCreationError> {
        transaction!(self.db, {
            let organization = self
                .organizations
                .create(Organization {
                    id: hero::distributed_id::try_gen_id().await?,
                    name: name.into(),
                    status: OrganizationStatus::Active,
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
        let base = &user.display_name;
        let base = if base.is_empty() {
            format!("user-{}", user.id)
        } else {
            base.to_owned()
        };

        let suffix = "'s organization";
        let base: String = base
            .chars()
            .take(Organization::MAX_NAME_LENGTH - suffix.len())
            .collect();
        // Default labels preserve the user's spelling and possessive suffix.
        self.persist(&format!("{base}{suffix}"), user).await
    }
}
