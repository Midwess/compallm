use super::Session;
use crate::{
    graphql::{base_type::Payload, error::Error},
    http::authentication::UserContext,
};
use compallm_core::persistent::{DbContext, organization::OrganizationRepository};

pub(crate) async fn session(
    user: &UserContext,
    repository: &dyn OrganizationRepository,
    database: &DbContext,
) -> Result<Payload<Session>, Error> {
    let session = user.require().await?.access().session.clone();
    let context = database.fresh().map_err(|_| Error::internal())?;
    let organization = context
        .scope(repository.find_by_id(session.organization_id))
        .await
        .map_err(compallm_core::organization::profile_service::ProfileError::from)?
        .filter(|org| org.status == compallm_core::organization::OrganizationStatus::Active)
        .ok_or(Error::internal())?;
    Ok(Session::from((session, organization.name)).into())
}
