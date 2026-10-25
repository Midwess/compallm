use crate::{
    persistent::{DbContext, organization::OrganizationRepository},
    transaction,
};

use super::{Organization, OrganizationConfigError};

pub struct ConfigService {
    db: &'static DbContext,
    #[allow(clippy::borrowed_box)]
    organizations: &'static Box<dyn OrganizationRepository>,
}

impl ConfigService {
    #[allow(clippy::borrowed_box)]
    pub fn new(
        db: &'static DbContext,
        organizations: &'static Box<dyn OrganizationRepository>,
    ) -> Self {
        Self { db, organizations }
    }

    /// The caller must authorize configuration access before invoking this service.
    pub async fn update_name(
        &self,
        organization_id: u64,
        name: &str,
    ) -> Result<Organization, OrganizationConfigError> {
        let name = name.trim();
        if !Organization::is_valid_name(name) {
            return Err(OrganizationConfigError::InvalidName);
        }
        transaction!(self.db, {
            Ok(self
                .organizations
                .update_name(organization_id, name)
                .await?)
        })
    }
}
