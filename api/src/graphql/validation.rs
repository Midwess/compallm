//! Export actual GraphQL input definitions for shared browser validation.
use schemars::JsonSchema;

#[derive(JsonSchema)]
#[allow(dead_code)]
struct Inputs {
    email: super::authentication::EmailInput,
    code: super::authentication::EmailCodeInput,
    provider_create: super::provider::input::ProvidersCreateInput,
    provider_replace: super::provider::input::ProviderReplaceInput,
    provider_patch: super::provider::input::ProviderPatchInput,
}

pub(crate) fn schema() -> serde_json::Value {
    serde_json::to_value(schemars::schema_for!(Inputs))
        .expect("serializable GraphQL validation schema")
}
