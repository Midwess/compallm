use super::{ComponentDeserializer, ToolType, deserialize};
use serde_json::Value;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ToolChoice {
    Auto,
    None,
    Required,
    Function(String),
}

deserialize!(ToolChoice, value, {
    if let Value::String(choice) = value {
        return match choice.as_str() {
            "auto" => Ok(Self::Auto),
            "none" => Ok(Self::None),
            "required" => Ok(Self::Required),
            _ => Err(ComponentDeserializer::invalid()),
        };
    }
    let mut map = ComponentDeserializer::object(value)?;
    let _: ToolType = ComponentDeserializer::required(&mut map, "type")?;
    let mut function =
        ComponentDeserializer::object(ComponentDeserializer::required(&mut map, "function")?)?;
    let name =
        ComponentDeserializer::nonempty(ComponentDeserializer::required(&mut function, "name")?)?;
    ComponentDeserializer::done(function)?;
    ComponentDeserializer::done(map)?;
    Ok(Self::Function(name))
});
