use crate::error::CodecError;

#[derive(Default)]
pub(crate) struct Utf8 {
    pub(super) bytes: [u8; 4],
    pub(super) len: usize,
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
                _ => return Err(CodecError::InvalidInput("invalid JSON string or UTF-8")),
            };
        } else {
            if byte & 0xc0 != 0x80 {
                return Err(CodecError::InvalidInput("invalid JSON string or UTF-8"));
            }
            self.bytes[self.len] = byte;
            self.len += 1;
            self.remaining -= 1;
        }
        if self.complete() {
            std::str::from_utf8(&self.bytes[..self.len])
                .map_err(|_| CodecError::InvalidInput("invalid JSON string or UTF-8"))?;
        }
        Ok(())
    }
}
