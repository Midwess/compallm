#[derive(Debug, PartialEq)]
pub enum JsonToken<'a> {
    ValueObject(Option<&'a str>),
    ObjectEnd,
    ValueArray(Option<&'a str>),
    ArrayEnd,
    ValueString(Option<&'a str>, &'a str),
    ValueNumber(Option<&'a str>, &'a str),
    ValueBoolean(Option<&'a str>, bool),
    ValueNull(Option<&'a str>),
}

#[derive(Debug, PartialEq)]
pub enum JsonStep<'a> {
    /// One token is ready for the consumer.
    Event(JsonToken<'a>),
    /// More input is needed to produce another token.
    NeedInput,
    /// The decoder has finished processing the source.
    Finished,
}

impl<'a> From<JsonStep<'a>> for DecoderStatus<JsonToken<'a>> {
    fn from(step: JsonStep<'a>) -> Self {
        match step {
            JsonStep::Event(token) => Self::Component(token),
            JsonStep::NeedInput => Self::NeedInput,
            JsonStep::Finished => Self::Finished,
        }
    }
}
use crate::decoder::DecoderStatus;
