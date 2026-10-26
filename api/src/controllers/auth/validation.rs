use axum::{Json, extract::rejection::JsonRejection};
use axum_valid::{ValidifiedByRef, ValidifyRejection};
use validify::ValidationError;

/// Normalize and validate JSON while retaining the API's centralized error response.
pub type ValidatedJson<T> = Result<ValidifiedByRef<Json<T>>, ValidifyRejection<JsonRejection>>;

/// Shared DTO rules adapt the existing email policy without changing accepted addresses.
pub struct AuthInput;

impl AuthInput {
    pub fn ascii_lowercase(value: &mut str) {
        value.make_ascii_lowercase();
    }

    pub fn email(value: &str) -> Result<(), ValidationError> {
        compallm_utils::email::verify_email(value).map_err(|_| ValidationError::new_field("email"))
    }

    pub fn otp(value: &str) -> Result<(), ValidationError> {
        if value.bytes().all(|byte| byte.is_ascii_digit()) {
            Ok(())
        } else {
            Err(ValidationError::new_field("otp_digits"))
        }
    }
}
