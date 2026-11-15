use super::{Invitation, InvitationIssued, input::InvitationCreateInput};
use crate::{
    graphql::{base_type::Payload, error::Error},
    http::{authentication::CookieDelivery, authorization::AuthorizationContext},
};
use compallm_core::{
    authorization::{condition::CheckPermission, error::AuthorizationError, policy::Action},
    organization::invitation_service::InvitationService,
};
use validify::Validify;

pub async fn create(
    mut input: InvitationCreateInput,
    service: &InvitationService,
    authority: &AuthorizationContext,
    cookies: &CookieDelivery,
) -> Result<Payload<InvitationIssued>, Error> {
    input.validify()?;
    cookies.validate_origin()?;
    cookies.invitation_url("")?;
    let permission = authority.member_permission().await?;
    let mut issued = service
        .invitation_create(&permission, &input.email, None)
        .await?;
    let url = cookies.invitation_url(&issued.token)?;
    let _ = service.invitation_send(&mut issued, &url).await;
    Ok(InvitationIssued {
        invitation: issued.invitation.into(),
        url,
        email_admitted: issued.email_admitted,
    }
    .into())
}
pub async fn resend(
    id: juniper::ID,
    service: &InvitationService,
    authority: &AuthorizationContext,
    cookies: &CookieDelivery,
) -> Result<Payload<InvitationIssued>, Error> {
    cookies.validate_origin()?;
    cookies.invitation_url("")?;
    let permission = authority.member_permission().await?;
    Action::MemberInvite
        .check_permission(&permission)
        .map_err(AuthorizationError::from)?;
    let id = id.parse().map_err(|_| Error::invalid_input())?;
    let mut issued = service.invitation_resend(&permission, id).await?;
    let url = cookies.invitation_url(&issued.token)?;
    let _ = service.invitation_send(&mut issued, &url).await;
    Ok(InvitationIssued {
        invitation: issued.invitation.into(),
        url,
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
        .invitation_revoke(&permission, id.parse().map_err(|_| Error::invalid_input())?)
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
    cookies.invitation_url("")?;
    let permission = authority.member_permission().await?;
    let emails: Vec<_> = input
        .invitations
        .into_iter()
        .map(|input| input.email)
        .collect();
    let issued = service.invitations_create(&permission, &emails).await?;
    let mut nodes = Vec::with_capacity(issued.len());
    for mut issued in issued {
        let url = cookies.invitation_url(&issued.token)?;
        let _ = service.invitation_send(&mut issued, &url).await;
        nodes.push(InvitationIssued {
            invitation: issued.invitation.into(),
            url,
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
