use super::{
    Member,
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

pub(crate) async fn list(
    filter: Option<MemberFilterInput>,
    order_by: Option<Vec<MemberOrderInput>>,
    pagination: compallm_query::Pagination,
    repository: &Box<dyn OrganizationRepository>,
    authority: &AuthorizationContext,
) -> Result<Collection<Member>, Error> {
    let permission = authority.member_permission().await?;
    Action::MemberRead
        .check_permission(&permission)
        .map_err(AuthorizationError::from)?;
    pagination.validate().map_err(|_| Error::invalid_input())?;
    let ordering = order_by
        .unwrap_or_default()
        .into_iter()
        .map(compallm_query::OrderBy::try_from)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| Error::invalid_input())?;
    let cursor_scope = format!(
        "members:{}:{}",
        permission.organization_id(),
        permission.user_id()
    );
    let page = repository
        .list_members(
            Box::new(filter.unwrap_or_default()),
            pagination,
            ordering,
            &cursor_scope,
        )
        .await
        .map_err(AuthorizationError::from)?;
    let page = compallm_query::Connection {
        page_info: page.page_info,
        edges: page
            .edges
            .into_iter()
            .map(|e| compallm_query::Edge {
                cursor: e.cursor,
                node: Member::from(e.node),
            })
            .collect(),
    };
    Ok(page.into())
}
