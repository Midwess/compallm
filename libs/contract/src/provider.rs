//! Input components only. Exporting these types does not register API endpoints.
use crate::ContractSchema;
use compallm_core::provider::input;
use serde::Deserialize;
use validify::Validify;

#[derive(Deserialize, Validify, ContractSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProviderFields {
    #[validate(length(min = 1, max = 120), custom(function = input::nonblank))]
    pub name: String,
    #[validate(custom(function = input::base_url))]
    pub base_url: String,
    #[validate(length(min = 1, max = 512), custom(function = input::api_key))]
    pub api_key: String,
}

#[derive(Deserialize, Validify, ContractSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProviderModelFields {
    #[validate(length(min = 1, max = 40), custom(function = input::model_name))]
    pub name: String,
    #[validate(custom(function = input::endpoint_path), length(max = 40))]
    pub path: String,
    #[validate(range(min = 1.0, max = 4294967295.0))]
    pub max_context_tokens: u32,
    #[validate(custom(function = input::price))]
    pub input_price: String,
    #[validate(custom(function = input::price))]
    pub output_price: String,
    #[validate(custom(function = input::price))]
    pub cached_price: String,
}

#[derive(Deserialize, Validify, ContractSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[validate(validate_header)]
pub struct ProviderHeader {
    #[validate(custom(function = input::header_name), length(max = 128))]
    pub key: String,
    #[validate(custom(function = input::header_value))]
    pub value: String,
}

fn validate_header(header: &ProviderHeader) -> Result<(), validify::ValidationErrors> {
    if header.key.eq_ignore_ascii_case("authorization") {
        input::authorization(&header.value).map_err(|error| {
            let mut errors = validify::ValidationErrors::new();
            errors.add(error);
            errors
        })?;
    }
    Ok(())
}
fn validate_header_contract() -> serde_json::Value {
    serde_json::json!({"x-compallm-object-rule": "header"})
}

#[derive(Deserialize, Validify, ContractSchema)]
#[serde(deny_unknown_fields)]
#[validate(validate_headers)]
pub struct ProviderHeaders {
    pub entries: Vec<ProviderHeader>,
}

fn validate_headers(headers: &ProviderHeaders) -> Result<(), validify::ValidationErrors> {
    use validify::Validate;
    let mut keys = std::collections::HashSet::new();
    for header in &headers.entries {
        header.validate()?;
        if !keys.insert(header.key.to_ascii_lowercase()) {
            let mut errors = validify::ValidationErrors::new();
            errors.add(validify::ValidationError::new_schema("duplicate_header"));
            return Err(errors);
        }
    }
    Ok(())
}
fn validate_headers_contract() -> serde_json::Value {
    serde_json::json!({"x-compallm-object-rule": "headers"})
}

#[derive(Deserialize, Validify, ContractSchema)]
#[serde(deny_unknown_fields)]
#[validate(validate_models)]
pub struct ProviderModels {
    pub entries: Vec<ProviderModelFields>,
}

fn validate_models(models: &ProviderModels) -> Result<(), validify::ValidationErrors> {
    use validify::Validate;
    let mut names = std::collections::HashSet::new();
    for model in &models.entries {
        model.validate()?;
        if !names.insert(&model.name) {
            let mut errors = validify::ValidationErrors::new();
            errors.add(validify::ValidationError::new_schema("duplicate_model"));
            return Err(errors);
        }
    }
    if models.entries.is_empty() {
        let mut errors = validify::ValidationErrors::new();
        errors.add(validify::ValidationError::new_schema("missing_model"));
        return Err(errors);
    }
    Ok(())
}
fn validate_models_contract() -> serde_json::Value {
    serde_json::json!({"x-compallm-object-rule": "models"})
}

/// Independently exported rules for optional/draft values and collection adapters.
#[derive(Deserialize, Validify, ContractSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProviderInputRules {
    #[validate(length(min = 1, max = 25), custom(function = input::effort_value))]
    pub effort_value: String,
    #[validate(custom(function = input::authorization))]
    pub authorization: String,
    #[validate(custom(function = input::positive_integer))]
    pub positive_integer: String,
}

#[derive(utoipa::OpenApi)]
#[openapi(components(schemas(
    ProviderFields,
    ProviderModelFields,
    ProviderModels,
    ProviderHeader,
    ProviderHeaders,
    ProviderInputRules
)))]
pub struct InputContracts;
