use crate::{
    graphql::{base_type::Payload, error::Error},
    http::authorization::AuthorizationContext,
};
use compallm_core::organization::profile_service::ProfileService;
use validify::Validify;

pub async fn update(
    mut input: super::input::OrganizationProfileUpdateInput,
    authority: &AuthorizationContext,
    service: &ProfileService,
) -> Result<Payload<super::Organization>, Error> {
    let permission = authority.member_permission().await?;
    input.validify()?;
    let patch = input.try_into()?;
    Ok(super::Organization::from(service.update_profile(&permission, patch).await?).into())
}
pub async fn generate(
    mut input: crate::graphql::user::input::ImageUploadInput,
    authority: &AuthorizationContext,
    service: &ProfileService,
) -> Result<Payload<crate::graphql::user::AvatarUpload>, Error> {
    let permission = authority.member_permission().await?;
    input.validify()?;
    Ok(crate::graphql::user::AvatarUpload::from(
        service
            .generate_avatar_url(&permission, input.media_type.into(), input.size as u64)
            .await?,
    )
    .into())
}
