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
#[derive(Debug)]
pub(super) enum StringReference {
    Input(std::ops::Range<usize>),
    Buffered,
}

impl StringReference {
    pub(super) fn new(value: &str, input: &[u8]) -> Self {
        if let Some(start) = (value.as_ptr() as usize).checked_sub(input.as_ptr() as usize)
            && let Some(end) = start.checked_add(value.len())
            && end <= input.len()
        {
            return Self::Input(start..end);
        }
        Self::Buffered
    }
}

#[derive(Debug)]
pub(super) enum ReadyToken {
    ObjectStart(Option<StringReference>),
    ObjectEnd,
    ArrayStart(Option<StringReference>),
    ArrayEnd,
    String(Option<StringReference>, StringReference),
    Number(Option<StringReference>, serde_json::Number),
    Boolean(Option<StringReference>, bool),
    Null(Option<StringReference>),
}

impl ReadyToken {
    pub(super) fn new(token: JsonToken<'_>, source: &[u8]) -> Self {
        let key = token.key().map(|key| StringReference::new(key, source));
        match token {
            JsonToken::ObjectStart(_) => Self::ObjectStart(key),
            JsonToken::ObjectEnd => Self::ObjectEnd,
            JsonToken::ArrayStart(_) => Self::ArrayStart(key),
            JsonToken::ArrayEnd => Self::ArrayEnd,
            JsonToken::ValueString(_, value) => {
                Self::String(key, StringReference::new(value, source))
            }
            JsonToken::ValueNumber(_, value) => Self::Number(key, value),
            JsonToken::ValueBoolean(_, value) => Self::Boolean(key, value),
            JsonToken::ValueNull(_) => Self::Null(key),
        }
    }
}
