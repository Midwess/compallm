use async_trait::async_trait;
use compallm_core::persistent::storage::user_space::*;

pub struct S3UserSpace {
    s3: std::sync::Arc<super::S3Storage>,
}

impl S3UserSpace {
    pub fn new(s3: std::sync::Arc<super::S3Storage>) -> Self {
        Self { s3 }
    }
}

#[async_trait]
impl UserStorageSpace for S3UserSpace {
    fn avatar_path(&self, id: u64) -> String {
        Self::get_avatar_path(id)
    }
    async fn avatar_uploaded(&self, user_id: u64) -> Result<bool, StorageError> {
        self.s3.avatar_uploaded(user_id).await
    }
    async fn generate_avatar_upload(
        &self,
        user_id: u64,
        constraints: &AvatarUploadConstraints,
    ) -> Result<AvatarUpload, StorageError> {
        self.s3.generate_avatar_upload(user_id, constraints).await
    }
}
