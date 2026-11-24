use compallm_core::{model::ModelCatalog, provider::ProviderCatalog};
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
impl TryFrom<crate::CatalogProviderBookSchema> for ProviderCatalog {
    type Error = DbErr;
    fn try_from(v: crate::CatalogProviderBookSchema) -> Result<Self, Self::Error> {
        Ok(Self {
            id: v.id,
            research_id: v.source_id,
            name: v.name,
            description: v.description,
            logo_url: v.logo_url,
            connection: decode(v.connection)?,
            authentication_options: decode(v.authentication_options)?,
            headers: decode(v.headers)?,
            models: vec![],
        })
    }
}
impl TryFrom<crate::CatalogModelRecordSchema> for ModelCatalog {
    type Error = DbErr;
    fn try_from(v: crate::CatalogModelRecordSchema) -> Result<Self, Self::Error> {
        Ok(Self {
            id: v.id,
            custom_fields: decode(v.custom_fields)?,
            research_id: v.research_id,
            provider_catalog_id: v.provider_catalog_id,
            name: v.name,
            display_name: v.display_name,
            description: v.description,
            service_kind: v
                .service_kind
                .map(|v| decode(serde_json::Value::String(v)))
                .transpose()?,
            connections: decode(v.connections)?,
            headers: decode(v.headers)?,
            body_defaults: decode(v.body_defaults)?,
            text: decode(v.text)?,
            vision: decode(v.vision)?,
            pdf: decode(v.pdf)?,
            audio: decode(v.audio)?,
            video: decode(v.video)?,
            image_generation: decode(v.image_generation)?,
            tools: decode(v.tools)?,
            structured_output: decode(v.structured_output)?,
            search: decode(v.search)?,
            limits: decode(v.limits)?,
            billing: v.billing.map(decode).transpose()?,
            route_selection: v.route_selection.map(decode).transpose()?,
        })
    }
}
