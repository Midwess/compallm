#[derive(Debug, PartialEq)]
pub enum JsonToken<'a> {
    ObjectStart(Option<&'a str>),
    ObjectEnd,
    ArrayStart(Option<&'a str>),
    ArrayEnd,
    ValueString(Option<&'a str>, &'a str),
    ValueNumber(Option<&'a str>, serde_json::Number),
    ValueBoolean(Option<&'a str>, bool),
    ValueNull(Option<&'a str>),
}

impl<'a> JsonToken<'a> {
    pub fn key(&self) -> Option<&'a str> {
        match self {
            Self::ObjectStart(key)
            | Self::ArrayStart(key)
            | Self::ValueString(key, _)
            | Self::ValueNumber(key, _)
            | Self::ValueBoolean(key, _)
            | Self::ValueNull(key) => *key,
            Self::ObjectEnd | Self::ArrayEnd => None,
        }
    }
}
