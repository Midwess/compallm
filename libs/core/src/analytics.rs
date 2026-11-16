use crate::{authentication::UserAccessToken, persistent::analytics::AnalyticsRepository};

#[derive(Clone, Debug, validify::Validify)]
pub struct UserProfileValue {
    #[validate(length(min = 1, max = 64))]
    pub key: String,
    #[validate(length(max = 256))]
    pub value: String,
}

#[derive(Debug, thiserror::Error)]
pub enum AnalyticsError {
    #[error(transparent)]
    Invalid(#[from] validify::ValidationErrors),
    #[error(transparent)]
    Authentication(#[from] crate::authentication::AuthError),
    #[error(transparent)]
    Persistence(#[from] crate::persistent::analytics::AnalyticsRepositoryError),
}

pub struct AnalyticsService {
    repository: &'static dyn AnalyticsRepository,
}
impl AnalyticsService {
    pub fn new(repository: &'static dyn AnalyticsRepository) -> Self {
        Self { repository }
    }
    pub async fn put(
        &self,
        actor: &UserAccessToken,
        mut value: UserProfileValue,
    ) -> Result<UserProfileValue, AnalyticsError> {
        use validify::Validify;
        actor.validate(compallm_utils::time::unix_timestamp())?;
        value.validify()?;
        Ok(self.repository.put(actor.session.user_id, value).await?)
    }
}

impl From<crate::persistent::ConnectionError> for AnalyticsError {
    fn from(error: crate::persistent::ConnectionError) -> Self {
        Self::Persistence(error.into())
    }
}
