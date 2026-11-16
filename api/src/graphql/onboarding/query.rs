use crate::{
    graphql::{base_type::Payload, error::Error},
    http::{authentication::UserContext, authorization::AuthorizationContext},
};
use compallm_core::{
    authorization::{condition::check::CheckPermission, policy::Action},
    persistent::onboarding::OnboardingRepository,
};
pub async fn context(
    user: &UserContext,
    repository: &dyn OnboardingRepository,
) -> Result<Option<Payload<super::OnboardingContext>>, Error> {
    let actor = user.require().await?;
    let session = &actor.access().session;
    Ok(repository
        .find(session.organization_id, session.user_id, false)
        .await
        .map_err(compallm_core::onboarding::OnboardingError::from)?
        .map(|v| super::OnboardingContext::from((v, session.id)).into()))
}
pub async fn settings(
    authority: &AuthorizationContext,
    repository: &dyn OnboardingRepository,
) -> Result<Payload<super::OrganizationOnboardingSettings>, Error> {
    let permission = authority.member_permission().await?;
    Action::OrganizationOnboardingRead
        .check_permission(&permission)
        .map_err(compallm_core::authorization::error::AuthorizationError::from)?;
    Ok(super::OrganizationOnboardingSettings::from(
        repository
            .settings(permission.organization_id())
            .await
            .map_err(compallm_core::onboarding::OnboardingError::from)?
            .unwrap_or_default(),
    )
    .into())
}
