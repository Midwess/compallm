#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ResponseFormat<S = String> {
    Text,
    JsonObject,
    JsonSchema(JsonSchema<S>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JsonSchema<S = String> {
    pub name: S,
    pub description: Option<S>,
    pub strict: Option<bool>,
    pub schema: crate::json::JsonValue<S>,
}

impl<S> ResponseFormat<S> {
    pub fn map_text<T>(self, mut convert: impl FnMut(S) -> T) -> ResponseFormat<T> {
        match self {
            Self::Text => ResponseFormat::Text,
            Self::JsonObject => ResponseFormat::JsonObject,
            Self::JsonSchema(value) => ResponseFormat::JsonSchema(JsonSchema {
                name: convert(value.name),
                description: value.description.map(&mut convert),
                strict: value.strict,
                schema: value.schema.map_text(&mut convert),
            }),
        }
    }
}
