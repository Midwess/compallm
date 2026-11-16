use crate::{
    graphql::{base_type::Payload, error::Error},
    http::{authentication::UserContext, authorization::AuthorizationContext},
};
use compallm_core::onboarding::OnboardingService;
use validify::Validify;
pub async fn settings(
    mut input: super::input::OrganizationOnboardingSettingsInput,
    authority: &AuthorizationContext,
    service: &OnboardingService,
) -> Result<Payload<super::OrganizationOnboardingSettings>, Error> {
    let permission = authority.member_permission().await?;
    input.validify()?;
    Ok(super::OrganizationOnboardingSettings::from(
        service.update_settings(&permission, input.into()).await?,
    )
    .into())
}
pub async fn advance(
    back: bool,
    user: &UserContext,
    service: &OnboardingService,
) -> Result<Payload<super::OnboardingContext>, Error> {
    let actor = user.require().await?;
    Ok(super::OnboardingContext::from((
        service.advance(actor.access(), back).await?,
        actor.access().session.id,
    ))
    .into())
}
pub async fn complete(
    user: &UserContext,
    service: &OnboardingService,
) -> Result<Payload<super::OnboardingContext>, Error> {
    let actor = user.require().await?;
    Ok(super::OnboardingContext::from((
        service.complete(actor.access()).await?,
        actor.access().session.id,
    ))
    .into())
}
pub async fn present(
    manual: bool,
    user: &UserContext,
    service: &OnboardingService,
) -> Result<Payload<super::OnboardingContext>, Error> {
    let actor = user.require().await?;
    Ok(super::OnboardingContext::from((
        service.present(actor.access(), manual).await?,
        actor.access().session.id,
    ))
    .into())
}
