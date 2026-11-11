use super::Role;
use crate::{graphql::error::Error, http::authorization::AuthorizationContext};
use compallm_core::authorization::rbac::RbacService;
use juniper::ID;

pub struct Query;

#[juniper::graphql_object(name = "RoleQuery", context = crate::graphql::GraphqlContext)]
impl Query {
    pub(crate) async fn get(
        id: ID,
        #[graphql(context)] service: &RbacService,
        #[graphql(context)] authority: &AuthorizationContext,
    ) -> Result<Option<Role>, Error> {
        let id = id.parse::<u64>().map_err(|_| Error::invalid_input())?;
        let permission = authority.member_permission().await?;
        Ok(service.get_role(&permission, id).await?.map(Into::into))
    }

    pub(crate) async fn list(
        #[graphql(context)] service: &RbacService,
        #[graphql(context)] authority: &AuthorizationContext,
    ) -> Result<Vec<Role>, Error> {
        let permission = authority.member_permission().await?;
        Ok(service
            .list_roles(&permission)
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }
}
