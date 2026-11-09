//! Offline form contracts and export-only input registry. No HTTP operations.
pub mod provider;

use schemars::JsonSchema;

#[derive(JsonSchema)]
#[allow(dead_code)]
struct Inputs {
    email: crate::graphql::auth::EmailInput,
    code: crate::graphql::auth::EmailCodeInput,
    fields: provider::ProviderFields,
    model: provider::ProviderModelFields,
    headers: provider::ProviderHeaders,
    models: provider::ProviderModels,
    rules: provider::ProviderInputRules,
    create: provider::payload::CreateProvidersRequest,
    replace: provider::payload::ReplaceProviderRequest,
    patch: provider::payload::PatchProviderRequest,
}

pub fn schema() -> serde_json::Value {
    serde_json::to_value(schemars::schema_for!(Inputs)).expect("serializable contract")
}
