use compallm_validation_derive::ExportValidation;
use juniper::{GraphQLEnum, GraphQLInputObject};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validify::Validify;

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
