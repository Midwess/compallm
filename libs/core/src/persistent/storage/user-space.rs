use async_trait::async_trait;
use std::{collections::BTreeMap, time::SystemTime};

pub struct AvatarUploadConstraints {
    pub min_bytes: u64,
    pub max_bytes: u64,
    pub content_type: String,
    pub expires_at: SystemTime,
}

/// Signed fields are bearer authorization and must stay out of logs.
pub struct AvatarUpload {
    pub url: String,
    pub fields: BTreeMap<String, String>,
    pub expires_at: SystemTime,
}

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("invalid storage request")]
    InvalidRequest,
    #[error("avatar has not been uploaded")]
    NotFound,
    #[error("storage operation cancelled")]
    Cancelled,
    #[error("storage operation unavailable")]
    Unavailable,
}

/// Direct public uploads; callers must authorize the user before issuing credentials.
/// User IDs come from verified authority. Storage calls never require a DB transaction.
#[async_trait]
pub trait UserStorageSpace: Send + Sync {
    fn get_avatar_path(id: u64) -> String
    where
        Self: Sized,
    {
        format!("/avatars/{id}")
    }

    fn avatar_path(&self, id: u64) -> String;

    async fn avatar_uploaded(&self, user_id: u64) -> Result<bool, StorageError>;
    async fn generate_avatar_upload(
        &self,
        user_id: u64,
        constraints: &AvatarUploadConstraints,
    ) -> Result<AvatarUpload, StorageError>;
}
