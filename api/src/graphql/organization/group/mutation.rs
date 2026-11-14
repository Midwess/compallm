use super::{Group, input::GroupCreateInput};
use crate::graphql::base_type::Payload;
use compallm_core::authorization::{condition::CheckPermission, policy::Action};
use juniper::ID;
use validify::Validify;

pub(crate) async fn create(
    mut input: GroupCreateInput,
    service: &compallm_core::authorization::rbac::RbacService,
    authority: &crate::http::authorization::AuthorizationContext,
) -> Result<Payload<Group>, crate::graphql::error::Error> {
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
    Ok(super::Group::from(group).into())
}

pub(crate) async fn update(
    id: ID,
    mut input: super::input::GroupUpdateInput,
    service: &compallm_core::authorization::rbac::RbacService,
    authority: &crate::http::authorization::AuthorizationContext,
) -> Result<Payload<super::Group>, crate::graphql::error::Error> {
    input.validify()?;
    let parse = |ids: Option<Vec<String>>| -> Result<Vec<u64>, crate::graphql::error::Error> {
        ids.unwrap_or_default()
            .iter()
            .map(|id| {
                id.parse()
                    .map_err(|_| crate::graphql::error::Error::invalid_input())
            })
            .collect()
    };
    let permission = authority.member_permission().await?;
    Action::GroupRead
        .check_permission(&permission)
        .map_err(compallm_core::authorization::error::AuthorizationError::from)?;
    let changes = compallm_core::organization::group::GroupChanges {
        name: input.name,
        description: input.description,
        add_member_ids: parse(input.add_member_ids)?,
        remove_member_ids: parse(input.remove_member_ids)?,
        add_role_ids: parse(input.add_role_ids)?,
        remove_role_ids: parse(input.remove_role_ids)?,
    };
    Ok(super::Group::from(
        service
            .update_group(
                &permission,
                id.parse()
                    .map_err(|_| crate::graphql::error::Error::invalid_input())?,
                changes,
            )
            .await?,
    )
    .into())
}

pub(crate) async fn delete(
    id: ID,
    service: &compallm_core::authorization::rbac::RbacService,
    authority: &crate::http::authorization::AuthorizationContext,
) -> Result<Payload<bool>, crate::graphql::error::Error> {
    let permission = authority.member_permission().await?;
    service
        .delete_group(
            &permission,
            id.parse()
                .map_err(|_| crate::graphql::error::Error::invalid_input())?,
        )
        .await?;
    Ok(true.into())
}
