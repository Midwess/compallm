//! Stateless email formatting and validation helpers.

use crate::error::EmailError;

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
