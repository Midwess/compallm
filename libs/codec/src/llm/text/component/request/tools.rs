use super::ComponentDeserializeHelper;

#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize)]
#[serde(try_from = "serde_json::Value")]
pub struct ToolDefinition {
    pub kind: ToolType,
    pub function: FunctionDefinition,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize)]
#[serde(try_from = "serde_json::Value")]
pub struct FunctionDefinition {
    pub name: String,
    pub description: Option<String>,
    pub parameters: Option<String>,
    pub strict: Option<bool>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Deserialize)]
#[serde(try_from = "serde_json::Value")]
pub enum ToolType {
    Function,
}

impl TryFrom<serde_json::Value> for ToolType {
    type Error = serde_json::Error;

    fn try_from(value: serde_json::Value) -> super::Result<Self> {
        if value.as_str() == Some("function") {
            Ok(Self::Function)
        } else {
            Err(Self::invalid())
        }
    }
}
impl TryFrom<serde_json::Value> for FunctionDefinition {
    type Error = serde_json::Error;

    fn try_from(value: serde_json::Value) -> super::Result<Self> {
        let mut map = Self::object(value)?;
        let name = Self::nonempty(Self::required(&mut map, "name")?)?;
        let description = Self::optional(&mut map, "description")?;
        let strict = Self::optional(&mut map, "strict")?;
        let parameters = map
            .remove("parameters")
            .filter(|value| !value.is_null())
            .map(Self::schema)
            .transpose()?;
        Self::done(map)?;
        Ok(Self {
            name,
            description,
            strict,
            parameters,
        })
    }
}
impl TryFrom<serde_json::Value> for ToolDefinition {
    type Error = serde_json::Error;

    fn try_from(value: serde_json::Value) -> super::Result<Self> {
        let mut map = Self::object(value)?;
        let kind = Self::required(&mut map, "type")?;
        let function = Self::required(&mut map, "function")?;
        Self::done(map)?;
        Ok(Self { kind, function })
    }
}

impl ComponentDeserializeHelper for ToolDefinition {}
impl ComponentDeserializeHelper for FunctionDefinition {}
impl ComponentDeserializeHelper for ToolType {}
