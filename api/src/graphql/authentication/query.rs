use super::Session;
use crate::{
    graphql::{base_type::Payload, error::Error},
    http::authentication::UserContext,
};
use compallm_core::persistent::organization::OrganizationRepository;

pub(crate) async fn session(
    user: &UserContext,
    repository: &dyn OrganizationRepository,
    sessions: &dyn compallm_core::persistent::session::SessionRepository,
    identity: &crate::http::authentication::AuthenticationContext,
) -> Result<Payload<Session>, Error> {
    let actor = user.require().await?.access();
    let session = identity
        .execute(sessions.find(&actor.session))
        .await
        .map_err(compallm_core::authentication::AuthError::from)?
        .filter(|session| session.is_active(&actor.session))
        .ok_or(compallm_core::authentication::AuthError::InvalidSession)?;
    let organization = identity
        .execute(repository.find_by_id(session.organization_id))
        .await
        .map_err(compallm_core::organization::profile_service::ProfileError::from)?
        .filter(|org| org.status == compallm_core::organization::OrganizationStatus::Active)
        .ok_or(Error::internal())?;
    Ok(Session::from((session, organization.name)).into())
}

pub(crate) async fn sessions(
    user: &UserContext,
    repository: &Box<dyn compallm_core::persistent::session::SessionRepository>,
    identity: &crate::http::authentication::AuthenticationContext,
    pagination: compallm_query::Pagination,
) -> Result<crate::graphql::base_type::Collection<super::Session>, Error> {
    pagination.validate().map_err(|_| Error::invalid_input())?;
    let actor = user.require().await?.access();
    let page = identity
        .execute(repository.list_owned(actor.session.user_id, pagination))
        .await
        .map_err(compallm_core::authentication::AuthError::from)?;
    Ok(page.into())
}
