use async_trait::async_trait;
use compallm_core::persistent::storage::user_space::*;
use hero::cancellation::FutureExtension;
use hero::s3::{ObjectPath, PostUploadOptions};

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
        if user_id == 0 {
            return Err(StorageError::InvalidRequest);
        }
        let cancellation = self.s3.cancellation();
        match self
            .s3
            .client
            .raw_client()
            .head_object()
            .bucket(&self.s3.settings.bucket)
            .key(Self::get_avatar_path(user_id).trim_start_matches('/'))
            .send()
            .with_cancel(&cancellation)
            .await
            .map_err(|_| StorageError::Cancelled)?
        {
            Ok(_) => Ok(true),
            Err(error) if super::error::status(&error) == Some(404) => Ok(false),
            Err(_) => Err(StorageError::Unavailable),
        }
    }

    async fn generate_avatar_upload(
        &self,
        user_id: u64,
        constraints: &AvatarUploadConstraints,
    ) -> Result<AvatarUpload, StorageError> {
        if user_id == 0 {
            return Err(StorageError::InvalidRequest);
        }
        let path = ObjectPath::new(
            &self.s3.settings.bucket,
            Self::get_avatar_path(user_id).trim_start_matches('/'),
        );
        if constraints.content_type.is_empty() {
            return Err(StorageError::InvalidRequest);
        }
        let cancellation = self.s3.cancellation();
        let post = self
            .s3
            .upload_client
            .sign_post_upload(
                &path,
                &PostUploadOptions {
                    expires_at: constraints.expires_at,
                    min_content_length: constraints.min_bytes,
                    max_content_length: constraints.max_bytes,
                    content_type: Some(constraints.content_type.clone()),
                },
            )
            .with_cancel(&cancellation)
            .await
            .map_err(|_| StorageError::Cancelled)?
            .map_err(|error| match error {
                hero::s3::Error::InvalidRequest(_) => StorageError::InvalidRequest,
                _ => StorageError::Unavailable,
            })?;
        Ok(AvatarUpload {
            url: post.url,
            fields: post.fields,
            expires_at: constraints.expires_at,
        })
    }
}
