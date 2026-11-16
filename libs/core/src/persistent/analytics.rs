use crate::analytics::UserProfileValue;
#[derive(Debug, thiserror::Error)]
#[error("analytics persistence failed")]
pub struct AnalyticsRepositoryError(#[from] pub anyhow::Error);
impl From<super::ConnectionError> for AnalyticsRepositoryError {
    fn from(value: super::ConnectionError) -> Self {
        Self(value.into())
    }
}
#[async_trait::async_trait]
pub trait AnalyticsRepository: Send + Sync {
    async fn find(
        &self,
        user_id: u64,
        key: &str,
    ) -> Result<Option<UserProfileValue>, AnalyticsRepositoryError>;
    async fn put(
        &self,
        user_id: u64,
        value: UserProfileValue,
    ) -> Result<UserProfileValue, AnalyticsRepositoryError>;
}
