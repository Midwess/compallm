#[path = "user-space.rs"]
pub mod user_space;

#[path = "org-space.rs"]
pub mod org_space;

use std::{
    collections::BTreeMap,
    time::{Duration, SystemTime},
};

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

pub fn avatar_path(id: u64) -> String {
    format!("/avatars/{id}")
}

#[derive(Clone, Copy)]
pub enum AvatarMediaType {
    Jpeg,
    Png,
    Webp,
}
impl AvatarMediaType {
    pub fn content_type(self) -> &'static str {
        match self {
            Self::Jpeg => "image/jpeg",
            Self::Png => "image/png",
            Self::Webp => "image/webp",
        }
    }
}
impl AvatarUploadConstraints {
    pub fn image(media_type: AvatarMediaType, size: u64) -> Result<Self, StorageError> {
        if size == 0 || size > 5 * 1024 * 1024 {
            return Err(StorageError::InvalidRequest);
        }
        Ok(Self {
            min_bytes: 1,
            max_bytes: 5 * 1024 * 1024,
            content_type: media_type.content_type().into(),
            expires_at: SystemTime::now() + Duration::from_secs(300),
        })
    }
}
