use super::{
    S3Storage,
    error::{map_error, status},
};
use aws_sdk_s3::types::{CorsConfiguration, CorsRule};
use compallm_core::persistent::storage::user_space::StorageError;
use hero::cancellation::FutureExtension;

impl S3Storage {
    /// Explicit provisioning replaces this dedicated bucket's CORS and access policy.
    pub async fn setup(&self) -> Result<(), StorageError> {
        if self.settings.allowed_origins.is_empty() {
            return Err(StorageError::InvalidRequest);
        }
        let cancellation = self.cancellation();
        let sdk = self.client.raw_client();
        let exists = sdk
            .head_bucket()
            .bucket(&self.settings.bucket)
            .send()
            .with_cancel(&cancellation)
            .await
            .map_err(|_| StorageError::Cancelled)?;
        if let Err(error) = exists {
            if status(&error) != Some(404) {
                return Err(StorageError::Unavailable);
            }
            let mut create = sdk.create_bucket().bucket(&self.settings.bucket);
            if self.settings.region != "us-east-1" {
                create = create.create_bucket_configuration(
                    aws_sdk_s3::types::CreateBucketConfiguration::builder()
                        .location_constraint(aws_sdk_s3::types::BucketLocationConstraint::from(
                            self.settings.region.as_str(),
                        ))
                        .build(),
                );
            }
            create
                .send()
                .with_cancel(&cancellation)
                .await
                .map_err(|_| StorageError::Cancelled)?
                .map_err(map_error)?;
        }
        let rule = CorsRule::builder()
            .set_allowed_origins(Some(self.settings.allowed_origins.clone()))
            .allowed_methods("POST")
            .allowed_methods("GET")
            .allowed_methods("HEAD")
            .allowed_headers("*")
            .expose_headers("ETag")
            .max_age_seconds(600)
            .build()
            .map_err(|_| StorageError::InvalidRequest)?;
        let cors = CorsConfiguration::builder()
            .cors_rules(rule)
            .build()
            .map_err(|_| StorageError::InvalidRequest)?;
        sdk.put_bucket_cors()
            .bucket(&self.settings.bucket)
            .cors_configuration(cors)
            .send()
            .with_cancel(&cancellation)
            .await
            .map_err(|_| StorageError::Cancelled)?
            .map_err(map_error)?;
        let policy = serde_json::json!({"Version":"2012-10-17", "Statement":[{
            "Effect":"Allow", "Principal":"*", "Action":["s3:GetObject"],
            "Resource":[format!("arn:aws:s3:::{}/avatars/*", self.settings.bucket)]
        }]});
        sdk.put_bucket_policy()
            .bucket(&self.settings.bucket)
            .policy(policy.to_string())
            .send()
            .with_cancel(&cancellation)
            .await
            .map_err(|_| StorageError::Cancelled)?
            .map_err(map_error)?;
        Ok(())
    }
}
