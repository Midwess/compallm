use crate::{
    graphql::{base_type::Payload, error::Error},
    http::authentication::UserContext,
};
use compallm_core::{persistent::DbContext, user::ProfileService};
use validify::Validify;

pub async fn update(
    mut input: super::input::UserProfileUpdateInput,
    user: &UserContext,
    service: &ProfileService,
    database: &DbContext,
) -> Result<Payload<super::User>, Error> {
    let actor = user.require().await?;
    input.validify()?;
    let patch = input.try_into()?;
    let context = database
        .fresh()
        .and_then(|context| context.restrict_user(actor.access()))
        .map_err(|_| Error::internal())?;
    let user = context
        .scope(service.update_profile(actor.access(), patch))
        .await?;
    Ok(super::User::from(user).into())
}

pub async fn generate_url_avatar_url(
    mut input: super::input::ImageUploadInput,
    user: &UserContext,
    service: &ProfileService,
    database: &DbContext,
) -> Result<Payload<super::AvatarUpload>, Error> {
    let actor = user.require().await?;
    input.validify()?;
    let context = database
        .fresh()
        .and_then(|context| context.restrict_user(actor.access()))
        .map_err(|_| Error::internal())?;
    let upload = context
        .scope(service.generate_url_avatar_url(
            actor.access(),
            input.media_type.into(),
            input.size as u64,
        ))
        .await?;
    Ok(super::AvatarUpload::from(upload).into())
}
