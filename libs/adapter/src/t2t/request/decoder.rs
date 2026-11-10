use super::{
    Format, MODEL_BYTES, RequestScratch,
    json::{JsonString, Piece},
};
use crate::{
    codec::DecodeStep,
    error::CodecError,
    t2t::{
        Text2TextTranslatorConfiguration,
        request::event::{RequestEvent, Role},
    },
};

pub trait RequestDecoder {
    /// Required decoded fragment and role-late capacities, prepared by the caller.
    fn scratch_requirements(&self) -> (usize, usize) {
        (16 * 1024, 0)
    }
    /// Consume a prefix of `input`, returning at most one bounded event.
    ///
    /// Advance `input` to its remaining suffix. NeedInput requires an empty
    /// suffix and is invalid at EOF. Finished requires EOF and no remaining
    /// bytes. Retain bounded state, validate trailing input, and reject semantics
    /// that cannot be represented instead of silently discarding them.
    /// Input and scratch borrow reusable caller storage; do not retain either.
    /// TextFragment reports a length into scratch.text(). Consume that prefix
    /// before calling advance again; the next call may overwrite it. Keep the
    /// same scratch throughout an attempt, including across NeedInput/Progress.
    fn advance(
        &mut self,
        input: &mut &[u8],
        eof: bool,
        scratch: &mut RequestScratch,
    ) -> Result<DecodeStep<RequestEvent>, CodecError>;
}

impl<D: RequestDecoder + ?Sized> RequestDecoder for Box<D> {
    fn scratch_requirements(&self) -> (usize, usize) {
        (**self).scratch_requirements()
    }
    fn advance(
        &mut self,
        input: &mut &[u8],
        eof: bool,
        scratch: &mut RequestScratch,
    ) -> Result<DecodeStep<RequestEvent>, CodecError> {
        (**self).advance(input, eof, scratch)
    }
}

const WORK_BYTES: usize = 4096;
const CONTROL_BYTES: usize = 24;

fn invalid() -> CodecError {
    CodecError::InvalidInput("invalid or incomplete text request")
}
fn unsupported() -> CodecError {
    CodecError::Unsupported("request field, role, or shape is outside the text-only subset")
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
}
#[derive(Clone, Copy)]
enum Purpose {
    RootKey,
    MessageKey,
    Model,
    Role,
    Type,
    Content,
}
struct Token {
    purpose: Purpose,
    string: JsonString,
}

#[derive(Default)]
struct Message {
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

pub(crate) struct Decoder {
    format: Format,
    phase: Phase,
    field: Field,
    root_seen: u8,
    has_message: bool,
    token: Option<Token>,
    false_literal: Option<usize>,
    max_tokens: Option<u32>,
    configuration: Option<Text2TextTranslatorConfiguration>,
    small: [u8; CONTROL_BYTES],
    small_len: usize,
    model_bytes: usize,
    message: Option<Message>,
    fragment_emitted: bool,
    piece: [u8; 4],
    piece_pos: usize,
    piece_len: usize,
    piece_purpose: Purpose,
    fragment_bytes: usize,
    deferred_bytes: usize,
    failed: bool,
    completed: bool,
}

impl Decoder {
    pub fn scratch_requirements(&self) -> (usize, usize) {
        (self.fragment_bytes, self.deferred_bytes)
    }
    pub fn new(
        format: Format,
        fragment_bytes: usize,
        deferred_bytes: usize,
    ) -> Result<Self, CodecError> {
        if fragment_bytes == 0 {
            return Err(CodecError::InvalidInput("fragment_bytes must exceed zero"));
        }
        Ok(Self {
            format,
            phase: Phase::Root,
            field: Field::Model,
            root_seen: 0,
            has_message: false,
            token: None,
            false_literal: None,
            max_tokens: None,
            configuration: None,
            small: [0; CONTROL_BYTES],
            small_len: 0,
            model_bytes: 0,
            message: None,
            fragment_emitted: false,
            piece: [0; 4],
            piece_pos: 0,
            piece_len: 0,
            piece_purpose: Purpose::Model,
            fragment_bytes,
            deferred_bytes,
            failed: false,
            completed: false,
        })
    }

    fn event(&mut self, scratch: &mut RequestScratch) -> Option<RequestEvent> {
        if let Some(configuration) = self.configuration.take() {
            return Some(RequestEvent::Configuration(configuration));
        }
        let message = self.message.as_mut()?;
        if message.content_started
            && let Some(role) = message.role
        {
            if !message.started {
                message.started = true;
                return Some(RequestEvent::MessageStarted { role });
            }
            if !message.text_started {
                message.text_started = true;
                return Some(RequestEvent::TextStarted);
            }
            if message.deferred_pos < scratch.deferred_len {
                let len = self
                    .fragment_bytes
                    .min(scratch.deferred_len - message.deferred_pos);
                scratch.copy_deferred(message.deferred_pos, len);
                message.deferred_pos += len;
                self.fragment_emitted = true;
                return Some(RequestEvent::TextFragment(len));
            }
            if scratch.fragment_len > 0
                && (scratch.fragment_len == self.fragment_bytes || message.content_done)
            {
                self.fragment_emitted = true;
                return Some(RequestEvent::TextFragment(scratch.fragment_len));
            }
            if message.content_done && !message.text_ended {
                message.text_ended = true;
                return Some(RequestEvent::TextEnded);
            }
            if message.closed && message.text_ended {
                let system = message.system;
                self.message = None;
                scratch.reset();
                self.has_message |= !system;
                return Some(RequestEvent::MessageEnded);
            }
        }
        None
    }

    fn start_string(&mut self, purpose: Purpose) {
        self.small_len = 0;
        self.token = Some(Token {
            purpose,
            string: JsonString::default(),
        });
    }

    fn finish_string(&mut self, purpose: Purpose) -> Result<(), CodecError> {
        match purpose {
            Purpose::RootKey => {
                let (field, bit) = match &self.small[..self.small_len] {
                    b"model" => (Field::Model, 1),
                    b"stream" => (Field::Stream, 2),
                    b"messages" if self.format == Format::Chat => (Field::Body, 4),
                    b"messages" if self.format == Format::Anthropic => (Field::Body, 4),
                    b"input" if self.format == Format::Responses => (Field::Body, 4),
                    b"max_tokens" if self.format != Format::Responses => (Field::MaxTokens, 8),
                    b"max_completion_tokens" if self.format == Format::Chat => {
                        (Field::MaxTokens, 8)
                    }
                    b"max_output_tokens" if self.format == Format::Responses => {
                        (Field::MaxTokens, 8)
                    }
                    b"system" if self.format == Format::Anthropic => {
                        if self.root_seen & 4 != 0 {
                            return Err(CodecError::Unsupported("system must precede messages"));
                        }
                        (Field::System, 16)
                    }
                    _ => return Err(unsupported()),
                };
                if self.root_seen & bit != 0 {
                    return Err(invalid());
                }
                self.root_seen |= bit;
                self.field = field;
                self.phase = Phase::RootColon;
            }
            Purpose::MessageKey => {
                let (field, bit) = match &self.small[..self.small_len] {
                    b"role" => (Field::Role, 1),
                    b"content" => (Field::Content, 2),
                    b"type" if self.format == Format::Responses => (Field::Type, 4),
                    _ => return Err(unsupported()),
                };
                let message = self.message.as_mut().expect("message grammar");
                if message.seen & bit != 0 {
                    return Err(invalid());
                }
                message.seen |= bit;
                self.field = field;
                self.phase = Phase::MessageColon;
            }
            Purpose::Model => {
                if self.model_bytes == 0 {
                    return Err(invalid());
                }
                self.phase = Phase::RootAfter;
            }
            Purpose::Role => {
                self.message.as_mut().expect("message grammar").role =
                    Some(match &self.small[..self.small_len] {
                        b"system" if self.format != Format::Anthropic => Role::System,
                        b"user" => Role::User,
                        b"assistant" => Role::Assistant,
                        _ => return Err(unsupported()),
                    });
                self.phase = Phase::MessageAfter;
            }
            Purpose::Type => {
                if &self.small[..self.small_len] != b"message" {
                    return Err(unsupported());
                }
                self.phase = Phase::MessageAfter;
            }
            Purpose::Content => {
                let message = self.message.as_mut().expect("message grammar");
                message.content_done = true;
                if message.short_input {
                    message.closed = true;
                    self.phase = Phase::RootAfter;
                } else {
                    self.phase = Phase::MessageAfter;
                }
            }
        }
        Ok(())
    }

    fn decoded_byte(
        &mut self,
        byte: u8,
        purpose: Purpose,
        scratch: &mut RequestScratch,
    ) -> Result<(), CodecError> {
        match purpose {
            Purpose::Content => {
                let message = self.message.as_mut().expect("message grammar");
                if message.role.is_none() {
                    if scratch.deferred_len == self.deferred_bytes {
                        return Err(CodecError::LimitExceeded(
                            "text before role exceeds deferral allowance",
                        ));
                    }
                    scratch.defer(byte)?;
                } else {
                    let index = scratch.fragment_len;
                    scratch.text_mut()[index] = byte;
                    scratch.fragment_len += 1;
                }
            }
            Purpose::Model => {
                if self.model_bytes == MODEL_BYTES {
                    return Err(CodecError::LimitExceeded(
                        "model exceeds identifier allowance",
                    ));
                }
                self.model_bytes += 1;
            }
            _ => {
                if self.small_len == CONTROL_BYTES {
                    return Err(unsupported());
                }
                self.small[self.small_len] = byte;
                self.small_len += 1;
            }
        }
        Ok(())
    }

    fn close_message(&mut self) -> Result<(), CodecError> {
        let message = self.message.as_mut().expect("message grammar");
        if message.seen & 3 != 3 || message.role.is_none() || !message.content_done {
            return Err(invalid());
        }
        message.closed = true;
        self.phase = Phase::ArrayAfter;
        Ok(())
    }

    fn consume(&mut self, byte: u8) -> Result<(), CodecError> {
        if let Some(token) = &mut self.token {
            let purpose = token.purpose;
            match token.string.push(byte)? {
                Some(Piece::Bytes(bytes, len)) => {
                    self.piece = bytes;
                    self.piece_pos = 0;
                    self.piece_len = len;
                    self.piece_purpose = purpose;
                }
                Some(Piece::End) => {
                    self.token = None;
                    self.finish_string(purpose)?;
                }
                None => {}
            }
            return Ok(());
        }
        if let Some(position) = self.false_literal {
            if byte != b"false"[position] {
                return Err(invalid());
            }
            if position == 4 {
                self.false_literal = None;
                self.phase = Phase::RootAfter;
            } else {
                self.false_literal = Some(position + 1);
            }
            return Ok(());
        }
        if let Some(value) = self.max_tokens {
            if byte.is_ascii_digit() {
                if value == 0 {
                    return Err(invalid());
                }
                self.max_tokens = Some(
                    value
                        .checked_mul(10)
                        .and_then(|value| value.checked_add(u32::from(byte - b'0')))
                        .ok_or(CodecError::LimitExceeded("max_tokens exceeds u32"))?,
                );
                return Ok(());
            }
            if value == 0 {
                return Err(invalid());
            }
            self.max_tokens = None;
            self.configuration = Some(Text2TextTranslatorConfiguration {
                max_output_tokens: value,
            });
            self.phase = Phase::RootAfter;
        }
        if matches!(byte, b' ' | b'\n' | b'\r' | b'\t') {
            return Ok(());
        }
        match (self.phase, byte) {
            (Phase::Root, b'{') => self.phase = Phase::RootKey(true),
            (Phase::RootKey(_), b'"') => self.start_string(Purpose::RootKey),
            (Phase::RootKey(true) | Phase::RootAfter, b'}') => {
                if self.root_seen & 5 != 5 || !self.has_message {
                    return Err(invalid());
                }
                if self.format == Format::Anthropic && self.root_seen & 8 == 0 {
                    return Err(invalid());
                }
                self.phase = Phase::Done;
            }
            (Phase::RootColon, b':') => self.phase = Phase::RootValue,
            (Phase::RootValue, _) => match self.field {
                Field::Model if byte == b'"' => {
                    self.model_bytes = 0;
                    self.start_string(Purpose::Model);
                }
                Field::Stream if byte == b'f' => self.false_literal = Some(1),
                Field::Stream if byte == b't' => return Err(unsupported()),
                Field::MaxTokens if byte.is_ascii_digit() => {
                    self.max_tokens = Some(u32::from(byte - b'0'));
                }
                Field::System if byte == b'"' => {
                    self.message = Some(Message {
                        role: Some(Role::System),
                        content_started: true,
                        short_input: true,
                        system: true,
                        ..Message::default()
                    });
                    self.start_string(Purpose::Content);
                }
                Field::System => return Err(unsupported()),
                Field::Body if byte == b'[' => self.phase = Phase::ArrayFirst,
                Field::Body if byte == b'"' && self.format == Format::Responses => {
                    self.message = Some(Message {
                        role: Some(Role::User),
                        content_started: true,
                        short_input: true,
                        ..Message::default()
                    });
                    self.start_string(Purpose::Content);
                }
                Field::Body => return Err(unsupported()),
                _ => return Err(invalid()),
            },
            (Phase::RootAfter, b',') => self.phase = Phase::RootKey(false),
            (Phase::ArrayFirst | Phase::ArrayNext, b'{') => {
                self.message = Some(Message::default());
                self.phase = Phase::MessageKey(true);
            }
            (Phase::MessageKey(_), b'"') => self.start_string(Purpose::MessageKey),
            (Phase::MessageKey(true) | Phase::MessageAfter, b'}') => self.close_message()?,
            (Phase::MessageColon, b':') => self.phase = Phase::MessageValue,
            (Phase::MessageValue, b'"') => match self.field {
                Field::Role => self.start_string(Purpose::Role),
                Field::Type => self.start_string(Purpose::Type),
                Field::Content => {
                    self.message
                        .as_mut()
                        .expect("message grammar")
                        .content_started = true;
                    self.start_string(Purpose::Content);
                }
                _ => return Err(invalid()),
            },
            (Phase::MessageValue, _) if matches!(self.field, Field::Content) => {
                return Err(unsupported());
            }
            (Phase::MessageAfter, b',') => self.phase = Phase::MessageKey(false),
            (Phase::ArrayAfter, b',') => self.phase = Phase::ArrayNext,
            (Phase::ArrayAfter, b']') => self.phase = Phase::RootAfter,
            _ => return Err(invalid()),
        }
        Ok(())
    }

    pub fn advance(
        &mut self,
        input: &mut &[u8],
        eof: bool,
        scratch: &mut RequestScratch,
    ) -> Result<DecodeStep<RequestEvent>, CodecError> {
        if self.failed {
            return Err(CodecError::Contract("decoder previously failed"));
        }
        if self.completed {
            if input.is_empty() && eof {
                return Ok(DecodeStep::Finished);
            }
            self.failed = true;
            return Err(invalid());
        }
        if scratch.text().len() < self.fragment_bytes
            || scratch.deferred_capacity() < self.deferred_bytes
        {
            self.failed = true;
            return Err(CodecError::Contract(
                "decoder scratch is smaller than declared requirements",
            ));
        }
        if self.fragment_emitted {
            scratch.fragment_len = 0;
            self.fragment_emitted = false;
        }
        let result = self.step(input, eof, scratch);
        if result.is_err() {
            self.failed = true;
        }
        result
    }

    fn step(
        &mut self,
        input: &mut &[u8],
        eof: bool,
        scratch: &mut RequestScratch,
    ) -> Result<DecodeStep<RequestEvent>, CodecError> {
        if let Some(event) = self.event(scratch) {
            return Ok(DecodeStep::Event(event));
        }
        let mut consumed = 0;
        while consumed < WORK_BYTES {
            if self.piece_pos < self.piece_len {
                let byte = self.piece[self.piece_pos];
                self.piece_pos += 1;
                self.decoded_byte(byte, self.piece_purpose, scratch)?;
            } else {
                let Some((&byte, remaining)) = input.split_first() else {
                    break;
                };
                *input = remaining;
                consumed += 1;
                self.consume(byte)?;
            }
            if let Some(event) = self.event(scratch) {
                return Ok(DecodeStep::Event(event));
            }
        }
        if input.is_empty() && eof {
            if self.token.is_some()
                || self.false_literal.is_some()
                || self.max_tokens.is_some()
                || !matches!(self.phase, Phase::Done)
                || self.message.is_some()
            {
                return Err(invalid());
            }
            self.completed = true;
            Ok(DecodeStep::Finished)
        } else if consumed > 0 {
            Ok(DecodeStep::Progress)
        } else {
            Ok(DecodeStep::NeedInput)
        }
    }
}
