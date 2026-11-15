use crate::{graphql::GraphqlContext, http::authorization::AuthorizationContext};

pub struct Me(pub compallm_core::authentication::Session);
#[juniper::graphql_object(context = GraphqlContext)]
impl Me {
    async fn organization_owner(
        &self,
        #[graphql(context)] service: &compallm_core::authorization::rbac::RbacService,
        #[graphql(context)] authority: &AuthorizationContext,
    ) -> Result<bool, crate::graphql::error::Error> {
        let permission = authority.member_permission().await?;
        Ok(service.is_owner(&permission).await?)
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
