use crate::persistent::organization::OrganizationRepository;

use super::{Organization, OrganizationConfigError};

pub struct ConfigService {
    organizations: &'static Box<dyn OrganizationRepository>,
}

impl ConfigService {
    pub fn new(organizations: &'static Box<dyn OrganizationRepository>) -> Self {
        Self { organizations }
    }

    /// The caller must authorize configuration access before invoking this service.
    pub async fn update_name(
        &self,
        organization_id: u64,
        name: &str,
    ) -> Result<Organization, OrganizationConfigError> {
        let name =
            Organization::normalize_name(name).map_err(|_| OrganizationConfigError::InvalidName)?;

        Ok(self
            .organizations
            .update_name(organization_id, &name)
            .await?)
    }
}
