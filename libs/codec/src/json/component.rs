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
