use crate::onboarding::{Context, Progress, Settings};
#[derive(Debug, thiserror::Error)]
#[error("onboarding persistence failed")]
pub struct OnboardingRepositoryError(#[from] pub anyhow::Error);
impl From<super::ConnectionError> for OnboardingRepositoryError {
    fn from(e: super::ConnectionError) -> Self {
        Self(e.into())
    }
}
#[async_trait::async_trait]
pub trait OnboardingRepository: Send + Sync {
    async fn settings(
        &self,
        organization_id: u64,
    ) -> Result<Option<Settings>, OnboardingRepositoryError>;
    async fn save_settings(
        &self,
        organization_id: u64,
        value: Settings,
    ) -> Result<Settings, OnboardingRepositoryError>;
    async fn initialize(
        &self,
        organization_id: u64,
        user_id: u64,
        progress: Progress,
    ) -> Result<(), OnboardingRepositoryError>;
    async fn find(
        &self,
        organization_id: u64,
        user_id: u64,
        lock: bool,
    ) -> Result<Option<Context>, OnboardingRepositoryError>;
    async fn save_progress(
        &self,
        organization_id: u64,
        user_id: u64,
        progress: Progress,
    ) -> Result<(), OnboardingRepositoryError>;
    async fn complete(
        &self,
        organization_id: u64,
        user_id: u64,
    ) -> Result<(), OnboardingRepositoryError>;
}
