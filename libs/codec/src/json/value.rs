use serde::de::{DeserializeSeed, MapAccess, SeqAccess, Visitor};
use serde::ser::{SerializeMap, SerializeSeq};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::cell::Cell;

pub(super) const JSON_VALUE: &str = "codec::json::value";
pub(super) const JSON_ARGUMENT: &str = "codec::json::argument";

/// Buffered JSON tokens and their string storage; encoding does not parse the source text.
/// Equality compares tokens independently of source whitespace.
#[derive(Clone, Debug)]
pub struct JsonValue<S = String> {
    source: S,
    tokens: Vec<Token>,
    text: String,
}

impl<S> PartialEq for JsonValue<S> {
    fn eq(&self, other: &Self) -> bool {
        self.tokens.len() == other.tokens.len()
            && self
                .tokens
                .iter()
                .zip(&other.tokens)
                .all(|(left, right)| match (left, right) {
                    (Token::Key(left), Token::Key(right))
                    | (Token::String(left), Token::String(right)) => {
                        self.text[left.clone()] == other.text[right.clone()]
                    }
                    _ => left == right,
                })
    }
}

impl<S> Eq for JsonValue<S> {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum Token {
    ObjectStart,
    ObjectEnd,
    ArrayStart,
    ArrayEnd,
    // Arena offsets stay valid when string storage grows.
    Key(std::ops::Range<usize>),
    String(std::ops::Range<usize>),
    Number(serde_json::Number),
    Boolean(bool),
    Null,
}

impl<S> JsonValue<S> {
    pub(super) fn from_capture(
        source: S,
        tokens: impl Iterator<Item = Token>,
        text: String,
    ) -> Self {
        let mut retained = Vec::with_capacity(4096 / std::mem::size_of::<Token>());
        retained.extend(tokens);
        Self {
            source,
            tokens: retained,
            text,
        }
    }

    pub fn map_text<T>(self, mut convert: impl FnMut(S) -> T) -> JsonValue<T> {
        JsonValue {
            source: convert(self.source),
            tokens: self.tokens,
            text: self.text,
        }
    }
}

impl<'a> TryFrom<&'a str> for JsonValue<&'a str> {
    type Error = super::JsonDecodeError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        super::JsonDecoder::decode_value(value)
    }
}

impl TryFrom<String> for JsonValue {
    type Error = super::JsonDecodeError;

    fn try_from(source: String) -> Result<Self, Self::Error> {
        let value = super::JsonDecoder::decode_value(&source)?;
        let JsonValue { tokens, text, .. } = value;
        Ok(Self {
            source,
            tokens,
            text,
        })
    }
}

impl<'de: 'a, 'a> Deserialize<'de> for JsonValue<&'a str> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_newtype_struct(JSON_VALUE, SchemaVisitor { object: true })
    }
}

struct SchemaVisitor {
    object: bool,
}

impl<'de> Visitor<'de> for SchemaVisitor {
    type Value = JsonValue<&'de str>;

    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("captured JSON object tokens or embedded JSON text")
    }

    fn visit_borrowed_str<E: serde::de::Error>(self, source: &'de str) -> Result<Self::Value, E> {
        let value = JsonValue::<&str>::try_from(source).map_err(E::custom)?;
        if self.object && !matches!(value.tokens.first(), Some(Token::ObjectStart)) {
            return Err(E::custom("expected JSON object"));
        }
        Ok(value)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let source = seq
            .next_element::<&str>()?
            .ok_or_else(|| serde::de::Error::custom("missing JSON source"))?;
        let mut tokens = Vec::with_capacity(4096 / std::mem::size_of::<Token>());
        let mut text = String::with_capacity(4096);
        seq.next_element_seed(Seed(&mut tokens, &mut text, 0))?
            .ok_or_else(|| serde::de::Error::custom("missing JSON tokens"))?;
        if self.object && !matches!(tokens.first(), Some(Token::ObjectStart)) {
            return Err(serde::de::Error::custom("expected JSON object"));
        }
        Ok(JsonValue {
            source,
            tokens,
            text,
        })
    }
}

impl<S: AsRef<str>> AsRef<str> for JsonValue<S> {
    fn as_ref(&self) -> &str {
        self.source.as_ref()
    }
}

impl<S: AsRef<str>> std::ops::Deref for JsonValue<S> {
    type Target = str;

    fn deref(&self) -> &str {
        self.as_ref()
    }
}

/// Tool arguments retain their exact text for string-valued protocols.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JsonArgument<S = String> {
    Value(super::JsonView<S>),
    Text(S),
}

impl<'de: 'a, 'a> Deserialize<'de> for JsonArgument<&'a str> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_newtype_struct(JSON_ARGUMENT, ArgumentVisitor)
    }
}

struct ArgumentVisitor;

impl<'de> Visitor<'de> for ArgumentVisitor {
    type Value = JsonArgument<&'de str>;

    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("captured argument JSON or opaque argument text")
    }

    fn visit_borrowed_str<E: serde::de::Error>(self, text: &'de str) -> Result<Self::Value, E> {
        Ok(JsonArgument::Text(text))
    }

    fn visit_newtype_struct<D: Deserializer<'de>>(
        self,
        deserializer: D,
    ) -> Result<Self::Value, D::Error> {
        let source = <&'de str>::deserialize(deserializer)?;
        Ok(JsonArgument::Value(super::JsonView::from_validated(source)))
    }
}

impl<'a> From<&'a str> for JsonArgument<&'a str> {
    fn from(text: &'a str) -> Self {
        if super::JsonDecoder::validate_value(text).is_ok() {
            Self::Value(super::JsonView::from_validated(text))
        } else {
            Self::Text(text)
        }
    }
}

impl From<String> for JsonArgument {
    fn from(text: String) -> Self {
        if super::JsonDecoder::validate_value(&text).is_ok() {
            Self::Value(super::JsonView::from_validated(text))
        } else {
            Self::Text(text)
        }
    }
}

impl<S> JsonArgument<S> {
    pub fn map_text<T>(self, mut convert: impl FnMut(S) -> T) -> JsonArgument<T> {
        match self {
            Self::Value(value) => JsonArgument::Value(value.map_text(convert)),
            Self::Text(text) => JsonArgument::Text(convert(text)),
        }
    }
}

impl<S: AsRef<str>> AsRef<str> for JsonArgument<S> {
    fn as_ref(&self) -> &str {
        match self {
            Self::Value(value) => value.as_ref(),
            Self::Text(text) => text.as_ref(),
        }
    }
}

impl<S: AsRef<str>> Serialize for JsonArgument<S> {
    fn serialize<T: Serializer>(&self, serializer: T) -> Result<T::Ok, T::Error> {
        serializer.serialize_str(self.as_ref())
    }
}

impl<S: AsRef<str>> JsonArgument<S> {}

struct Seed<'a>(&'a mut Vec<Token>, &'a mut String, usize);

impl Seed<'_> {
    fn text(&mut self, value: &str) -> std::ops::Range<usize> {
        let start = self.1.len();
        self.1.push_str(value);
        start..self.1.len()
    }
}

impl<'de> DeserializeSeed<'de> for Seed<'_> {
    type Value = ();

    fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<(), D::Error> {
        deserializer.deserialize_any(self)
    }
}

impl<'de> Visitor<'de> for Seed<'_> {
    type Value = ();

    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("a JSON value")
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<(), A::Error> {
        if self.2 == 64 {
            return Err(serde::de::Error::custom("JSON nesting limit exceeded"));
        }
        self.0.push(Token::ObjectStart);
        while let Some(key) = map.next_key_seed(Key(self.1))? {
            self.0.push(Token::Key(key));
            map.next_value_seed(Seed(self.0, self.1, self.2 + 1))?;
        }
        self.0.push(Token::ObjectEnd);
        Ok(())
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<(), A::Error> {
        if self.2 == 64 {
            return Err(serde::de::Error::custom("JSON nesting limit exceeded"));
        }
        self.0.push(Token::ArrayStart);
        while seq
            .next_element_seed(Seed(self.0, self.1, self.2 + 1))?
            .is_some()
        {}
        self.0.push(Token::ArrayEnd);
        Ok(())
    }

    fn visit_borrowed_str<E: serde::de::Error>(self, value: &'de str) -> Result<(), E> {
        self.visit_str(value)
    }

    fn visit_str<E: serde::de::Error>(mut self, value: &str) -> Result<(), E> {
        let value = self.text(value);
        self.0.push(Token::String(value));
        Ok(())
    }

    fn visit_i64<E: serde::de::Error>(self, value: i64) -> Result<(), E> {
        self.0.push(Token::Number(value.into()));
        Ok(())
    }

    fn visit_u64<E: serde::de::Error>(self, value: u64) -> Result<(), E> {
        self.0.push(Token::Number(value.into()));
        Ok(())
    }

    fn visit_f64<E: serde::de::Error>(self, value: f64) -> Result<(), E> {
        self.0.push(Token::Number(
            serde_json::Number::from_f64(value).ok_or_else(|| E::custom("invalid JSON number"))?,
        ));
        Ok(())
    }

    fn visit_bool<E: serde::de::Error>(self, value: bool) -> Result<(), E> {
        self.0.push(Token::Boolean(value));
        Ok(())
    }

    fn visit_unit<E: serde::de::Error>(self) -> Result<(), E> {
        self.0.push(Token::Null);
        Ok(())
    }
}

struct Key<'a>(&'a mut String);

impl<'de> DeserializeSeed<'de> for Key<'_> {
    type Value = std::ops::Range<usize>;

    fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
        deserializer.deserialize_str(self)
    }
}

impl<'de> Visitor<'de> for Key<'_> {
    type Value = std::ops::Range<usize>;

    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("a JSON object key")
    }

    fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
        let start = self.0.len();
        self.0.push_str(value);
        Ok(start..self.0.len())
    }
}

impl<S: AsRef<str>> Serialize for JsonValue<S> {
    fn serialize<T: Serializer>(&self, serializer: T) -> Result<T::Ok, T::Error> {
        Value {
            tokens: &self.tokens,
            text: &self.text,
            position: &Cell::new(0),
        }
        .serialize(serializer)
    }
}

struct Value<'a> {
    tokens: &'a [Token],
    text: &'a str,
    position: &'a Cell<usize>,
}

impl Serialize for Value<'_> {
    fn serialize<T: Serializer>(&self, serializer: T) -> Result<T::Ok, T::Error> {
        let token = self
            .tokens
            .get(self.position.get())
            .ok_or_else(|| serde::ser::Error::custom("incomplete JSON tokens"))?;
        self.position.set(self.position.get() + 1);
        match token {
            Token::ObjectStart => {
                let mut map = serializer.serialize_map(None)?;
                loop {
                    match self.tokens.get(self.position.get()) {
                        Some(Token::ObjectEnd) => break,
                        Some(Token::Key(key)) => {
                            self.position.set(self.position.get() + 1);
                            map.serialize_entry(&self.text[key.clone()], self)?;
                        }
                        _ => return Err(serde::ser::Error::custom("invalid JSON object tokens")),
                    }
                }
                self.position.set(self.position.get() + 1);
                map.end()
            }
            Token::ArrayStart => {
                let mut seq = serializer.serialize_seq(None)?;
                while !matches!(self.tokens.get(self.position.get()), Some(Token::ArrayEnd)) {
                    seq.serialize_element(self)?;
                }
                self.position.set(self.position.get() + 1);
                seq.end()
            }
            Token::String(value) => serializer.serialize_str(&self.text[value.clone()]),
            Token::Number(value) => value.serialize(serializer),
            Token::Boolean(value) => serializer.serialize_bool(*value),
            Token::Null => serializer.serialize_none(),
            _ => Err(serde::ser::Error::custom("invalid JSON value tokens")),
        }
    }
}
