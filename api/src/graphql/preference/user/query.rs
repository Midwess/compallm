use crate::{
    graphql::{base_type::Payload, error::Error},
    http::authentication::UserContext,
};
use compallm_core::persistent::preference::user::UserPreferenceRepository;

pub async fn get(
    user: &UserContext,
    repository: &dyn UserPreferenceRepository,
) -> Result<Payload<super::UserPreferences>, Error> {
    let actor = user.require().await?;
    let preferences = repository
        .find(actor.access().session.user_id)
        .await
        .map_err(compallm_core::preference::UserPreferenceError::from)?
        .unwrap_or_default();
    Ok(super::UserPreferences::from(preferences).into())
}
