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
