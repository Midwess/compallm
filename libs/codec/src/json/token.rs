use super::JsonDecodeError;
use super::state::JsonState;
use crate::decoder::{Decoder, DecoderStatus};

#[derive(Debug)]
pub(super) enum JsonToken<'a> {
    ObjectStart,
    ObjectEnd,
    ArrayStart,
    ArrayEnd,
    Colon,
    Comma,
    String(&'a str),
    Number(serde_json::Number),
    Boolean(bool),
    Null,
}

#[derive(Debug)]
pub(super) struct TokenDecoder {
    state: JsonState,
    bytes: Vec<u8>,
    decoded: Vec<u8>,
    escaped_string: bool,
}

impl Default for TokenDecoder {
    fn default() -> Self {
        Self::with_capacity(super::INTERNAL_BUFFER_BYTES)
    }
}

impl Decoder for TokenDecoder {
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
        self.escaped_string = string && bytes.contains(&b'\\');
        match Self::token(bytes, string, &mut self.decoded) {
            Ok(token) => Ok(DecoderStatus::Component(token)),
            Err(error) => {
                self.state = JsonState::Failed;
                Err(error)
            }
        }
    }

    fn finish(self) -> Result<(), Self::Error> {
        if matches!(self.state, JsonState::Failed) {
            return Err(JsonDecodeError::InvalidState);
        }
        match self.state {
            JsonState::Idle | JsonState::Finished => Ok(()),
            JsonState::Scalar | JsonState::String { .. } => Err(JsonDecodeError::IncompleteToken),
            JsonState::Failed => Err(JsonDecodeError::InvalidState),
        }
    }
}

impl TokenDecoder {
    pub(super) fn reset(&mut self) {
        self.state = JsonState::Idle;
        self.bytes.clear();
        self.decoded.clear();
        self.escaped_string = false;
    }

    pub(super) fn with_capacity(bytes: usize) -> Self {
        Self {
            state: JsonState::Idle,
            bytes: Vec::with_capacity(bytes),
            decoded: Vec::with_capacity(bytes),
            escaped_string: false,
        }
    }

    pub(super) fn buffered_string(&self) -> Result<&str, JsonDecodeError> {
        if self.escaped_string {
            return std::str::from_utf8(&self.decoded).map_err(|_| JsonDecodeError::InvalidState);
        }
        serde_json::from_slice::<&str>(&self.bytes).map_err(|_| JsonDecodeError::InvalidState)
    }

    fn token<'a>(
        bytes: &'a [u8],
        string: bool,
        decoded: &'a mut Vec<u8>,
    ) -> Result<JsonToken<'a>, JsonDecodeError> {
        if string {
            if bytes.contains(&b'\\') {
                decoded.resize(bytes.len(), 0);
                let length = compallm_serde_rules::string::decode(bytes, decoded)
                    .map_err(|_| JsonDecodeError::InvalidToken)?
                    .len();
                decoded.truncate(length);
                return std::str::from_utf8(decoded)
                    .map(JsonToken::String)
                    .map_err(|_| JsonDecodeError::InvalidToken);
            }
            return serde_json::from_slice::<&str>(bytes)
                .map(JsonToken::String)
                .map_err(|_| JsonDecodeError::InvalidToken);
        }
        match bytes {
            b"true" => Ok(JsonToken::Boolean(true)),
            b"false" => Ok(JsonToken::Boolean(false)),
            b"null" => Ok(JsonToken::Null),
            _ => serde_json::from_slice::<serde_json::Number>(bytes)
                .map(JsonToken::Number)
                .map_err(|_| JsonDecodeError::InvalidToken),
        }
    }
}
