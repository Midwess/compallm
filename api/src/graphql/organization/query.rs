use super::Organization;
use crate::{
    graphql::{base_type::Payload, error::Error},
    http::authentication::UserContext,
};
use compallm_core::{
    organization::OrganizationStatus,
    persistent::{DbContext, organization::OrganizationRepository},
};

pub(crate) async fn profile(
    id: juniper::ID,
    repository: &dyn OrganizationRepository,
    database: &DbContext,
) -> Result<Option<Payload<Organization>>, Error> {
    let id = id.parse().map_err(|_| Error::invalid_input())?;
    let context = database.fresh().map_err(|_| Error::internal())?;
    let organization = context
        .scope(repository.find_by_id(id))
        .await
        .map_err(compallm_core::organization::profile_service::ProfileError::from)?;
    Ok(organization
        .filter(|org| org.status == OrganizationStatus::Active)
        .map(|org| Organization::from(org).into()))
}

pub(crate) async fn get(
    user: &UserContext,
    repository: &dyn OrganizationRepository,
    database: &DbContext,
) -> Result<Payload<Organization>, Error> {
    let verified = user.require().await?;
    profile(
        juniper::ID::new(verified.access().session.organization_id.to_string()),
        repository,
        database,
    )
    .await?
    .ok_or(Error::internal())
}
