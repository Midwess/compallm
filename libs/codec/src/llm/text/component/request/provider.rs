use super::{ComponentDeserializer, deserialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Provider {
    RequireParameter(bool),
}

deserialize!(Provider, value, {
    let mut map = ComponentDeserializer::object(value)?;
    if map.contains_key("require_parameters") {
        if map.contains_key("require_parameter") {
            return Err(ComponentDeserializer::invalid());
        }
        let value = map
            .remove("require_parameters")
            .ok_or_else(ComponentDeserializer::invalid)?;
        map.insert("require_parameter".into(), value);
    }
    let required = ComponentDeserializer::required(&mut map, "require_parameter")?;
    ComponentDeserializer::done(map)?;
    Ok(Self::RequireParameter(required))
});
