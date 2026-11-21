use super::JsonToken;
use crate::encoder::{Encoder, EncoderStatus};

#[derive(Debug, thiserror::Error)]
pub enum JsonEncodeError {
    #[error("invalid JSON encoder state")]
    InvalidState,
    #[error("incomplete JSON document")]
    Incomplete,
    #[error("JSON value serialization failed: {0}")]
    Serialize(#[source] serde_json::Error),
}

#[derive(Clone, Copy, Debug, Default)]
struct Container {
    object: bool,
    populated: bool,
}

#[derive(Debug)]
pub struct JsonEncoder {
    stack: [Container; 64],
    depth: usize,
    root: bool,
    failed: bool,
    finished: bool,
    active: bool,
    comma: bool,
    phase: u8,
    position: usize,
    escape_position: usize,
    serialized: Vec<u8>,
    serialized_position: usize,
    serializing: bool,
}

impl Default for JsonEncoder {
    fn default() -> Self {
        Self {
            stack: [Container::default(); 64],
            depth: 0,
            root: false,
            failed: false,
            finished: false,
            active: false,
            comma: false,
            phase: 0,
            position: 0,
            escape_position: 0,
            serialized: Vec::new(),
            serialized_position: 0,
            serializing: false,
        }
    }
}

impl JsonEncoder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn serialize_next<T: serde::Serialize + ?Sized>(
        &mut self,
        key: Option<&str>,
        value: &T,
        output: &mut [u8],
    ) -> Result<EncoderStatus, JsonEncodeError> {
        if self.failed || self.finished || self.active {
            return Err(JsonEncodeError::InvalidState);
        }
        if self.serializing {
            let remaining = &self.serialized[self.serialized_position..];
            let written = remaining.len().min(output.len());
            output[..written].copy_from_slice(&remaining[..written]);
            self.serialized_position += written;
            return if self.serialized_position == self.serialized.len() {
                self.serialized.clear();
                self.serialized_position = 0;
                self.serializing = false;
                Ok(EncoderStatus::ComponentFinished { written })
            } else {
                Ok(EncoderStatus::NeedOutput { written })
            };
        }
        if let Err(error) = self.begin(&JsonToken::ValueNull(key)) {
            self.failed = true;
            return Err(error);
        }
        self.active = false;
        let mut writer = SerializeOutput {
            output,
            overflow: &mut self.serialized,
            written: 0,
        };
        if let Err(error) = writer.serialize(self.comma, key, value) {
            self.failed = true;
            return Err(JsonEncodeError::Serialize(error));
        }
        let written = writer.written;
        self.serializing = !self.serialized.is_empty();
        if self.serializing {
            Ok(EncoderStatus::NeedOutput { written })
        } else {
            Ok(EncoderStatus::ComponentFinished { written })
        }
    }

    fn begin(&mut self, token: &JsonToken<'_>) -> Result<(), JsonEncodeError> {
        let closing = matches!(token, JsonToken::ObjectEnd | JsonToken::ArrayEnd);
        if closing {
            if self.depth == 0
                || self.stack[self.depth - 1].object != matches!(token, JsonToken::ObjectEnd)
            {
                return Err(JsonEncodeError::InvalidState);
            }
            self.depth -= 1;
            self.comma = false;
            self.phase = 3;
        } else {
            let key = key(token);
            if self.depth == 0 {
                if self.root || key.is_some() {
                    return Err(JsonEncodeError::InvalidState);
                }
                self.comma = false;
            } else {
                let parent = &mut self.stack[self.depth - 1];
                if parent.object != key.is_some() {
                    return Err(JsonEncodeError::InvalidState);
                }
                self.comma = parent.populated;
                parent.populated = true;
            }
            self.root = true;
            if matches!(token, JsonToken::ObjectStart(_) | JsonToken::ArrayStart(_)) {
                if self.depth == self.stack.len() {
                    return Err(JsonEncodeError::InvalidState);
                }
                self.stack[self.depth] = Container {
                    object: matches!(token, JsonToken::ObjectStart(_)),
                    populated: false,
                };
                self.depth += 1;
            }
            self.phase = 0;
        }
        self.position = 0;
        self.escape_position = 0;
        self.active = true;
        Ok(())
    }

    fn string_byte(&mut self, value: &str) -> Option<u8> {
        if self.position == 0 {
            self.position = 1;
            return Some(b'"');
        }
        let index = self.position - 1;
        if index == value.len() {
            self.position += 1;
            return Some(b'"');
        }
        let &byte = value.as_bytes().get(index)?;
        let escape: &[u8] = match byte {
            b'"' => b"\\\"",
            b'\\' => b"\\\\",
            b'\n' => b"\\n",
            b'\r' => b"\\r",
            b'\t' => b"\\t",
            8 => b"\\b",
            12 => b"\\f",
            0..=31 => {
                const HEX: &[u8] = b"0123456789abcdef";
                let encoded = match self.escape_position {
                    0 => b'\\',
                    1 => b'u',
                    2 | 3 => b'0',
                    4 => HEX[(byte >> 4) as usize],
                    _ => HEX[(byte & 15) as usize],
                };
                self.escape_position += 1;
                if self.escape_position == 6 {
                    self.escape_position = 0;
                    self.position += 1;
                }
                return Some(encoded);
            }
            _ => {
                self.position += 1;
                return Some(byte);
            }
        };
        let encoded = escape[self.escape_position];
        self.escape_position += 1;
        if self.escape_position == escape.len() {
            self.escape_position = 0;
            self.position += 1;
        }
        Some(encoded)
    }

    fn byte(&mut self, token: &JsonToken<'_>) -> Result<Option<u8>, JsonEncodeError> {
        loop {
            let byte = match self.phase {
                0 => {
                    self.phase = 1;
                    self.comma.then_some(b',')
                }
                1 => {
                    if let Some(key) = key(token)
                        && let Some(byte) = self.string_byte(key)
                    {
                        return Ok(Some(byte));
                    }
                    self.position = 0;
                    self.phase = 2;
                    None
                }
                2 => {
                    self.phase = 3;
                    key(token).map(|_| b':')
                }
                3 => match token {
                    JsonToken::ValueString(_, value) => {
                        if let Some(byte) = self.string_byte(value) {
                            return Ok(Some(byte));
                        }
                        self.phase = 4;
                        None
                    }
                    JsonToken::ValueNumber(_, _) => return Ok(None),
                    _ => {
                        let bytes: &[u8] = match token {
                            JsonToken::ObjectStart(_) => b"{",
                            JsonToken::ObjectEnd => b"}",
                            JsonToken::ArrayStart(_) => b"[",
                            JsonToken::ArrayEnd => b"]",
                            JsonToken::ValueBoolean(_, true) => b"true",
                            JsonToken::ValueBoolean(_, false) => b"false",
                            JsonToken::ValueNull(_) => b"null",
                            _ => unreachable!(),
                        };
                        if let Some(&byte) = bytes.get(self.position) {
                            self.position += 1;
                            return Ok(Some(byte));
                        }
                        self.phase = 4;
                        None
                    }
                },
                _ => return Ok(None),
            };
            if byte.is_some() {
                return Ok(byte);
            }
        }
    }
}

impl<'a> Encoder<JsonToken<'a>> for JsonEncoder {
    type Error = JsonEncodeError;

    fn next(
        &mut self,
        component: Option<&mut JsonToken<'a>>,
        output: &mut [u8],
    ) -> Result<EncoderStatus, Self::Error> {
        if self.failed || self.serializing {
            return Err(JsonEncodeError::InvalidState);
        }
        let Some(component) = component else {
            if self.active || !self.root || self.depth != 0 {
                self.failed = true;
                return Err(JsonEncodeError::Incomplete);
            }
            self.finished = true;
            return Ok(EncoderStatus::Finished { written: 0 });
        };
        if self.finished {
            return Err(JsonEncodeError::InvalidState);
        }
        if !self.active
            && let Err(error) = self.begin(component)
        {
            self.failed = true;
            return Err(error);
        }
        let mut written = 0;
        while written < output.len() {
            if self.phase == 3
                && let JsonToken::ValueNumber(_, number) = component
            {
                let mut writer = NumberOutput {
                    output: &mut output[written..],
                    skip: self.position,
                    written: 0,
                };
                let result = std::fmt::write(&mut writer, format_args!("{number}"));
                self.position += writer.written;
                written += writer.written;
                if result.is_err() {
                    return Ok(EncoderStatus::NeedOutput { written });
                }
                self.phase = 4;
                self.active = false;
                return Ok(EncoderStatus::ComponentFinished { written });
            }
            match self.byte(component) {
                Ok(Some(byte)) => {
                    output[written] = byte;
                    written += 1;
                }
                Ok(None) => {
                    if self.phase == 3 && matches!(component, JsonToken::ValueNumber(_, _)) {
                        continue;
                    }
                    self.active = false;
                    return Ok(EncoderStatus::ComponentFinished { written });
                }
                Err(error) => {
                    self.failed = true;
                    return Err(error);
                }
            }
        }
        Ok(EncoderStatus::NeedOutput { written })
    }

    fn finish(self) -> Result<(), Self::Error> {
        if self.failed {
            Err(JsonEncodeError::InvalidState)
        } else if !self.finished {
            Err(JsonEncodeError::Incomplete)
        } else {
            Ok(())
        }
    }
}

struct NumberOutput<'a> {
    output: &'a mut [u8],
    skip: usize,
    written: usize,
}

struct SerializeOutput<'a> {
    output: &'a mut [u8],
    overflow: &'a mut Vec<u8>,
    written: usize,
}

impl SerializeOutput<'_> {
    fn serialize<T: serde::Serialize + ?Sized>(
        &mut self,
        comma: bool,
        key: Option<&str>,
        value: &T,
    ) -> Result<(), serde_json::Error> {
        use std::io::Write;

        if comma {
            self.write_all(b",").map_err(serde_json::Error::io)?;
        }
        if let Some(key) = key {
            serde_json::to_writer(&mut *self, key)?;
            self.write_all(b":").map_err(serde_json::Error::io)?;
        }
        serde_json::to_writer(self, value)
    }
}

impl std::io::Write for SerializeOutput<'_> {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        let count = bytes.len().min(self.output.len() - self.written);
        self.output[self.written..self.written + count].copy_from_slice(&bytes[..count]);
        self.written += count;
        self.overflow.extend_from_slice(&bytes[count..]);
        Ok(bytes.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl std::fmt::Write for NumberOutput<'_> {
    fn write_str(&mut self, value: &str) -> std::fmt::Result {
        let skipped = self.skip.min(value.len());
        self.skip -= skipped;
        let bytes = &value.as_bytes()[skipped..];
        let count = bytes.len().min(self.output.len() - self.written);
        self.output[self.written..self.written + count].copy_from_slice(&bytes[..count]);
        self.written += count;
        if count == bytes.len() {
            Ok(())
        } else {
            Err(std::fmt::Error)
        }
    }
}

fn key<'a>(token: &JsonToken<'a>) -> Option<&'a str> {
    match token {
        JsonToken::ObjectStart(key)
        | JsonToken::ArrayStart(key)
        | JsonToken::ValueString(key, _)
        | JsonToken::ValueNumber(key, _)
        | JsonToken::ValueBoolean(key, _)
        | JsonToken::ValueNull(key) => *key,
        JsonToken::ObjectEnd | JsonToken::ArrayEnd => None,
    }
}
