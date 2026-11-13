use crate::{
    graphql::{base_type::Payload, error::Error},
    http::authentication::UserContext,
};
use compallm_core::persistent::{DbContext, preference::user::UserPreferenceRepository};

pub async fn get(
    user: &UserContext,
    repository: &dyn UserPreferenceRepository,
    database: &DbContext,
) -> Result<Payload<super::UserPreferences>, Error> {
    let actor = user.require().await?;
    let context = database.fresh().map_err(|_| Error::internal())?;
    let preferences = context
        .scope(repository.find(actor.access().session.user_id))
        .await
        .map_err(compallm_core::preference::UserPreferenceError::from)?
        .unwrap_or_default();
    Ok(super::UserPreferences::from(preferences).into())
}
