use super::output_format::FormatDecoder;
use super::stop::{CHAT_STOP_COUNT, STOP_BYTES, STOP_COUNT};
use super::{
    Format, MODEL_BYTES, RequestScratch,
    json::{JsonString, Number, Piece},
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
    /// Consume a prefix of `input`, returning at most one bounded event.
    ///
    /// Advance `input` to its remaining suffix. NeedInput requires an empty
    /// suffix and is invalid at EOF. Finished requires EOF and no remaining
    /// bytes. Retain bounded state, validate trailing input, and reject semantics
    /// that cannot be represented instead of silently discarding them.
    /// Input and scratch borrow reusable caller storage; do not retain either.
    /// Supplied scratch capacities determine fragment and role-late bounds.
    /// TextFragment and SchemaFragment report lengths into scratch.text(). Consume that prefix
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
    StreamOptions,
    IncludeUsage,
    TopP,
    Stop,
    OutputFormat,
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
    root_seen: u16,
    output_format: Option<FormatDecoder>,
    has_message: bool,
    token: Option<Token>,
    literal: Option<(&'static [u8], usize, bool)>,
    null_literal: Option<usize>,
    options_seen: bool,
    options_object: bool,
    stream: bool,
    include_usage: Option<bool>,
    pending_streaming: bool,
    probability: Option<Number>,
    probability_bytes: [u8; 256],
    probability_len: usize,
    pending_top_p: Option<f64>,
    pending_stop: Option<RequestEvent>,
    stop_start_pending: bool,
    stop_end_pending: bool,
    stop_list_end_pending: bool,
    stop_single: bool,
    stop_count: usize,
    stop_bytes: usize,
    stop_sequence_bytes: usize,
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
    failed: bool,
    completed: bool,
}

impl Decoder {
    pub fn new(format: Format) -> Self {
        Self {
            format,
            phase: Phase::Root,
            field: Field::Model,
            root_seen: 0,
            output_format: None,
            has_message: false,
            token: None,
            literal: None,
            null_literal: None,
            options_seen: false,
            options_object: false,
            stream: false,
            include_usage: None,
            pending_streaming: false,
            probability: None,
            probability_bytes: [0; 256],
            probability_len: 0,
            pending_top_p: None,
            pending_stop: None,
            stop_start_pending: false,
            stop_end_pending: false,
            stop_list_end_pending: false,
            stop_single: false,
            stop_count: 0,
            stop_bytes: 0,
            stop_sequence_bytes: 0,
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
            failed: false,
            completed: false,
        }
    }

    fn event(&mut self, scratch: &mut RequestScratch) -> Option<RequestEvent> {
        if let Some(event) = self.pending_stop.take() {
            return Some(event);
        }
        if self.stop_start_pending {
            self.stop_start_pending = false;
            return Some(RequestEvent::StopSequenceStarted);
        }
        if self.stop_end_pending
            || self
                .token
                .as_ref()
                .is_some_and(|token| matches!(token.purpose, Purpose::Stop))
        {
            if scratch.fragment_len > 0
                && (scratch.fragment_len == scratch.text().len() || self.stop_end_pending)
            {
                self.fragment_emitted = true;
                return Some(RequestEvent::TextFragment(scratch.fragment_len));
            }
            if self.stop_end_pending {
                self.stop_end_pending = false;
                return Some(RequestEvent::StopSequenceEnded);
            }
        }
        if self.stop_list_end_pending {
            self.stop_list_end_pending = false;
            return Some(RequestEvent::StopSequencesEnded);
        }
        if let Some(value) = self.pending_top_p.take() {
            return Some(RequestEvent::TopP(value));
        }
        if let Some(configuration) = self.configuration.take() {
            return Some(RequestEvent::Configuration(configuration));
        }
        if self.pending_streaming {
            self.pending_streaming = false;
            return Some(RequestEvent::Streaming {
                stream: self.stream,
                include_usage: self.include_usage,
            });
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
                let len = scratch
                    .text()
                    .len()
                    .min(scratch.deferred_len - message.deferred_pos);
                scratch.copy_deferred(message.deferred_pos, len);
                message.deferred_pos += len;
                self.fragment_emitted = true;
                return Some(RequestEvent::TextFragment(len));
            }
            if scratch.fragment_len > 0
                && (scratch.fragment_len == scratch.text().len() || message.content_done)
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
                    b"top_p" if self.format != Format::Anthropic => (Field::TopP, 64),
                    b"response_format" if self.format == Format::Chat => (Field::OutputFormat, 256),
                    b"text" if self.format == Format::Responses => (Field::OutputFormat, 256),
                    b"output_config" if self.format == Format::Anthropic => {
                        (Field::OutputFormat, 256)
                    }
                    b"stop" if self.format == Format::Chat => (Field::Stop, 128),
                    b"stop_sequences" if self.format == Format::Anthropic => (Field::Stop, 128),
                    b"stream_options" if self.format == Format::Chat => (Field::StreamOptions, 32),
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
            Purpose::OptionKey => {
                if &self.small[..self.small_len] != b"include_usage" {
                    return Err(unsupported());
                }
                if self.options_seen {
                    return Err(invalid());
                }
                self.options_seen = true;
                self.field = Field::IncludeUsage;
                self.phase = Phase::OptionsColon;
            }
            Purpose::Stop => {
                if self.stop_sequence_bytes == 0 {
                    return Err(invalid());
                }
                self.stop_count += 1;
                self.stop_end_pending = true;
                self.phase = if self.stop_single {
                    self.stop_list_end_pending = true;
                    Phase::RootAfter
                } else {
                    Phase::StopAfter
                };
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
            Purpose::Stop => {
                if self.stop_bytes == STOP_BYTES {
                    return Err(CodecError::LimitExceeded("stop text exceeds 4096 bytes"));
                }
                self.stop_bytes += 1;
                self.stop_sequence_bytes += 1;
                let index = scratch.fragment_len;
                scratch.text_mut()[index] = byte;
                scratch.fragment_len += 1;
            }
            Purpose::Content => {
                let message = self.message.as_mut().expect("message grammar");
                if message.role.is_none() {
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

    fn start_stop(&mut self) -> Result<(), CodecError> {
        let limit = if self.format == Format::Anthropic {
            STOP_COUNT
        } else {
            CHAT_STOP_COUNT
        };
        if self.stop_count == limit {
            return Err(CodecError::LimitExceeded("too many stop sequences"));
        }
        self.stop_sequence_bytes = 0;
        self.start_string(Purpose::Stop);
        self.stop_start_pending = true;
        Ok(())
    }

    fn consume(&mut self, byte: u8) -> Result<(), CodecError> {
        if let Some(mut number) = self.probability {
            if matches!(byte, b' ' | b'\n' | b'\r' | b'\t' | b',' | b'}') {
                self.pending_top_p =
                    Some(number.probability(&self.probability_bytes[..self.probability_len])?);
                self.probability = None;
                self.phase = Phase::RootAfter;
            } else {
                if self.probability_len == self.probability_bytes.len() {
                    return Err(CodecError::LimitExceeded(
                        "top_p number exceeds control capacity",
                    ));
                }
                number.push(byte)?;
                self.probability_bytes[self.probability_len] = byte;
                self.probability_len += 1;
                self.probability = Some(number);
                return Ok(());
            }
        }
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
        if let Some((literal, position, value)) = self.literal {
            if byte != literal[position] {
                return Err(invalid());
            }
            if position + 1 == literal.len() {
                self.literal = None;
                if matches!(self.field, Field::Stream) {
                    self.stream = value;
                    self.phase = Phase::RootAfter;
                } else {
                    self.include_usage = Some(value);
                    self.phase = Phase::OptionsAfter;
                }
            } else {
                self.literal = Some((literal, position + 1, value));
            }
            return Ok(());
        }
        if let Some(position) = self.null_literal {
            if byte != b"null"[position] {
                return Err(invalid());
            }
            if position == 3 {
                self.null_literal = None;
                self.phase = Phase::RootAfter;
                if matches!(self.field, Field::Stop) {
                    self.stop_list_end_pending = true;
                }
            } else {
                self.null_literal = Some(position + 1);
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
                if self.options_object && !self.stream {
                    return Err(invalid());
                }
                if self.root_seen & 34 != 0 {
                    self.pending_streaming = true;
                }
                self.phase = Phase::Done;
            }
            (Phase::RootColon, b':') => self.phase = Phase::RootValue,
            (Phase::RootValue, _) => match self.field {
                Field::Model if byte == b'"' => {
                    self.model_bytes = 0;
                    self.start_string(Purpose::Model);
                }
                Field::Stream if byte == b'f' => self.literal = Some((b"false", 1, false)),
                Field::Stream if byte == b't' => self.literal = Some((b"true", 1, true)),
                Field::TopP if byte.is_ascii_digit() || byte == b'-' => {
                    let mut number = Number::Start;
                    number.push(byte)?;
                    self.probability = Some(number);
                    self.probability_bytes[0] = byte;
                    self.probability_len = 1;
                }
                Field::TopP if byte == b'n' => self.null_literal = Some(1),
                Field::OutputFormat if byte == b'{' => {
                    self.output_format = Some(FormatDecoder::new(self.format))
                }
                Field::OutputFormat if byte == b'n' => self.null_literal = Some(1),
                Field::Stop if byte == b'[' => {
                    self.pending_stop = Some(RequestEvent::StopSequencesStarted);
                    self.phase = Phase::StopFirst;
                }
                Field::Stop if byte == b'"' && self.format == Format::Chat => {
                    self.pending_stop = Some(RequestEvent::StopSequencesStarted);
                    self.stop_single = true;
                    self.start_stop()?;
                }
                Field::Stop if byte == b'n' && self.format == Format::Chat => {
                    self.pending_stop = Some(RequestEvent::StopSequencesStarted);
                    self.null_literal = Some(1);
                }
                Field::StreamOptions if byte == b'{' => {
                    self.options_object = true;
                    self.phase = Phase::OptionsKey(true);
                }
                Field::StreamOptions if byte == b'n' => self.null_literal = Some(1),
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
            (Phase::OptionsKey(_), b'"') => self.start_string(Purpose::OptionKey),
            (Phase::OptionsColon, b':') => self.phase = Phase::OptionsValue,
            (Phase::OptionsValue, b't') => self.literal = Some((b"true", 1, true)),
            (Phase::OptionsValue, b'f') => self.literal = Some((b"false", 1, false)),
            (Phase::OptionsAfter, b',') => self.phase = Phase::OptionsKey(false),
            (Phase::OptionsKey(true) | Phase::OptionsAfter, b'}') => self.phase = Phase::RootAfter,
            (Phase::StopFirst | Phase::StopNext, b'"') => self.start_stop()?,
            (Phase::StopFirst | Phase::StopAfter, b']') => {
                self.phase = Phase::RootAfter;
                self.stop_list_end_pending = true;
            }
            (Phase::StopAfter, b',') => self.phase = Phase::StopNext,
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
            if let Some(decoder) = &mut self.output_format {
                let available = (WORK_BYTES - consumed).min(input.len());
                let final_window = eof && available == input.len();
                let mut window = &input[..available];
                let result = decoder.advance(&mut window, final_window, scratch);
                let used = available - window.len();
                *input = &input[used..];
                consumed += used;
                let step = result?;
                if matches!(step, DecodeStep::Finished) {
                    self.output_format = None;
                    self.phase = Phase::RootAfter;
                    continue;
                }
                if matches!(step, DecodeStep::NeedInput) && !input.is_empty() {
                    return Ok(DecodeStep::Progress);
                }
                return Ok(step);
            }
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
                || self.literal.is_some()
                || self.null_literal.is_some()
                || self.probability.is_some()
                || self.max_tokens.is_some()
                || !matches!(self.phase, Phase::Done)
                || self.message.is_some()
                || self.output_format.is_some()
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
