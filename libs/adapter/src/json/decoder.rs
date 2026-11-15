use super::state::State;
use super::{JsonDecodeError, JsonStep, JsonToken};

#[derive(Debug, Default)]
pub struct JsonDecoder {
    state: State,
    bytes: Vec<u8>,
    decoded: String,
}

impl JsonDecoder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn next<'a, 'input: 'a>(
        &'a mut self,
        input: &mut &'input [u8],
    ) -> Result<JsonStep<'a>, JsonDecodeError> {
        if matches!(self.state, State::Failed | State::Finished) {
            return Err(JsonDecodeError::InvalidState);
        }
        if matches!(self.state, State::Idle) {
            self.bytes.clear();
            while let Some((&byte, rest)) = input.split_first() {
                if !matches!(byte, b' ' | b'\t' | b'\r' | b'\n') {
                    break;
                }
                *input = rest;
            }
            let Some(&byte) = input.first() else {
                return Ok(JsonStep::NeedInput);
            };
            let token = match byte {
                b'{' => Some(JsonToken::ObjectStart),
                b'}' => Some(JsonToken::ObjectEnd),
                b'[' => Some(JsonToken::ArrayStart),
                b']' => Some(JsonToken::ArrayEnd),
                b':' => Some(JsonToken::Colon),
                b',' => Some(JsonToken::Comma),
                b'"' => {
                    self.state = State::String { escaped: false };
                    None
                }
                b'-' | b'0'..=b'9' | b't' | b'f' | b'n' => {
                    self.state = State::Scalar;
                    None
                }
                _ => {
                    self.state = State::Failed;
                    return Err(JsonDecodeError::InvalidToken);
                }
            };
            if let Some(token) = token {
                *input = &input[1..];
                return Ok(JsonStep::Event(token));
            }
        }

        let mut consumed = 0;
        let mut complete = false;
        match &mut self.state {
            State::String { escaped } => {
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
                        self.state = State::Failed;
                        return Err(JsonDecodeError::InvalidToken);
                    }
                }
            }
            State::Scalar => {
                for &byte in *input {
                    if Self::is_delimiter(byte) {
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
        if !complete {
            self.bytes.extend_from_slice(part);
            return Ok(JsonStep::NeedInput);
        }

        let string = matches!(self.state, State::String { .. });
        self.state = State::Idle;
        let bytes = if self.bytes.is_empty() {
            part
        } else {
            self.bytes.extend_from_slice(part);
            &self.bytes
        };
        match Self::token(bytes, string, &mut self.decoded) {
            Ok(token) => Ok(JsonStep::Event(token)),
            Err(error) => {
                self.state = State::Failed;
                Err(error)
            }
        }
    }

    pub fn finish(&mut self) -> Result<JsonStep<'_>, JsonDecodeError> {
        match self.state {
            State::Idle | State::Finished => {
                self.state = State::Finished;
                Ok(JsonStep::Finished)
            }
            State::Scalar => {
                self.state = State::Finished;
                match Self::token(&self.bytes, false, &mut self.decoded) {
                    Ok(token) => Ok(JsonStep::Event(token)),
                    Err(error) => {
                        self.state = State::Failed;
                        Err(error)
                    }
                }
            }
            State::String { .. } => {
                self.state = State::Failed;
                Err(JsonDecodeError::IncompleteToken)
            }
            State::Failed => Err(JsonDecodeError::InvalidState),
        }
    }

    fn is_delimiter(byte: u8) -> bool {
        matches!(
            byte,
            b' ' | b'\t' | b'\r' | b'\n' | b'{' | b'}' | b'[' | b']' | b':' | b',' | b'"'
        )
    }

    fn token<'a>(
        bytes: &'a [u8],
        string: bool,
        decoded: &'a mut String,
    ) -> Result<JsonToken<'a>, JsonDecodeError> {
        if string {
            if !bytes.contains(&b'\\') {
                let text = serde_json::from_slice::<&str>(bytes)
                    .map_err(|_| JsonDecodeError::InvalidToken)?;
                return Ok(JsonToken::String(text));
            }
            Self::decode_string(bytes, decoded)?;
            return Ok(JsonToken::String(decoded));
        }
        Ok(match bytes {
            b"true" => JsonToken::Boolean(true),
            b"false" => JsonToken::Boolean(false),
            b"null" => JsonToken::Null,
            bytes => {
                serde_json::from_slice::<serde_json::Number>(bytes)
                    .map_err(|_| JsonDecodeError::InvalidToken)?;
                let text = std::str::from_utf8(bytes).map_err(|_| JsonDecodeError::InvalidToken)?;
                JsonToken::Number(text)
            }
        })
    }

    fn decode_string(bytes: &[u8], decoded: &mut String) -> Result<(), JsonDecodeError> {
        let invalid = || JsonDecodeError::InvalidToken;
        let content = bytes
            .strip_prefix(b"\"")
            .and_then(|bytes| bytes.strip_suffix(b"\""))
            .ok_or_else(invalid)?;
        decoded.clear();
        let mut remaining = content;
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
        let mut value = 0u16;
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
