use super::Decoded;
use crate::llm::text::component::Stop;
use serde::Deserialize;

impl<'de> Deserialize<'de> for Decoded<Stop> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::Error;
        StopFields::deserialize(deserializer)?
            .try_into()
            .map(Self)
            .map_err(D::Error::custom)
    }
}

#[derive(serde::Deserialize)]
#[serde(untagged)]
pub(super) enum StopFields {
    Null,
    Text(String),
    Array(Vec<String>),
}

impl TryFrom<StopFields> for Stop {
    type Error = &'static str;

    fn try_from(value: StopFields) -> Result<Self, Self::Error> {
        match value {
            StopFields::Null => Ok(Self::Null),
            StopFields::Text(text) if !text.is_empty() && text.len() <= 4096 => {
                Ok(Self::Text(text))
            }
            StopFields::Array(stops)
                if stops.len() <= 4
                    && stops.iter().all(|text| !text.is_empty())
                    && stops.iter().map(String::len).sum::<usize>() <= 4096 =>
            {
                Ok(Self::Array(stops))
            }
            _ => Err("invalid stop sequences"),
        }
    }
}
