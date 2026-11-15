#[derive(Debug, PartialEq)]
pub enum JsonToken<'a> {
    /// Opens a JSON object: `{`.
    ObjectStart,
    /// Closes a JSON object: `}`.
    ObjectEnd,
    /// Opens a JSON array: `[`.
    ArrayStart,
    /// Closes a JSON array: `]`.
    ArrayEnd,
    /// Separates an object key from its value: `:`.
    Colon,
    /// Separates object members or array elements: `,`.
    Comma,
    /// Decoded string content, borrowed from input or decoder scratch; also used for object keys.
    String(&'a str),
    /// Validated JSON number text, borrowed without conversion to a numeric type.
    Number(&'a str),
    /// A JSON `true` or `false` value.
    Boolean(bool),
    /// A JSON `null` value.
    Null,
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
