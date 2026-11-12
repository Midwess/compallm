use super::Group;
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

#[juniper::graphql_object(name = "GroupQuery", context = crate::graphql::GraphqlContext, scalar = juniper::DefaultScalarValue)]
impl Query {
    pub(crate) async fn get(
        id: ID,
        #[graphql(context)] service: &RbacService,
        #[graphql(context)] authority: &AuthorizationContext,
    ) -> Result<Option<Payload<Group>>, Error> {
        let id = id.parse::<u64>().map_err(|_| Error::invalid_input())?;
        let permission = authority.member_permission().await?;
        Ok(service
            .get_group(&permission, id)
            .await?
            .map(|node| Payload::from(Group::from(node))))
    }

    pub(crate) async fn list(
        #[graphql(context)] service: &RbacService,
        #[graphql(context)] authority: &AuthorizationContext,
    ) -> Result<Collection<Group>, Error> {
        let permission = authority.member_permission().await?;
        let nodes = service
            .list_groups(&permission)
            .await?
            .into_iter()
            .map(Into::into)
            .collect();
        let scope = format!(
            "groups:{}:{}:id:asc",
            permission.organization_id(),
            permission.user_id()
        );
        Collection::complete(nodes, &scope, |node: &Group| {
            vec![Some(node.id.to_string())]
        })
        .map_err(|_| Error::internal())
    }
}
