use super::{ComponentDeserializer, deserialize};
use serde_json::Value;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Stop {
    Null,
    Text(String),
    Array(Vec<String>),
}

deserialize!(Stop, value, {
    match value {
        Value::Null => Ok(Self::Null),
        Value::String(text) if !text.is_empty() && text.len() <= 4096 => Ok(Self::Text(text)),
        value => {
            let stops: Vec<String> = serde_json::from_value(value)?;
            if stops.len() > 4
                || stops.iter().any(String::is_empty)
                || stops.iter().map(String::len).sum::<usize>() > 4096
            {
                return Err(ComponentDeserializer::invalid());
            }
            Ok(Self::Array(stops))
        }
    }
});
