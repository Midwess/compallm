use crate::persistent::{DbContext, organization::OrganizationRepository};

use super::{Organization, OrganizationConfigError};

pub struct ConfigService {
    _db: &'static DbContext,
    organizations: &'static Box<dyn OrganizationRepository>,
}

impl ConfigService {
    pub fn new(
        db: &'static DbContext,
        organizations: &'static Box<dyn OrganizationRepository>,
    ) -> Self {
        Self {
            _db: db,
            organizations,
        }
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

        Ok(self
            .organizations
            .update_name(organization_id, name)
            .await?)
    }
}
