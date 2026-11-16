use super::{ComponentDeserializer, deserialize};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ResponseFormat {
    Text,
    JsonObject,
    JsonSchema(JsonSchema),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JsonSchema {
    pub name: String,
    pub description: Option<String>,
    pub strict: Option<bool>,
    pub schema: String,
}

deserialize!(JsonSchema, value, {
    let mut map = ComponentDeserializer::object(value)?;
    let name = ComponentDeserializer::nonempty(ComponentDeserializer::required(&mut map, "name")?)?;
    let description = ComponentDeserializer::optional(&mut map, "description")?;
    let strict = ComponentDeserializer::optional(&mut map, "strict")?;
    let schema =
        ComponentDeserializer::schema(ComponentDeserializer::required(&mut map, "schema")?)?;
    ComponentDeserializer::done(map)?;
    Ok(Self {
        name,
        description,
        strict,
        schema,
    })
});
deserialize!(ResponseFormat, value, {
    let mut map = ComponentDeserializer::object(value)?;
    let kind: String = ComponentDeserializer::required(&mut map, "type")?;
    let format = match kind.as_str() {
        "text" => Self::Text,
        "json_object" => Self::JsonObject,
        "json_schema" => {
            Self::JsonSchema(ComponentDeserializer::required(&mut map, "json_schema")?)
        }
        _ => return Err(ComponentDeserializer::invalid()),
    };
    ComponentDeserializer::done(map)?;
    Ok(format)
});
