mod decoder;
mod encoder;
mod value;

pub(crate) use decoder::FormatDecoder;
pub(crate) use encoder::FormatWriter;
pub(crate) use value::SchemaValue;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutputFormatKind {
    Text,
    JsonObject,
    JsonSchema,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutputFormatEvent {
    Started,
    Mode(OutputFormatKind),
    NameStarted,
    NameEnded,
    DescriptionStarted,
    DescriptionEnded,
    Strict(Option<bool>),
    SchemaStarted,
    SchemaEnded,
    Ended,
}
