//! Export actual GraphQL input definitions for shared browser validation.
use schemars::JsonSchema;

#[derive(JsonSchema)]
#[allow(dead_code)]
struct Inputs {
    email: super::auth::EmailInput,
    code: super::auth::EmailCodeInput,
}

pub(crate) fn schema() -> serde_json::Value {
    serde_json::to_value(schemars::schema_for!(Inputs))
        .expect("serializable GraphQL validation schema")
}
