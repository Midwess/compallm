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
        if name.is_empty() {
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
        self.create(&format!("{}'s workspace", user.display_name.trim()), user)
            .await
    }
}
