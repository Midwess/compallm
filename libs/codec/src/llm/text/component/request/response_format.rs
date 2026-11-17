use super::ComponentDeserializeHelper;

#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize)]
#[serde(try_from = "serde_json::Value")]
pub enum ResponseFormat {
    Text,
    JsonObject,
    JsonSchema(JsonSchema),
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize)]
#[serde(try_from = "serde_json::Value")]
pub struct JsonSchema {
    pub name: String,
    pub description: Option<String>,
    pub strict: Option<bool>,
    pub schema: String,
}

impl TryFrom<serde_json::Value> for JsonSchema {
    type Error = serde_json::Error;

    fn try_from(value: serde_json::Value) -> super::Result<Self> {
        let mut map = Self::object(value)?;
        let name = Self::nonempty(Self::required(&mut map, "name")?)?;
        let description = Self::optional(&mut map, "description")?;
        let strict = Self::optional(&mut map, "strict")?;
        let schema = Self::schema(Self::required(&mut map, "schema")?)?;
        Self::done(map)?;
        Ok(Self {
            name,
            description,
            strict,
            schema,
        })
    }
}
impl TryFrom<serde_json::Value> for ResponseFormat {
    type Error = serde_json::Error;

    fn try_from(value: serde_json::Value) -> super::Result<Self> {
        let mut map = Self::object(value)?;
        let kind: String = Self::required(&mut map, "type")?;
        let format = match kind.as_str() {
            "text" => Self::Text,
            "json_object" => Self::JsonObject,
            "json_schema" => Self::JsonSchema(Self::required(&mut map, "json_schema")?),
            _ => return Err(Self::invalid()),
        };
        Self::done(map)?;
        Ok(format)
    }
}

impl ComponentDeserializeHelper for ResponseFormat {}
impl ComponentDeserializeHelper for JsonSchema {}
