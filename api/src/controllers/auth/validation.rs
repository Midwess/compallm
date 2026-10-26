use axum::{Json, extract::rejection::JsonRejection};
use axum_valid::{ValidifiedByRef, ValidifyRejection};
use validify::ValidationError;

/// Normalize and validate JSON while retaining the API's centralized error response.
pub type ValidatedJson<T> = Result<ValidifiedByRef<Json<T>>, ValidifyRejection<JsonRejection>>;

/// Shared DTO rules adapt the existing email policy without changing accepted addresses.
pub struct AuthInput;

impl AuthInput {
    pub fn otp(value: &str) -> Result<(), ValidationError> {
        if value.bytes().all(|byte| byte.is_ascii_digit()) {
            Ok(())
        } else {
            Err(ValidationError::new_field("otp_digits"))
        }
    }
}
