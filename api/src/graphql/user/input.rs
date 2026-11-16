use compallm_validation_derive::ExportValidation;
use juniper::{GraphQLEnum, GraphQLInputObject};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validify::Validify;

#[derive(Serialize, Deserialize, GraphQLInputObject, Validify, ExportValidation)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct UserProfileUpdateInput {
    #[serde(
        default,
        skip_serializing_if = "juniper::Nullable::is_implicit_null",
        serialize_with = "serialize_nullable",
        deserialize_with = "deserialize_nullable"
    )]
    #[schemars(with = "Option<String>", extend("x-compallm-non-null" = true, "x-compallm-trim" = true, "minLength" = 0, "maxLength" = 256))]
    pub display_name: juniper::Nullable<String>,
    #[serde(
        default,
        skip_serializing_if = "juniper::Nullable::is_implicit_null",
        serialize_with = "serialize_nullable",
        deserialize_with = "deserialize_nullable"
    )]
    #[schemars(with = "Option<String>", extend("x-compallm-non-null" = true, "x-compallm-trim" = true, "x-compallm-ascii-lowercase" = true, "minLength" = 1, "maxLength" = 256, "pattern" = r"^(?!\.)(?!.*\.\.)(?!.*\.$)[A-Za-z0-9.!#$%&'*+/=?^_`{|}~\-]+$"))]
    pub username: juniper::Nullable<String>,
    #[serde(
        default,
        skip_serializing_if = "juniper::Nullable::is_implicit_null",
        serialize_with = "serialize_nullable",
        deserialize_with = "deserialize_nullable"
    )]
    #[schemars(with = "Option<bool>", extend("x-compallm-non-null" = true, "default" = false))]
    #[graphql(default = juniper::Nullable::Some(false))]
    pub avatar_uploaded: juniper::Nullable<bool>,
}

pub(crate) fn serialize_nullable<S: serde::Serializer, T: Serialize>(
    value: &juniper::Nullable<T>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    match value {
        juniper::Nullable::Some(value) => value.serialize(serializer),
        _ => serializer.serialize_none(),
    }
}
pub(crate) fn deserialize_nullable<'de, D: serde::Deserializer<'de>, T: Deserialize<'de>>(
    deserializer: D,
) -> Result<juniper::Nullable<T>, D::Error> {
    Option::<T>::deserialize(deserializer)
        .map(|value| value.map_or(juniper::Nullable::ExplicitNull, juniper::Nullable::Some))
}

impl TryFrom<UserProfileUpdateInput> for compallm_core::user::ProfilePatch {
    type Error = crate::graphql::error::Error;
    fn try_from(input: UserProfileUpdateInput) -> Result<Self, Self::Error> {
        let display_name = match input.display_name {
            juniper::Nullable::Some(value) => Some(value),
            juniper::Nullable::ImplicitNull => None,
            juniper::Nullable::ExplicitNull => return Err(Self::Error::invalid_input()),
        };
        let username = match input.username {
            juniper::Nullable::Some(value) => Some(value),
            juniper::Nullable::ImplicitNull => None,
            juniper::Nullable::ExplicitNull => return Err(Self::Error::invalid_input()),
        };
        let avatar_uploaded = match input.avatar_uploaded {
            juniper::Nullable::Some(value) => value,
            juniper::Nullable::ImplicitNull => false,
            juniper::Nullable::ExplicitNull => return Err(Self::Error::invalid_input()),
        };
        Ok(Self {
            display_name,
            username,
            avatar_uploaded,
        })
    }
}

#[derive(Serialize, Deserialize, GraphQLInputObject, Validify, ExportValidation)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProfileUpdateInput {
    #[modify(trim)]
    #[validate(length(min = 1))]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    #[schemars(extend("x-compallm-non-null" = true))]
    pub name: Option<String>,
    #[modify(trim)]
    #[validate(length(min = 1))]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    #[schemars(extend("x-compallm-non-null" = true))]
    pub username: Option<String>,
}

fn present<'de, D: serde::Deserializer<'de>, T: Deserialize<'de>>(
    deserializer: D,
) -> Result<Option<T>, D::Error> {
    T::deserialize(deserializer).map(Some)
}

#[derive(Clone, Copy, Serialize, Deserialize, GraphQLEnum, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HomeDestination {
    Chat,
    Models,
    Providers,
}

#[derive(Clone, Copy, Serialize, Deserialize, GraphQLEnum, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ImageMediaType {
    Jpeg,
    Png,
    Webp,
}

impl From<ImageMediaType> for compallm_core::user::AvatarMediaType {
    fn from(value: ImageMediaType) -> Self {
        match value {
            ImageMediaType::Jpeg => Self::Jpeg,
            ImageMediaType::Png => Self::Png,
            ImageMediaType::Webp => Self::Webp,
        }
    }
}

#[derive(Serialize, Deserialize, GraphQLInputObject, Validify, ExportValidation)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ImageUploadInput {
    pub media_type: ImageMediaType,
    #[validate(range(min = 1.0, max = 5242880.0))]
    pub size: i32,
}

#[derive(Serialize, Deserialize, GraphQLInputObject, Validify, ExportValidation)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CredentialCreateInput {
    #[modify(trim)]
    #[validate(length(min = 1))]
    pub name: String,
}
