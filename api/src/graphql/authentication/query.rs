use super::Session;
use crate::{
    graphql::{GraphqlContext, base_type::Payload, error::Error},
    http::authentication::UserContext,
};

pub struct Query;

#[juniper::graphql_object(name = "AuthenticationQuery", context = GraphqlContext, scalar = juniper::DefaultScalarValue)]
impl Query {
    async fn session(#[graphql(context)] user: &UserContext) -> Result<Payload<Session>, Error> {
        Ok(Session::from(user.require().await?.access().session.clone()).into())
    }
}
