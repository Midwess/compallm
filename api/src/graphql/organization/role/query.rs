use super::Role;
use crate::{
    graphql::{
        base_type::{Collection, Payload},
        error::Error,
    },
    http::authorization::AuthorizationContext,
};
use compallm_core::authorization::rbac::RbacService;
use juniper::ID;

pub struct Query;

#[juniper::graphql_object(name = "RoleQuery", context = crate::graphql::GraphqlContext, scalar = juniper::DefaultScalarValue)]
impl Query {
    pub(crate) async fn get(
        id: ID,
        #[graphql(context)] service: &RbacService,
        #[graphql(context)] authority: &AuthorizationContext,
    ) -> Result<Option<Payload<Role>>, Error> {
        let id = id.parse::<u64>().map_err(|_| Error::invalid_input())?;
        let permission = authority.member_permission().await?;
        Ok(service
            .get_role(&permission, id)
            .await?
            .map(|node| Payload::from(Role::from(node))))
    }

    pub(crate) async fn list(
        #[graphql(context)] service: &RbacService,
        #[graphql(context)] authority: &AuthorizationContext,
    ) -> Result<Collection<Role>, Error> {
        let permission = authority.member_permission().await?;
        let nodes = service
            .list_roles(&permission)
            .await?
            .into_iter()
            .map(Into::into)
            .collect();
        let scope = format!(
            "roles:{}:{}:id:asc",
            permission.organization_id(),
            permission.user_id()
        );
        Collection::complete(nodes, &scope, |node: &Role| vec![Some(node.id.to_string())])
            .map_err(|_| Error::internal())
    }
}
