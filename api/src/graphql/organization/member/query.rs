use super::{
    GroupMemberOption, Member,
    input::{MemberFilterInput, MemberOrderInput},
};
use crate::{
    graphql::{base_type::Collection, error::Error},
    http::authorization::AuthorizationContext,
};
use compallm_core::{
    authorization::{condition::CheckPermission, error::AuthorizationError, policy::Action},
    persistent::organization::OrganizationRepository,
};

pub(crate) async fn options(
    group_id: Option<u64>,
    filter: Option<MemberFilterInput>,
    order_by: Option<Vec<MemberOrderInput>>,
    pagination: compallm_query::Pagination,
    repository: &Box<dyn OrganizationRepository>,
    authority: &AuthorizationContext,
) -> Result<compallm_query::Connection<compallm_core::organization::group::MemberOption>, Error> {
    let permission = authority.member_permission().await?;
    Action::MemberRead
        .check_permission(&permission)
        .map_err(AuthorizationError::from)?;
    if let Some(id) = group_id {
        Action::GroupRead
            .check_permission(&permission)
            .map_err(AuthorizationError::from)?;
        if repository
            .find_group_info(permission.organization_id(), id)
            .await
            .map_err(AuthorizationError::from)?
            .is_none()
        {
            return Err(AuthorizationError::from(
                compallm_core::persistent::organization::OrganizationRepositoryError::NotFound,
            )
            .into());
        }
    }
    pagination.validate().map_err(|_| Error::invalid_input())?;
    let ordering = order_by
        .unwrap_or_default()
        .into_iter()
        .map(compallm_query::OrderBy::try_from)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| Error::invalid_input())?;
    repository
        .list_members(
            permission.organization_id(),
            permission.user_id(),
            group_id,
            Box::new(filter.unwrap_or_default()),
            pagination,
            ordering,
        )
        .await
        .map_err(|e| AuthorizationError::from(e).into())
}

pub(crate) async fn list(
    filter: Option<MemberFilterInput>,
    order_by: Option<Vec<MemberOrderInput>>,
    pagination: compallm_query::Pagination,
    repository: &Box<dyn OrganizationRepository>,
    authority: &AuthorizationContext,
) -> Result<Collection<Member>, Error> {
    let page = options(None, filter, order_by, pagination, repository, authority).await?;
    let page = compallm_query::Connection {
        page_info: page.page_info,
        edges: page
            .edges
            .into_iter()
            .map(|e| compallm_query::Edge {
                cursor: e.cursor,
                node: Member::from(e.node.member),
            })
            .collect(),
    };
    Ok(page.into())
}
pub(crate) async fn group_options(
    group: u64,
    filter: Option<MemberFilterInput>,
    order_by: Option<Vec<MemberOrderInput>>,
    pagination: compallm_query::Pagination,
    repository: &Box<dyn OrganizationRepository>,
    authority: &AuthorizationContext,
) -> Result<Collection<GroupMemberOption>, Error> {
    let page = options(
        Some(group),
        filter,
        order_by,
        pagination,
        repository,
        authority,
    )
    .await?;
    let page = compallm_query::Connection {
        page_info: page.page_info,
        edges: page
            .edges
            .into_iter()
            .map(|e| compallm_query::Edge {
                cursor: e.cursor,
                node: GroupMemberOption::from(e.node),
            })
            .collect(),
    };
    Ok(page.into())
}
