use super::{CreatedGroup, input::GroupCreateInput};
use crate::graphql::base_type::Payload;
use compallm_core::authorization::{condition::CheckPermission, policy::Action};
use juniper::ID;
use validify::Validify;

pub(crate) async fn create(
    mut input: GroupCreateInput,
    service: &compallm_core::authorization::rbac::RbacService,
    authority: &crate::http::authorization::AuthorizationContext,
) -> Result<Payload<CreatedGroup>, crate::graphql::error::Error> {
    input.validify()?;
    let role_ids = input
        .role_ids
        .iter()
        .map(|id| id.parse::<u64>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| crate::graphql::error::Error::invalid_input())?;
    let permission = authority.member_permission().await?;
    Action::GroupRead
        .check_permission(&permission)
        .map_err(compallm_core::authorization::error::AuthorizationError::from)?;
    let group = service
        .create_group(&permission, &input.name, &input.description, role_ids)
        .await?;
    Ok(CreatedGroup {
        id: ID::new(group.id.to_string()),
        name: group.name,
        description: group.description,
        role_ids: group
            .roles
            .iter()
            .map(|role| ID::new(role.id.to_string()))
            .collect(),
    }
    .into())
}
