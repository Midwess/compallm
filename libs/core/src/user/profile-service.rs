use super::{User, UserStatus};
use crate::{
    authentication::{AuthError, UserAccessToken},
    persistent::{
        DbContext,
        storage::user_space::{
            AvatarUpload, AvatarUploadConstraints, StorageError, UserStorageSpace,
        },
        user::{UserRepository, UserRepositoryError},
    },
};
use validify::Validify;

pub use crate::persistent::storage::AvatarMediaType;

#[derive(Default, Validify)]
pub struct ProfilePatch {
    #[modify(trim)]
    #[validate(length(max = 256))]
    pub display_name: Option<String>,
    #[modify(trim, custom(str::make_ascii_lowercase))]
    #[validate(length(min = 1, max = 256), custom(function = User::validate_username_characters))]
    pub username: Option<String>,
    pub avatar_uploaded: bool,
}

#[derive(Debug, thiserror::Error)]
pub enum ProfileError {
    #[error(transparent)]
    Authentication(#[from] AuthError),
    #[error(transparent)]
    Persistence(#[from] UserRepositoryError),
    #[error(transparent)]
    Storage(#[from] StorageError),
    #[error(transparent)]
    Invalid(#[from] validify::ValidationErrors),
    #[error("invalid profile operation")]
    InvalidOperation,
    #[error("profile operation denied")]
    Denied,
}

pub struct ProfileService {
    users: &'static dyn UserRepository,
    storage: Option<&'static dyn UserStorageSpace>,
}

impl ProfileService {
    pub fn new(
        users: &'static dyn UserRepository,
        storage: Option<&'static dyn UserStorageSpace>,
    ) -> Self {
        Self { users, storage }
    }

    async fn require_user(&self, actor: &UserAccessToken) -> Result<User, ProfileError> {
        actor.validate(compallm_utils::time::unix_timestamp())?;
        let user = self
            .users
            .find_by_id(actor.session.user_id)
            .await?
            .ok_or(ProfileError::Denied)?;
        if user.status != UserStatus::Active {
            return Err(ProfileError::Denied);
        }
        Ok(user)
    }

    fn outside_transaction() -> Result<(), ProfileError> {
        if DbContext::current()
            .map_err(UserRepositoryError::from)?
            .postgres()
            .is_transaction()
        {
            return Err(ProfileError::InvalidOperation);
        }
        Ok(())
    }

    pub async fn generate_url_avatar_url(
        &self,
        actor: &UserAccessToken,
        media_type: AvatarMediaType,
        size: u64,
    ) -> Result<AvatarUpload, ProfileError> {
        actor.validate(compallm_utils::time::unix_timestamp())?;
        if size == 0 || size > 5 * 1024 * 1024 {
            return Err(ProfileError::InvalidOperation);
        }
        Self::outside_transaction()?;
        let user = self.require_user(actor).await?;
        Ok(self
            .storage
            .ok_or(StorageError::Unavailable)?
            .generate_avatar_upload(user.id, &AvatarUploadConstraints::image(media_type, size)?)
            .await?)
    }

    pub async fn update_profile(
        &self,
        actor: &UserAccessToken,
        mut patch: ProfilePatch,
    ) -> Result<User, ProfileError> {
        actor.validate(compallm_utils::time::unix_timestamp())?;
        patch.validify()?;
        let user = self.require_user(actor).await?;
        let avatar_path = if patch.avatar_uploaded {
            Self::outside_transaction()?;
            let storage = self.storage.ok_or(StorageError::Unavailable)?;
            if !storage.avatar_uploaded(user.id).await? {
                return Err(StorageError::NotFound.into());
            }
            Some(storage.avatar_path(user.id))
        } else {
            None
        };
        if patch.display_name.is_none() && patch.username.is_none() && avatar_path.is_none() {
            return Ok(user);
        }
        actor.validate(compallm_utils::time::unix_timestamp())?;
        Ok(self
            .users
            .update_profile(user.id, patch.display_name, patch.username, avatar_path)
            .await?)
    }
}
