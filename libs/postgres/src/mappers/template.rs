use crate::{ProviderModelTemplateSchema, ProviderTemplateSchema};
use compallm_core::provider_template::*;
use sea_orm::DbErr;
use serde::de::DeserializeOwned;

fn decode<T: DeserializeOwned>(value: serde_json::Value) -> Result<T, DbErr> {
    serde_json::from_value(value).map_err(|error| {
        DbErr::Custom(format!(
            "invalid stored {}: {error}",
            std::any::type_name::<T>()
        ))
    })
}
fn enum_value<T: DeserializeOwned, E: sea_orm::ActiveEnum<Value = sea_orm::sea_query::Enum>>(
    value: E,
) -> Result<T, DbErr> {
    decode(serde_json::Value::String(
        value.to_value().value.into_owned(),
    ))
}
impl TryFrom<ProviderTemplateSchema> for ProviderTemplate {
    type Error = DbErr;
    fn try_from(value: ProviderTemplateSchema) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            name: value.name,
            description: value.description,
            logo_path: value.logo_path,
            connection: decode(value.connection)?,
            headers: decode(value.headers)?,
            readiness: enum_value(value.readiness)?,
        })
    }
}
impl TryFrom<ProviderModelTemplateSchema> for ModelTemplate {
    type Error = DbErr;
    fn try_from(value: ProviderModelTemplateSchema) -> Result<Self, Self::Error> {
        Ok(Self {
            provider_template_id: value.provider_template_id,
            id: value.id,
            upstream_model_id: value.upstream_model_id,
            display_name: value.display_name,
            description: value.description,
            service_kind: enum_value(value.service_kind)?,
            connection: decode(value.connection)?,
            headers: decode(value.headers)?,
            capabilities: decode(value.capabilities)?,
            limits: decode(value.limits)?,
            thinking: decode(value.thinking)?,
            pricing: decode(value.pricing)?,
            readiness: enum_value(value.readiness)?,
        })
    }
}
