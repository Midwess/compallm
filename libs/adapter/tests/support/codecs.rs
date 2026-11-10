//! Deliberately bounded-object demo codecs, not production OpenAI support.

use std::collections::VecDeque;

use adapter::{
    codec::{DecodeStep, EncodeStep},
    error::CodecError,
    t2t::request::{
        RequestScratch,
        decoder::RequestDecoder,
        encoder::RequestEncoder,
        event::{RequestEvent, Role},
    },
};
use bytes::{Buf, Bytes};
use serde::Deserialize;

const MAX_VALUE: usize = 64 * 1024;

fn invalid() -> CodecError {
    CodecError::InvalidInput("expected the example's bounded text-only OpenAI JSON subset")
}

#[derive(Clone, Copy)]
enum Phase {
    Root,
    Key { allow_end: bool },
    Colon,
    Value,
    AfterField,
    Messages,
    Message { allow_end: bool },
    AfterMessage,
    End,
}

/// Captures one bounded JSON key, scalar, or message object. The complete
/// captured value is subsequently validated by serde_json, not this scanner.
#[derive(Default)]
struct Scan {
    bytes: Vec<u8>,
    depth: usize,
    quoted: bool,
    escaped: bool,
}

impl Scan {
    fn push(&mut self, byte: u8) -> Result<bool, CodecError> {
        if self.bytes.len() == MAX_VALUE {
            return Err(CodecError::LimitExceeded("one JSON value exceeds 64 KiB"));
        }
        self.bytes.push(byte);
        if self.escaped {
            self.escaped = false;
        } else if self.quoted {
            match byte {
                b'\\' => self.escaped = true,
                b'"' => {
                    self.quoted = false;
                    if self.depth == 0 {
                        return Ok(true);
                    }
                }
                _ => {}
            }
        } else {
            match byte {
                b'"' => self.quoted = true,
                b'{' | b'[' => {
                    self.depth += 1;
                    if self.depth > 32 {
                        return Err(CodecError::LimitExceeded("JSON nesting exceeds 32"));
                    }
                }
                b'}' | b']' => {
                    self.depth = self.depth.checked_sub(1).ok_or_else(invalid)?;
                    if self.depth == 0 {
                        return Ok(true);
                    }
                }
                _ => {}
            }
        }
        // The only unquoted scalar accepted here is false.
        Ok(self.bytes.first() == Some(&b'f') && self.bytes.len() == 5)
    }

    fn take(&mut self) -> Vec<u8> {
        std::mem::take(&mut self.bytes)
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Message {
    role: String,
    content: String,
}

pub struct OpenAiTextDecoder {
    phase: Phase,
    scan: Scan,
    scanning: bool,
    field: u8,
    seen: u8,
    message_count: usize,
    events: VecDeque<RequestEvent<Bytes>>,
}

impl Default for OpenAiTextDecoder {
    fn default() -> Self {
        Self {
            phase: Phase::Root,
            scan: Scan::default(),
            scanning: false,
            field: 0,
            seen: 0,
            message_count: 0,
            events: VecDeque::new(),
        }
    }
}

impl OpenAiTextDecoder {
    fn complete_value(&mut self) -> Result<(), CodecError> {
        let bytes = self.scan.take();
        match self.phase {
            Phase::Key { .. } => {
                let key: String = serde_json::from_slice(&bytes).map_err(|_| invalid())?;
                self.field = match key.as_str() {
                    "model" => 1,
                    "messages" => 2,
                    "stream" => 4,
                    _ => return Err(CodecError::Unsupported("unknown request field")),
                };
                if self.seen & self.field != 0 {
                    return Err(invalid());
                }
                self.seen |= self.field;
                self.phase = Phase::Colon;
            }
            Phase::Value => {
                match self.field {
                    1 => {
                        let model: String =
                            serde_json::from_slice(&bytes).map_err(|_| invalid())?;
                        if model.is_empty() {
                            return Err(invalid());
                        }
                    }
                    4 => {
                        let stream: bool = serde_json::from_slice(&bytes).map_err(|_| invalid())?;
                        if stream {
                            return Err(CodecError::Unsupported("stream must be false"));
                        }
                    }
                    _ => return Err(invalid()),
                }
                self.phase = Phase::AfterField;
            }
            Phase::Message { .. } => {
                let message: Message = serde_json::from_slice(&bytes).map_err(|_| invalid())?;
                let role = match message.role.as_str() {
                    "user" => Role::User,
                    "assistant" => Role::Assistant,
                    _ => {
                        return Err(CodecError::Unsupported(
                            "only user/assistant roles are supported",
                        ));
                    }
                };
                self.events.extend([
                    RequestEvent::MessageStarted { role },
                    RequestEvent::TextStarted,
                    RequestEvent::TextFragment(Bytes::from(message.content)),
                    RequestEvent::TextEnded,
                    RequestEvent::MessageEnded,
                ]);
                self.message_count += 1;
                self.phase = Phase::AfterMessage;
            }
            _ => return Err(invalid()),
        }
        Ok(())
    }

    fn byte(&mut self, byte: u8) -> Result<(), CodecError> {
        if self.scanning {
            if self.scan.push(byte)? {
                self.scanning = false;
                self.complete_value()?;
            }
            return Ok(());
        }
        if matches!(byte, b' ' | b'\n' | b'\r' | b'\t') {
            return Ok(());
        }
        match (self.phase, byte) {
            (Phase::Root, b'{') => self.phase = Phase::Key { allow_end: true },
            (Phase::Key { allow_end: true }, b'}') => self.phase = Phase::End,
            (Phase::Key { .. }, b'"') => self.scanning = true,
            (Phase::Colon, b':') => {
                self.phase = if self.field == 2 {
                    Phase::Messages
                } else {
                    Phase::Value
                };
            }
            (Phase::Value, b'"') if self.field == 1 => self.scanning = true,
            (Phase::Value, b'f') if self.field == 4 => self.scanning = true,
            (Phase::Messages, b'[') => self.phase = Phase::Message { allow_end: true },
            (Phase::Message { allow_end: true }, b']') => self.phase = Phase::AfterField,
            (Phase::Message { .. }, b'{') => self.scanning = true,
            (Phase::AfterMessage, b',') => self.phase = Phase::Message { allow_end: false },
            (Phase::AfterMessage, b']') => self.phase = Phase::AfterField,
            (Phase::AfterField, b',') => self.phase = Phase::Key { allow_end: false },
            (Phase::AfterField, b'}') => self.phase = Phase::End,
            _ => return Err(invalid()),
        }
        if self.scanning {
            self.scan.push(byte)?;
        }
        Ok(())
    }
}

impl RequestDecoder for OpenAiTextDecoder {
    fn scratch_requirements(&self) -> (usize, usize) {
        (MAX_VALUE, 0)
    }
    fn advance(
        &mut self,
        input: &mut &[u8],
        eof: bool,
        scratch: &mut RequestScratch,
    ) -> Result<DecodeStep<RequestEvent>, CodecError> {
        if let Some(event) = self.events.pop_front() {
            return Ok(DecodeStep::Event(event.map_text(|bytes| {
                scratch.text_mut()[..bytes.len()].copy_from_slice(&bytes);
                bytes.len()
            })));
        }
        let count = input.len().min(4096);
        for _ in 0..count {
            self.byte(input.get_u8())?;
            if let Some(event) = self.events.pop_front() {
                return Ok(DecodeStep::Event(event.map_text(|bytes| {
                    scratch.text_mut()[..bytes.len()].copy_from_slice(&bytes);
                    bytes.len()
                })));
            }
        }
        if eof && input.is_empty() {
            if self.scanning
                || !matches!(self.phase, Phase::End)
                || self.seen & 3 != 3
                || self.message_count == 0
            {
                return Err(invalid());
            }
            Ok(DecodeStep::Finished)
        } else if count > 0 {
            Ok(DecodeStep::Progress)
        } else {
            Ok(DecodeStep::NeedInput)
        }
    }
}

#[derive(Clone)]
pub struct TargetProperties {
    pub model: String,
    pub max_tokens: std::num::NonZeroU32,
}

pub struct AnthropicTextEncoder {
    pending: Bytes,
    text_len: Option<usize>,
    first: bool,
    phase: u8,
    finished: bool,
}

impl AnthropicTextEncoder {
    pub fn new(properties: &TargetProperties) -> Self {
        let model = serde_json::to_string(&properties.model).expect("string serialization");
        Self {
            pending: Bytes::from(format!(
                "{{\"model\":{model},\"max_tokens\":{},\"stream\":false,\"messages\":[",
                properties.max_tokens
            )),
            first: true,
            text_len: None,
            phase: 0,
            finished: false,
        }
    }
}

impl RequestEncoder for AnthropicTextEncoder {
    fn accept(&mut self, event: RequestEvent) -> Result<(), CodecError> {
        match (self.phase, event) {
            (0, RequestEvent::MessageStarted { role }) => {
                let role = match role {
                    Role::User => "user",
                    Role::Assistant => "assistant",
                    _ => return Err(CodecError::Unsupported("system role")),
                };
                self.pending = Bytes::from(format!(
                    "{}{{\"role\":\"{role}\",\"content\":[{{\"type\":\"text\",\"text\":",
                    if self.first { "" } else { "," }
                ));
                self.first = false;
                self.phase = 1;
            }
            (1, RequestEvent::TextStarted) => self.phase = 2,
            (2, RequestEvent::TextFragment(len)) => {
                self.text_len = Some(len);
                self.phase = 3;
            }
            (3, RequestEvent::TextEnded) => self.phase = 4,
            (4, RequestEvent::MessageEnded) => {
                self.pending = Bytes::from_static(b"}]}");
                self.phase = 0;
            }
            _ => return Err(CodecError::Contract("unexpected example event sequence")),
        }
        Ok(())
    }

    fn advance(&mut self, text: &mut &[u8], output: &mut [u8]) -> Result<EncodeStep, CodecError> {
        if let Some(len) = self.text_len.take() {
            assert_eq!(text.len(), len);
            let value = std::str::from_utf8(text).map_err(|_| invalid())?;
            self.pending = Bytes::from(serde_json::to_vec(value).map_err(|_| invalid())?);
            *text = &[];
        }
        if !self.pending.is_empty() {
            let count = output.len().min(self.pending.len());
            output[..count].copy_from_slice(&self.pending[..count]);
            self.pending.advance(count);
            Ok(EncodeStep::Wrote(count))
        } else if self.finished {
            Ok(EncodeStep::Finished)
        } else {
            Ok(EncodeStep::NeedEvent)
        }
    }

    fn finish(&mut self) -> Result<(), CodecError> {
        if self.phase != 0 || self.first {
            return Err(invalid());
        }
        self.finished = true;
        self.pending = Bytes::from_static(b"]}");
        Ok(())
    }
}
