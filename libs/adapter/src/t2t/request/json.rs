//! Constant-size JSON string/UTF-8 state, shared by the two request formats.
use crate::error::CodecError;

fn invalid() -> CodecError {
    CodecError::InvalidInput("invalid JSON string or UTF-8")
}

#[derive(Default)]
pub(super) struct Utf8 {
    bytes: [u8; 4],
    len: usize,
    remaining: usize,
}
impl Utf8 {
    pub fn complete(&self) -> bool {
        self.remaining == 0
    }
    pub fn push(&mut self, byte: u8) -> Result<(), CodecError> {
        if self.remaining == 0 {
            self.len = 1;
            self.bytes[0] = byte;
            self.remaining = match byte {
                0..=0x7f => 0,
                0xc2..=0xdf => 1,
                0xe0..=0xef => 2,
                0xf0..=0xf4 => 3,
                _ => return Err(invalid()),
            };
        } else {
            if byte & 0xc0 != 0x80 {
                return Err(invalid());
            }
            self.bytes[self.len] = byte;
            self.len += 1;
            self.remaining -= 1;
        }
        if self.complete() {
            std::str::from_utf8(&self.bytes[..self.len]).map_err(|_| invalid())?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy)]
enum Mode {
    Raw,
    Escape,
    Hex {
        value: u16,
        digits: u8,
        high: Option<u16>,
    },
    LowSlash(u16),
    LowU(u16),
}

pub(super) enum Piece {
    Bytes([u8; 4], usize),
    End,
}

pub(super) struct JsonString {
    mode: Mode,
    utf8: Utf8,
}
impl Default for JsonString {
    fn default() -> Self {
        Self {
            mode: Mode::Raw,
            utf8: Utf8::default(),
        }
    }
}

impl JsonString {
    fn scalar(&mut self, scalar: u32) -> Result<Option<Piece>, CodecError> {
        let scalar = char::from_u32(scalar).ok_or_else(invalid)?;
        let mut bytes = [0; 4];
        let len = scalar.encode_utf8(&mut bytes).len();
        self.mode = Mode::Raw;
        Ok(Some(Piece::Bytes(bytes, len)))
    }

    pub fn push(&mut self, byte: u8) -> Result<Option<Piece>, CodecError> {
        if !self.utf8.complete() {
            self.utf8.push(byte)?;
            return Ok(if self.utf8.complete() {
                Some(Piece::Bytes(self.utf8.bytes, self.utf8.len))
            } else {
                None
            });
        }
        match self.mode {
            Mode::Raw => match byte {
                b'"' => Ok(Some(Piece::End)),
                b'\\' => {
                    self.mode = Mode::Escape;
                    Ok(None)
                }
                0..=0x1f => Err(invalid()),
                0x20..=0x7f => {
                    let mut bytes = [0; 4];
                    bytes[0] = byte;
                    Ok(Some(Piece::Bytes(bytes, 1)))
                }
                _ => {
                    self.utf8.push(byte)?;
                    Ok(None)
                }
            },
            Mode::Escape => {
                let value = match byte {
                    b'"' | b'\\' | b'/' => byte,
                    b'b' => 8,
                    b'f' => 12,
                    b'n' => b'\n',
                    b'r' => b'\r',
                    b't' => b'\t',
                    b'u' => {
                        self.mode = Mode::Hex {
                            value: 0,
                            digits: 0,
                            high: None,
                        };
                        return Ok(None);
                    }
                    _ => return Err(invalid()),
                };
                self.scalar(u32::from(value))
            }
            Mode::Hex {
                value,
                digits,
                high,
            } => {
                let hex = match byte {
                    b'0'..=b'9' => byte - b'0',
                    b'a'..=b'f' => byte - b'a' + 10,
                    b'A'..=b'F' => byte - b'A' + 10,
                    _ => return Err(invalid()),
                };
                let value = (value << 4) | u16::from(hex);
                if digits < 3 {
                    self.mode = Mode::Hex {
                        value,
                        digits: digits + 1,
                        high,
                    };
                    return Ok(None);
                }
                if let Some(high) = high {
                    if !(0xdc00..=0xdfff).contains(&value) {
                        return Err(invalid());
                    }
                    self.scalar(
                        0x10000 + ((u32::from(high) - 0xd800) << 10) + (u32::from(value) - 0xdc00),
                    )
                } else if (0xd800..=0xdbff).contains(&value) {
                    self.mode = Mode::LowSlash(value);
                    Ok(None)
                } else if (0xdc00..=0xdfff).contains(&value) {
                    Err(invalid())
                } else {
                    self.scalar(u32::from(value))
                }
            }
            Mode::LowSlash(high) => {
                if byte != b'\\' {
                    return Err(invalid());
                }
                self.mode = Mode::LowU(high);
                Ok(None)
            }
            Mode::LowU(high) => {
                if byte != b'u' {
                    return Err(invalid());
                }
                self.mode = Mode::Hex {
                    value: 0,
                    digits: 0,
                    high: Some(high),
                };
                Ok(None)
            }
        }
    }
}
