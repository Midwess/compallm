use super::JsonDecodeError;

#[derive(Clone, Copy, Debug)]
pub struct JsonView<S = String> {
    source: S,
    validated: bool,
}

impl<S: PartialEq> PartialEq for JsonView<S> {
    fn eq(&self, other: &Self) -> bool {
        self.source == other.source
    }
}

impl<S: Eq> Eq for JsonView<S> {}

impl<'a> JsonView<&'a str> {
    pub fn deserialize_collection<'de: 'a, D, V>(
        deserializer: D,
        validation: V,
    ) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
        V: serde::de::DeserializeSeed<'de, Value = ()>,
    {
        struct CollectionVisitor<V>(V);
        impl<'de, V: serde::de::DeserializeSeed<'de, Value = ()>> serde::de::Visitor<'de>
            for CollectionVisitor<V>
        {
            type Value = JsonView<&'de str>;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a validated collection")
            }
            fn visit_seq<Q: serde::de::SeqAccess<'de>>(
                self,
                mut seq: Q,
            ) -> Result<Self::Value, Q::Error> {
                use serde::de::Error;
                let source = seq
                    .next_element::<&'de str>()?
                    .ok_or_else(|| Q::Error::custom("missing collection source"))?;
                seq.next_element_seed(self.0)?
                    .ok_or_else(|| Q::Error::custom("missing collection items"))?;
                Ok(JsonView::new(source))
            }
        }
        deserializer
            .deserialize_newtype_struct("codec::json::collection", CollectionVisitor(validation))
    }

    pub fn deserialize_array<'de: 'a, D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Self, D::Error> {
        deserializer.deserialize_newtype_struct(JSON_ARRAY_VIEW, ArrayVisitor)
    }
    pub(super) fn new(source: &'a str) -> Self {
        Self {
            source: source.trim_ascii(),
            validated: true,
        }
    }

    pub fn as_str(self) -> &'a str {
        self.source
    }

    pub fn members(self) -> Option<ObjectMembers<'a>> {
        Some(ObjectMembers {
            remaining: self.source.strip_prefix('{')?.strip_suffix('}')?,
        })
    }

    pub fn items(self) -> Option<ArrayItems<'a>> {
        Some(ArrayItems {
            remaining: self.source.strip_prefix('[')?.strip_suffix(']')?,
        })
    }

    pub fn decode_string<'b>(&'b self, output: &'b mut [u8]) -> Result<&'b str, JsonDecodeError> {
        compallm_serde_rules::string::decode(self.source.as_bytes(), output)
            .map_err(|_| JsonDecodeError::InvalidToken)
    }
}

impl<S: AsRef<str>> AsRef<str> for JsonView<S> {
    fn as_ref(&self) -> &str {
        self.source.as_ref()
    }
}

impl<S: AsRef<str>> JsonView<S> {
    pub fn borrowed(&self) -> JsonView<&str> {
        JsonView {
            source: self.source.as_ref(),
            validated: self.validated,
        }
    }
}

#[derive(Debug)]
pub struct ObjectMembers<'a> {
    remaining: &'a str,
}

impl<'a> Iterator for ObjectMembers<'a> {
    type Item = (JsonView<&'a str>, JsonView<&'a str>);

    fn next(&mut self) -> Option<Self::Item> {
        let key = take_value(&mut self.remaining)?;
        self.remaining = self.remaining.trim_ascii_start().strip_prefix(':')?;
        let value = take_value(&mut self.remaining)?;
        self.remaining = self.remaining.trim_ascii_start();
        if let Some(rest) = self.remaining.strip_prefix(',') {
            self.remaining = rest;
        }
        Some((JsonView::new(key), JsonView::new(value)))
    }
}

#[derive(Debug)]
pub struct ArrayItems<'a> {
    remaining: &'a str,
}

impl<'a> Iterator for ArrayItems<'a> {
    type Item = JsonView<&'a str>;

    fn next(&mut self) -> Option<Self::Item> {
        let value = take_value(&mut self.remaining)?;
        self.remaining = self.remaining.trim_ascii_start();
        if let Some(rest) = self.remaining.strip_prefix(',') {
            self.remaining = rest;
        }
        Some(JsonView::new(value))
    }
}

fn take_value<'a>(remaining: &mut &'a str) -> Option<&'a str> {
    let source = remaining.trim_ascii_start();
    let first = *source.as_bytes().first()?;
    let container = matches!(first, b'{' | b'[');
    let mut depth = 0usize;
    let mut string = false;
    let mut escaped = false;
    for (position, byte) in source.bytes().enumerate() {
        if string {
            if escaped {
                escaped = false;
            } else if byte == b'\\' {
                escaped = true;
            } else if byte == b'"' {
                string = false;
                if !container {
                    let end = position + 1;
                    *remaining = &source[end..];
                    return Some(&source[..end]);
                }
            }
            continue;
        }
        match byte {
            b'"' => string = true,
            b'{' | b'[' => depth += 1,
            b'}' | b']' if container => {
                depth -= 1;
                if depth == 0 {
                    let end = position + 1;
                    *remaining = &source[end..];
                    return Some(&source[..end]);
                }
            }
            b',' | b':' | b' ' | b'\t' | b'\r' | b'\n' if !container => {
                *remaining = &source[position..];
                return Some(&source[..position]);
            }
            _ => {}
        }
    }
    *remaining = "";
    Some(source)
}

impl<S> JsonView<S> {
    pub(super) fn from_validated(source: S) -> Self {
        Self {
            source,
            validated: true,
        }
    }
    pub fn map_text<T>(self, mut convert: impl FnMut(S) -> T) -> JsonView<T> {
        JsonView {
            source: convert(self.source),
            validated: false,
        }
    }
}

impl<S: AsRef<str>> serde::Serialize for JsonView<S> {
    fn serialize<T: serde::Serializer>(&self, serializer: T) -> Result<T::Ok, T::Error> {
        if self.validated {
            compallm_serde_rules::serialize::serialize_validated_raw(self.as_ref(), serializer)
        } else {
            compallm_serde_rules::serialize::serialize_raw(self, serializer)
        }
    }
}

impl<'a> TryFrom<&'a str> for JsonView<&'a str> {
    type Error = JsonDecodeError;
    fn try_from(source: &'a str) -> Result<Self, Self::Error> {
        super::JsonDecoder::validate_value(source)?;
        Ok(Self::new(source))
    }
}

impl TryFrom<String> for JsonView {
    type Error = JsonDecodeError;
    fn try_from(source: String) -> Result<Self, Self::Error> {
        super::JsonDecoder::validate_value(&source)?;
        Ok(Self {
            source,
            validated: true,
        })
    }
}

impl<'de: 'a, 'a> serde::Deserialize<'de> for JsonView<&'a str> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_newtype_struct(JSON_VIEW, ViewVisitor)
    }
}

pub(super) const JSON_VIEW: &str = "codec::json::view";
pub(super) const JSON_ARRAY_VIEW: &str = "codec::json::array_view";

struct ArrayVisitor;

impl<'de> serde::de::Visitor<'de> for ArrayVisitor {
    type Value = JsonView<&'de str>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("a validated JSON array view")
    }

    fn visit_borrowed_str<E: serde::de::Error>(self, source: &'de str) -> Result<Self::Value, E> {
        let view = JsonView::new(source);
        if view.items().is_none() {
            return Err(E::custom("expected JSON array"));
        }
        Ok(view)
    }
}

struct ViewVisitor;
impl<'de> serde::de::Visitor<'de> for ViewVisitor {
    type Value = JsonView<&'de str>;
    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("a validated JSON object view")
    }
    fn visit_borrowed_str<E: serde::de::Error>(self, source: &'de str) -> Result<Self::Value, E> {
        let view = JsonView::new(source);
        if view.members().is_none() {
            return Err(E::custom("expected JSON object"));
        }
        Ok(view)
    }
}
