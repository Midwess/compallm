use crate::{
    graphql::{base_type::Payload, error::Error},
    http::authorization::AuthorizationContext,
};
use compallm_core::{organization::profile_service::ProfileService, persistent::DbContext};
use validify::Validify;

pub async fn update(
    mut input: super::input::OrganizationProfileUpdateInput,
    authority: &AuthorizationContext,
    service: &ProfileService,
    database: &DbContext,
) -> Result<Payload<super::Organization>, Error> {
    let permission = authority.member_permission().await?;
    input.validify()?;
    let patch = input.try_into()?;
    let context = database.fresh().map_err(|_| Error::internal())?;
    Ok(super::Organization::from(
        context
            .scope(service.update_profile(&permission, patch))
            .await?,
    )
    .into())
}
pub async fn generate(
    mut input: crate::graphql::user::input::ImageUploadInput,
    authority: &AuthorizationContext,
    service: &ProfileService,
    database: &DbContext,
) -> Result<Payload<crate::graphql::user::AvatarUpload>, Error> {
    let permission = authority.member_permission().await?;
    input.validify()?;
    let context = database.fresh().map_err(|_| Error::internal())?;
    Ok(crate::graphql::user::AvatarUpload::from(
        context
            .scope(service.generate_avatar_url(
                &permission,
                input.media_type.into(),
                input.size as u64,
            ))
            .await?,
    )
    .into())
}
