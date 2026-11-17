use super::{ComponentDeserializeHelper, ToolType};
use serde_json::Value;

#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize)]
#[serde(try_from = "serde_json::Value")]
pub enum ToolChoice {
    Auto,
    None,
    Required,
    Function(String),
}

impl TryFrom<Value> for ToolChoice {
    type Error = serde_json::Error;

    fn try_from(value: Value) -> super::Result<Self> {
        if let Value::String(choice) = value {
            return match choice.as_str() {
                "auto" => Ok(Self::Auto),
                "none" => Ok(Self::None),
                "required" => Ok(Self::Required),
                _ => Err(Self::invalid()),
            };
        }
        let mut map = Self::object(value)?;
        let _: ToolType = Self::required(&mut map, "type")?;
        let mut function = Self::object(Self::required(&mut map, "function")?)?;
        let name = Self::nonempty(Self::required(&mut function, "name")?)?;
        Self::done(function)?;
        Self::done(map)?;
        Ok(Self::Function(name))
    }
}

impl ComponentDeserializeHelper for ToolChoice {}
