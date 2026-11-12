use super::{
    super::{
        RequestScratch,
        event::RequestEvent,
        json::{JsonString, Piece},
    },
    SchemaValue,
};
use crate::{codec::DecodeStep, error::CodecError};

pub(crate) const REASONING_PREFIX: &[u8] = b"{\"type\":\"reasoning\"";

pub(crate) struct ReasoningValue {
    syntax: SchemaValue,
    depth: usize,
    string: Option<JsonString>,
    key: bool,
    expect_key: bool,
    expect_value: bool,
    field: u8,
    seen: u8,
    control: [u8; 32],
    len: usize,
    pub done: bool,
}

impl ReasoningValue {
    pub fn new() -> Result<Self, CodecError> {
        let mut syntax = SchemaValue::default();
        for &byte in REASONING_PREFIX {
            syntax.push(byte)?;
        }
        Ok(Self {
            syntax,
            depth: 1,
            string: None,
            key: false,
            expect_key: false,
            expect_value: false,
            field: 0,
            seen: 0,
            control: [0; 32],
            len: 0,
            done: false,
        })
    }

    pub fn push(&mut self, byte: u8) -> Result<(), CodecError> {
        let invalid = || CodecError::InvalidInput("invalid Responses reasoning item");
        self.syntax.push(byte)?;
        if let Some(string) = &mut self.string {
            match string.push(byte)? {
                Some(Piece::Bytes(bytes, len)) if self.key => {
                    if len > self.control.len() - self.len {
                        return Err(invalid());
                    }
                    self.control[self.len..self.len + len].copy_from_slice(&bytes[..len]);
                    self.len += len;
                }
                Some(Piece::End) => {
                    self.string = None;
                    if self.key {
                        let bit = match &self.control[..self.len] {
                            b"id" => 1,
                            b"summary" => 2,
                            b"encrypted_content" => 4,
                            b"status" => 8,
                            _ => {
                                return Err(CodecError::Unsupported(
                                    "unsupported Responses reasoning field",
                                ));
                            }
                        };
                        if self.seen & bit != 0 {
                            return Err(invalid());
                        }
                        self.seen |= bit;
                        self.field = bit;
                        self.expect_key = false;
                    }
                }
                _ => {}
            }
            return Ok(());
        }
        if matches!(byte, b' ' | b'\n' | b'\r' | b'\t') {
            return Ok(());
        }
        if self.depth == 1 && self.expect_value {
            if !(self.field == 2 && byte == b'['
                || self.field != 2 && byte == b'"'
                || self.field == 4 && byte == b'n')
            {
                return Err(invalid());
            }
            self.expect_value = false;
        }
        match byte {
            b'"' => {
                self.key = self.depth == 1 && self.expect_key;
                self.len = 0;
                self.string = Some(JsonString::default());
            }
            b'{' | b'[' => self.depth += 1,
            b'}' | b']' => {
                self.depth -= 1;
                if self.depth == 0 {
                    if self.seen & 2 == 0 {
                        return Err(invalid());
                    }
                    self.done = true;
                }
            }
            b',' if self.depth == 1 => self.expect_key = true,
            b':' if self.depth == 1 => self.expect_value = true,
            _ => {}
        }
        Ok(())
    }
}

pub(super) struct Reasoning {
    validator: ReasoningValue,
    started: bool,
    emitted: bool,
    ended: bool,
    prefix: [u8; 256 * 6 + 8],
    prefix_len: usize,
    prefix_pos: usize,
}

impl Reasoning {
    pub fn new(id: &[u8]) -> Result<Self, CodecError> {
        let validator = ReasoningValue::new()?;
        let mut result = Self {
            validator,
            started: false,
            emitted: false,
            ended: false,
            prefix: [0; 256 * 6 + 8],
            prefix_len: 0,
            prefix_pos: 0,
        };
        if !id.is_empty() {
            result.prefix[..7].copy_from_slice(b",\"id\":\"");
            result.prefix_len = 7;
            for &byte in id {
                let escaped;
                let bytes: &[u8] = match byte {
                    b'"' | b'\\' => {
                        escaped = [b'\\', byte, 0, 0, 0, 0];
                        &escaped[..2]
                    }
                    0..=31 => {
                        escaped = [
                            b'\\',
                            b'u',
                            b'0',
                            b'0',
                            b"0123456789abcdef"[(byte >> 4) as usize],
                            b"0123456789abcdef"[(byte & 15) as usize],
                        ];
                        &escaped
                    }
                    _ => {
                        escaped = [byte, 0, 0, 0, 0, 0];
                        &escaped[..1]
                    }
                };
                let end = result.prefix_len + bytes.len();
                result.prefix[result.prefix_len..end].copy_from_slice(bytes);
                result.prefix_len = end;
            }
            result.prefix[result.prefix_len] = b'"';
            result.prefix_len += 1;
        }
        Ok(result)
    }

    pub fn advance(
        &mut self,
        input: &mut &[u8],
        eof: bool,
        scratch: &mut RequestScratch,
    ) -> Result<DecodeStep<RequestEvent>, CodecError> {
        if !self.started {
            self.started = true;
            return Ok(DecodeStep::Event(RequestEvent::ResponsesReasoningStarted));
        }
        if self.emitted {
            self.emitted = false;
            scratch.fragment_len = 0;
        }
        if self.validator.done {
            if self.ended {
                return Ok(DecodeStep::Finished);
            }
            self.ended = true;
            return Ok(DecodeStep::Event(RequestEvent::ResponsesReasoningEnded));
        }
        let available = input.len().min(scratch.text().len()).min(4096);
        if self.prefix_pos < self.prefix_len {
            let len = (self.prefix_len - self.prefix_pos).min(scratch.text().len());
            let bytes = &self.prefix[self.prefix_pos..self.prefix_pos + len];
            for &byte in bytes {
                self.validator.push(byte)?;
            }
            scratch.text_mut()[..len].copy_from_slice(bytes);
            scratch.fragment_len = len;
            self.prefix_pos += len;
            self.emitted = true;
            return Ok(DecodeStep::Event(RequestEvent::ResponsesReasoningFragment(
                len,
            )));
        }
        for _ in 0..available {
            let byte = input[0];
            self.validator.push(byte)?;
            *input = &input[1..];
            let index = scratch.fragment_len;
            scratch.text_mut()[index] = byte;
            scratch.fragment_len += 1;
            if self.validator.done {
                break;
            }
        }
        if scratch.fragment_len > 0 {
            self.emitted = true;
            return Ok(DecodeStep::Event(RequestEvent::ResponsesReasoningFragment(
                scratch.fragment_len,
            )));
        }
        if eof {
            return Err(CodecError::InvalidInput(
                "incomplete Responses reasoning item",
            ));
        }
        Ok(DecodeStep::NeedInput)
    }
}
