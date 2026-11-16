use crate::{
    graphql::{base_type::Payload, error::Error},
    http::authentication::UserContext,
};
use compallm_core::preference::UserPreferenceService;
use validify::Validify;

pub async fn update(
    mut input: super::input::UserPreferencesPatchInput,
    user: &UserContext,
    service: &UserPreferenceService,
) -> Result<Payload<super::UserPreferences>, Error> {
    let actor = user.require().await?;
    input.validify()?;
    let patch = input.try_into()?;
    let value = service.update(actor.access(), patch).await?;
    Ok(super::UserPreferences::from(value).into())
}
pub async fn reset(
    groups: Vec<super::input::UserPreferenceGroup>,
    user: &UserContext,
    service: &UserPreferenceService,
) -> Result<Payload<super::UserPreferences>, Error> {
    let actor = user.require().await?;
    let groups: Vec<_> = groups.into_iter().map(Into::into).collect();
    let value = service.reset(actor.access(), &groups).await?;
    Ok(super::UserPreferences::from(value).into())
}
