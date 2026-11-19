use super::Decoded;
use crate::llm::text::component::Stop;
use serde::Deserialize;

impl<'de> Deserialize<'de> for Decoded<Stop> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::Error;
        CompatibleOpenAIStop::deserialize(deserializer)?
            .try_into()
            .map(Self)
            .map_err(D::Error::custom)
    }
}

#[derive(serde::Deserialize)]
#[serde(untagged)]
pub(super) enum CompatibleOpenAIStop {
    Null,
    Text(String),
    Array(Vec<String>),
}

impl TryFrom<CompatibleOpenAIStop> for Stop {
    type Error = &'static str;

    fn try_from(value: CompatibleOpenAIStop) -> Result<Self, Self::Error> {
        match value {
            CompatibleOpenAIStop::Null => Ok(Self::Null),
            CompatibleOpenAIStop::Text(text) if !text.is_empty() && text.len() <= 4096 => {
                Ok(Self::Text(text))
            }
            CompatibleOpenAIStop::Array(stops)
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
