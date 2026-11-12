use super::Session;
use crate::{
    graphql::{base_type::Payload, error::Error},
    http::authentication::UserContext,
};

pub(crate) async fn session(user: &UserContext) -> Result<Payload<Session>, Error> {
    Ok(Session::from(user.require().await?.access().session.clone()).into())
}
