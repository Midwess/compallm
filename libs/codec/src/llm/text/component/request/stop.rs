use super::ComponentDeserializeHelper;
use serde_json::Value;

#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize)]
#[serde(try_from = "serde_json::Value")]
pub enum Stop {
    Null,
    Text(String),
    Array(Vec<String>),
}

impl TryFrom<Value> for Stop {
    type Error = serde_json::Error;

    fn try_from(value: Value) -> super::Result<Self> {
        match value {
            Value::Null => Ok(Self::Null),
            Value::String(text) if !text.is_empty() && text.len() <= 4096 => Ok(Self::Text(text)),
            value => {
                let stops: Vec<String> = serde_json::from_value(value)?;
                if stops.len() > 4
                    || stops.iter().any(String::is_empty)
                    || stops.iter().map(String::len).sum::<usize>() > 4096
                {
                    return Err(Self::invalid());
                }
                Ok(Self::Array(stops))
            }
        }
    }
}

impl ComponentDeserializeHelper for Stop {}
