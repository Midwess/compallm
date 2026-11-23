use crate::llm::text::component::Stop;
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::DeserializeAs;

#[derive(Serialize, Deserialize)]
#[serde(remote = "Stop", untagged)]
pub(crate) enum StopWire {
    Null,
    Text(String),
    Array(Vec<String>),
}

impl<'de> DeserializeAs<'de, Stop> for StopWire {
    fn deserialize_as<D: Deserializer<'de>>(deserializer: D) -> Result<Stop, D::Error> {
        let value = Self::deserialize(deserializer)?;
        let valid = match &value {
            Stop::Null => true,
            Stop::Text(text) => !text.is_empty() && text.len() <= 4096,
            Stop::Array(values) => {
                values.len() <= 4
                    && values.iter().all(|v| !v.is_empty())
                    && values.iter().map(String::len).sum::<usize>() <= 4096
            }
        };
        if !valid {
            return Err(serde::de::Error::custom("invalid stop sequences"));
        }
        Ok(value)
    }
}
