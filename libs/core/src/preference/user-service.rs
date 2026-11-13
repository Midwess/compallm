use super::{
    UserPreferences,
    patch::{UserPreferenceGroup, UserPreferencesPatch},
};
use crate::{
    authentication::UserAccessToken,
    persistent::preference::user::{UserPreferenceRepository, UserPreferenceRepositoryError},
};

#[derive(Debug, thiserror::Error)]
pub enum UserPreferenceError {
    #[error(transparent)]
    Invalid(#[from] validify::ValidationErrors),
    #[error("invalid preference reset selection")]
    InvalidReset,
    #[error(transparent)]
    Persistence(#[from] UserPreferenceRepositoryError),
    #[error(transparent)]
    Authentication(#[from] crate::authentication::AuthError),
}

pub struct UserPreferenceService {
    repository: &'static Box<dyn UserPreferenceRepository>,
}

impl UserPreferenceService {
    pub fn new(repository: &'static Box<dyn UserPreferenceRepository>) -> Self {
        Self { repository }
    }

    pub async fn create_defaults(&self, user_id: u64) -> Result<(), UserPreferenceRepositoryError> {
        self.repository.create_defaults(user_id).await
    }

    pub async fn update(
        &self,
        actor: &UserAccessToken,
        patch: UserPreferencesPatch,
    ) -> Result<UserPreferences, UserPreferenceError> {
        actor.validate(compallm_utils::time::unix_timestamp())?;
        patch.apply(&mut UserPreferences::default())?;
        Ok(self
            .repository
            .update(actor.session.user_id, &patch)
            .await?)
    }

    pub async fn reset(
        &self,
        actor: &UserAccessToken,
        groups: &[UserPreferenceGroup],
    ) -> Result<UserPreferences, UserPreferenceError> {
        actor.validate(compallm_utils::time::unix_timestamp())?;
        let mut seen = std::collections::HashSet::new();
        if groups.is_empty()
            || groups.iter().any(|group| !seen.insert(*group))
            || (groups.len() > 1 && groups.contains(&UserPreferenceGroup::All))
        {
            return Err(UserPreferenceError::InvalidReset);
        }
        Ok(self.repository.reset(actor.session.user_id, groups).await?)
    }
}
