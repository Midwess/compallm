use super::super::{
    Format, RequestScratch,
    event::RequestEvent,
    json::{JsonString, Piece},
};
use super::{
    super::event::{OutputFormatEvent as E, OutputFormatKind as K},
    SchemaValue,
};
use crate::{codec::DecodeStep, error::CodecError};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Context {
    Outer,
    Chat,
    Definition,
    Flat,
    Anthropic,
}
#[derive(Clone, Copy)]
enum Phase {
    Key(bool),
    Colon,
    Value,
    After,
}
#[derive(Clone, Copy, PartialEq, Eq)]
enum Field {
    Format,
    Type,
    Definition,
    Name,
    Description,
    Schema,
    Strict,
}
#[derive(Clone, Copy)]
struct Frame {
    context: Context,
    phase: Phase,
    field: Field,
    seen: u8,
}
#[derive(Clone, Copy, PartialEq, Eq)]
enum Purpose {
    Key,
    Type,
    Name,
    Description,
}

pub(crate) struct FormatDecoder {
    format: Format,
    frames: [Frame; 3],
    depth: usize,
    string: Option<JsonString>,
    purpose: Purpose,
    small: [u8; 64],
    small_len: usize,
    value_len: usize,
    piece: [u8; 4],
    piece_pos: usize,
    piece_len: usize,
    literal: Option<(&'static [u8], usize, Option<bool>)>,
    schema: Option<SchemaValue>,
    pending: Option<E>,
    emitted: bool,
    started: bool,
    done: bool,
    ending: bool,
    mode: Option<K>,
    definition: bool,
    has_schema: bool,
    has_name: bool,
    has_strict: bool,
}
fn invalid() -> CodecError {
    CodecError::InvalidInput("invalid output format configuration")
}
fn unsupported() -> CodecError {
    CodecError::Unsupported("unsupported output format configuration")
}
fn event(value: E) -> DecodeStep<RequestEvent> {
    DecodeStep::Event(RequestEvent::OutputFormat(value))
}

impl FormatDecoder {
    pub fn new(format: Format) -> Self {
        let context = if format == Format::Chat {
            Context::Chat
        } else {
            Context::Outer
        };
        let frame = Frame {
            context,
            phase: Phase::Key(true),
            field: Field::Type,
            seen: 0,
        };
        Self {
            format,
            frames: [frame; 3],
            depth: 1,
            string: None,
            purpose: Purpose::Key,
            small: [0; 64],
            small_len: 0,
            value_len: 0,
            piece: [0; 4],
            piece_pos: 0,
            piece_len: 0,
            literal: None,
            schema: None,
            pending: None,
            emitted: false,
            started: false,
            done: false,
            ending: false,
            mode: None,
            definition: false,
            has_schema: false,
            has_name: false,
            has_strict: false,
        }
    }
    fn start_string(&mut self, purpose: Purpose) {
        self.purpose = purpose;
        self.string = Some(JsonString::default());
        self.small_len = 0;
        self.value_len = 0;
    }
    fn finish_string(&mut self) -> Result<Option<E>, CodecError> {
        let frame = &mut self.frames[self.depth - 1];
        match self.purpose {
            Purpose::Key => {
                let field = match (frame.context, &self.small[..self.small_len]) {
                    (Context::Outer, b"format") => Field::Format,
                    (Context::Chat | Context::Flat | Context::Anthropic, b"type") => Field::Type,
                    (Context::Chat, b"json_schema") => Field::Definition,
                    (Context::Definition | Context::Flat, b"name") => Field::Name,
                    (Context::Definition | Context::Flat, b"description") => Field::Description,
                    (Context::Definition | Context::Flat | Context::Anthropic, b"schema") => {
                        Field::Schema
                    }
                    (Context::Definition | Context::Flat, b"strict") => Field::Strict,
                    _ => return Err(unsupported()),
                };
                let bit = 1 << (field as u8);
                if frame.seen & bit != 0 {
                    return Err(invalid());
                }
                frame.seen |= bit;
                frame.field = field;
                frame.phase = Phase::Colon;
                Ok(None)
            }
            Purpose::Type => {
                let mode = match &self.small[..self.small_len] {
                    b"text" => K::Text,
                    b"json_object" => K::JsonObject,
                    b"json_schema" => K::JsonSchema,
                    _ => return Err(unsupported()),
                };
                if self.format == Format::Anthropic && mode != K::JsonSchema {
                    return Err(unsupported());
                }
                self.mode = Some(mode);
                frame.phase = Phase::After;
                Ok(Some(E::Mode(mode)))
            }
            Purpose::Name => {
                if self.value_len == 0 {
                    return Err(invalid());
                }
                self.has_name = true;
                frame.phase = Phase::After;
                Ok(Some(E::NameEnded))
            }
            Purpose::Description => {
                frame.phase = Phase::After;
                Ok(Some(E::DescriptionEnded))
            }
        }
    }
    fn close(&mut self) -> Result<(), CodecError> {
        let frame = self.frames[self.depth - 1];
        match frame.context {
            Context::Outer if frame.seen & 1 == 0 => {
                self.mode = Some(K::Text);
                self.pending = Some(E::Mode(K::Text));
            }
            Context::Definition => {
                if !self.has_name || !self.has_schema {
                    return Err(invalid());
                }
            }
            Context::Chat | Context::Flat | Context::Anthropic => match self.mode {
                Some(K::JsonSchema) => {
                    if !self.has_schema || frame.context != Context::Anthropic && !self.has_name {
                        return Err(invalid());
                    }
                    if frame.context == Context::Chat && !self.definition {
                        return Err(invalid());
                    }
                }
                Some(K::Text | K::JsonObject) => {
                    if frame.seen != 1 << (Field::Type as u8) {
                        return Err(invalid());
                    }
                }
                None => return Err(invalid()),
            },
            _ => {}
        }
        self.depth -= 1;
        if self.depth == 0 {
            self.ending = true;
        } else {
            self.frames[self.depth - 1].phase = Phase::After;
        }
        Ok(())
    }
    pub fn advance(
        &mut self,
        input: &mut &[u8],
        eof: bool,
        scratch: &mut RequestScratch,
    ) -> Result<DecodeStep<RequestEvent>, CodecError> {
        if !self.started {
            self.started = true;
            return Ok(event(E::Started));
        }
        if self.emitted {
            scratch.fragment_len = 0;
            self.emitted = false;
        }
        if let Some(value) = self.pending.take() {
            return Ok(event(value));
        }
        if self.ending {
            if self.format == Format::Anthropic
                && self.mode == Some(K::JsonSchema)
                && !self.has_strict
            {
                self.has_strict = true;
                return Ok(event(E::Strict(Some(true))));
            }
            self.done = true;
            self.ending = false;
            return Ok(event(E::Ended));
        }
        if self.done {
            return Ok(DecodeStep::Finished);
        }
        let mut consumed = 0;
        while consumed < 4096 {
            if self.schema.is_some() && scratch.fragment_len == scratch.text().len() {
                self.emitted = true;
                return Ok(DecodeStep::Event(RequestEvent::SchemaFragment(
                    scratch.fragment_len,
                )));
            }
            if self.piece_pos < self.piece_len {
                let byte = self.piece[self.piece_pos];
                self.piece_pos += 1;
                match self.purpose {
                    Purpose::Key | Purpose::Type => {
                        if self.small_len == self.small.len() {
                            return Err(unsupported());
                        }
                        self.small[self.small_len] = byte;
                        self.small_len += 1;
                    }
                    Purpose::Name | Purpose::Description => {
                        let limit = if self.purpose == Purpose::Name {
                            64
                        } else {
                            256
                        };
                        if self.value_len == limit {
                            return Err(CodecError::LimitExceeded(
                                "format metadata exceeds control capacity",
                            ));
                        }
                        if self.purpose == Purpose::Name
                            && !matches!(byte,b'a'..=b'z'|b'A'..=b'Z'|b'0'..=b'9'|b'_'|b'-')
                        {
                            return Err(invalid());
                        }
                        self.value_len += 1;
                        let index = scratch.fragment_len;
                        scratch.text_mut()[index] = byte;
                        scratch.fragment_len += 1;
                        if scratch.fragment_len == scratch.text().len() {
                            self.emitted = true;
                            return Ok(DecodeStep::Event(RequestEvent::TextFragment(
                                scratch.fragment_len,
                            )));
                        }
                    }
                }
                continue;
            }
            let Some((&byte, remaining)) = input.split_first() else {
                break;
            };
            *input = remaining;
            consumed += 1;
            if let Some(schema) = &mut self.schema {
                schema.push(byte)?;
                let index = scratch.fragment_len;
                scratch.text_mut()[index] = byte;
                scratch.fragment_len += 1;
                if schema.done {
                    self.schema = None;
                    self.frames[self.depth - 1].phase = Phase::After;
                    self.pending = Some(E::SchemaEnded);
                }
                if scratch.fragment_len == scratch.text().len() || self.schema.is_none() {
                    self.emitted = true;
                    return Ok(DecodeStep::Event(RequestEvent::SchemaFragment(
                        scratch.fragment_len,
                    )));
                }
                continue;
            }
            if let Some(string) = &mut self.string {
                match string.push(byte)? {
                    Some(Piece::Bytes(bytes, len)) => {
                        self.piece = bytes;
                        self.piece_pos = 0;
                        self.piece_len = len;
                    }
                    Some(Piece::End) => {
                        self.string = None;
                        if let Some(end) = self.finish_string()? {
                            if scratch.fragment_len > 0 {
                                self.pending = Some(end);
                                self.emitted = true;
                                return Ok(DecodeStep::Event(RequestEvent::TextFragment(
                                    scratch.fragment_len,
                                )));
                            }
                            return Ok(event(end));
                        }
                    }
                    None => {}
                }
                continue;
            }
            if let Some((literal, position, value)) = self.literal {
                if byte != literal[position] {
                    return Err(invalid());
                }
                if position + 1 == literal.len() {
                    self.literal = None;
                    self.frames[self.depth - 1].phase = Phase::After;
                    if self.frames[self.depth - 1].field == Field::Format {
                        self.mode = Some(K::Text);
                        return Ok(event(E::Mode(K::Text)));
                    }
                    self.has_strict = true;
                    return Ok(event(E::Strict(value)));
                }
                self.literal = Some((literal, position + 1, value));
                continue;
            }
            if matches!(byte, b' ' | b'\n' | b'\r' | b'\t') {
                continue;
            }
            let frame = self.frames[self.depth - 1];
            match (frame.phase, byte) {
                (Phase::Key(_), b'"') => self.start_string(Purpose::Key),
                (Phase::Key(true) | Phase::After, b'}') => {
                    self.close()?;
                    if self.ending {
                        return self.advance(input, eof, scratch);
                    }
                }
                (Phase::Colon, b':') => self.frames[self.depth - 1].phase = Phase::Value,
                (Phase::After, b',') => self.frames[self.depth - 1].phase = Phase::Key(false),
                (Phase::Value, _) => match frame.field {
                    Field::Type if byte == b'"' => self.start_string(Purpose::Type),
                    Field::Name | Field::Description if byte == b'"' => {
                        let name = frame.field == Field::Name;
                        self.start_string(if name {
                            Purpose::Name
                        } else {
                            Purpose::Description
                        });
                        return Ok(event(if name {
                            E::NameStarted
                        } else {
                            E::DescriptionStarted
                        }));
                    }
                    Field::Strict if matches!(byte, b't' | b'f' | b'n') => {
                        self.literal = Some(match byte {
                            b't' => (b"true", 1, Some(true)),
                            b'f' => (b"false", 1, Some(false)),
                            _ => (b"null", 1, None),
                        })
                    }
                    Field::Schema if byte == b'{' => {
                        if self.has_schema {
                            return Err(invalid());
                        }
                        self.has_schema = true;
                        let mut schema = SchemaValue::default();
                        schema.push(byte)?;
                        self.schema = Some(schema);
                        scratch.text_mut()[0] = byte;
                        scratch.fragment_len = 1;
                        return Ok(event(E::SchemaStarted));
                    }
                    Field::Definition | Field::Format if byte == b'{' => {
                        if self.depth == self.frames.len() {
                            return Err(invalid());
                        }
                        let context = if frame.field == Field::Definition {
                            self.definition = true;
                            Context::Definition
                        } else if self.format == Format::Responses {
                            Context::Flat
                        } else {
                            Context::Anthropic
                        };
                        self.frames[self.depth] = Frame {
                            context,
                            phase: Phase::Key(true),
                            field: Field::Type,
                            seen: 0,
                        };
                        self.depth += 1;
                    }
                    Field::Format if byte == b'n' => self.literal = Some((b"null", 1, None)),
                    _ => return Err(unsupported()),
                },
                _ => return Err(invalid()),
            }
        }
        if input.is_empty() && eof {
            return Err(invalid());
        }
        Ok(if consumed > 0 {
            DecodeStep::Progress
        } else {
            DecodeStep::NeedInput
        })
    }
}
