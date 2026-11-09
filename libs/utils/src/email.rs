//! Stateless email normalization, validation, and matching contract export.

pub fn normalize_email(email: &str) -> String {
    email.trim().to_ascii_lowercase()
}

pub fn validate_email(value: &str) -> Result<(), validify::ValidationError> {
    email_address::EmailAddress::is_valid(value)
        .then_some(())
        .ok_or_else(|| validify::ValidationError::new_field("email"))
}

/// Describe the existing parser for the explicit browser adapter. Character
/// classes are measured from the Rust dependency, including its Unicode policy.
pub fn validate_email_contract() -> serde_json::Value {
    static POLICY: std::sync::LazyLock<serde_json::Value> = std::sync::LazyLock::new(|| {
        fn class(accepts: impl Fn(char) -> bool) -> String {
            let mut ranges = String::from("^[");
            let mut start = None;
            let mut end = 0;
            let append = |ranges: &mut String, start: u32, end: u32| {
                use std::fmt::Write;
                write!(ranges, "\\u{{{start:x}}}").unwrap();
                if start != end {
                    write!(ranges, "-\\u{{{end:x}}}").unwrap();
                }
            };
            for point in 0..=0x10ffff {
                if char::from_u32(point).is_some_and(&accepts) {
                    if start.is_none() {
                        start = Some(point);
                    }
                    end = point;
                } else if let Some(first) = start.take() {
                    append(&mut ranges, first, end);
                }
            }
            if let Some(first) = start {
                append(&mut ranges, first, end);
            }
            ranges.push_str("]+$");
            ranges
        }
        let local_limit = (1..=1024)
            .take_while(|n| email_address::EmailAddress::is_valid_local_part(&"a".repeat(*n)))
            .last()
            .unwrap();
        let label_limit = (1..=1024)
            .take_while(|n| email_address::EmailAddress::is_valid_domain(&"a".repeat(*n)))
            .last()
            .unwrap();
        let domain_limit = (2..=1024)
            .take_while(|n| {
                email_address::EmailAddress::is_valid_domain(&format!("[{}]", "a".repeat(*n - 2)))
            })
            .last()
            .unwrap();
        serde_json::json!({
            "localMaxBytes": local_limit,
            "labelMaxBytes": label_limit,
            "domainMaxBytes": domain_limit,
            "atom": class(|c| email_address::EmailAddress::is_valid_local_part(&c.to_string())),
            "quoted": class(|c| email_address::EmailAddress::is_valid_local_part(&format!("\"{c}\""))),
            "literal": class(|c| email_address::EmailAddress::is_valid_domain(&format!("[{c}]"))),
            "edge": class(char::is_alphanumeric),
        })
    });
    serde_json::json!({"x-compallm-format": "emailAddress", "x-compallm-email-policy": *POLICY})
}
