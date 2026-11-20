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
fn tokens(value: Option<i64>) -> Result<Option<u32>, DbErr> {
    value
        .map(|v| u32::try_from(v).map_err(|_| DbErr::Custom("invalid catalog token limit".into())))
        .transpose()
}

impl TryFrom<crate::catalog::defaults::ProviderDefaultsRow> for ProviderTemplate {
    type Error = DbErr;
    fn try_from(value: crate::catalog::defaults::ProviderDefaultsRow) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            name: value.name,
            description: value.description,
            logo_path: value.logo_path,
            protocol: decode(serde_json::Value::String(value.protocol))?,
            base_url: value.base_url,
            headers: decode(value.headers)?,
            authentication: decode(value.authentication)?,
            endpoint_config: decode(value.endpoint_config)?,
            endpoint_mode: enum_value(value.endpoint_mode)?,
            readiness: enum_value(value.readiness)?,
        })
    }
}

impl TryFrom<crate::catalog::defaults::ModelDefaultsRow> for ModelTemplate {
    type Error = DbErr;
    fn try_from(value: crate::catalog::defaults::ModelDefaultsRow) -> Result<Self, Self::Error> {
        Ok(Self {
            provider_template_id: value.provider_template_id,
            id: value.id,
            upstream_model_id: value.upstream_model_id,
            display_name: value.display_name,
            description: value.description,
            operations: decode(value.operations)?,
            capabilities: decode(value.capabilities)?,
            max_context_tokens: tokens(value.max_context_tokens)?,
            max_input_tokens: tokens(value.max_input_tokens)?,
            max_output_tokens: tokens(value.max_output_tokens)?,
            effort_mapping: decode(value.effort_mapping)?,
            input_price: value.input_price,
            output_price: value.output_price,
            cached_price: value.cached_price,
            service_kind: enum_value(value.service_kind)?,
            billing_model: enum_value(value.billing_model)?,
            billing_unit: value.billing_unit.map(enum_value).transpose()?,
            pricing_mode: enum_value(value.pricing_mode)?,
            cache_billing: enum_value(value.cache_billing)?,
            specification_mode: enum_value(value.specification_mode)?,
            unresolved_fields: decode(value.unresolved_fields)?,
            readiness: enum_value(value.readiness)?,
        })
    }
}

impl TryFrom<crate::catalog::summary::Model> for ProviderTemplateSummary {
    type Error = DbErr;
    fn try_from(v: crate::catalog::summary::Model) -> Result<Self, Self::Error> {
        Ok(Self {
            id: v.id,
            name: v.name,
            description: v.description,
            logo_path: v.logo_path,
            protocol: decode(serde_json::Value::String(v.protocol))?,
            base_url: v.base_url,
            readiness: enum_value(v.readiness)?,
        })
    }
}

impl TryFrom<ProviderTemplateSchema> for ProviderTemplate {
    type Error = DbErr;
    fn try_from(value: ProviderTemplateSchema) -> Result<Self, Self::Error> {
        Self::try_from(crate::catalog::defaults::ProviderDefaultsRow::from(value))
    }
}

impl TryFrom<ProviderModelTemplateSchema> for ModelTemplate {
    type Error = DbErr;
    fn try_from(value: ProviderModelTemplateSchema) -> Result<Self, Self::Error> {
        Self::try_from(crate::catalog::defaults::ModelDefaultsRow::from(value))
    }
}
