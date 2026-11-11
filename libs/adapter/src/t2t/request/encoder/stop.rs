use crate::error::CodecError;

use super::super::decoder::stop::{STOP_BYTES, STOP_COUNT};

pub(crate) struct Stops {
    bytes: [u8; STOP_BYTES],
    ends: [usize; STOP_COUNT],
    count: usize,
    len: usize,
}
impl Default for Stops {
    fn default() -> Self {
        Self {
            bytes: [0; STOP_BYTES],
            ends: [0; STOP_COUNT],
            count: 0,
            len: 0,
        }
    }
}
impl Stops {
    pub fn count(&self) -> usize {
        self.count
    }
    pub fn begin(&self, limit: usize) -> Result<(), CodecError> {
        if self.count >= limit {
            return Err(CodecError::LimitExceeded("too many stop sequences"));
        }
        Ok(())
    }
    pub fn append(&mut self, text: &[u8]) -> Result<(), CodecError> {
        if text.len() > STOP_BYTES - self.len {
            return Err(CodecError::LimitExceeded("stop text exceeds 4096 bytes"));
        }
        self.bytes[self.len..self.len + text.len()].copy_from_slice(text);
        self.len += text.len();
        Ok(())
    }
    pub fn end(&mut self) -> Result<(), CodecError> {
        let start = if self.count == 0 {
            0
        } else {
            self.ends[self.count - 1]
        };
        if start == self.len {
            return Err(CodecError::InvalidInput("empty stop sequence"));
        }
        self.ends[self.count] = self.len;
        self.count += 1;
        Ok(())
    }
    fn sequence(&self, index: usize) -> &[u8] {
        let start = if index == 0 { 0 } else { self.ends[index - 1] };
        &self.bytes[start..self.ends[index]]
    }
}

#[derive(Default)]
pub(crate) struct StopWriter {
    stage: u8,
    index: usize,
    offset: usize,
    escape: [u8; 6],
    escape_pos: usize,
    escape_len: usize,
}
impl StopWriter {
    pub fn done(&self) -> bool {
        self.stage == 6
    }
    pub fn write(&mut self, output: &mut [u8], stops: &Stops) -> usize {
        let mut written = 0;
        while written < output.len() && !self.done() {
            if self.escape_pos < self.escape_len {
                output[written] = self.escape[self.escape_pos];
                self.escape_pos += 1;
                written += 1;
                continue;
            }
            let byte = match self.stage {
                0 => {
                    self.stage = 1;
                    b'['
                }
                1 => {
                    if self.index == stops.count() {
                        self.stage = 5;
                        continue;
                    }
                    self.stage = 2;
                    b'"'
                }
                2 => {
                    let text = stops.sequence(self.index);
                    if self.offset == text.len() {
                        self.stage = 3;
                        continue;
                    }
                    let byte = text[self.offset];
                    self.offset += 1;
                    self.escape_pos = 0;
                    self.escape_len = match byte {
                        b'"' | b'\\' => {
                            self.escape[0] = b'\\';
                            self.escape[1] = byte;
                            2
                        }
                        0..=31 => {
                            self.escape[..4].copy_from_slice(b"\\u00");
                            self.escape[4] = b"0123456789abcdef"[(byte >> 4) as usize];
                            self.escape[5] = b"0123456789abcdef"[(byte & 15) as usize];
                            6
                        }
                        _ => {
                            self.escape[0] = byte;
                            1
                        }
                    };
                    continue;
                }
                3 => {
                    self.index += 1;
                    self.offset = 0;
                    self.stage = if self.index == stops.count() { 5 } else { 4 };
                    b'"'
                }
                4 => {
                    self.stage = 1;
                    b','
                }
                _ => {
                    self.stage = 6;
                    b']'
                }
            };
            output[written] = byte;
            written += 1;
        }
        written
    }
}
