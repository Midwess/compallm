use super::Role;
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
    include_unseen: bool,
    service: &compallm_core::authorization::rbac::RbacService,
    repository: &Box<dyn OrganizationRepository>,
    authority: &AuthorizationContext,
) -> Result<Option<Payload<Role>>, Error> {
    let id = id.parse::<u64>().map_err(|_| Error::invalid_input())?;
    let permission = authority.member_permission().await?;
    Action::RoleRead
        .check_permission(&permission)
        .map_err(AuthorizationError::from)?;
    let owner = service.is_owner(&permission).await?;
    Ok(repository
        .find_roles_for_assignment(permission.organization_id(), &[id])
        .await
        .map_err(AuthorizationError::from)?
        .into_iter()
        .next()
        .filter(|node| !node.org_owner() || include_unseen || owner)
        .map(|node| Payload::from(Role::from(node))))
}

pub(crate) async fn list(
    include_unseen: bool,
    service: &compallm_core::authorization::rbac::RbacService,
    repository: &Box<dyn OrganizationRepository>,
    authority: &AuthorizationContext,
) -> Result<Collection<Role>, Error> {
    let permission = authority.member_permission().await?;
    Action::RoleRead
        .check_permission(&permission)
        .map_err(AuthorizationError::from)?;
    let owner = service.is_owner(&permission).await?;
    let nodes = repository
        .list_role_info()
        .await
        .map_err(AuthorizationError::from)?
        .into_iter()
        .filter(|node| !node.org_owner() || include_unseen || owner)
        .map(Into::into)
        .collect();
    let scope = format!(
        "roles:{}:{}:unseen:{include_unseen}:owner:{owner}:id:asc",
        permission.organization_id(),
        permission.user_id()
    );
    Collection::complete(nodes, &scope, |node: &Role| vec![Some(node.id.to_string())])
        .map_err(|_| Error::internal())
}
