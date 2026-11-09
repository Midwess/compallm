use compallm_utils::email;
use compallm_validation_derive::ExportValidation;
use juniper::{GraphQLEnum, GraphQLInputObject};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validify::{ValidationError, Validify};

/// How the request delivers credentials; this is never stored in signed claims.
#[derive(Clone, Copy, Default, GraphQLEnum, Serialize, Deserialize, JsonSchema, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CredentialDelivery {
    /// Set HttpOnly cookies and return metadata without credential values.
    #[default]
    Cookie,
    /// Return credentials explicitly without setting browser cookies.
    Token,
}

/// Shared input for requesting either a signup or a signin email challenge.
///
/// Call `validify()` before Core execution. GraphQL parsing alone does not run
/// normalization, application validation, or email ownership checks.
#[derive(GraphQLInputObject, Validify, ExportValidation)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EmailInput {
    /// Trimmed and ASCII-lowercased using the shared Rust email policy.
    #[modify(trim, custom(str::make_ascii_lowercase))]
    #[validate(custom(function = email::validate_email))]
    pub email: String,
    /// Cookie delivery is the default; token delivery must be requested explicitly.
    #[graphql(default)]
    #[serde(default)]
    pub delivery: CredentialDelivery,
}

/// Shared input for confirming a signup or signin email challenge.
///
/// The verified encrypted challenge comes from request context, never this input.
/// The adapter calls `validify()` before Core verifies the code and challenge.
#[derive(GraphQLInputObject, Validify, ExportValidation)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EmailCodeInput {
    /// Exactly six ASCII digits after trimming surrounding whitespace.
    #[modify(trim)]
    #[validate(length(equal = 6), custom(function = Self::ascii_digits))]
    pub code: String,
    /// Cookie delivery is the default; token delivery must be requested explicitly.
    #[graphql(default)]
    #[serde(default)]
    pub delivery: CredentialDelivery,
}

impl EmailCodeInput {
    fn ascii_digits_contract() -> serde_json::Value {
        serde_json::json!({"pattern":"^[0-9]*$"})
    }

    fn ascii_digits(value: &str) -> Result<(), ValidationError> {
        value
            .bytes()
            .all(|byte| byte.is_ascii_digit())
            .then_some(())
            .ok_or_else(|| ValidationError::new_field("code"))
    }
}
