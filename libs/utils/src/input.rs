//! Shared stateless input validators and their contract exporters. Contract descriptions contain only safe rules,
//! never submitted values. Custom formats have explicit browser implementations.
use regex::Regex;
use serde_json::{Value, json};
use std::sync::LazyLock;
use validify::ValidationError;

pub const BASE_URL_MAX_LENGTH: usize = 512;
pub const PRICE_INTEGER_DIGITS: u32 = 12;
pub const PRICE_SCALE: u32 = 8;

macro_rules! pattern_rule {
    ($validate:ident, $contract:ident, $pattern:literal) => {
        pub fn $validate(value: &str) -> Result<(), ValidationError> {
            static PATTERN: LazyLock<Regex> = LazyLock::new(|| Regex::new($pattern).unwrap());
            PATTERN.is_match(value).then_some(()).ok_or_else(|| ValidationError::new_field(stringify!($validate)))
        }
        pub fn $contract() -> Value { json!({"pattern": $pattern}) }
    };
}

pattern_rule!(model_name, model_name_contract, r"^[A-Za-z0-9_-]+$");
pattern_rule!(effort_value, effort_value_contract, r"^[A-Za-z0-9_-]+$");
pattern_rule!(
    endpoint_path,
    endpoint_path_contract,
    r"^/(?:[A-Za-z0-9_-][A-Za-z0-9_/-]*)?$"
);
pattern_rule!(api_key, api_key_contract, r"^[^\s]+$");
pattern_rule!(
    header_name,
    header_name_contract,
    r"^[!#$%&'*+.^_`|~0-9A-Za-z-]+$"
);
pattern_rule!(
    header_value,
    header_value_contract,
    r"^[\t\x20-\x7e\x80-\xff]*$"
);
fn price_pattern() -> String {
    format!(r"^0*[0-9]{{1,{PRICE_INTEGER_DIGITS}}}(?:\.[0-9]{{1,{PRICE_SCALE}}})?$")
}
pub fn price(value: &str) -> Result<(), ValidationError> {
    static PATTERN: LazyLock<Regex> = LazyLock::new(|| Regex::new(&price_pattern()).unwrap());
    PATTERN
        .is_match(value)
        .then_some(())
        .ok_or_else(|| ValidationError::new_field("price"))
}
pub fn price_contract() -> Value {
    json!({
        "pattern": price_pattern(),
        "x-compallm-input-pattern": format!(r"^0*[0-9]{{0,{PRICE_INTEGER_DIGITS}}}(?:\.[0-9]{{0,{PRICE_SCALE}}})?$")
    })
}

pattern_rule!(
    positive_integer,
    positive_integer_contract,
    r"^[1-9][0-9]*$"
);

pub fn nonblank(value: &str) -> Result<(), ValidationError> {
    (!value.trim().is_empty())
        .then_some(())
        .ok_or_else(|| ValidationError::new_field("nonblank"))
}
pub fn nonblank_contract() -> Value {
    json!({"x-compallm-format": "nonblank"})
}

pub fn base_url(value: &str) -> Result<(), ValidationError> {
    let valid = value.chars().count() <= BASE_URL_MAX_LENGTH
        && !value
            .chars()
            .any(|c| c.is_whitespace() || matches!(c, '?' | '#' | '\\'))
        && (value.to_ascii_lowercase().starts_with("http://")
            || value.to_ascii_lowercase().starts_with("https://"))
        && url::Url::parse(value).is_ok_and(|url| {
            matches!(url.scheme(), "http" | "https")
                && url.has_host()
                && url.username().is_empty()
                && url.password().is_none()
        });
    valid
        .then_some(())
        .ok_or_else(|| ValidationError::new_field("base_url"))
}
pub fn base_url_contract() -> Value {
    json!({"maxLength": BASE_URL_MAX_LENGTH, "x-compallm-format": "baseUrl"})
}

pub fn authorization(value: &str) -> Result<(), ValidationError> {
    static VALUE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r#"^[!#$%&'*+.^_`|~0-9A-Za-z-]+(?: +(.+))?$"#).unwrap());
    static TOKEN68: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^[A-Za-z0-9\-._~+/]+=*$").unwrap());
    static PARAMS: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r#"^[!#$%&'*+.^_`|~0-9A-Za-z-]+[ \t]*=[ \t]*(?:[!#$%&'*+.^_`|~0-9A-Za-z-]+|"(?:[\t !#-\[\]-~\x80-\xff]|\\[\t\x20-\x7e\x80-\xff])*")(?:[ \t]*,[ \t]*[!#$%&'*+.^_`|~0-9A-Za-z-]+[ \t]*=[ \t]*(?:[!#$%&'*+.^_`|~0-9A-Za-z-]+|"(?:[\t !#-\[\]-~\x80-\xff]|\\[\t\x20-\x7e\x80-\xff])*"))*$"#).unwrap()
    });
    let value = value.trim();
    let valid = VALUE.captures(value).is_some_and(|m| {
        let scheme = value.split(' ').next().unwrap();
        let credentials = m.get(1).map(|m| m.as_str());
        if scheme.eq_ignore_ascii_case("basic") || scheme.eq_ignore_ascii_case("bearer") {
            credentials.is_some_and(|v| TOKEN68.is_match(v))
        } else {
            credentials.is_none_or(|v| TOKEN68.is_match(v) || PARAMS.is_match(v))
        }
    });
    valid
        .then_some(())
        .ok_or_else(|| ValidationError::new_field("authorization"))
}
pub fn authorization_contract() -> Value {
    json!({"x-compallm-format": "authorization"})
}

/// Positive decimal-string IDs preserve u64 precision in JavaScript.
pub fn resource_id(value: &str) -> Result<(), ValidationError> {
    (value.parse::<u64>().is_ok_and(|id| id > 0)
        && !value.starts_with('0')
        && value.bytes().all(|c| c.is_ascii_digit()))
    .then_some(())
    .ok_or_else(|| ValidationError::new_field("resource_id"))
}
pub fn resource_id_contract() -> Value {
    json!({"pattern": "^[1-9][0-9]*$", "x-compallm-format": "resourceId"})
}
