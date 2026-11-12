use crate::authorization::{MemberPermission, condition::CheckPermission, policy::Action};
use crate::persistent::organization::OrganizationRepository;

use super::{Organization, OrganizationConfigError};

pub struct ConfigService {
    organizations: &'static Box<dyn OrganizationRepository>,
}

impl ConfigService {
    pub fn new(organizations: &'static Box<dyn OrganizationRepository>) -> Self {
        Self { organizations }
    }

    /// Rename the member's active organization after checking settings authority.
    pub async fn update_name(
        &self,
        permission: &MemberPermission,
        name: &str,
    ) -> Result<Organization, OrganizationConfigError> {
        Action::OrganizationSettingsUpdate.check_permission(permission)?;
        let name =
            Organization::normalize_name(name).map_err(|_| OrganizationConfigError::InvalidName)?;

        Ok(self
            .organizations
            .update_name(permission.organization_id(), &name)
            .await?)
    }
}
