use serde::{Deserialize, Deserializer, de::Error};
use serde_json::value::RawValue;
use std::borrow::Cow;

const SCHEMA: &str = "compallm_serde_rules::borrowed::schema";

struct Schema<'a>(Cow<'a, str>);

impl<'de> Deserialize<'de> for Schema<'de> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        schema(deserializer).map(Self)
    }
}

#[derive(Deserialize)]
pub struct Text<'a>(#[serde(borrow)] pub Cow<'a, str>);

pub fn text<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Cow<'de, str>, D::Error> {
    Text::deserialize(deserializer).map(|Text(value)| value)
}

pub fn present_text<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<Cow<'de, str>>, D::Error> {
    text(deserializer).map(Some)
}

pub fn present_optional_text<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<Option<Cow<'de, str>>>, D::Error> {
    optional_text(deserializer).map(Some)
}

pub fn present_schema<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<Cow<'de, str>>, D::Error> {
    schema(deserializer).map(Some)
}

pub fn nonempty<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Cow<'de, str>, D::Error> {
    let Text(value) = Text::deserialize(deserializer)?;
    if value.is_empty() {
        return Err(D::Error::custom("expected nonempty string"));
    }
    Ok(value)
}

pub fn optional_text<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<Cow<'de, str>>, D::Error> {
    Option::<Text<'de>>::deserialize(deserializer).map(|value| value.map(|Text(value)| value))
}

fn schema_text<'a, E: Error>(value: &'a RawValue) -> Result<Cow<'a, str>, E> {
    match value.get().as_bytes().first() {
        Some(b'{') => Ok(Cow::Borrowed(value.get())),
        Some(b'"') => serde_json::from_str::<Text<'a>>(value.get())
            .map(|Text(value)| value)
            .map_err(E::custom),
        _ => Err(E::custom("expected schema object or string")),
    }
}

pub fn schema<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Cow<'de, str>, D::Error> {
    struct SchemaVisitor;
    impl<'de> serde::de::Visitor<'de> for SchemaVisitor {
        type Value = Cow<'de, str>;
        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str("schema object or string")
        }
        fn visit_borrowed_str<E: Error>(self, text: &'de str) -> Result<Self::Value, E> {
            Ok(Cow::Borrowed(text))
        }
        fn visit_newtype_struct<D: Deserializer<'de>>(
            self,
            deserializer: D,
        ) -> Result<Self::Value, D::Error> {
            schema_text(<&RawValue>::deserialize(deserializer)?)
        }
    }
    deserializer.deserialize_newtype_struct(SCHEMA, SchemaVisitor)
}

pub fn optional_schema<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<Cow<'de, str>>, D::Error> {
    Option::<Schema<'de>>::deserialize(deserializer).map(|value| value.map(|Schema(text)| text))
}
mod deserializer;
pub use deserializer::from_slice;
