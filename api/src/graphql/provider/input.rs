//! Provider GraphQL input contracts. Operations are not exposed yet.
//! Optional-field null rejection is exported for browsers/Serde; omission-aware
//! Juniper coercion must be added before exposing write operations.
use super::scalar::ContextTokens;
use compallm_utils::input;
use compallm_validation_derive::ExportValidation;
use juniper::{GraphQLEnum, GraphQLInputObject, ID};
use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{Value, json};
use validify::{ValidationError, ValidationErrors, Validify};

/// Catalog connection family hint; each configured model selects its own operation.
#[derive(Clone, Copy, Debug, GraphQLEnum, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProviderApiProtocol {
    OpenaiCompatible,
    Anthropic,
    ProviderSpecific,
}

/// Text operation determining the upstream request and response format.
#[derive(Clone, Copy, Debug, GraphQLEnum, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TextModelProtocol {
    OpenaiChatCompletions,
    OpenaiResponses,
    AnthropicMessages,
}

/// Capabilities declared by a catalog or configured model; not an authorization grant.
#[derive(Clone, Copy, Debug, GraphQLEnum, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProviderModelCapability {
    Text,
    Vision,
    Pdf,
    Audio,
    Video,
    ImageGeneration,
    AudioGeneration,
    Search,
    Reasoning,
    ToolCall,
    ForcedToolChoice,
    StructuredOutput,
}

// Missing optional fields preserve existing state. Explicit JSON null is invalid.
fn present<'de, D: Deserializer<'de>, T: Deserialize<'de>>(
    deserializer: D,
) -> Result<Option<T>, D::Error> {
    T::deserialize(deserializer).map(Some)
}
fn invalid(code: &'static str) -> ValidationErrors {
    let mut errors = ValidationErrors::new();
    let message = match code {
        "new_model_id" => "New models must not include a saved model identity.",
        "missing_provider" | "duplicate_provider" => {
            "Add at least one connection and use unique connection names."
        }
        "authorization" => "Enter a valid Authorization header value.",
        "empty_patch" => "Change at least one connection field.",
        _ => "Check the connection fields and try again.",
    };
    errors.add(ValidationError::new_schema(code).with_message(message.into()));
    errors
}

/// Opaque registry identity, independent of an edited upstream model name.
#[derive(Clone, Debug, Serialize, Deserialize, GraphQLInputObject, Validify, ExportValidation)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProviderCatalogRef {
    pub provider: String,
    pub model: String,
}

/// Empty object means no configured effort levels. Configured values cannot be empty.
#[derive(Default, Serialize, Deserialize, GraphQLInputObject, Validify, ExportValidation)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProviderEffortMapping {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    #[schemars(extend("x-compallm-non-null" = true))]
    #[validate(length(min = 1, max = 25, message = "Enter an effort value of 1 to 25 characters."), custom(function = input::effort_value, message = "Use letters, numbers, hyphens, or underscores for effort values."))]
    pub low: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    #[schemars(extend("x-compallm-non-null" = true))]
    #[validate(length(min = 1, max = 25, message = "Enter an effort value of 1 to 25 characters."), custom(function = input::effort_value, message = "Use letters, numbers, hyphens, or underscores for effort values."))]
    pub medium: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    #[schemars(extend("x-compallm-non-null" = true))]
    #[validate(length(min = 1, max = 25, message = "Enter an effort value of 1 to 25 characters."), custom(function = input::effort_value, message = "Use letters, numbers, hyphens, or underscores for effort values."))]
    pub high: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    #[schemars(extend("x-compallm-non-null" = true))]
    #[validate(length(min = 1, max = 25, message = "Enter an effort value of 1 to 25 characters."), custom(function = input::effort_value, message = "Use letters, numbers, hyphens, or underscores for effort values."))]
    pub xhigh: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    #[schemars(extend("x-compallm-non-null" = true))]
    #[validate(length(min = 1, max = 25, message = "Enter an effort value of 1 to 25 characters."), custom(function = input::effort_value, message = "Use letters, numbers, hyphens, or underscores for effort values."))]
    pub max: Option<String>,
}

/// Complete configured text-model values. Decimal prices are USD per million tokens.
#[derive(Serialize, Deserialize, GraphQLInputObject, Validify, ExportValidation)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProviderModelConfiguration {
    #[validate(length(min = 1, max = 40, message = "Model name must contain 1 to 40 characters."), custom(function = input::model_name, message = "Use letters, numbers, hyphens, or underscores for the model name."))]
    pub name: String,
    pub protocol: TextModelProtocol,
    #[validate(custom(function = input::endpoint_path, message = "Enter a path beginning with / using letters, numbers, hyphens, underscores, and slashes."), length(max = 40, message = "Path must be 40 characters or fewer."))]
    pub path: String,
    pub max_context_tokens: ContextTokens,
    #[graphql(default = true)]
    #[serde(default = "enabled")]
    pub enabled: bool,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    #[schemars(extend("x-compallm-non-null" = true))]
    #[validify]
    pub effort_mapping: Option<ProviderEffortMapping>,
    #[graphql(default)]
    #[serde(default)]
    pub capabilities: Vec<ProviderModelCapability>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    #[schemars(extend("x-compallm-non-null" = true))]
    #[validify]
    pub catalog_ref: Option<ProviderCatalogRef>,
    #[validate(custom(function = input::price, message = "Enter a nonnegative price with at most 12 integer and 8 decimal digits."))]
    pub input_price: String,
    #[validate(custom(function = input::price, message = "Enter a nonnegative price with at most 12 integer and 8 decimal digits."))]
    pub output_price: String,
    #[validate(custom(function = input::price, message = "Enter a nonnegative price with at most 12 integer and 8 decimal digits."))]
    pub cached_price: String,
}
fn enabled() -> bool {
    true
}

/// Persisted identity is omitted for new models and forbidden during provider creation.
#[derive(Serialize, Deserialize, GraphQLInputObject, Validify, ExportValidation)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProviderModelWrite {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    #[schemars(extend("x-compallm-non-null" = true))]
    #[validate(custom(function = input::resource_id, message = "Choose a valid resource identity."))]
    /// Omit for a new model; supplied IDs must belong to this connection.
    #[schemars(with = "Option<String>")]
    pub id: Option<ID>,
    #[validify]
    /// Complete text-model configuration, including explicit token prices.
    pub configuration: ProviderModelConfiguration,
}

fn model_writes(entries: &[ProviderModelWrite]) -> Result<(), ValidationError> {
    let mut names = std::collections::HashSet::new();
    let mut ids = std::collections::HashSet::new();
    for model in entries {
        if !names.insert(&model.configuration.name)
            || model.id.as_ref().is_some_and(|id| !ids.insert(&**id))
        {
            return Err(ValidationError::new_field("duplicate_model"));
        }
    }
    if !entries.iter().any(|model| model.configuration.enabled) {
        return Err(ValidationError::new_field("missing_enabled_model"));
    }
    Ok(())
}
fn model_writes_contract() -> Value {
    json!({"minItems": 1, "x-compallm-array-rule": "modelWrites"})
}
fn header_entries(entries: &[ProviderHeader]) -> Result<(), ValidationError> {
    unique_headers(entries.iter().map(|header| header.key.as_str()))
}
fn header_entries_contract() -> Value {
    json!({"x-compallm-array-rule": "headers"})
}
fn unique_headers<'a>(keys: impl Iterator<Item = &'a str>) -> Result<(), ValidationError> {
    let mut names = std::collections::HashSet::new();
    if keys
        .into_iter()
        .any(|key| !names.insert(key.to_ascii_lowercase()))
    {
        return Err(ValidationError::new_field("duplicate_header"));
    }
    Ok(())
}

/// New connection definition; credentials and header values are write-only.
#[derive(Serialize, Deserialize, GraphQLInputObject, Validify, ExportValidation)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[validate(validate_create)]
pub struct ProviderCreateInput {
    #[modify(trim)]
    #[validate(length(min = 1, max = 120, message = "Connection name must contain 1 to 120 characters."), custom(function = input::nonblank, message = "Enter a connection name."))]
    pub name: String,
    #[validate(custom(function = input::base_url, message = "Enter an HTTP or HTTPS server URL without credentials, query, or fragment."))]
    pub base_url: String,
    #[validate(length(min = 1, max = 512, message = "Enter an API key of 1 to 512 characters."), custom(function = input::api_key, message = "API keys cannot contain whitespace."))]
    pub api_key: String,
    #[validate(custom(function = header_entries, message = "Header names must be unique, ignoring case."))]
    #[validify]
    pub headers: Vec<ProviderHeader>,
    #[validate(custom(function = model_writes, message = "Add at least one enabled model and use unique model names and IDs."))]
    #[validify]
    pub models: Vec<ProviderModelWrite>,
}
fn validate_create(value: &ProviderCreateInput) -> Result<(), ValidationErrors> {
    if value.models.iter().any(|model| model.id.is_some()) {
        return Err(invalid("new_model_id"));
    }
    Ok(())
}
fn validate_create_contract() -> Value {
    json!({"x-compallm-object-rule": "createProvider", "x-compallm-messages": {"x-compallm-object-rule": "New models must not include a saved model identity."}})
}

/// Atomic batch of new provider connections for the verified organization.
#[derive(Serialize, Deserialize, GraphQLInputObject, Validify, ExportValidation)]
#[serde(deny_unknown_fields)]
#[validate(validate_create_batch)]
pub struct ProvidersCreateInput {
    #[validify]
    pub providers: Vec<ProviderCreateInput>,
}
fn validate_create_batch(value: &ProvidersCreateInput) -> Result<(), ValidationErrors> {
    let mut names = std::collections::HashSet::new();
    if value.providers.is_empty() {
        return Err(invalid("missing_provider"));
    }
    for provider in &value.providers {
        if !names.insert(provider.name.trim()) {
            return Err(invalid("duplicate_provider"));
        }
    }
    Ok(())
}
fn validate_create_batch_contract() -> Value {
    json!({"x-compallm-object-rule": "createProviders", "x-compallm-messages": {"x-compallm-object-rule": "Add at least one connection and use unique connection names."}})
}

/// Omit value to retain an existing header secret; new names require a value.
#[derive(Serialize, Deserialize, GraphQLInputObject, Validify, ExportValidation)]
#[serde(deny_unknown_fields)]
#[validate(validate_header_update)]
pub struct ProviderHeaderUpdate {
    #[validate(custom(function = input::header_name, message = "Enter a valid HTTP header name."), length(max = 128, message = "Header name must be 128 characters or fewer."))]
    pub key: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    #[schemars(extend("x-compallm-non-null" = true))]
    #[validate(custom(function = input::header_value, message = "Header values cannot contain newlines or unsupported characters."))]
    /// Omit to preserve the saved value for this header name. New names require a value.
    pub value: Option<String>,
}
fn validate_header_update(value: &ProviderHeaderUpdate) -> Result<(), ValidationErrors> {
    if value.key.eq_ignore_ascii_case("authorization")
        && let Some(content) = &value.value
    {
        input::authorization(content).map_err(|_| invalid("authorization"))?;
    }
    Ok(())
}
fn validate_header_update_contract() -> Value {
    json!({"x-compallm-object-rule": "headerUpdate", "x-compallm-messages": {"x-compallm-object-rule": "Enter a valid Authorization header value."}})
}

fn header_updates(entries: &[ProviderHeaderUpdate]) -> Result<(), ValidationError> {
    unique_headers(entries.iter().map(|header| header.key.as_str()))
}
fn header_updates_contract() -> Value {
    json!({"x-compallm-array-rule": "headers"})
}

/// Full non-secret replacement. Omitting apiKey preserves the saved credential.
#[derive(Serialize, Deserialize, GraphQLInputObject, Validify, ExportValidation)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProviderReplaceInput {
    #[modify(trim)]
    #[validate(length(min = 1, max = 120, message = "Connection name must contain 1 to 120 characters."), custom(function = input::nonblank, message = "Enter a connection name."))]
    pub name: String,
    #[validate(custom(function = input::base_url, message = "Enter an HTTP or HTTPS server URL without credentials, query, or fragment."))]
    pub base_url: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    #[schemars(extend("x-compallm-non-null" = true))]
    #[validate(length(min = 1, max = 512, message = "Enter an API key of 1 to 512 characters."), custom(function = input::api_key, message = "API keys cannot contain whitespace."))]
    /// Omit to preserve the current credential. Empty strings and null are invalid.
    pub api_key: Option<String>,
    #[validate(custom(function = header_updates, message = "Header names must be unique, ignoring case."))]
    #[validify]
    pub headers: Vec<ProviderHeaderUpdate>,
    #[validate(custom(function = model_writes, message = "Add at least one enabled model and use unique model names and IDs."))]
    #[validify]
    pub models: Vec<ProviderModelWrite>,
}

/// Partial update. Omitted fields are unchanged; explicit null is rejected.
#[derive(Serialize, Deserialize, GraphQLInputObject, Validify, ExportValidation)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[validate(validate_patch)]
pub struct ProviderPatchInput {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    #[schemars(extend("x-compallm-non-null" = true))]
    #[modify(trim)]
    #[validate(length(min = 1, max = 120, message = "Connection name must contain 1 to 120 characters."), custom(function = input::nonblank, message = "Enter a connection name."))]
    pub name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    #[schemars(extend("x-compallm-non-null" = true))]
    #[validate(custom(function = input::base_url, message = "Enter an HTTP or HTTPS server URL without credentials, query, or fragment."))]
    pub base_url: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    #[schemars(extend("x-compallm-non-null" = true))]
    #[validate(length(min = 1, max = 512, message = "Enter an API key of 1 to 512 characters."), custom(function = input::api_key, message = "API keys cannot contain whitespace."))]
    /// Omit to preserve the current credential. Empty strings and null are invalid.
    pub api_key: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    #[schemars(extend("x-compallm-non-null" = true))]
    #[validate(custom(function = header_updates, message = "Header names must be unique, ignoring case."))]
    #[validify]
    pub headers: Option<Vec<ProviderHeaderUpdate>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    #[schemars(extend("x-compallm-non-null" = true))]
    #[validate(custom(function = model_writes, message = "Add at least one enabled model and use unique model names and IDs."))]
    #[validify]
    pub models: Option<Vec<ProviderModelWrite>>,
}
fn validate_patch(value: &ProviderPatchInput) -> Result<(), ValidationErrors> {
    if value.name.is_none()
        && value.base_url.is_none()
        && value.api_key.is_none()
        && value.headers.is_none()
        && value.models.is_none()
    {
        return Err(invalid("empty_patch"));
    }
    Ok(())
}
fn validate_patch_contract() -> Value {
    json!({"x-compallm-object-rule": "providerPatch", "x-compallm-messages": {"x-compallm-object-rule": "Change at least one connection field."}})
}

#[derive(Serialize, Deserialize, GraphQLInputObject, Validify, ExportValidation)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[validate(validate_header)]
pub struct ProviderHeader {
    #[validate(custom(function = input::header_name, message = "Enter a valid HTTP header name."), length(max = 128, message = "Header name must be 128 characters or fewer."))]
    pub key: String,
    /// Write-only header content; never returned by management reads.
    #[validate(custom(function = input::header_value, message = "Header values cannot contain newlines or unsupported characters."))]
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
    serde_json::json!({"x-compallm-object-rule": "header", "x-compallm-messages": {"x-compallm-object-rule": "Enter a valid Authorization header value."}})
}
