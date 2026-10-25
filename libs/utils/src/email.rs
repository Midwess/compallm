//! Stateless email formatting and validation helpers.

#[derive(Debug, thiserror::Error)]
pub enum EmailError {
    #[error("invalid email address")]
    InvalidEmail,
    #[error("invalid email domain")]
    InvalidDomain,
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

/// Compare an address against one exact domain, excluding subdomains.
pub fn verify_email_domain(email: &str, domain: &str) -> Result<(), EmailError> {
    let domain = domain.trim().to_ascii_lowercase();
    if verify_email(&format!("user@{domain}")).is_err() {
        return Err(EmailError::InvalidDomain);
    }
    let email = normalize_email(email)?;
    if email.rsplit_once('@').map(|(_, domain)| domain) != Some(domain.as_str()) {
        return Err(EmailError::InvalidEmail);
    }
    Ok(())
}
