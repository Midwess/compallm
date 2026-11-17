use super::ComponentDeserializeHelper;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Deserialize)]
#[serde(try_from = "serde_json::Value")]
pub struct StreamOptions {
    pub include_usage: Option<bool>,
}

impl TryFrom<serde_json::Value> for StreamOptions {
    type Error = serde_json::Error;

    fn try_from(value: serde_json::Value) -> super::Result<Self> {
        let mut map = Self::object(value)?;
        let include_usage = Self::optional(&mut map, "include_usage")?;
        Self::done(map)?;
        Ok(Self { include_usage })
    }
}

impl ComponentDeserializeHelper for StreamOptions {}
