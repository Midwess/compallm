//! Stateless email normalization and validation.

pub fn normalize_email(email: &str) -> String {
    email.trim().to_ascii_lowercase()
}

pub fn validate_email(value: &str) -> Result<(), validify::ValidationError> {
    email_address::EmailAddress::is_valid(value)
        .then_some(())
        .ok_or_else(|| validify::ValidationError::new_field("email"))
}
