use super::Group;
use crate::{
    graphql::{
        base_type::{Collection, Payload},
        error::Error,
    },
    http::authorization::AuthorizationContext,
};
use compallm_core::{
    authorization::{condition::CheckPermission, error::AuthorizationError, policy::Action},
    persistent::organization::OrganizationRepository,
};
use juniper::ID;

pub(crate) async fn get(
    id: ID,
    repository: &Box<dyn OrganizationRepository>,
    authority: &AuthorizationContext,
) -> Result<Option<Payload<Group>>, Error> {
    let id = id.parse::<u64>().map_err(|_| Error::invalid_input())?;
    let permission = authority.member_permission().await?;
    Action::GroupRead
        .check_permission(&permission)
        .map_err(AuthorizationError::from)?;
    Ok(repository
        .find_group_info(permission.organization_id(), id)
        .await
        .map_err(AuthorizationError::from)?
        .map(|node| Payload::from(Group::from(node))))
}

pub(crate) async fn list(
    repository: &Box<dyn OrganizationRepository>,
    authority: &AuthorizationContext,
) -> Result<Collection<Group>, Error> {
    let permission = authority.member_permission().await?;
    Action::GroupRead
        .check_permission(&permission)
        .map_err(AuthorizationError::from)?;
    let nodes = repository
        .list_group_info(permission.organization_id())
        .await
        .map_err(AuthorizationError::from)?
        .into_iter()
        .map(Into::into)
        .collect();
    let scope = format!(
        "groups:{}:{}:id:asc",
        permission.organization_id(),
        permission.user_id()
    );
    Collection::complete(nodes, &scope, |node: &Group| {
        vec![Some(node.id.to_string())]
    })
    .map_err(|_| Error::internal())
}
