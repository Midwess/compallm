use super::Group;
use crate::{graphql::error::Error, http::authorization::AuthorizationContext};
use compallm_core::authorization::rbac::RbacService;
use juniper::ID;

pub struct Query;

#[juniper::graphql_object(name = "GroupQuery", context = crate::graphql::GraphqlContext)]
impl Query {
    pub(crate) async fn get(
        id: ID,
        #[graphql(context)] service: &RbacService,
        #[graphql(context)] authority: &AuthorizationContext,
    ) -> Result<Option<Group>, Error> {
        let id = id.parse::<u64>().map_err(|_| Error::invalid_input())?;
        let permission = authority.member_permission().await?;
        Ok(service.get_group(&permission, id).await?.map(Into::into))
    }

    pub(crate) async fn list(
        #[graphql(context)] service: &RbacService,
        #[graphql(context)] authority: &AuthorizationContext,
    ) -> Result<Vec<Group>, Error> {
        let permission = authority.member_permission().await?;
        Ok(service
            .list_groups(&permission)
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }
}
