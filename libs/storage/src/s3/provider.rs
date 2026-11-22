use super::S3Storage;
use compallm_core::persistent::storage::{AvatarUpload, AvatarUploadConstraints, StorageError};
use hero::{
    cancellation::FutureExtension,
    s3::{ObjectPath, PostUploadOptions},
};
impl S3Storage {
    pub async fn generate_provider_logo(
        &self,
        organization: u64,
        user: u64,
        draft: uuid::Uuid,
        constraints: &AvatarUploadConstraints,
    ) -> Result<AvatarUpload, StorageError> {
        let key = format!("provider-logos/{organization}/{user}/{draft}");
        let token = self.cancellation();
        let post = self
            .upload_client
            .sign_post_upload(
                &ObjectPath::new(&self.settings.bucket, &key),
                &PostUploadOptions {
                    expires_at: constraints.expires_at,
                    min_content_length: constraints.min_bytes,
                    max_content_length: constraints.max_bytes,
                    content_type: Some(constraints.content_type.clone()),
                },
            )
            .with_cancel(&token)
            .await
            .map_err(|_| StorageError::Cancelled)?
            .map_err(|_| StorageError::Unavailable)?;
        Ok(AvatarUpload {
            url: post.url,
            fields: post.fields,
            expires_at: constraints.expires_at,
        })
    }
    pub async fn provider_logo_uploaded(
        &self,
        organization: u64,
        user: u64,
        draft: uuid::Uuid,
    ) -> Result<String, StorageError> {
        let key = format!("provider-logos/{organization}/{user}/{draft}");
        let token = self.cancellation();
        let object = self
            .client
            .raw_client()
            .head_object()
            .bucket(&self.settings.bucket)
            .key(&key)
            .send()
            .with_cancel(&token)
            .await
            .map_err(|_| StorageError::Cancelled)?
            .map_err(|_| StorageError::NotFound)?;
        if !matches!(
            object.content_type(),
            Some("image/jpeg" | "image/png" | "image/webp")
        ) || !object
            .content_length()
            .is_some_and(|size| size > 0 && size <= 5 * 1024 * 1024)
        {
            return Err(StorageError::InvalidRequest);
        }
        self.public_url(&key)
    }
}
