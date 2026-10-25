/// Report characters outside ASCII letters, digits, and an explicit allowlist.
/// Whitespace and non-ASCII characters are special unless explicitly allowed.
pub fn has_special_characters(text: &str, allowed: &str) -> bool {
    text.chars()
        .any(|character| !character.is_ascii_alphanumeric() && !allowed.contains(character))
}
