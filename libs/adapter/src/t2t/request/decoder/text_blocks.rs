use super::super::{
    Format, RequestScratch,
    event::{RequestEvent, Role},
    json::{JsonString, Piece},
};
use crate::{codec::DecodeStep, error::CodecError};

enum Phase {
    Array(bool),
    Key,
    Colon,
    Value,
    After,
    AfterBlock,
    Done,
    MetadataStart,
    MetadataEnd,
}
#[derive(Clone, Copy)]
enum Field {
    Key,
    Type,
    Text,
    Metadata,
}

pub(super) struct TextBlocks {
    content: Option<(Format, Role)>,
    started: bool,
    ended: bool,
    has_block: bool,
    output: bool,
    phase: Phase,
    field: Field,
    string: Option<JsonString>,
    control: [u8; 16],
    len: usize,
    seen: u8,
    text_len: usize,
    piece: [u8; 4],
    piece_pos: usize,
    piece_len: usize,
    pending: Option<RequestEvent>,
    emitted: bool,
}

impl TextBlocks {
    pub fn content(format: Format, role: Role) -> Self {
        Self {
            content: Some((format, role)),
            ..Self::new()
        }
    }
    pub fn new() -> Self {
        Self {
            content: None,
            started: false,
            ended: false,
            has_block: false,
            output: false,
            phase: Phase::Array(true),
            field: Field::Key,
            string: None,
            control: [0; 16],
            len: 0,
            seen: 0,
            text_len: 0,
            piece: [0; 4],
            piece_pos: 0,
            piece_len: 0,
            pending: None,
            emitted: false,
        }
    }

    pub fn advance(
        &mut self,
        input: &mut &[u8],
        eof: bool,
        scratch: &mut RequestScratch,
    ) -> Result<DecodeStep<RequestEvent>, CodecError> {
        if self.content.is_some() && !self.started {
            self.started = true;
            return Ok(DecodeStep::Event(RequestEvent::TextPartsStarted));
        }
        if self.emitted {
            scratch.fragment_len = 0;
            self.emitted = false;
        }
        if let Some(event) = self.pending.take() {
            return Ok(DecodeStep::Event(event));
        }
        if matches!(self.phase, Phase::Done) {
            if self.content.is_some() && !self.ended {
                self.ended = true;
                return Ok(DecodeStep::Event(RequestEvent::TextPartsEnded));
            }
            return Ok(DecodeStep::Finished);
        }
        let invalid = || CodecError::InvalidInput("invalid text blocks");
        let mut consumed = 0;
        while consumed < 4096 {
            if self.piece_pos < self.piece_len {
                let byte = self.piece[self.piece_pos];
                self.piece_pos += 1;
                if matches!(self.field, Field::Text) {
                    let index = scratch.fragment_len;
                    scratch.text_mut()[index] = byte;
                    scratch.fragment_len += 1;
                    self.text_len += 1;
                    if scratch.fragment_len == scratch.text().len() {
                        self.emitted = true;
                        return Ok(DecodeStep::Event(RequestEvent::TextFragment(
                            scratch.fragment_len,
                        )));
                    }
                } else {
                    if self.len == self.control.len() {
                        return Err(invalid());
                    }
                    self.control[self.len] = byte;
                    self.len += 1;
                }
                continue;
            }
            let Some((&byte, rest)) = input.split_first() else {
                break;
            };
            *input = rest;
            consumed += 1;
            if let Some(string) = &mut self.string {
                match string.push(byte)? {
                    Some(Piece::Bytes(bytes, len)) => {
                        self.piece = bytes;
                        self.piece_pos = 0;
                        self.piece_len = len;
                    }
                    Some(Piece::End) => {
                        self.string = None;
                        match self.field {
                            Field::Key => {
                                let (field, bit) = match &self.control[..self.len] {
                                    b"type" => (Field::Type, 1),
                                    b"text" => (Field::Text, 2),
                                    b"annotations"
                                        if self.content
                                            == Some((Format::Responses, Role::Assistant)) =>
                                    {
                                        (Field::Metadata, 4)
                                    }
                                    b"logprobs"
                                        if self.content
                                            == Some((Format::Responses, Role::Assistant)) =>
                                    {
                                        (Field::Metadata, 8)
                                    }
                                    _ => {
                                        return Err(CodecError::Unsupported(
                                            "unsupported text block metadata",
                                        ));
                                    }
                                };
                                if self.seen & bit != 0 {
                                    return Err(invalid());
                                }
                                self.seen |= bit;
                                self.field = field;
                                self.phase = Phase::Colon;
                            }
                            Field::Type => {
                                self.output = &self.control[..self.len] == b"output_text";
                                let expected: &[u8] = match self.content {
                                    Some((Format::Responses, Role::Assistant)) => b"output_text",
                                    Some((Format::Responses, _)) => b"input_text",
                                    _ => b"text",
                                };
                                if &self.control[..self.len] != expected
                                    && !(self.content == Some((Format::Responses, Role::Assistant))
                                        && &self.control[..self.len] == b"input_text")
                                {
                                    return Err(invalid());
                                }
                                self.phase = Phase::After;
                            }
                            Field::Text => {
                                if self.text_len == 0
                                    && self
                                        .content
                                        .is_none_or(|(format, _)| format == Format::Anthropic)
                                {
                                    return Err(invalid());
                                }
                                self.phase = Phase::After;
                                if scratch.fragment_len > 0 {
                                    self.pending = Some(RequestEvent::TextEnded);
                                    self.emitted = true;
                                    return Ok(DecodeStep::Event(RequestEvent::TextFragment(
                                        scratch.fragment_len,
                                    )));
                                }
                                return Ok(DecodeStep::Event(RequestEvent::TextEnded));
                            }
                            Field::Metadata => return Err(invalid()),
                        }
                    }
                    None => {}
                }
                continue;
            }
            if byte.is_ascii_whitespace() {
                if !matches!(byte, b' ' | b'\n' | b'\r' | b'\t') {
                    return Err(invalid());
                }
                continue;
            }
            match (&self.phase, byte) {
                (Phase::Array(_), b'{') => {
                    self.seen = 0;
                    self.text_len = 0;
                    self.output = false;
                    self.phase = Phase::Key;
                }
                (Phase::Array(true) | Phase::AfterBlock, b']') => {
                    if self.content.is_some() && !self.has_block {
                        return Err(invalid());
                    }
                    self.phase = Phase::Done;
                    return self.advance(input, eof, scratch);
                }
                (Phase::AfterBlock, b',') => self.phase = Phase::Array(false),
                (Phase::Key, b'"') => {
                    self.field = Field::Key;
                    self.len = 0;
                    self.string = Some(JsonString::default());
                }
                (Phase::Colon, b':') => {
                    self.phase = if matches!(self.field, Field::Metadata) {
                        Phase::MetadataStart
                    } else {
                        Phase::Value
                    }
                }
                (Phase::MetadataStart, b'[') => self.phase = Phase::MetadataEnd,
                (Phase::MetadataEnd, b']') => self.phase = Phase::After,
                (Phase::MetadataEnd, _) => {
                    return Err(CodecError::Unsupported("nonempty text block metadata"));
                }
                (Phase::Value, b'"') => {
                    self.len = 0;
                    self.string = Some(JsonString::default());
                    if matches!(self.field, Field::Text) {
                        if self.content.is_some() {
                            return Ok(DecodeStep::Event(RequestEvent::TextStarted));
                        }
                        self.pending = Some(RequestEvent::TextStarted);
                        return Ok(DecodeStep::Event(RequestEvent::MessageStarted {
                            role: Role::System,
                        }));
                    }
                }
                (Phase::After, b',') => self.phase = Phase::Key,
                (Phase::After, b'}') if self.seen & 3 == 3 => {
                    if self.seen & 12 != 0 && !self.output {
                        return Err(invalid());
                    }
                    self.has_block = true;
                    self.phase = Phase::AfterBlock;
                    if self.content.is_none() {
                        return Ok(DecodeStep::Event(RequestEvent::MessageEnded));
                    }
                }
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
