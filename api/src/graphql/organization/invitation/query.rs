use super::{Invitation, InvitationContext};
use crate::{
    graphql::{
        base_type::{Collection, Payload},
        error::Error,
    },
    http::{authentication::AuthenticationContext, authorization::AuthorizationContext},
};
use compallm_core::{
    authorization::{condition::CheckPermission, error::AuthorizationError, policy::Action},
    organization::invitation_service::InvitationService,
    persistent::invitation::InvitationRepository,
};

pub async fn list(
    pagination: compallm_query::Pagination,
    repository: &dyn InvitationRepository,
    authority: &AuthorizationContext,
) -> Result<Collection<Invitation>, Error> {
    pagination.validate().map_err(|_| Error::invalid_input())?;
    let permission = authority.member_permission().await?;
    Action::MemberRead
        .check_permission(&permission)
        .map_err(AuthorizationError::from)?;
    let scope = format!(
        "invitations:{}:{}",
        permission.organization_id(),
        permission.user_id()
    );
    let page = repository
        .list(
            Box::new(compallm_query::Query::new()),
            pagination,
            vec![],
            &scope,
        )
        .await?;
    Ok(compallm_query::Connection {
        page_info: page.page_info,
        edges: page
            .edges
            .into_iter()
            .map(|edge| compallm_query::Edge {
                cursor: edge.cursor,
                node: Invitation::from(edge.node),
            })
            .collect(),
    }
    .into())
}
pub async fn context(
    service: &InvitationService,
    identity: &AuthenticationContext,
) -> Result<Payload<InvitationContext>, Error> {
    let claims = identity
        .invitation()
        .await?
        .ok_or(compallm_core::authentication::AuthError::InvalidInvitation)?;
    Ok(InvitationContext::from(
        identity
            .execute(service.invitation_context(&claims))
            .await?,
    )
    .into())
}
