use super::{Format, MODEL_BYTES, json::Utf8};
use crate::{
    codec::EncodeStep,
    error::CodecError,
    t2t::{
        Text2TextTranslatorConfiguration,
        request::event::{RequestEvent, Role},
    },
};

pub trait RequestEncoder {
    /// Called only after NeedEvent. Retain metadata, never payload bytes.
    fn accept(&mut self, event: RequestEvent) -> Result<(), CodecError>;

    /// Write into the supplied slice without blocking. Wrote must report a
    /// nonzero length within the slice. NeedEvent must remain repeatable until
    /// accept/finish. Finished is valid only after finish and all output drains.
    /// Consume the pending text suffix; caller retains it across output flushes.
    fn advance(&mut self, text: &mut &[u8], output: &mut [u8]) -> Result<EncodeStep, CodecError>;

    /// Called once after source EOF validation, while the encoder needs an event.
    /// Subsequent advance calls must drain final output and then return Finished.
    fn finish(&mut self) -> Result<(), CodecError>;
}

impl<E: RequestEncoder + ?Sized> RequestEncoder for Box<E> {
    fn accept(&mut self, event: RequestEvent) -> Result<(), CodecError> {
        (**self).accept(event)
    }
    fn advance(&mut self, text: &mut &[u8], output: &mut [u8]) -> Result<EncodeStep, CodecError> {
        (**self).advance(text, output)
    }
    fn finish(&mut self) -> Result<(), CodecError> {
        (**self).finish()
    }
}

enum TextSource {
    Model,
    Fragment,
    Number,
}

enum Segment {
    Literal(&'static [u8], usize),
    Text {
        source: TextSource,
        len: usize,
        offset: usize,
        escape: [u8; 6],
        escaped: usize,
        escape_len: usize,
    },
}

impl Segment {
    fn text(source: TextSource, len: usize) -> Self {
        Self::Text {
            source,
            len,
            offset: 0,
            escape: [0; 6],
            escaped: 0,
            escape_len: 0,
        }
    }
    fn done(&self) -> bool {
        match self {
            Self::Literal(bytes, offset) => *offset == bytes.len(),
            Self::Text {
                len,
                offset,
                escaped,
                escape_len,
                ..
            } => offset == len && escaped == escape_len,
        }
    }
    fn write(
        &mut self,
        text: &mut &[u8],
        model: &[u8],
        number: &[u8],
        output: &mut [u8],
        utf8: &mut Utf8,
    ) -> Result<usize, CodecError> {
        match self {
            Self::Literal(bytes, offset) => {
                let len = output.len().min(bytes.len() - *offset);
                output[..len].copy_from_slice(&bytes[*offset..*offset + len]);
                *offset += len;
                Ok(len)
            }
            Self::Text {
                source,
                len,
                offset,
                escape,
                escaped,
                escape_len,
            } => {
                let mut written = 0;
                while written < output.len() {
                    if *escaped < *escape_len {
                        let len = (output.len() - written).min(*escape_len - *escaped);
                        output[written..written + len]
                            .copy_from_slice(&escape[*escaped..*escaped + len]);
                        *escaped += len;
                        written += len;
                        continue;
                    }
                    if *offset == *len {
                        break;
                    }
                    let byte = match source {
                        TextSource::Model => model[*offset],
                        TextSource::Number => number[*offset],
                        TextSource::Fragment => {
                            let Some((&byte, remaining)) = text.split_first() else {
                                return Err(CodecError::Contract(
                                    "pending text suffix is incomplete",
                                ));
                            };
                            *text = remaining;
                            byte
                        }
                    };
                    utf8.push(byte)?;
                    *offset += 1;
                    *escaped = 0;
                    *escape_len = match byte {
                        b'"' | b'\\' => {
                            escape[0] = b'\\';
                            escape[1] = byte;
                            2
                        }
                        0..=0x1f => {
                            escape[..4].copy_from_slice(b"\\u00");
                            escape[4] = b"0123456789abcdef"[(byte >> 4) as usize];
                            escape[5] = b"0123456789abcdef"[(byte & 15) as usize];
                            6
                        }
                        _ => {
                            escape[0] = byte;
                            1
                        }
                    };
                }
                Ok(written)
            }
        }
    }
}

/// A fixed number of syntax/data segments; never an event queue or
/// a serialized body buffer. Text segments retain only source/offset metadata.
struct Output {
    segments: [Option<Segment>; 6],
    head: usize,
    len: usize,
}
impl Output {
    fn new() -> Self {
        Self {
            segments: std::array::from_fn(|_| None),
            head: 0,
            len: 0,
        }
    }
    fn literal(&mut self, value: &'static [u8]) {
        self.push(Segment::Literal(value, 0));
    }
    fn push(&mut self, value: Segment) {
        assert!(
            self.len < self.segments.len(),
            "one event has bounded output segments"
        );
        self.segments[(self.head + self.len) % self.segments.len()] = Some(value);
        self.len += 1;
    }
    fn empty(&self) -> bool {
        self.len == 0
    }
    fn write(
        &mut self,
        text: &mut &[u8],
        model: &[u8],
        number: &[u8],
        output: &mut [u8],
        utf8: &mut Utf8,
    ) -> Result<usize, CodecError> {
        let mut written = 0;
        while !self.empty() && written < output.len() {
            let segment = self.segments[self.head]
                .as_mut()
                .expect("occupied output slot");
            written += segment.write(text, model, number, &mut output[written..], utf8)?;
            if segment.done() {
                self.segments[self.head] = None;
                self.head = (self.head + 1) % self.segments.len();
                self.len -= 1;
            } else {
                break;
            }
        }
        Ok(written)
    }
}

#[derive(Clone, Copy)]
enum Phase {
    Outside,
    Message,
    Text,
    AfterText,
}

pub(crate) struct Encoder<'model> {
    format: Format,
    phase: Phase,
    output: Output,
    has_message: bool,
    has_system: bool,
    system: bool,
    utf8: Utf8,
    fragment_bytes: usize,
    finished: bool,
    failed: bool,
    ready: bool,
    configuration: Text2TextTranslatorConfiguration,
    has_configuration: bool,
    model: &'model str,
    number: [u8; 10],
    number_start: usize,
    text_remaining: usize,
}

impl<'model> Encoder<'model> {
    pub fn new(
        format: Format,
        model: &'model str,
        fragment_bytes: usize,
        configuration: Text2TextTranslatorConfiguration,
    ) -> Result<Self, CodecError> {
        if model.is_empty() {
            return Err(CodecError::InvalidInput("target model must not be empty"));
        }
        if model.len() > MODEL_BYTES {
            return Err(CodecError::LimitExceeded(
                "target model exceeds identifier allowance",
            ));
        }
        if fragment_bytes == 0 {
            return Err(CodecError::InvalidInput("fragment_bytes must exceed zero"));
        }
        if configuration.max_output_tokens == 0 {
            return Err(CodecError::InvalidInput(
                "max_output_tokens must exceed zero",
            ));
        }
        let mut output = Output::new();
        output.literal(b"{\"model\":\"");
        output.push(Segment::text(TextSource::Model, model.len()));
        match format {
            Format::Chat => output.literal(b"\",\"stream\":false,\"messages\":["),
            Format::Responses => output.literal(b"\",\"stream\":false,\"input\":["),
            Format::Anthropic => output.literal(b"\",\"stream\":false"),
        }
        Ok(Self {
            format,
            phase: Phase::Outside,
            output,
            has_message: false,
            has_system: false,
            system: false,
            utf8: Utf8::default(),
            fragment_bytes,
            finished: false,
            failed: false,
            ready: false,
            configuration,
            has_configuration: false,
            model,
            number: [0; 10],
            number_start: 10,
            text_remaining: 0,
        })
    }

    pub fn accept(&mut self, event: RequestEvent) -> Result<(), CodecError> {
        if self.failed {
            return Err(CodecError::Contract("encoder previously failed"));
        }
        let result = self.event(event);
        if result.is_err() {
            self.failed = true;
        }
        result
    }

    fn event(&mut self, event: RequestEvent) -> Result<(), CodecError> {
        if self.finished || !self.output.empty() || !self.ready {
            return Err(CodecError::Contract(
                "encoder accepts events only after NeedEvent",
            ));
        }
        self.ready = false;
        match (self.phase, event) {
            (Phase::Outside, RequestEvent::Configuration(configuration)) => {
                if self.has_configuration || configuration.max_output_tokens == 0 {
                    return Err(CodecError::InvalidInput(
                        "invalid or duplicate request configuration",
                    ));
                }
                self.configuration = configuration;
                self.has_configuration = true;
            }
            (Phase::Outside, RequestEvent::MessageStarted { role }) => {
                self.system = self.format == Format::Anthropic && role == Role::System;
                if self.system {
                    if self.has_message || self.has_system {
                        return Err(CodecError::Unsupported(
                            "system must be a single leading message",
                        ));
                    }
                    self.output.literal(b",\"system\":");
                    self.phase = Phase::Message;
                    return Ok(());
                }
                if self.format == Format::Anthropic && !self.has_message {
                    self.output.literal(b",\"messages\":[");
                }
                if self.has_message {
                    self.output.literal(b",");
                }
                self.output.literal(if self.format != Format::Responses {
                    b"{\"role\":\""
                } else {
                    b"{\"type\":\"message\",\"role\":\""
                });
                self.output.literal(match role {
                    Role::System => b"system",
                    Role::User => b"user",
                    Role::Assistant => b"assistant",
                });
                self.output.literal(b"\",\"content\":");
                self.phase = Phase::Message;
            }
            (Phase::Message, RequestEvent::TextStarted) => {
                self.utf8 = Utf8::default();
                self.output.literal(b"\"");
                self.phase = Phase::Text;
            }
            (Phase::Text, RequestEvent::TextFragment(len)) => {
                if len > self.fragment_bytes {
                    return Err(CodecError::LimitExceeded(
                        "text fragment exceeds encoder allowance",
                    ));
                }
                self.text_remaining = len;
                if len > 0 {
                    self.output.push(Segment::text(TextSource::Fragment, len));
                }
            }
            (Phase::Text, RequestEvent::TextEnded) => {
                if !self.utf8.complete() {
                    return Err(CodecError::InvalidInput("incomplete UTF-8 text"));
                }
                self.output.literal(b"\"");
                self.phase = Phase::AfterText;
            }
            (Phase::AfterText, RequestEvent::MessageEnded) => {
                if self.system {
                    self.has_system = true;
                } else {
                    self.output.literal(b"}");
                    self.has_message = true;
                }
                self.phase = Phase::Outside;
            }
            (Phase::AfterText, RequestEvent::TextStarted) => {
                return Err(CodecError::Unsupported(
                    "multiple text parts are outside the string-content subset",
                ));
            }
            _ => {
                return Err(CodecError::InvalidInput(
                    "invalid text request event ordering",
                ));
            }
        }
        Ok(())
    }

    pub fn advance(
        &mut self,
        text: &mut &[u8],
        output: &mut [u8],
    ) -> Result<EncodeStep, CodecError> {
        if self.failed {
            return Err(CodecError::Contract("encoder previously failed"));
        }
        if output.is_empty() {
            self.failed = true;
            return Err(CodecError::Contract(
                "encoder output slice must not be empty",
            ));
        }
        if text.len() != self.text_remaining {
            self.failed = true;
            return Err(CodecError::Contract(
                "pending text length does not match event progress",
            ));
        }
        let len = output.len().min(4096);
        let before = text.len();
        let written = match self.output.write(
            text,
            self.model.as_bytes(),
            &self.number[self.number_start..],
            &mut output[..len],
            &mut self.utf8,
        ) {
            Ok(written) => written,
            Err(error) => {
                self.failed = true;
                return Err(error);
            }
        };
        self.text_remaining -= before - text.len();
        if written > 0 {
            self.ready = false;
            return Ok(EncodeStep::Wrote(written));
        }
        self.ready = !self.finished;
        Ok(if self.finished {
            EncodeStep::Finished
        } else {
            EncodeStep::NeedEvent
        })
    }

    pub fn finish(&mut self) -> Result<(), CodecError> {
        if self.failed {
            return Err(CodecError::Contract("encoder previously failed"));
        }
        if self.finished || !self.output.empty() || !self.ready {
            self.failed = true;
            return Err(CodecError::Contract(
                "finish requires NeedEvent and a fresh finalization",
            ));
        }
        if !matches!(self.phase, Phase::Outside) || !self.has_message {
            self.failed = true;
            return Err(CodecError::InvalidInput(
                "request requires complete text messages",
            ));
        }
        self.output.literal(match self.format {
            Format::Chat => b"],\"max_completion_tokens\":",
            Format::Responses => b"],\"max_output_tokens\":",
            Format::Anthropic => b"],\"max_tokens\":",
        });
        let mut value = self.configuration.max_output_tokens;
        self.number_start = self.number.len();
        while value > 0 {
            self.number_start -= 1;
            self.number[self.number_start] = b'0' + (value % 10) as u8;
            value /= 10;
        }
        self.output.push(Segment::text(
            TextSource::Number,
            self.number.len() - self.number_start,
        ));
        self.output.literal(b"}");
        self.finished = true;
        self.ready = false;
        Ok(())
    }
}
