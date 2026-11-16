pub use super::{AvatarUpload, AvatarUploadConstraints, StorageError};
use async_trait::async_trait;

/// Direct public uploads; callers must authorize the user before issuing credentials.
/// User IDs come from verified authority. Storage calls never require a DB transaction.
#[async_trait]
pub trait UserStorageSpace: Send + Sync {
    fn get_avatar_path(id: u64) -> String
    where
        Self: Sized,
    {
        super::avatar_path(id)
    }

    fn avatar_path(&self, id: u64) -> String;

    async fn avatar_uploaded(&self, user_id: u64) -> Result<bool, StorageError>;
    async fn generate_avatar_upload(
        &self,
        user_id: u64,
        constraints: &AvatarUploadConstraints,
    ) -> Result<AvatarUpload, StorageError>;
}
