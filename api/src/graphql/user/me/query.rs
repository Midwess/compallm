use crate::{
    graphql::{base_type::Payload, error::Error},
    http::authentication::UserContext,
};

pub async fn get(user: &UserContext) -> Result<Payload<super::Me>, Error> {
    Ok(super::Me(user.require().await?.access().session.clone()).into())
}
