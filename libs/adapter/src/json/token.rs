#[derive(Debug, PartialEq)]
pub enum JsonToken<'a> {
    ObjectStart,
    ObjectEnd,
    ArrayStart,
    ArrayEnd,
    Colon,
    Comma,
    String(&'a str),
    Number(&'a str),
    Boolean(bool),
    Null,
}

#[derive(Debug, PartialEq)]
pub enum JsonStep<'a> {
    Event(JsonToken<'a>),
    NeedInput,
    Finished,
}
