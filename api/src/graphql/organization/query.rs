use super::Organization;
use crate::{
    graphql::{base_type::Payload, error::Error},
    http::authentication::UserContext,
};

pub(crate) async fn get(user: &UserContext) -> Result<Payload<Organization>, Error> {
    let verified = user.require().await?;
    let session = &verified.access().session;
    Ok(Organization {
        id: session.organization_id,
        name: session.organization_name.clone(),
    }
    .into())
}
