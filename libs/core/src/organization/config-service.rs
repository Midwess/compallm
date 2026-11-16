use super::{
    Organization, OrganizationConfigError,
    profile_service::{ProfileError, ProfilePatch, ProfileService},
};
use crate::{authorization::MemberPermission, persistent::organization::OrganizationRepository};

pub struct ConfigService {
    profile: ProfileService,
}
impl ConfigService {
    pub fn new(organizations: &'static Box<dyn OrganizationRepository>) -> Self {
        Self {
            profile: ProfileService::new(organizations.as_ref(), None),
        }
    }
    /// Rename through the shared organization profile rules.
    pub async fn update_name(
        &self,
        permission: &MemberPermission,
        name: &str,
    ) -> Result<Organization, OrganizationConfigError> {
        self.profile
            .update_profile(
                permission,
                ProfilePatch {
                    name: Some(name.into()),
                    avatar_uploaded: false,
                },
            )
            .await
            .map_err(|error| match error {
                ProfileError::Permission(error) => OrganizationConfigError::Permission(error),
                ProfileError::Persistence(error) => OrganizationConfigError::Persistence(error),
                ProfileError::Connection(error) => OrganizationConfigError::Connection(error),
                _ => OrganizationConfigError::InvalidName,
            })
    }
}
