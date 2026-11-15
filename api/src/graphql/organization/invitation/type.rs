use crate::graphql::GraphqlContext;
pub struct Invitation(pub compallm_core::organization::invitation::Invitation);
impl From<compallm_core::organization::invitation::Invitation> for Invitation {
    fn from(value: compallm_core::organization::invitation::Invitation) -> Self {
        Self(value)
    }
}
#[juniper::graphql_object(context = GraphqlContext)]
impl Invitation {
    fn id(&self) -> juniper::ID {
        juniper::ID::new(self.0.id.to_string())
    }
    fn organization_id(&self) -> juniper::ID {
        juniper::ID::new(self.0.organization_id.to_string())
    }
    fn email(&self) -> &str {
        &self.0.email
    }
    fn inviter_id(&self) -> juniper::ID {
        juniper::ID::new(self.0.inviter_id.to_string())
    }
    fn status(&self) -> &'static str {
        use compallm_core::organization::invitation::InvitationStatus;
        match self.0.status {
            InvitationStatus::Pending
                if self.0.expires_at <= compallm_utils::time::unix_timestamp() =>
            {
                "expired"
            }
            InvitationStatus::Pending => "pending",
            InvitationStatus::Accepted => "accepted",
            InvitationStatus::Revoked => "revoked",
        }
    }
    fn issued_at(&self) -> f64 {
        self.0
            .expires_at
            .saturating_sub(compallm_core::authentication::token::InvitationToken::TTL_SECONDS)
            as f64
    }
    fn expires_at(&self) -> f64 {
        self.0.expires_at as f64
    }
}
#[derive(juniper::GraphQLObject)]
#[graphql(context = GraphqlContext)]
pub struct InvitationIssued {
    pub invitation: Invitation,
    pub url: String,
    pub email_admitted: bool,
}
#[derive(juniper::GraphQLObject)]
#[graphql(context = GraphqlContext)]
pub struct InvitationContext {
    pub organization_name: String,
    pub organization_id: juniper::ID,
    pub email: String,
    pub expires_at: f64,
    pub signup: bool,
}
impl From<compallm_core::organization::invitation_service::InvitationContext>
    for InvitationContext
{
    fn from(value: compallm_core::organization::invitation_service::InvitationContext) -> Self {
        Self {
            organization_name: value.organization_name,
            organization_id: juniper::ID::new(value.invitation.organization_id.to_string()),
            email: value.invitation.email,
            expires_at: value.invitation.expires_at as f64,
            signup: value.signup,
        }
    }
}
