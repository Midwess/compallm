use serde::{Deserialize, Deserializer};
use serde_json::value::RawValue;

/// Deserializes a string-like value and rejects empty text without trimming it.
pub fn nonempty<'de, D: Deserializer<'de>, S: Deserialize<'de> + AsRef<str>>(
    deserializer: D,
) -> Result<S, D::Error> {
    let value = S::deserialize(deserializer)?;
    if value.as_ref().is_empty() {
        return Err(serde::de::Error::custom("expected nonempty string"));
    }
    Ok(value)
}

/// Wraps a supplied field in `Some`; pair with `#[serde(default)]` for missing fields.
/// Explicit null is accepted only if `T` itself accepts null.
pub fn present<'de, D: Deserializer<'de>, T: Deserialize<'de>>(
    deserializer: D,
) -> Result<Option<T>, D::Error> {
    T::deserialize(deserializer).map(Some)
}

fn schema_text<E: serde::de::Error>(value: Box<RawValue>) -> Result<String, E> {
    match value.get().as_bytes().first() {
        Some(b'{') => Ok(Box::<str>::from(value).into_string()),
        Some(b'"') => serde_json::from_str(value.get()).map_err(E::custom),
        _ => Err(E::custom("expected schema object or string")),
    }
}

/// Captures a JSON object as text or decodes a JSON string without validating its contents.
/// Other JSON shapes, including null, are rejected.
pub fn schema<'de, D: Deserializer<'de>, S: From<String>>(deserializer: D) -> Result<S, D::Error> {
    schema_text(Box::<RawValue>::deserialize(deserializer)?).map(Into::into)
}

/// Applies [`schema`] to non-null values; explicit null becomes `None`.
pub fn optional_schema<'de, D: Deserializer<'de>, S: From<String>>(
    deserializer: D,
) -> Result<Option<S>, D::Error> {
    Option::<Box<RawValue>>::deserialize(deserializer)?
        .map(schema_text)
        .transpose()
        .map(|value| value.map(Into::into))
}
