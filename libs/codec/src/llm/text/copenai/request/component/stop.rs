use crate::llm::text::component::Stop;
use compallm_serde_rules::borrowed::Text;
use serde::{Deserialize, Deserializer, Serialize, de::Error};
use std::borrow::Cow;

#[derive(Serialize)]
#[serde(remote = "Stop", untagged)]
pub(crate) enum StopWire<S> {
    Null,
    Text(S),
    Array(Vec<S>),
}

pub(crate) struct Stops<'a>(pub Stop<Cow<'a, str>>);

impl<'de> Deserialize<'de> for Stops<'de> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Stops<'de>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("null, a stop string, or an array of stop strings")
            }

            fn visit_unit<E: Error>(self) -> Result<Self::Value, E> {
                Ok(Stops(Stop::Null))
            }

            fn visit_borrowed_str<E: Error>(self, value: &'de str) -> Result<Self::Value, E> {
                Ok(Stops(Stop::Text(Cow::Borrowed(value))))
            }

            fn visit_seq<A: serde::de::SeqAccess<'de>>(
                self,
                mut sequence: A,
            ) -> Result<Self::Value, A::Error> {
                let mut values = Vec::new();
                while let Some(Text(value)) = sequence.next_element::<Text<'de>>()? {
                    if values.len() == 4 {
                        return Err(A::Error::custom("invalid stop sequences"));
                    }
                    values.push(value);
                }
                Ok(Stops(Stop::Array(values)))
            }
        }
        let value = deserializer.deserialize_any(Visitor)?;
        let valid = match &value.0 {
            Stop::Null => true,
            Stop::Text(text) => !text.is_empty() && text.len() <= 4096,
            Stop::Array(texts) => {
                texts.iter().all(|text| !text.is_empty())
                    && texts.iter().map(|text| text.len()).sum::<usize>() <= 4096
            }
        };
        if !valid {
            return Err(D::Error::custom("invalid stop sequences"));
        }
        Ok(value)
    }
}
