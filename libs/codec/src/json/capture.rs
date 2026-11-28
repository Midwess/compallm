use super::value::Token;
use super::{JsonToken, JsonValue};
use serde::de::{DeserializeSeed, EnumAccess, MapAccess, SeqAccess, VariantAccess, Visitor};
use serde::{Deserialize, Deserializer};
use std::ops::Range;

#[derive(Debug)]
pub(super) struct Capture {
    pub(super) entries: Vec<Entry>,
    pub(super) text: String,
}

#[derive(Debug)]
pub(super) struct Entry {
    pub(super) token: Token,
    source: Range<usize>,
    borrowed: bool,
}

impl Default for Capture {
    fn default() -> Self {
        let bytes = 4096;
        Self {
            entries: Vec::with_capacity(bytes / std::mem::size_of::<Entry>()),
            text: String::with_capacity(bytes),
        }
    }
}

impl Capture {
    pub(super) fn clear(&mut self) {
        self.entries.clear();
        self.text.clear();
    }

    fn text(&mut self, value: &str, input: &[u8], offset: usize) -> (Range<usize>, bool) {
        if let super::StringReference::Input(range) = super::StringReference::new(value, input) {
            return (offset + range.start..offset + range.end, true);
        }
        let start = self.text.len();
        self.text.push_str(value);
        (start..self.text.len(), false)
    }

    pub(super) fn push(
        &mut self,
        token: JsonToken<'_>,
        source: Range<usize>,
        nested: bool,
        input: &[u8],
        offset: usize,
    ) {
        if nested && let Some(key) = token.key() {
            let (key, borrowed) = self.text(key, input, offset);
            self.entries.push(Entry {
                token: Token::Key(key),
                source: source.clone(),
                borrowed,
            });
        }
        let mut borrowed = false;
        let token = match token {
            JsonToken::ObjectStart(_) => Token::ObjectStart,
            JsonToken::ObjectEnd => Token::ObjectEnd,
            JsonToken::ArrayStart(_) => Token::ArrayStart,
            JsonToken::ArrayEnd => Token::ArrayEnd,
            JsonToken::ValueString(_, value) => {
                let (range, source) = self.text(value, input, offset);
                borrowed = source;
                Token::String(range)
            }
            JsonToken::ValueNumber(_, value) => Token::Number(value),
            JsonToken::ValueBoolean(_, value) => Token::Boolean(value),
            JsonToken::ValueNull(_) => Token::Null,
        };
        self.entries.push(Entry {
            token,
            source,
            borrowed,
        });
    }

    pub(super) fn into_value<S: AsRef<str>>(mut self, source: S) -> JsonValue<S> {
        for entry in &mut self.entries {
            if !entry.borrowed {
                continue;
            }
            let range = match &mut entry.token {
                Token::Key(range) | Token::String(range) => range,
                _ => unreachable!(),
            };
            let start = self.text.len();
            self.text.push_str(&source.as_ref()[range.clone()]);
            *range = start..self.text.len();
        }
        JsonValue::from_capture(
            source,
            self.entries.into_iter().map(|entry| entry.token),
            self.text,
        )
    }

    pub(super) fn deserialize<'de, T: Deserialize<'de>>(
        &'de self,
        source: &'de str,
        inner: &mut super::decoder::JsonParser,
    ) -> Result<T, serde_json::Error> {
        let mut decoder = TokenDeserializer {
            capture: self,
            source,
            position: 0,
            borrowed_text: Some(&self.text),
            inner: Some(inner),
        };
        let value = T::deserialize(&mut decoder)?;
        if decoder.position != self.entries.len() {
            return Err(serde::de::Error::custom("unconsumed JSON tokens"));
        }
        Ok(value)
    }
}

struct TokenDeserializer<'tokens, 'de> {
    capture: &'tokens Capture,
    source: &'de str,
    position: usize,
    borrowed_text: Option<&'de str>,
    inner: Option<&'tokens mut super::decoder::JsonParser>,
}

enum TokenText<'tokens, 'de> {
    Borrowed(&'de str),
    Transient(&'tokens str),
}

impl<'de> Deserializer<'de> for TokenText<'_, 'de> {
    type Error = serde_json::Error;

    fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        match self {
            Self::Borrowed(text) => visitor.visit_borrowed_str(text),
            Self::Transient(text) => visitor.visit_str(text),
        }
    }

    fn deserialize_enum<V: Visitor<'de>>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        match self {
            Self::Borrowed(text) => {
                serde::de::value::BorrowedStrDeserializer::<Self::Error>::new(text)
                    .deserialize_enum(name, variants, visitor)
            }
            Self::Transient(text) => serde::de::value::StrDeserializer::<Self::Error>::new(text)
                .deserialize_enum(name, variants, visitor),
        }
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string bytes byte_buf
        option unit unit_struct newtype_struct seq tuple tuple_struct map struct identifier ignored_any
    }
}

impl<'tokens, 'de> TokenDeserializer<'tokens, 'de> {
    fn peek(&self) -> Result<&'tokens Entry, serde_json::Error> {
        self.capture
            .entries
            .get(self.position)
            .ok_or_else(|| serde::de::Error::custom("incomplete JSON tokens"))
    }

    fn end(&self) -> Result<usize, serde_json::Error> {
        let mut position = self.position;
        let mut depth = 0usize;
        loop {
            let entry = self
                .capture
                .entries
                .get(position)
                .ok_or_else(|| serde::de::Error::custom("incomplete JSON tokens"))?;
            match entry.token {
                Token::ObjectStart | Token::ArrayStart => depth += 1,
                Token::ObjectEnd | Token::ArrayEnd => depth -= 1,
                _ => {}
            }
            position += 1;
            if depth == 0 {
                return Ok(position);
            }
        }
    }

    fn raw(&self) -> Result<&'de str, serde_json::Error> {
        let start = self.peek()?.source.start;
        let end = self.capture.entries[self.end()? - 1].source.end;
        self.source
            .get(start..end)
            .ok_or_else(|| serde::de::Error::custom("invalid JSON source range"))
    }

    fn text(&self, entry: &Entry, range: Range<usize>) -> TokenText<'_, 'de> {
        if entry.borrowed {
            TokenText::Borrowed(&self.source[range])
        } else if let Some(text) = self.borrowed_text {
            TokenText::Borrowed(&text[range])
        } else {
            TokenText::Transient(&self.capture.text[range])
        }
    }

    fn embedded<V: Visitor<'de>>(
        &mut self,
        text: &'de str,
        visitor: V,
        argument: bool,
    ) -> Result<V::Value, serde_json::Error> {
        let parser = self
            .inner
            .as_deref_mut()
            .ok_or_else(|| serde::de::Error::custom("missing embedded JSON parser"))?;
        let capture = match parser.capture_value(text) {
            Ok(capture) => capture,
            Err(_) if argument => return visitor.visit_borrowed_str(text),
            Err(error) => return Err(serde::de::Error::custom(error)),
        };
        let mut decoder = TokenDeserializer {
            capture,
            source: text,
            position: 0,
            // Reused inner scratch must be copied by visitors, never borrowed into a result.
            borrowed_text: None,
            inner: None,
        };
        let value = visitor.visit_seq(ValueSequence {
            decoder: &mut decoder,
            source: Some(text),
            finished: false,
        })?;
        if decoder.position != capture.entries.len() {
            return Err(serde::de::Error::custom("unconsumed embedded JSON tokens"));
        }
        Ok(value)
    }
}

impl<'de> Deserializer<'de> for &mut TokenDeserializer<'_, 'de> {
    type Error = serde_json::Error;

    fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        let entry = self.peek()?;
        self.position += 1;
        match &entry.token {
            Token::ObjectStart => {
                let value = visitor.visit_map(TokenMap(self))?;
                if !matches!(self.peek()?.token, Token::ObjectEnd) {
                    return Err(serde::de::Error::custom("unconsumed JSON object tokens"));
                }
                self.position += 1;
                Ok(value)
            }
            Token::ArrayStart => {
                let value = visitor.visit_seq(TokenSequence(self))?;
                if !matches!(self.peek()?.token, Token::ArrayEnd) {
                    return Err(serde::de::Error::custom("unconsumed JSON array tokens"));
                }
                self.position += 1;
                Ok(value)
            }
            Token::String(range) | Token::Key(range) => {
                self.text(entry, range.clone()).deserialize_any(visitor)
            }
            Token::Number(value) => {
                if let Some(value) = value.as_u64() {
                    visitor.visit_u64(value)
                } else if let Some(value) = value.as_i64() {
                    visitor.visit_i64(value)
                } else {
                    visitor.visit_f64(
                        value
                            .as_f64()
                            .ok_or_else(|| serde::de::Error::custom("invalid JSON number"))?,
                    )
                }
            }
            Token::Boolean(value) => visitor.visit_bool(*value),
            Token::Null => visitor.visit_unit(),
            _ => Err(serde::de::Error::custom("unexpected JSON boundary")),
        }
    }

    fn deserialize_option<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        if matches!(self.peek()?.token, Token::Null) {
            self.position += 1;
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    fn deserialize_newtype_struct<V: Visitor<'de>>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        if name == "codec::json::collection" {
            if !matches!(self.peek()?.token, Token::ArrayStart) {
                return Err(serde::de::Error::custom("expected collection array"));
            }
            let source = self.raw()?;
            return visitor.visit_seq(ValueSequence {
                decoder: self,
                source: Some(source),
                finished: false,
            });
        }
        if name == super::view::JSON_ARRAY_VIEW {
            if !matches!(self.peek()?.token, Token::ArrayStart) {
                return Err(serde::de::Error::custom("expected JSON array"));
            }
            let source = self.raw()?;
            self.position = self.end()?;
            return visitor.visit_borrowed_str(source);
        }
        if name == super::view::JSON_VIEW || name == super::value::JSON_ARGUMENT {
            let entry = self.peek()?;
            if let Token::String(range) = &entry.token {
                self.position += 1;
                let TokenText::Borrowed(text) = self.text(entry, range.clone()) else {
                    return Err(serde::de::Error::custom(
                        "embedded source must outlive its parser",
                    ));
                };
                let parser = self
                    .inner
                    .as_deref_mut()
                    .ok_or_else(|| serde::de::Error::custom("missing embedded JSON parser"))?;
                match parser.capture_value(text) {
                    Ok(_) if name == super::value::JSON_ARGUMENT => {
                        return visitor.visit_newtype_struct(
                            serde::de::value::BorrowedStrDeserializer::<Self::Error>::new(text),
                        );
                    }
                    Ok(capture)
                        if matches!(
                            capture.entries.first().map(|entry| &entry.token),
                            Some(Token::ObjectStart)
                        ) =>
                    {
                        return visitor.visit_borrowed_str(text);
                    }
                    Err(_) if name == super::value::JSON_ARGUMENT => {
                        return visitor.visit_borrowed_str(text);
                    }
                    _ => return Err(serde::de::Error::custom("expected valid JSON object")),
                }
            }
            if name == super::value::JSON_ARGUMENT {
                return Err(serde::de::Error::custom("expected argument string"));
            }
            if !matches!(entry.token, Token::ObjectStart) {
                return Err(serde::de::Error::custom("expected JSON object"));
            }
            let source = self.raw()?;
            self.position = self.end()?;
            return visitor.visit_borrowed_str(source);
        }
        if name == super::value::JSON_VALUE {
            let entry = self.peek()?;
            if let Token::String(range) = &entry.token {
                self.position += 1;
                let TokenText::Borrowed(text) = self.text(entry, range.clone()) else {
                    return Err(serde::de::Error::custom(
                        "embedded source must outlive its parser",
                    ));
                };
                return self.embedded(text, visitor, false);
            }
            let source = self.raw()?;
            return visitor.visit_seq(ValueSequence {
                decoder: self,
                source: Some(source),
                finished: false,
            });
        }
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_enum<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        if matches!(self.peek()?.token, Token::ObjectStart) {
            self.position += 1;
            let value = visitor.visit_enum(TokenEnum(self))?;
            if !matches!(self.peek()?.token, Token::ObjectEnd) {
                return Err(serde::de::Error::custom("expected enum object end"));
            }
            self.position += 1;
            return Ok(value);
        }
        let entry = self.peek()?;
        let Token::String(range) = &entry.token else {
            return Err(serde::de::Error::custom("expected string enum"));
        };
        self.position += 1;
        self.text(entry, range.clone())
            .deserialize_enum(_name, _variants, visitor)
    }

    fn deserialize_ignored_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.position = self.end()?;
        visitor.visit_unit()
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string bytes byte_buf
        unit unit_struct seq tuple tuple_struct map struct identifier
    }
}

struct TokenEnum<'a, 'tokens, 'de>(&'a mut TokenDeserializer<'tokens, 'de>);

impl<'a, 'tokens, 'de> EnumAccess<'de> for TokenEnum<'a, 'tokens, 'de> {
    type Error = serde_json::Error;
    type Variant = Self;

    fn variant_seed<S: DeserializeSeed<'de>>(
        self,
        seed: S,
    ) -> Result<(S::Value, Self), Self::Error> {
        let entry = self.0.peek()?;
        let Token::Key(range) = &entry.token else {
            return Err(serde::de::Error::custom("expected enum variant"));
        };
        self.0.position += 1;
        let value = seed.deserialize(self.0.text(entry, range.clone()))?;
        Ok((value, self))
    }
}

impl<'de> VariantAccess<'de> for TokenEnum<'_, '_, 'de> {
    type Error = serde_json::Error;

    fn unit_variant(self) -> Result<(), Self::Error> {
        <()>::deserialize(&mut *self.0)
    }

    fn newtype_variant_seed<S: DeserializeSeed<'de>>(
        self,
        seed: S,
    ) -> Result<S::Value, Self::Error> {
        seed.deserialize(&mut *self.0)
    }

    fn tuple_variant<V: Visitor<'de>>(
        self,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        (&mut *self.0).deserialize_any(visitor)
    }

    fn struct_variant<V: Visitor<'de>>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        (&mut *self.0).deserialize_any(visitor)
    }
}

struct TokenMap<'a, 'tokens, 'de>(&'a mut TokenDeserializer<'tokens, 'de>);

impl<'de> MapAccess<'de> for TokenMap<'_, '_, 'de> {
    type Error = serde_json::Error;

    fn next_key_seed<S: DeserializeSeed<'de>>(
        &mut self,
        seed: S,
    ) -> Result<Option<S::Value>, Self::Error> {
        let entry = self.0.peek()?;
        match &entry.token {
            Token::ObjectEnd => Ok(None),
            Token::Key(range) => {
                self.0.position += 1;
                seed.deserialize(self.0.text(entry, range.clone()))
                    .map(Some)
            }
            _ => Err(serde::de::Error::custom("expected JSON object key")),
        }
    }

    fn next_value_seed<S: DeserializeSeed<'de>>(
        &mut self,
        seed: S,
    ) -> Result<S::Value, Self::Error> {
        seed.deserialize(&mut *self.0)
    }
}

struct TokenSequence<'a, 'tokens, 'de>(&'a mut TokenDeserializer<'tokens, 'de>);

impl<'de> SeqAccess<'de> for TokenSequence<'_, '_, 'de> {
    type Error = serde_json::Error;

    fn next_element_seed<S: DeserializeSeed<'de>>(
        &mut self,
        seed: S,
    ) -> Result<Option<S::Value>, Self::Error> {
        if matches!(self.0.peek()?.token, Token::ArrayEnd) {
            Ok(None)
        } else {
            seed.deserialize(&mut *self.0).map(Some)
        }
    }
}

struct ValueSequence<'a, 'tokens, 'de> {
    decoder: &'a mut TokenDeserializer<'tokens, 'de>,
    source: Option<&'de str>,
    finished: bool,
}

impl<'de> SeqAccess<'de> for ValueSequence<'_, '_, 'de> {
    type Error = serde_json::Error;

    fn next_element_seed<S: DeserializeSeed<'de>>(
        &mut self,
        seed: S,
    ) -> Result<Option<S::Value>, Self::Error> {
        if let Some(source) = self.source.take() {
            seed.deserialize(serde::de::value::BorrowedStrDeserializer::new(source))
                .map(Some)
        } else if !self.finished {
            self.finished = true;
            seed.deserialize(&mut *self.decoder).map(Some)
        } else {
            Ok(None)
        }
    }
}
