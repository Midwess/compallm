//! Stateless email formatting. Validation policy belongs to domain types.

pub fn normalize_email(email: &str) -> String {
    email.trim().to_ascii_lowercase()
}
