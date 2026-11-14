use crate::S3Settings;
use compallm_core::persistent::storage::user_space::StorageError;
use hero::{
    cancellation::CancellationToken,
    s3::{S3Client, S3Config},
};
use std::time::Duration;

pub struct S3Storage {
    pub(super) client: S3Client,
    pub(super) upload_client: S3Client,
    pub(super) settings: S3Settings,
}

impl S3Storage {
    /// Build a public delivery URL without contacting storage or exposing credentials.
    pub fn public_url(&self, path: &str) -> Result<String, StorageError> {
        let path = path.trim_start_matches('/');
        if path.is_empty()
            || path
                .split('/')
                .any(|part| part.is_empty() || part == "." || part == "..")
        {
            return Err(StorageError::InvalidRequest);
        }
        let mut url = url::Url::parse(&self.settings.public_base_url)
            .map_err(|_| StorageError::InvalidRequest)?;
        url.path_segments_mut()
            .map_err(|_| StorageError::InvalidRequest)?
            .pop_if_empty()
            .extend(path.split('/'));
        Ok(url.into())
    }

    pub fn new(settings: S3Settings) -> Result<Self, StorageError> {
        for value in [
            &settings.endpoint,
            &settings.upload_endpoint,
            &settings.public_base_url,
        ] {
            let url = url::Url::parse(value).map_err(|_| StorageError::InvalidRequest)?;
            if !matches!(url.scheme(), "http" | "https")
                || url.host_str().is_none()
                || !url.username().is_empty()
                || url.password().is_some()
                || url.query().is_some()
                || url.fragment().is_some()
            {
                return Err(StorageError::InvalidRequest);
            }
        }
        if settings.bucket.is_empty()
            || settings.bucket.contains(['/', ':'])
            || settings.region.is_empty()
            || settings.access_key.is_empty()
            || settings.secret_key.is_empty()
            || settings.timeout_seconds == 0
        {
            return Err(StorageError::InvalidRequest);
        }
        let config = S3Config {
            region: settings.region.clone(),
            access_key_id: settings.access_key.clone(),
            secret_access_key: settings.secret_key.clone(),
            endpoint_url: Some(settings.endpoint.clone()),
            force_path_style: settings.force_path_style,
        };
        let client = config.build_client();
        let upload_client = if settings.endpoint == settings.upload_endpoint {
            client.clone()
        } else {
            S3Config {
                endpoint_url: Some(settings.upload_endpoint.clone()),
                ..config
            }
            .build_client()
        };
        Ok(Self {
            client,
            upload_client,
            settings,
        })
    }

    pub(super) fn cancellation(&self) -> CancellationToken {
        let token = CancellationToken::current_or_new().child_token();
        token.cancel_after(Duration::from_secs(self.settings.timeout_seconds));
        token
    }
}

impl S3Storage {
    pub(super) async fn avatar_uploaded(&self, id: u64) -> Result<bool, StorageError> {
        use hero::cancellation::FutureExtension;
        if id == 0 {
            return Err(StorageError::InvalidRequest);
        }
        let cancellation = self.cancellation();
        match self
            .client
            .raw_client()
            .head_object()
            .bucket(&self.settings.bucket)
            .key(compallm_core::persistent::storage::avatar_path(id).trim_start_matches('/'))
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
    pub(super) async fn generate_avatar_upload(
        &self,
        id: u64,
        constraints: &compallm_core::persistent::storage::AvatarUploadConstraints,
    ) -> Result<compallm_core::persistent::storage::AvatarUpload, StorageError> {
        use hero::{
            cancellation::FutureExtension,
            s3::{ObjectPath, PostUploadOptions},
        };
        if id == 0 || constraints.content_type.is_empty() {
            return Err(StorageError::InvalidRequest);
        }
        let path = ObjectPath::new(
            &self.settings.bucket,
            compallm_core::persistent::storage::avatar_path(id).trim_start_matches('/'),
        );
        let cancellation = self.cancellation();
        let post = self
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
        Ok(compallm_core::persistent::storage::AvatarUpload {
            url: post.url,
            fields: post.fields,
            expires_at: constraints.expires_at,
        })
    }
}
