use crate::{
    graphql::{base_type::Payload, error::Error},
    http::authentication::UserContext,
};
use compallm_core::persistent::analytics::AnalyticsRepository;
pub async fn get(
    key: String,
    user: &UserContext,
    repository: &dyn AnalyticsRepository,
) -> Result<Option<Payload<super::AnalyticUserProfile>>, Error> {
    let actor = user.require().await?;
    if key.is_empty() || key.chars().count() > 64 {
        return Err(Error::invalid_input());
    }
    Ok(repository
        .find(actor.access().session.user_id, &key)
        .await
        .map_err(compallm_core::analytics::AnalyticsError::from)?
        .map(|v| super::AnalyticUserProfile::from(v).into()))
}
