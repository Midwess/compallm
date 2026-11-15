use super::state::JsonState;
use super::{JsonDecodeError, JsonToken};
use crate::decoder::{Decoder, DecoderStatus};

#[derive(Debug, Default)]
pub struct JsonDecoder {
    state: JsonState,
    bytes: Vec<u8>,
    decoded: String,
}

impl Decoder for JsonDecoder {
    type Component<'a> = JsonToken<'a>;
    type Error = JsonDecodeError;

    fn next<'a, 'input: 'a>(
        &'a mut self,
        input: &mut &'input [u8],
        eof: bool,
    ) -> Result<DecoderStatus<Self::Component<'a>>, Self::Error> {
        if matches!(self.state, JsonState::Failed | JsonState::Finished) {
            return Err(JsonDecodeError::InvalidState);
        }
        if matches!(self.state, JsonState::Idle) {
            self.bytes.clear();
            while input
                .first()
                .is_some_and(|byte| matches!(byte, b' ' | b'\t' | b'\r' | b'\n'))
            {
                *input = &input[1..];
            }
            let Some(&byte) = input.first() else {
                if eof {
                    self.state = JsonState::Finished;
                    return Ok(DecoderStatus::Finished);
                }
                return Ok(DecoderStatus::NeedInput);
            };
            let token = match byte {
                b'{' => Some(JsonToken::ObjectStart),
                b'}' => Some(JsonToken::ObjectEnd),
                b'[' => Some(JsonToken::ArrayStart),
                b']' => Some(JsonToken::ArrayEnd),
                b':' => Some(JsonToken::Colon),
                b',' => Some(JsonToken::Comma),
                b'"' => {
                    self.state = JsonState::String { escaped: false };
                    None
                }
                b'-' | b'0'..=b'9' | b't' | b'f' | b'n' => {
                    self.state = JsonState::Scalar;
                    None
                }
                _ => {
                    self.state = JsonState::Failed;
                    return Err(JsonDecodeError::InvalidToken);
                }
            };
            if let Some(token) = token {
                *input = &input[1..];
                return Ok(DecoderStatus::Component(token));
            }
        }
        let mut consumed = 0;
        let mut complete = false;
        match &mut self.state {
            JsonState::String { escaped } => {
                let opening = self.bytes.is_empty();
                for &byte in *input {
                    consumed += 1;
                    if opening && consumed == 1 {
                        continue;
                    }
                    if *escaped {
                        *escaped = false;
                    } else if byte == b'\\' {
                        *escaped = true;
                    } else if byte == b'"' {
                        complete = true;
                        break;
                    } else if byte < 0x20 {
                        self.state = JsonState::Failed;
                        return Err(JsonDecodeError::InvalidToken);
                    }
                }
            }
            JsonState::Scalar => {
                for &byte in *input {
                    if matches!(
                        byte,
                        b' ' | b'\t'
                            | b'\r'
                            | b'\n'
                            | b'{'
                            | b'}'
                            | b'['
                            | b']'
                            | b':'
                            | b','
                            | b'"'
                    ) {
                        complete = true;
                        break;
                    }
                    consumed += 1;
                }
            }
            _ => return Err(JsonDecodeError::InvalidState),
        }
        let (part, rest) = input.split_at(consumed);
        *input = rest;
        if !complete && eof {
            if matches!(self.state, JsonState::Scalar) {
                complete = true;
            } else {
                self.state = JsonState::Failed;
                return Err(JsonDecodeError::IncompleteToken);
            }
        }
        if !complete {
            self.bytes.extend_from_slice(part);
            return Ok(DecoderStatus::NeedInput);
        }
        let string = matches!(self.state, JsonState::String { .. });
        self.state = JsonState::Idle;
        let bytes = if self.bytes.is_empty() {
            part
        } else {
            self.bytes.extend_from_slice(part);
            &self.bytes
        };
        match Self::token(bytes, string, &mut self.decoded) {
            Ok(token) => Ok(DecoderStatus::Component(token)),
            Err(error) => {
                self.state = JsonState::Failed;
                Err(error)
            }
        }
    }

    fn finish(self) -> Result<(), Self::Error> {
        match self.state {
            JsonState::Idle | JsonState::Finished => Ok(()),
            JsonState::Scalar | JsonState::String { .. } => Err(JsonDecodeError::IncompleteToken),
            JsonState::Failed => Err(JsonDecodeError::InvalidState),
        }
    }
}

impl JsonDecoder {
    pub fn new() -> Self {
        Self::default()
    }

    fn token<'a>(
        bytes: &'a [u8],
        string: bool,
        decoded: &'a mut String,
    ) -> Result<JsonToken<'a>, JsonDecodeError> {
        let invalid = || JsonDecodeError::InvalidToken;
        if string {
            if !bytes.contains(&b'\\') {
                return serde_json::from_slice::<&str>(bytes)
                    .map(JsonToken::String)
                    .map_err(|_| invalid());
            }
            Self::decode_string(bytes, decoded)?;
            return Ok(JsonToken::String(decoded));
        }
        Ok(match bytes {
            b"true" => JsonToken::Boolean(true),
            b"false" => JsonToken::Boolean(false),
            b"null" => JsonToken::Null,
            _ => {
                serde_json::from_slice::<serde_json::Number>(bytes).map_err(|_| invalid())?;
                JsonToken::Number(std::str::from_utf8(bytes).map_err(|_| invalid())?)
            }
        })
    }

    fn decode_string(bytes: &[u8], decoded: &mut String) -> Result<(), JsonDecodeError> {
        let invalid = || JsonDecodeError::InvalidToken;
        let mut remaining = bytes
            .strip_prefix(b"\"")
            .and_then(|bytes| bytes.strip_suffix(b"\""))
            .ok_or_else(invalid)?;
        decoded.clear();
        while !remaining.is_empty() {
            let plain_len = remaining
                .iter()
                .position(|&byte| byte == b'\\' || byte == b'"' || byte < 0x20)
                .unwrap_or(remaining.len());
            decoded.push_str(std::str::from_utf8(&remaining[..plain_len]).map_err(|_| invalid())?);
            remaining = &remaining[plain_len..];
            if remaining.is_empty() {
                break;
            }
            let Some((&b'\\', tail)) = remaining.split_first() else {
                return Err(invalid());
            };
            let (&escape, tail) = tail.split_first().ok_or_else(invalid)?;
            remaining = tail;
            let character = match escape {
                b'"' => '"',
                b'\\' => '\\',
                b'/' => '/',
                b'b' => '\u{0008}',
                b'f' => '\u{000c}',
                b'n' => '\n',
                b'r' => '\r',
                b't' => '\t',
                b'u' => {
                    let high = Self::unicode_unit(&mut remaining)?;
                    let value = if (0xd800..=0xdbff).contains(&high) {
                        remaining = remaining.strip_prefix(b"\\u").ok_or_else(invalid)?;
                        let low = Self::unicode_unit(&mut remaining)?;
                        if !(0xdc00..=0xdfff).contains(&low) {
                            return Err(invalid());
                        }
                        0x10000 + ((u32::from(high) - 0xd800) << 10) + (u32::from(low) - 0xdc00)
                    } else {
                        u32::from(high)
                    };
                    char::from_u32(value).ok_or_else(invalid)?
                }
                _ => return Err(invalid()),
            };
            decoded.push(character);
        }
        Ok(())
    }

    fn unicode_unit(input: &mut &[u8]) -> Result<u16, JsonDecodeError> {
        let digits = input.get(..4).ok_or(JsonDecodeError::InvalidToken)?;
        let mut value = 0_u16;
        for &byte in digits {
            let digit = match byte {
                b'0'..=b'9' => byte - b'0',
                b'a'..=b'f' => byte - b'a' + 10,
                b'A'..=b'F' => byte - b'A' + 10,
                _ => return Err(JsonDecodeError::InvalidToken),
            };
            value = (value << 4) | u16::from(digit);
        }
        *input = &input[4..];
        Ok(value)
    }
}
