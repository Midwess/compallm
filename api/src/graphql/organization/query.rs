use super::{Organization, group, role};
use crate::{graphql::{GraphqlContext, error::Error}, http::authentication::UserContext};

pub struct Query;

#[juniper::graphql_object(name = "OrganizationQuery", context = GraphqlContext)]
impl Query {
    async fn get(#[graphql(context)] user: &UserContext) -> Result<Organization, Error> {
        let verified = user.require().await?;
        let session = &verified.access().session;
        Ok(Organization {
            id: session.organization_id,
            name: session.organization_name.clone(),
        })
    }

    fn roles() -> role::query::Query { role::query::Query }
    fn groups() -> group::query::Query { group::query::Query }
}
