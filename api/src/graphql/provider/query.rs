use super::Provider;
use crate::{
    graphql::{
        base_type::{Collection, Payload},
        error::Error,
    },
    http::authorization::AuthorizationContext,
};
use compallm_core::authorization::{condition::CheckPermission, policy::Action};
use compallm_postgres::repository::provider::ProviderRepository;
use juniper::ID;
pub async fn get(
    id: ID,
    authority: &AuthorizationContext,
    repository: &ProviderRepository,
) -> Result<Option<Payload<Provider>>, Error> {
    let p = authority.member_permission().await?;
    let value = repository
        .find_by_id(id.parse().map_err(|_| Error::invalid_input())?)
        .await
        .map_err(|_| Error::internal())?;
    let Some(value) = value else {
        return Ok(None);
    };
    Action::ProviderRead
        .check_permission(&p)
        .map_err(compallm_core::authorization::error::AuthorizationError::from)?;
    Ok(Some(Provider::from(value).into()))
}
pub async fn list(
    pagination: compallm_query::Pagination,
    authority: &AuthorizationContext,
    repository: &ProviderRepository,
) -> Result<Collection<Provider>, Error> {
    let p = authority.member_permission().await?;
    Action::ProviderRead
        .check_permission(&p)
        .map_err(compallm_core::authorization::error::AuthorizationError::from)?;
    Ok(repository
        .list(
            pagination,
            &format!("providers:{}:{}", p.organization_id(), p.user_id()),
        )
        .await
        .map_err(|_| Error::internal())?
        .into())
}
