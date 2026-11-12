use super::{Role, input::RoleCreateInput};
use crate::{
    graphql::{base_type::Payload, error::Error},
    http::authorization::AuthorizationContext,
};
use compallm_core::authorization::{condition::CheckPermission, policy::Action, rbac::RbacService};
use validify::Validify;

pub(crate) async fn update(
    id: juniper::ID,
    mut input: super::input::RoleUpdateInput,
    service: &RbacService,
    authority: &AuthorizationContext,
) -> Result<Payload<Role>, Error> {
    input.validify()?;
    let id = id.parse::<u64>().map_err(|_| Error::invalid_input())?;
    let permission = authority.member_permission().await?;
    Action::RoleRead
        .check_permission(&permission)
        .map_err(compallm_core::authorization::error::AuthorizationError::from)?;
    let role = service
        .update_role(
            &permission,
            id,
            input.name,
            input.description,
            input.cedar_source,
        )
        .await?;
    Ok(Role::from(role).into())
}

pub(crate) async fn create(
    mut input: RoleCreateInput,
    service: &RbacService,
    authority: &AuthorizationContext,
) -> Result<Payload<Role>, Error> {
    input.validify()?;
    let permission = authority.member_permission().await?;
    Action::RoleRead
        .check_permission(&permission)
        .map_err(compallm_core::authorization::error::AuthorizationError::from)?;
    let role = service
        .new_role(
            &permission,
            &input.name,
            &input.description,
            input.cedar_source,
        )
        .await?;
    Ok(Role::from(role).into())
}
