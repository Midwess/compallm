//! Stateless email formatting and validation helpers.

use crate::error::EmailError;
use crate::text::has_special_characters;

/// Validate an unquoted email local-part without a domain or length policy.
pub fn verify_email_local_part(value: &str) -> Result<(), EmailError> {
    if value.is_empty()
        || has_special_characters(value, ".!#$%&'*+-/=?^_`{|}~")
        || value.starts_with('.')
        || value.ends_with('.')
        || value.contains("..")
    {
        return Err(EmailError::InvalidEmail);
    }
    Ok(())
}

pub fn verify_email(email: &str) -> Result<(), EmailError> {
    if !email_address::EmailAddress::is_valid(email) {
        return Err(EmailError::InvalidEmail);
    }
    Ok(())
}

pub fn normalize_email(email: &str) -> Result<String, EmailError> {
    let email = email.trim().to_ascii_lowercase();
    verify_email(&email)?;
    Ok(email)
}
