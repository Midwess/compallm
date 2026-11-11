use super::Session;
use crate::{graphql::{GraphqlContext, error::Error}, http::authentication::UserContext};

pub struct Query;

#[juniper::graphql_object(name = "AuthenticationQuery", context = GraphqlContext)]
impl Query {
    async fn session(#[graphql(context)] user: &UserContext) -> Result<Session, Error> {
        Ok(user.require().await?.access().session.clone().into())
    }
}
