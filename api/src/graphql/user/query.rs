use crate::graphql::{base_type::Payload, error::Error};
use compallm_core::{
    persistent::{DbContext, user::UserRepository},
    user::UserStatus,
};

pub async fn profile(
    id: juniper::ID,
    user: &crate::http::authentication::UserContext,
    repository: &dyn UserRepository,
    database: &DbContext,
) -> Result<Option<Payload<super::User>>, Error> {
    let actor = user.require().await?;
    let id: u64 = id.parse().map_err(|_| Error::invalid_input())?;
    let context = database
        .fresh()
        .and_then(|context| context.restrict_user(actor.access()))
        .map_err(|_| Error::internal())?;
    let user = context
        .scope(repository.find_by_id(id))
        .await
        .map_err(compallm_core::user::ProfileError::from)?;
    Ok(user
        .filter(|user| user.status == UserStatus::Active)
        .map(|user| super::User::from(user).into()))
}
