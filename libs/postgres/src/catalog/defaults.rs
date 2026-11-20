use sea_orm::entity::prelude::*;
#[derive(sea_orm::DerivePartialModel)]
#[sea_orm(entity = "<crate::ProviderTemplateSchema as sea_orm::ModelTrait>::Entity")]
pub struct ProviderDefaultsRow {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub logo_path: String,
    pub protocol: String,
    pub base_url: Option<String>,
    pub headers: Json,
    pub authentication: Json,
    pub endpoint_config: Json,
    pub endpoint_mode: crate::CatalogEndpointModeSchema,
    pub readiness: crate::CatalogReadinessSchema,
}

impl From<crate::ProviderTemplateSchema> for ProviderDefaultsRow {
    fn from(value: crate::ProviderTemplateSchema) -> Self {
        Self {
            id: value.id,
            name: value.name,
            description: value.description,
            logo_path: value.logo_path,
            protocol: value.protocol,
            base_url: value.base_url,
            headers: value.headers,
            authentication: value.authentication,
            endpoint_config: value.endpoint_config,
            endpoint_mode: value.endpoint_mode,
            readiness: value.readiness,
        }
    }
}
#[derive(sea_orm::DerivePartialModel)]
#[sea_orm(entity = "<crate::ProviderModelTemplateSchema as sea_orm::ModelTrait>::Entity")]
pub struct ModelDefaultsRow {
    pub provider_template_id: String,
    pub id: String,
    pub upstream_model_id: String,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub operations: Json,
    pub capabilities: Json,
    pub max_context_tokens: Option<i64>,
    pub max_input_tokens: Option<i64>,
    pub max_output_tokens: Option<i64>,
    pub effort_mapping: Json,
    pub input_price: Option<Decimal>,
    pub output_price: Option<Decimal>,
    pub cached_price: Option<Decimal>,
    pub service_kind: crate::CatalogServiceKindSchema,
    pub billing_model: crate::CatalogBillingModelSchema,
    pub billing_unit: Option<crate::CatalogBillingUnitSchema>,
    pub pricing_mode: crate::CatalogPricingModeSchema,
    pub cache_billing: crate::CatalogCacheBillingSchema,
    pub specification_mode: crate::CatalogSpecificationModeSchema,
    pub unresolved_fields: Json,
    pub readiness: crate::CatalogReadinessSchema,
}

impl From<crate::ProviderModelTemplateSchema> for ModelDefaultsRow {
    fn from(value: crate::ProviderModelTemplateSchema) -> Self {
        Self {
            provider_template_id: value.provider_template_id,
            id: value.id,
            upstream_model_id: value.upstream_model_id,
            display_name: value.display_name,
            description: value.description,
            operations: value.operations,
            capabilities: value.capabilities,
            max_context_tokens: value.max_context_tokens,
            max_input_tokens: value.max_input_tokens,
            max_output_tokens: value.max_output_tokens,
            effort_mapping: value.effort_mapping,
            input_price: value.input_price,
            output_price: value.output_price,
            cached_price: value.cached_price,
            service_kind: value.service_kind,
            billing_model: value.billing_model,
            billing_unit: value.billing_unit,
            pricing_mode: value.pricing_mode,
            cache_billing: value.cache_billing,
            specification_mode: value.specification_mode,
            unresolved_fields: value.unresolved_fields,
            readiness: value.readiness,
        }
    }
}
