use serde::{Deserialize, Deserializer};
use serde_json::value::RawValue;

pub(super) fn nonempty<'de, D: Deserializer<'de>, S: Deserialize<'de> + AsRef<str>>(
    deserializer: D,
) -> Result<S, D::Error> {
    let value = S::deserialize(deserializer)?;
    if value.as_ref().is_empty() {
        return Err(serde::de::Error::custom("expected nonempty string"));
    }
    Ok(value)
}

pub(super) fn present<'de, D: Deserializer<'de>, T: Deserialize<'de>>(
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

pub(super) fn schema<'de, D: Deserializer<'de>, S: From<String>>(
    deserializer: D,
) -> Result<S, D::Error> {
    schema_text(Box::<RawValue>::deserialize(deserializer)?).map(Into::into)
}

pub(super) fn optional_schema<'de, D: Deserializer<'de>, S: From<String>>(
    deserializer: D,
) -> Result<Option<S>, D::Error> {
    Option::<Box<RawValue>>::deserialize(deserializer)?
        .map(schema_text)
        .transpose()
        .map(|value| value.map(Into::into))
}
