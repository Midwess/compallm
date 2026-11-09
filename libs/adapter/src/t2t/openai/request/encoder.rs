use bytes::Bytes;

use super::{Format, MODEL_BYTES, json::Utf8};
use crate::{
    codec::EncodeStep,
    error::CodecError,
    t2t::request::event::{RequestEvent, Role},
};

enum Segment {
    Literal(&'static [u8], usize),
    Text {
        bytes: Bytes,
        offset: usize,
        escape: [u8; 6],
        escaped: usize,
        escape_len: usize,
    },
}

impl Segment {
    fn text(bytes: Bytes) -> Self {
        Self::Text {
            bytes,
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
                bytes,
                offset,
                escaped,
                escape_len,
                ..
            } => *offset == bytes.len() && escaped == escape_len,
        }
    }
    fn write(&mut self, output: &mut [u8], utf8: &mut Utf8) -> Result<usize, CodecError> {
        match self {
            Self::Literal(bytes, offset) => {
                let len = output.len().min(bytes.len() - *offset);
                output[..len].copy_from_slice(&bytes[*offset..*offset + len]);
                *offset += len;
                Ok(len)
            }
            Self::Text {
                bytes,
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
                    if *offset == bytes.len() {
                        break;
                    }
                    let byte = bytes[*offset];
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

/// At most four syntax/data segments for a single event; never an event queue or
/// a serialized body buffer. Text segments retain only their supplied Bytes.
struct Output {
    segments: [Option<Segment>; 4],
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
    fn write(&mut self, output: &mut [u8], utf8: &mut Utf8) -> Result<usize, CodecError> {
        let mut written = 0;
        while !self.empty() && written < output.len() {
            let segment = self.segments[self.head]
                .as_mut()
                .expect("occupied output slot");
            written += segment.write(&mut output[written..], utf8)?;
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

pub(super) struct Encoder {
    format: Format,
    phase: Phase,
    output: Output,
    has_message: bool,
    utf8: Utf8,
    fragment_bytes: usize,
    finished: bool,
    failed: bool,
    ready: bool,
}

impl Encoder {
    pub fn new(format: Format, model: &str, fragment_bytes: usize) -> Result<Self, CodecError> {
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
        let mut output = Output::new();
        output.literal(b"{\"model\":\"");
        output.push(Segment::text(Bytes::copy_from_slice(model.as_bytes())));
        output.literal(match format {
            Format::Chat => b"\",\"stream\":false,\"messages\":[",
            Format::Responses => b"\",\"stream\":false,\"input\":[",
        });
        Ok(Self {
            format,
            phase: Phase::Outside,
            output,
            has_message: false,
            utf8: Utf8::default(),
            fragment_bytes,
            finished: false,
            failed: false,
            ready: false,
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
            (Phase::Outside, RequestEvent::MessageStarted { role }) => {
                if self.has_message {
                    self.output.literal(b",");
                }
                self.output.literal(if self.format == Format::Chat {
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
            (Phase::Text, RequestEvent::TextFragment(bytes)) => {
                if bytes.len() > self.fragment_bytes {
                    return Err(CodecError::LimitExceeded(
                        "text fragment exceeds encoder allowance",
                    ));
                }
                if !bytes.is_empty() {
                    self.output.push(Segment::text(bytes));
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
                self.output.literal(b"}");
                self.has_message = true;
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

    pub fn advance(&mut self, output: &mut [u8]) -> Result<EncodeStep, CodecError> {
        if self.failed {
            return Err(CodecError::Contract("encoder previously failed"));
        }
        if output.is_empty() {
            self.failed = true;
            return Err(CodecError::Contract(
                "encoder output slice must not be empty",
            ));
        }
        let len = output.len().min(4096);
        let written = match self.output.write(&mut output[..len], &mut self.utf8) {
            Ok(written) => written,
            Err(error) => {
                self.failed = true;
                return Err(error);
            }
        };
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
        self.output.literal(b"]}");
        self.finished = true;
        self.ready = false;
        Ok(())
    }
}
