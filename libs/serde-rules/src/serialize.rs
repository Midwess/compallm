use serde::{Serialize, Serializer};
use serde_json::value::RawValue;

/// Validates stored JSON text and emits it as a JSON value rather than a quoted string.
pub fn serialize_raw<T: AsRef<str>, S: Serializer>(
    value: &T,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let raw: &RawValue = serde_json::from_str(value.as_ref()).map_err(serde::ser::Error::custom)?;
    raw.serialize(serializer)
}

/// Emits stored JSON text as a value, or null for `None`.
/// Use `skip_serializing_if = "Option::is_none"` to omit absent fields instead.
pub fn optional_raw<T: AsRef<str>, S: Serializer>(
    value: &Option<T>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    value
        .as_ref()
        .map(|value| serde_json::from_str::<&RawValue>(value.as_ref()))
        .transpose()
        .map_err(serde::ser::Error::custom)?
        .serialize(serializer)
}
