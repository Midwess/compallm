use bytes::Bytes;

use super::{
    Format, MODEL_BYTES,
    json::{JsonString, Piece},
};
use crate::{
    codec::DecodeStep,
    error::CodecError,
    t2t::request::event::{RequestEvent, Role},
};

const WORK_BYTES: usize = 4096;
const CONTROL_BYTES: usize = 16;

fn invalid() -> CodecError {
    CodecError::InvalidInput("invalid or incomplete OpenAI text request")
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
    deferred: Vec<u8>,
    deferred_ready: Bytes,
    short_input: bool,
}

pub(super) struct Decoder {
    format: Format,
    phase: Phase,
    field: Field,
    root_seen: u8,
    has_message: bool,
    token: Option<Token>,
    false_literal: Option<usize>,
    small: Vec<u8>,
    model_bytes: usize,
    message: Option<Message>,
    fragment: Vec<u8>,
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
            small: Vec::new(),
            model_bytes: 0,
            message: None,
            fragment: Vec::new(),
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

    fn event(&mut self) -> Option<RequestEvent> {
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
            if !message.deferred_ready.is_empty() {
                let end = self.fragment_bytes.min(message.deferred_ready.len());
                let bytes = message.deferred_ready.split_to(end);
                return Some(RequestEvent::TextFragment(bytes));
            }
            if !self.fragment.is_empty()
                && (self.fragment.len() == self.fragment_bytes || message.content_done)
            {
                return Some(RequestEvent::TextFragment(Bytes::from(std::mem::take(
                    &mut self.fragment,
                ))));
            }
            if message.content_done && !message.text_ended {
                message.text_ended = true;
                return Some(RequestEvent::TextEnded);
            }
            if message.closed && message.text_ended {
                self.message = None;
                self.has_message = true;
                return Some(RequestEvent::MessageEnded);
            }
        }
        None
    }

    fn start_string(&mut self, purpose: Purpose) {
        self.small.clear();
        self.token = Some(Token {
            purpose,
            string: JsonString::default(),
        });
    }

    fn finish_string(&mut self, purpose: Purpose) -> Result<(), CodecError> {
        match purpose {
            Purpose::RootKey => {
                let (field, bit) = match self.small.as_slice() {
                    b"model" => (Field::Model, 1),
                    b"stream" => (Field::Stream, 2),
                    b"messages" if self.format == Format::Chat => (Field::Body, 4),
                    b"input" if self.format == Format::Responses => (Field::Body, 4),
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
                let (field, bit) = match self.small.as_slice() {
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
                    Some(match self.small.as_slice() {
                        b"system" => Role::System,
                        b"user" => Role::User,
                        b"assistant" => Role::Assistant,
                        _ => return Err(unsupported()),
                    });
                let message = self.message.as_mut().expect("message grammar");
                message.deferred_ready = Bytes::from(std::mem::take(&mut message.deferred));
                self.phase = Phase::MessageAfter;
            }
            Purpose::Type => {
                if self.small != b"message" {
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

    fn decoded_byte(&mut self, byte: u8, purpose: Purpose) -> Result<(), CodecError> {
        match purpose {
            Purpose::Content => {
                let message = self.message.as_mut().expect("message grammar");
                if message.role.is_none() {
                    if message.deferred.len() == self.deferred_bytes {
                        return Err(CodecError::LimitExceeded(
                            "text before role exceeds deferral allowance",
                        ));
                    }
                    message.deferred.push(byte);
                } else {
                    self.fragment.push(byte);
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
                if self.small.len() == CONTROL_BYTES {
                    return Err(unsupported());
                }
                self.small.push(byte);
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
        let result = self.step(input, eof);
        if result.is_err() {
            self.failed = true;
        }
        result
    }

    fn step(
        &mut self,
        input: &mut &[u8],
        eof: bool,
    ) -> Result<DecodeStep<RequestEvent>, CodecError> {
        if let Some(event) = self.event() {
            return Ok(DecodeStep::Event(event));
        }
        let mut consumed = 0;
        while consumed < WORK_BYTES {
            if self.piece_pos < self.piece_len {
                let byte = self.piece[self.piece_pos];
                self.piece_pos += 1;
                self.decoded_byte(byte, self.piece_purpose)?;
            } else {
                let Some((&byte, remaining)) = input.split_first() else {
                    break;
                };
                *input = remaining;
                consumed += 1;
                self.consume(byte)?;
            }
            if let Some(event) = self.event() {
                return Ok(DecodeStep::Event(event));
            }
        }
        if input.is_empty() && eof {
            if self.token.is_some()
                || self.false_literal.is_some()
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
