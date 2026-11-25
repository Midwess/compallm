use serde::de::{self, DeserializeSeed, EnumAccess, MapAccess, SeqAccess, VariantAccess, Visitor};
use serde::{Deserialize, Deserializer};

pub fn from_slice<'de, T: Deserialize<'de>>(
    input: &'de [u8],
    mut scratch: &'de mut [u8],
) -> Result<T, serde_json::Error> {
    let mut json = serde_json::Deserializer::from_slice(input);
    let value = T::deserialize(Borrowing {
        inner: &mut json,
        scratch: &mut scratch,
    })?;
    json.end()?;
    Ok(value)
}

struct Borrowing<'a, 'de, D> {
    inner: D,
    scratch: &'a mut &'de mut [u8],
}

macro_rules! forward {
    ($($method:ident $(($($arg:ident: $ty:ty),*))?),* $(,)?) => {
        $(fn $method<V: Visitor<'de>>(self, $($($arg: $ty,)*)? visitor: V) -> Result<V::Value, Self::Error> {
            self.inner.$method($($($arg,)*)? Borrowing { inner: visitor, scratch: self.scratch })
        })*
    };
}

impl<'de, D: Deserializer<'de>> Deserializer<'de> for Borrowing<'_, 'de, D> {
    type Error = D::Error;

    forward! {
        deserialize_bool,
        deserialize_i8, deserialize_i16, deserialize_i32, deserialize_i64, deserialize_i128,
        deserialize_u8, deserialize_u16, deserialize_u32, deserialize_u64, deserialize_u128,
        deserialize_f32, deserialize_f64, deserialize_char,
        deserialize_bytes, deserialize_byte_buf, deserialize_option, deserialize_unit,
        deserialize_unit_struct(name: &'static str),
        deserialize_seq,
        deserialize_tuple(len: usize), deserialize_tuple_struct(name: &'static str, len: usize),
        deserialize_map, deserialize_struct(name: &'static str, fields: &'static [&'static str]),
        deserialize_enum(name: &'static str, variants: &'static [&'static str]),
        deserialize_identifier, deserialize_ignored_any,
    }

    fn is_human_readable(&self) -> bool {
        self.inner.is_human_readable()
    }

    fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        let raw = <&serde_json::value::RawValue>::deserialize(self.inner)?;
        if raw.get().starts_with('"') {
            return visitor.visit_borrowed_str(
                decode_string(raw.get().as_bytes(), self.scratch).map_err(de::Error::custom)?,
            );
        }
        let mut json = serde_json::Deserializer::from_str(raw.get());
        let value = json
            .deserialize_any(Borrowing {
                inner: visitor,
                scratch: self.scratch,
            })
            .map_err(de::Error::custom)?;
        json.end().map_err(de::Error::custom)?;
        Ok(value)
    }

    fn deserialize_str<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        let raw = <&serde_json::value::RawValue>::deserialize(self.inner)?;
        visitor.visit_borrowed_str(
            decode_string(raw.get().as_bytes(), self.scratch).map_err(de::Error::custom)?,
        )
    }

    fn deserialize_string<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_str(visitor)
    }

    fn deserialize_newtype_struct<V: Visitor<'de>>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        if name != super::SCHEMA {
            return self.inner.deserialize_newtype_struct(
                name,
                Borrowing {
                    inner: visitor,
                    scratch: self.scratch,
                },
            );
        }
        let raw = <&serde_json::value::RawValue>::deserialize(self.inner)?;
        match raw.get().as_bytes().first() {
            Some(b'{') => visitor.visit_borrowed_str(raw.get()),
            Some(b'"') => visitor.visit_borrowed_str(
                decode_string(raw.get().as_bytes(), self.scratch).map_err(de::Error::custom)?,
            ),
            _ => Err(de::Error::custom("expected schema object or string")),
        }
    }
}

fn decode_string<'de>(
    input: &'de [u8],
    scratch: &mut &'de mut [u8],
) -> Result<&'de str, serde_json::Error> {
    if !input.contains(&b'\\') {
        return crate::string::decode(input, &mut []);
    }
    if input.len() > scratch.len() {
        return Err(de::Error::custom("decoded string storage exhausted"));
    }
    let storage = std::mem::take(scratch);
    let (output, rest) = storage.split_at_mut(input.len());
    *scratch = rest;
    crate::string::decode(input, output)
}

macro_rules! scalar {
    ($($method:ident($ty:ty)),* $(,)?) => {
        $(fn $method<E: de::Error>(self, value: $ty) -> Result<Self::Value, E> {
            self.inner.$method(value)
        })*
    };
}

impl<'de, V: Visitor<'de>> Visitor<'de> for Borrowing<'_, 'de, V> {
    type Value = V::Value;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.expecting(formatter)
    }

    scalar! {
        visit_bool(bool), visit_i8(i8), visit_i16(i16), visit_i32(i32), visit_i64(i64), visit_i128(i128),
        visit_u8(u8), visit_u16(u16), visit_u32(u32), visit_u64(u64), visit_u128(u128),
        visit_f32(f32), visit_f64(f64), visit_char(char),
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
        if value.len() > self.scratch.len() {
            return Err(E::custom("decoded string storage exhausted"));
        }
        let storage = std::mem::take(self.scratch);
        let (text, rest) = storage.split_at_mut(value.len());
        text.copy_from_slice(value.as_bytes());
        *self.scratch = rest;
        self.inner
            .visit_borrowed_str(std::str::from_utf8(text).map_err(E::custom)?)
    }

    fn visit_string<E: de::Error>(self, value: String) -> Result<Self::Value, E> {
        self.visit_str(&value)
    }
    fn visit_borrowed_str<E: de::Error>(self, value: &'de str) -> Result<Self::Value, E> {
        self.inner.visit_borrowed_str(value)
    }
    fn visit_bytes<E: de::Error>(self, value: &[u8]) -> Result<Self::Value, E> {
        self.inner.visit_bytes(value)
    }
    fn visit_borrowed_bytes<E: de::Error>(self, value: &'de [u8]) -> Result<Self::Value, E> {
        self.inner.visit_borrowed_bytes(value)
    }
    fn visit_byte_buf<E: de::Error>(self, value: Vec<u8>) -> Result<Self::Value, E> {
        self.inner.visit_byte_buf(value)
    }
    fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> {
        self.inner.visit_none()
    }
    fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
        self.inner.visit_unit()
    }
    fn visit_some<D: Deserializer<'de>>(self, inner: D) -> Result<Self::Value, D::Error> {
        self.inner.visit_some(Borrowing {
            inner,
            scratch: self.scratch,
        })
    }
    fn visit_newtype_struct<D: Deserializer<'de>>(self, inner: D) -> Result<Self::Value, D::Error> {
        self.inner.visit_newtype_struct(Borrowing {
            inner,
            scratch: self.scratch,
        })
    }
    fn visit_seq<A: SeqAccess<'de>>(self, inner: A) -> Result<Self::Value, A::Error> {
        self.inner.visit_seq(Borrowing {
            inner,
            scratch: self.scratch,
        })
    }
    fn visit_map<A: MapAccess<'de>>(self, inner: A) -> Result<Self::Value, A::Error> {
        self.inner.visit_map(Borrowing {
            inner,
            scratch: self.scratch,
        })
    }
    fn visit_enum<A: EnumAccess<'de>>(self, inner: A) -> Result<Self::Value, A::Error> {
        self.inner.visit_enum(Borrowing {
            inner,
            scratch: self.scratch,
        })
    }
}

impl<'de, S: DeserializeSeed<'de>> DeserializeSeed<'de> for Borrowing<'_, 'de, S> {
    type Value = S::Value;
    fn deserialize<D: Deserializer<'de>>(self, inner: D) -> Result<Self::Value, D::Error> {
        self.inner.deserialize(Borrowing {
            inner,
            scratch: self.scratch,
        })
    }
}

impl<'de, A: SeqAccess<'de>> SeqAccess<'de> for Borrowing<'_, 'de, A> {
    type Error = A::Error;
    fn next_element_seed<S: DeserializeSeed<'de>>(
        &mut self,
        seed: S,
    ) -> Result<Option<S::Value>, Self::Error> {
        self.inner.next_element_seed(Borrowing {
            inner: seed,
            scratch: self.scratch,
        })
    }
    fn size_hint(&self) -> Option<usize> {
        self.inner.size_hint()
    }
}

impl<'de, A: MapAccess<'de>> MapAccess<'de> for Borrowing<'_, 'de, A> {
    type Error = A::Error;
    fn next_key_seed<S: DeserializeSeed<'de>>(
        &mut self,
        seed: S,
    ) -> Result<Option<S::Value>, Self::Error> {
        self.inner.next_key_seed(Borrowing {
            inner: seed,
            scratch: self.scratch,
        })
    }
    fn next_value_seed<S: DeserializeSeed<'de>>(
        &mut self,
        seed: S,
    ) -> Result<S::Value, Self::Error> {
        self.inner.next_value_seed(Borrowing {
            inner: seed,
            scratch: self.scratch,
        })
    }
    fn size_hint(&self) -> Option<usize> {
        self.inner.size_hint()
    }
}

impl<'a, 'de, A: EnumAccess<'de>> EnumAccess<'de> for Borrowing<'a, 'de, A> {
    type Error = A::Error;
    type Variant = Borrowing<'a, 'de, A::Variant>;
    fn variant_seed<S: DeserializeSeed<'de>>(
        self,
        seed: S,
    ) -> Result<(S::Value, Self::Variant), Self::Error> {
        let (value, inner) = self.inner.variant_seed(Borrowing {
            inner: seed,
            scratch: self.scratch,
        })?;
        Ok((
            value,
            Borrowing {
                inner,
                scratch: self.scratch,
            },
        ))
    }
}

impl<'de, A: VariantAccess<'de>> VariantAccess<'de> for Borrowing<'_, 'de, A> {
    type Error = A::Error;
    fn unit_variant(self) -> Result<(), Self::Error> {
        self.inner.unit_variant()
    }
    fn newtype_variant_seed<S: DeserializeSeed<'de>>(
        self,
        seed: S,
    ) -> Result<S::Value, Self::Error> {
        self.inner.newtype_variant_seed(Borrowing {
            inner: seed,
            scratch: self.scratch,
        })
    }
    fn tuple_variant<V: Visitor<'de>>(
        self,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.inner.tuple_variant(
            len,
            Borrowing {
                inner: visitor,
                scratch: self.scratch,
            },
        )
    }
    fn struct_variant<V: Visitor<'de>>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.inner.struct_variant(
            fields,
            Borrowing {
                inner: visitor,
                scratch: self.scratch,
            },
        )
    }
}
