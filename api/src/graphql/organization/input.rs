use compallm_validation_derive::ExportValidation;
use juniper::GraphQLInputObject;
use serde::{Deserialize, Serialize};
use validify::Validify;

#[derive(Serialize, Deserialize, GraphQLInputObject, Validify, ExportValidation)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct OrganizationSettingsUpdateInput {
    #[modify(trim)]
    #[validate(length(min = 1))]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    #[schemars(extend("x-compallm-non-null" = true))]
    pub name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    #[schemars(extend("x-compallm-non-null" = true))]
    pub slug: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    #[schemars(extend("x-compallm-non-null" = true))]
    pub fiscal_month: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    #[schemars(extend("x-compallm-non-null" = true))]
    pub welcome: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    #[schemars(extend("x-compallm-non-null" = true))]
    pub announcements: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    #[schemars(extend("x-compallm-non-null" = true))]
    pub home: Option<super::super::user::input::HomeDestination>,
}

fn present<'de, D: serde::Deserializer<'de>, T: Deserialize<'de>>(
    deserializer: D,
) -> Result<Option<T>, D::Error> {
    T::deserialize(deserializer).map(Some)
}

#[derive(Serialize, Deserialize, GraphQLInputObject, Validify, ExportValidation)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SpendingLimitInput {
    #[validate(custom(function = spending_amount))]
    pub amount: Option<String>,
}

#[derive(Serialize, Deserialize, GraphQLInputObject, Validify, ExportValidation)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ReportingScheduleInput {
    pub timezone: String,
}

fn spending_amount(value: &str) -> Result<(), validify::ValidationError> {
    let valid = match value.split_once('.') {
        Some((whole, fractional)) => {
            !whole.is_empty()
                && !fractional.is_empty()
                && whole.bytes().all(|byte| byte.is_ascii_digit())
                && fractional.bytes().all(|byte| byte.is_ascii_digit())
        }
        None => !value.is_empty() && value.bytes().all(|byte| byte.is_ascii_digit()),
    };
    valid
        .then_some(())
        .ok_or_else(|| validify::ValidationError::new_field("spending_amount"))
}

fn spending_amount_contract() -> serde_json::Value {
    serde_json::json!({"pattern": "^[0-9]+(?:\\.[0-9]+)?$"})
}

#[derive(Serialize, Deserialize, GraphQLInputObject, Validify, ExportValidation)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct OrganizationProfileUpdateInput {
    #[serde(
        default,
        skip_serializing_if = "juniper::Nullable::is_implicit_null",
        serialize_with = "crate::graphql::user::input::serialize_nullable",
        deserialize_with = "crate::graphql::user::input::deserialize_nullable"
    )]
    #[schemars(with = "Option<String>", extend("x-compallm-non-null" = true, "x-compallm-trim" = true, "minLength" = 1, "maxLength" = 63, "pattern" = "^[A-Za-z0-9-]+$"))]
    pub name: juniper::Nullable<String>,
    #[serde(
        default,
        skip_serializing_if = "juniper::Nullable::is_implicit_null",
        serialize_with = "crate::graphql::user::input::serialize_nullable",
        deserialize_with = "crate::graphql::user::input::deserialize_nullable"
    )]
    #[schemars(with = "Option<bool>", extend("x-compallm-non-null" = true, "default" = false))]
    #[graphql(default = juniper::Nullable::Some(false))]
    pub avatar_uploaded: juniper::Nullable<bool>,
}
impl TryFrom<OrganizationProfileUpdateInput>
    for compallm_core::organization::profile_service::ProfilePatch
{
    type Error = crate::graphql::error::Error;
    fn try_from(value: OrganizationProfileUpdateInput) -> Result<Self, Self::Error> {
        let name = match value.name {
            juniper::Nullable::Some(name) => Some(name),
            juniper::Nullable::ImplicitNull => None,
            juniper::Nullable::ExplicitNull => return Err(Self::Error::invalid_input()),
        };
        let avatar_uploaded = match value.avatar_uploaded {
            juniper::Nullable::Some(value) => value,
            juniper::Nullable::ImplicitNull => false,
            juniper::Nullable::ExplicitNull => return Err(Self::Error::invalid_input()),
        };
        Ok(Self {
            name,
            avatar_uploaded,
        })
    }
}
