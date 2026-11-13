use crate::preference::{
    UserPreferences,
    patch::{UserPreferenceGroup, UserPreferencesPatch},
};
use async_trait::async_trait;

#[derive(Debug, thiserror::Error)]
#[error("preference persistence failed")]
pub struct UserPreferenceRepositoryError(#[from] pub anyhow::Error);

impl From<crate::persistent::ConnectionError> for UserPreferenceRepositoryError {
    fn from(error: crate::persistent::ConnectionError) -> Self {
        Self(error.into())
    }
}

#[async_trait]
pub trait UserPreferenceRepository: Send + Sync {
    async fn find(
        &self,
        user_id: u64,
    ) -> Result<Option<UserPreferences>, UserPreferenceRepositoryError>;
    async fn create_defaults(&self, user_id: u64) -> Result<(), UserPreferenceRepositoryError>;
    async fn update(
        &self,
        user_id: u64,
        patch: &UserPreferencesPatch,
    ) -> Result<UserPreferences, UserPreferenceRepositoryError>;
    async fn reset(
        &self,
        user_id: u64,
        groups: &[UserPreferenceGroup],
    ) -> Result<UserPreferences, UserPreferenceRepositoryError>;
}
