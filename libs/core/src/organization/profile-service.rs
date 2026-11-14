use super::{Organization, OrganizationStatus};
use crate::{
    authorization::{
        MemberPermission, MemberPermissionError, condition::CheckPermission, policy::Action,
    },
    persistent::{
        ConnectionError, DbContext,
        organization::{OrganizationRepository, OrganizationRepositoryError},
        storage::{
            AvatarMediaType, AvatarUpload, AvatarUploadConstraints, StorageError,
            org_space::OrganizationStorage,
        },
    },
};

#[derive(Default)]
pub struct ProfilePatch {
    pub name: Option<String>,
    pub avatar_uploaded: bool,
}

#[derive(Debug, thiserror::Error)]
pub enum ProfileError {
    #[error(transparent)]
    Permission(#[from] MemberPermissionError),
    #[error(transparent)]
    Persistence(#[from] OrganizationRepositoryError),
    #[error(transparent)]
    Connection(#[from] ConnectionError),
    #[error(transparent)]
    Storage(#[from] StorageError),
    #[error("invalid organization profile operation")]
    Invalid,
}

pub struct ProfileService {
    organizations: &'static dyn OrganizationRepository,
    storage: Option<&'static dyn OrganizationStorage>,
}

impl ProfileService {
    pub fn new(
        organizations: &'static dyn OrganizationRepository,
        storage: Option<&'static dyn OrganizationStorage>,
    ) -> Self {
        Self {
            organizations,
            storage,
        }
    }

    async fn require_organization(
        &self,
        permission: &MemberPermission,
    ) -> Result<Organization, ProfileError> {
        Action::OrganizationSettingsUpdate.check_permission(permission)?;
        self.organizations
            .find_by_id(permission.organization_id())
            .await?
            .filter(|organization| organization.status == OrganizationStatus::Active)
            .ok_or(OrganizationRepositoryError::NotFound.into())
    }

    fn outside_transaction() -> Result<(), ProfileError> {
        if DbContext::current()?.postgres().is_transaction() {
            return Err(ProfileError::Invalid);
        }
        Ok(())
    }

    pub async fn generate_avatar_url(
        &self,
        permission: &MemberPermission,
        media_type: AvatarMediaType,
        size: u64,
    ) -> Result<AvatarUpload, ProfileError> {
        let organization = self.require_organization(permission).await?;
        let constraints = AvatarUploadConstraints::image(media_type, size)?;
        Self::outside_transaction()?;
        Ok(self
            .storage
            .ok_or(StorageError::Unavailable)?
            .generate_avatar_upload(organization.id, &constraints)
            .await?)
    }

    pub async fn update_profile(
        &self,
        permission: &MemberPermission,
        patch: ProfilePatch,
    ) -> Result<Organization, ProfileError> {
        let organization = self.require_organization(permission).await?;
        let name = patch
            .name
            .as_deref()
            .map(Organization::normalize_name)
            .transpose()
            .map_err(|_| ProfileError::Invalid)?;
        let avatar_path = if patch.avatar_uploaded {
            Self::outside_transaction()?;
            let storage = self.storage.ok_or(StorageError::Unavailable)?;
            if !storage.avatar_uploaded(organization.id).await? {
                return Err(StorageError::NotFound.into());
            }
            Some(storage.avatar_path(organization.id))
        } else {
            None
        };
        if name.is_none() && avatar_path.is_none() {
            return Ok(organization);
        }
        Ok(self
            .organizations
            .update_profile(organization.id, name, avatar_path)
            .await?)
    }
}
