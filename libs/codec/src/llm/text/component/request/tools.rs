use super::{ComponentDeserializer, deserialize};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ToolDefinition {
    pub kind: ToolType,
    pub function: FunctionDefinition,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FunctionDefinition {
    pub name: String,
    pub description: Option<String>,
    pub parameters: Option<String>,
    pub strict: Option<bool>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToolType {
    Function,
}

deserialize!(ToolType, value, {
    if value.as_str() == Some("function") {
        Ok(Self::Function)
    } else {
        Err(ComponentDeserializer::invalid())
    }
});
deserialize!(FunctionDefinition, value, {
    let mut map = ComponentDeserializer::object(value)?;
    let name = ComponentDeserializer::nonempty(ComponentDeserializer::required(&mut map, "name")?)?;
    let description = ComponentDeserializer::optional(&mut map, "description")?;
    let strict = ComponentDeserializer::optional(&mut map, "strict")?;
    let parameters = map
        .remove("parameters")
        .filter(|value| !value.is_null())
        .map(ComponentDeserializer::schema)
        .transpose()?;
    ComponentDeserializer::done(map)?;
    Ok(Self {
        name,
        description,
        strict,
        parameters,
    })
});
deserialize!(ToolDefinition, value, {
    let mut map = ComponentDeserializer::object(value)?;
    let kind = ComponentDeserializer::required(&mut map, "type")?;
    let function = ComponentDeserializer::required(&mut map, "function")?;
    ComponentDeserializer::done(map)?;
    Ok(Self { kind, function })
});
