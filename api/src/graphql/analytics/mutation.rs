use crate::{
    graphql::{base_type::Payload, error::Error},
    http::authentication::UserContext,
};
use validify::Validify;
pub async fn put(
    mut input: super::input::AnalyticUserProfilePutInput,
    user: &UserContext,
    service: &compallm_core::analytics::AnalyticsService,
) -> Result<Payload<super::AnalyticUserProfile>, Error> {
    let actor = user.require().await?;
    input.validify()?;
    Ok(super::AnalyticUserProfile::from(service.put(actor.access(), input.into()).await?).into())
}
