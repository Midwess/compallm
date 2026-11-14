use super::S3Storage;
use async_trait::async_trait;
use compallm_core::persistent::storage::{
    AvatarUpload, AvatarUploadConstraints, StorageError, org_space::OrganizationStorage,
};
use std::sync::Arc;

pub struct OrgStorage {
    s3: Arc<S3Storage>,
}
impl OrgStorage {
    pub fn new(s3: Arc<S3Storage>) -> Self {
        Self { s3 }
    }
}

#[async_trait]
impl OrganizationStorage for OrgStorage {
    fn avatar_path(&self, id: u64) -> String {
        Self::get_avatar_path(id)
    }
    async fn avatar_uploaded(&self, id: u64) -> Result<bool, StorageError> {
        self.s3.avatar_uploaded(id).await
    }
    async fn generate_avatar_upload(
        &self,
        id: u64,
        constraints: &AvatarUploadConstraints,
    ) -> Result<AvatarUpload, StorageError> {
        self.s3.generate_avatar_upload(id, constraints).await
    }
}
