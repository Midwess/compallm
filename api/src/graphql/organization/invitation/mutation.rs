use super::{Invitation, InvitationIssued, input::InvitationCreateInput};
use crate::{
    graphql::{base_type::Payload, error::Error},
    http::{authentication::CookieDelivery, authorization::AuthorizationContext},
};
use compallm_core::organization::invitation_service::InvitationService;
use validify::Validify;

pub async fn create(
    mut input: InvitationCreateInput,
    service: &InvitationService,
    authority: &AuthorizationContext,
    cookies: &CookieDelivery,
) -> Result<Payload<InvitationIssued>, Error> {
    input.validify()?;
    cookies.validate_origin()?;
    let link_prefix = cookies.invitation_url("")?;
    let permission = authority.member_permission().await?;
    let issued = service
        .send_invitation(&permission, &input.email, &link_prefix)
        .await?;
    Ok(InvitationIssued {
        invitation: issued.invitation.into(),
        url: issued.url,
        email_admitted: issued.email_admitted,
    }
    .into())
}
pub async fn revoke(
    id: juniper::ID,
    service: &InvitationService,
    authority: &AuthorizationContext,
) -> Result<Payload<Invitation>, Error> {
    let permission = authority.member_permission().await?;
    let invitation = service
        .revoke_invitation(&permission, id.parse().map_err(|_| Error::invalid_input())?)
        .await?;
    Ok(Invitation::from(invitation).into())
}

pub async fn create_many(
    mut input: super::input::InvitationBatchCreateInput,
    service: &InvitationService,
    authority: &AuthorizationContext,
    cookies: &CookieDelivery,
) -> Result<crate::graphql::base_type::Collection<InvitationIssued>, Error> {
    input.validify()?;
    cookies.validate_origin()?;
    let link_prefix = cookies.invitation_url("")?;
    let permission = authority.member_permission().await?;
    let emails: Vec<_> = input
        .invitations
        .into_iter()
        .map(|input| input.email)
        .collect();
    let issued = service
        .send_invitations(&permission, &emails, &link_prefix)
        .await?;
    let mut nodes = Vec::with_capacity(issued.len());
    for issued in issued {
        nodes.push(InvitationIssued {
            invitation: issued.invitation.into(),
            url: issued.url,
            email_admitted: issued.email_admitted,
        });
    }
    crate::graphql::base_type::Collection::complete(
        nodes,
        &format!("issued-invitations:{}", permission.organization_id()),
        |node| vec![Some(node.invitation.0.id.to_string())],
    )
    .map_err(|_| Error::internal())
}
