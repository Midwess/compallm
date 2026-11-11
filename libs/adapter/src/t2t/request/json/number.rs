use crate::error::CodecError;

#[derive(Clone, Copy)]
pub(crate) enum Number {
    Start,
    Minus,
    Zero,
    Integer,
    Dot,
    Fraction,
    Exp,
    Sign,
    Exponent,
}

impl Number {
    pub fn push(&mut self, byte: u8) -> Result<(), CodecError> {
        *self = match (*self, byte) {
            (Self::Start, b'-') => Self::Minus,
            (Self::Start | Self::Minus, b'0') => Self::Zero,
            (Self::Start | Self::Minus, b'1'..=b'9') => Self::Integer,
            (Self::Integer, b'0'..=b'9') => Self::Integer,
            (Self::Integer | Self::Zero, b'.') => Self::Dot,
            (Self::Dot | Self::Fraction, b'0'..=b'9') => Self::Fraction,
            (Self::Integer | Self::Zero | Self::Fraction, b'e' | b'E') => Self::Exp,
            (Self::Exp, b'+' | b'-') => Self::Sign,
            (Self::Exp | Self::Sign | Self::Exponent, b'0'..=b'9') => Self::Exponent,
            _ => return Err(CodecError::InvalidInput("invalid JSON number")),
        };
        Ok(())
    }
    pub fn complete(self) -> bool {
        matches!(
            self,
            Self::Zero | Self::Integer | Self::Fraction | Self::Exponent
        )
    }
    pub fn probability(self, bytes: &[u8]) -> Result<f64, CodecError> {
        if !self.complete() {
            return Err(CodecError::InvalidInput("incomplete JSON number"));
        }
        let value: f64 = std::str::from_utf8(bytes)
            .map_err(|_| CodecError::InvalidInput("invalid JSON string or UTF-8"))?
            .parse()
            .map_err(|_| CodecError::InvalidInput("invalid top_p"))?;
        if !value.is_finite() || !(0.0..=1.0).contains(&value) {
            return Err(CodecError::InvalidInput(
                "top_p must be between zero and one",
            ));
        }
        Ok(value)
    }
}

pub(crate) struct JsonFloat {
    bytes: [u8; 32],
    len: usize,
}
impl TryFrom<f64> for JsonFloat {
    type Error = CodecError;
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        use std::io::Write;
        if !value.is_finite() {
            return Err(CodecError::InvalidInput("nonfinite JSON number"));
        }
        let mut bytes = [0; 32];
        let len = {
            let mut cursor = std::io::Cursor::new(bytes.as_mut_slice());
            write!(&mut cursor, "{value:e}")
                .map_err(|_| CodecError::Contract("JSON number formatting overflow"))?;
            cursor.position() as usize
        };
        Ok(Self { bytes, len })
    }
}
impl AsRef<[u8]> for JsonFloat {
    fn as_ref(&self) -> &[u8] {
        &self.bytes[..self.len]
    }
}
