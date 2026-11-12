mod output_format;
mod parser;
pub(crate) mod reasoning;
mod schema_value;
pub(crate) mod stop;
mod text_blocks;

use super::{Format, RequestScratch, json::JsonString};
use crate::{
    codec::DecodeStep,
    error::CodecError,
    t2t::{
        Text2TextTranslatorConfiguration,
        request::event::{RequestEvent, Role},
    },
};

pub(crate) use output_format::FormatDecoder;
pub(crate) use parser::Decoder;
pub(crate) use schema_value::SchemaValue;

const WORK_BYTES: usize = 4096;
pub(super) const CONTROL_BYTES: usize = 32;

pub trait RequestDecoder {
    fn advance(
        &mut self,
        input: &mut &[u8],
        eof: bool,
        scratch: &mut RequestScratch,
    ) -> Result<DecodeStep<RequestEvent>, CodecError>;
}

impl<D: RequestDecoder + ?Sized> RequestDecoder for Box<D> {
    fn advance(
        &mut self,
        input: &mut &[u8],
        eof: bool,
        scratch: &mut RequestScratch,
    ) -> Result<DecodeStep<RequestEvent>, CodecError> {
        (**self).advance(input, eof, scratch)
    }
}

#[derive(Clone, Copy)]
enum Field {
    Model,
    Stream,
    Body,
    Role,
    Content,
    Type,
    MaxTokens,
    System,
    StreamOptions,
    IncludeUsage,
    TopP,
    Stop,
    OutputFormat,
    Store,
    Include,
    ItemId,
    ItemStatus,
}

#[derive(Clone, Copy)]
enum Phase {
    Root,
    RootKey(bool),
    RootColon,
    RootValue,
    RootAfter,
    ArrayFirst,
    ArrayNext,
    MessageKey(bool),
    MessageColon,
    MessageValue,
    MessageAfter,
    ArrayAfter,
    Done,
    OptionsKey(bool),
    OptionsColon,
    OptionsValue,
    OptionsAfter,
    StopFirst,
    StopNext,
    StopAfter,
    IncludeFirst,
    IncludeAfter,
}

#[derive(Clone, Copy)]
enum Purpose {
    RootKey,
    MessageKey,
    Model,
    Role,
    Type,
    Content,
    OptionKey,
    Stop,
    Include,
    ItemId,
    ItemStatus,
}

struct Token {
    purpose: Purpose,
    string: JsonString,
}

#[derive(Default)]
struct Message {
    content_array: bool,
    role: Option<Role>,
    seen: u8,
    content_started: bool,
    content_done: bool,
    closed: bool,
    started: bool,
    text_started: bool,
    text_ended: bool,
    deferred_pos: usize,
    short_input: bool,
    system: bool,
}
