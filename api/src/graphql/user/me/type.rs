use crate::{graphql::GraphqlContext, http::authorization::AuthorizationContext};

pub struct OrganizationPermission {
    pub action: String,
    pub allowed: bool,
}
#[juniper::graphql_object(context = GraphqlContext)]
impl OrganizationPermission {
    fn action(&self) -> &str {
        &self.action
    }
    fn allowed(&self) -> bool {
        self.allowed
    }
}

pub struct Me(pub compallm_core::authentication::Session);
#[juniper::graphql_object(context = GraphqlContext)]
impl Me {
    async fn organization_owner(
        &self,
        #[graphql(context)] repository: &Box<
            dyn compallm_core::persistent::organization::OrganizationRepository,
        >,
        #[graphql(context)] authority: &AuthorizationContext,
    ) -> Result<bool, crate::graphql::error::Error> {
        let permission = authority.member_permission().await?;
        Ok(repository
            .active_owner_ids(permission.organization_id())
            .await
            .map_err(compallm_core::authorization::error::AuthorizationError::from)?
            .contains(&permission.user_id()))
    }
    async fn organization_permissions(
        &self,
        #[graphql(context)] authority: &AuthorizationContext,
    ) -> Result<Vec<OrganizationPermission>, crate::graphql::error::Error> {
        use compallm_core::authorization::{
            MemberPermissionError,
            condition::CheckPermission,
            policy::{Action, EntityType},
        };
        let permission = authority.member_permission().await?;
        let mut allowed = Vec::new();
        for action in Action::ALL.iter().filter(|action| {
            action.resource() == EntityType::Organization && action.context().is_none()
        }) {
            let permitted = match action.check_permission(&permission) {
                Ok(()) => true,
                Err(MemberPermissionError::Denied) => false,
                Err(_) => return Err(crate::graphql::error::Error::internal()),
            };
            allowed.push(OrganizationPermission {
                action: action.name().to_owned(),
                allowed: permitted,
            });
        }
        Ok(allowed)
    }
    fn id(&self) -> juniper::ID {
        juniper::ID::new(self.0.user_id.to_string())
    }
    fn session_id(&self) -> juniper::ID {
        juniper::ID::new(self.0.id.to_string())
    }
    fn organization_id(&self) -> juniper::ID {
        juniper::ID::new(self.0.organization_id.to_string())
    }
    /// Effective policies for this verified user only. UI decisions do not grant server authority.
    async fn cedar_source(
        &self,
        #[graphql(context)] user: &crate::http::authentication::UserContext,
    ) -> Result<String, crate::graphql::error::Error> {
        Ok(user
            .require()
            .await?
            .access()
            .policy_snapshot
            .cedar_source
            .clone())
    }
    /// Cedar JSON entity facts for the verified principal and its role/group ancestry; contains no credentials.
    async fn cedar_entities(
        &self,
        #[graphql(context)] authority: &AuthorizationContext,
    ) -> Result<String, crate::graphql::error::Error> {
        authority
            .member_permission()
            .await?
            .entities()
            .to_json_value()
            .map(|value| value.to_string())
            .map_err(|_| crate::graphql::error::Error::internal())
    }
}
