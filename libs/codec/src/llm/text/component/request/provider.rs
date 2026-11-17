use super::ComponentDeserializeHelper;

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Deserialize)]
#[serde(try_from = "serde_json::Value")]
pub enum Provider {
    RequireParameter(bool),
}

impl TryFrom<serde_json::Value> for Provider {
    type Error = serde_json::Error;

    fn try_from(value: serde_json::Value) -> super::Result<Self> {
        let mut map = Self::object(value)?;
        if map.contains_key("require_parameters") {
            if map.contains_key("require_parameter") {
                return Err(Self::invalid());
            }
            let value = map.remove("require_parameters").ok_or_else(Self::invalid)?;
            map.insert("require_parameter".into(), value);
        }
        let required = Self::required(&mut map, "require_parameter")?;
        Self::done(map)?;
        Ok(Self::RequireParameter(required))
    }
}

impl ComponentDeserializeHelper for Provider {}
