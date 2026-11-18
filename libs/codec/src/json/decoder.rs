use super::state::JsonState;
use super::{JsonDecodeError, JsonToken};
use crate::decoder::{Decoder, DecoderStatus};

#[derive(Debug)]
pub struct JsonDecoder {
    state: JsonState,
    bytes: Vec<u8>,
    decoded: String,
    object: Vec<u8>,
    nesting: Vec<u8>,
    quoted: bool,
    escaped: bool,
    object_active: bool,
    object_string: bool,
    object_container: bool,
    member_phase: MemberPhase,
    expand_member: bool,
}

#[derive(Debug, Default, Clone, Copy)]
enum MemberPhase {
    #[default]
    Start,
    KeyOrEnd,
    Key,
    Colon,
    Value,
    Array,
    Element,
    ElementValue,
    ElementEnd,
    End,
    Closed,
}

impl Default for JsonDecoder {
    fn default() -> Self {
        Self {
            state: JsonState::default(),
            bytes: Vec::new(),
            decoded: String::new(),
            object: Vec::with_capacity(64),
            nesting: Vec::new(),
            quoted: false,
            escaped: false,
            object_active: false,
            object_string: false,
            object_container: false,
            member_phase: MemberPhase::default(),
            expand_member: false,
        }
    }
}

impl Decoder for JsonDecoder {
    type Component<'a> = JsonToken<'a>;
    type Error = JsonDecodeError;

    fn next<'a, 'input: 'a>(
        &'a mut self,
        input: &mut &'input [u8],
        eof: bool,
    ) -> Result<DecoderStatus<Self::Component<'a>>, Self::Error> {
        if self.object_active || matches!(self.state, JsonState::Failed | JsonState::Finished) {
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
        if self.object_active {
            return Err(JsonDecodeError::IncompleteToken);
        }
        match self.state {
            JsonState::Idle | JsonState::Finished => Ok(()),
            JsonState::Scalar | JsonState::String { .. } => Err(JsonDecodeError::IncompleteToken),
            JsonState::Failed => Err(JsonDecodeError::InvalidState),
        }
    }
}

impl JsonDecoder {
    pub fn next_member<'a, 'input: 'a>(
        &'a mut self,
        input: &mut &'input [u8],
        eof: bool,
        select: impl FnMut(&str) -> Result<bool, JsonDecodeError>,
    ) -> Result<DecoderStatus<&'a [u8]>, JsonDecodeError> {
        let result = self.collect_member(input, eof, select);
        match result {
            Ok(DecoderStatus::Component(value)) => {
                Ok(DecoderStatus::Component(value.unwrap_or(&self.object)))
            }
            Ok(DecoderStatus::NeedInput) => Ok(DecoderStatus::NeedInput),
            Ok(DecoderStatus::Finished) => Ok(DecoderStatus::Finished),
            Err(error) => {
                self.state = JsonState::Failed;
                Err(error)
            }
        }
    }

    fn collect_member<'input>(
        &mut self,
        input: &mut &'input [u8],
        eof: bool,
        mut select: impl FnMut(&str) -> Result<bool, JsonDecodeError>,
    ) -> Result<DecoderStatus<Option<&'input [u8]>>, JsonDecodeError> {
        use JsonToken as T;
        use MemberPhase as P;
        loop {
            if matches!(self.member_phase, P::Array | P::Element) {
                while input
                    .first()
                    .is_some_and(|b| matches!(b, b' ' | b'\t' | b'\n' | b'\r'))
                {
                    *input = &input[1..];
                }
                match input.first() {
                    Some(b']') if matches!(self.member_phase, P::Array) => {}
                    Some(_) => self.member_phase = P::ElementValue,
                    None if eof => return Err(JsonDecodeError::IncompleteToken),
                    None => return Ok(DecoderStatus::NeedInput),
                }
            }
            if matches!(self.member_phase, P::ElementValue)
                || (matches!(self.member_phase, P::Value) && !self.expand_member)
            {
                match self.collect_object(input, eof)? {
                    DecoderStatus::Component(value) => {
                        self.member_phase = if matches!(self.member_phase, P::ElementValue) {
                            P::ElementEnd
                        } else {
                            P::End
                        };
                        return Ok(DecoderStatus::Component(value));
                    }
                    DecoderStatus::NeedInput => return Ok(DecoderStatus::NeedInput),
                    DecoderStatus::Finished => return Err(JsonDecodeError::IncompleteToken),
                }
            }
            let phase = self.member_phase;
            let expand = self.expand_member;
            let token = match self.next(input, eof)? {
                DecoderStatus::Component(token) => token,
                DecoderStatus::NeedInput => return Ok(DecoderStatus::NeedInput),
                DecoderStatus::Finished if matches!(phase, P::Closed) => {
                    return Ok(DecoderStatus::Finished);
                }
                DecoderStatus::Finished => return Err(JsonDecodeError::IncompleteToken),
            };
            match (phase, token) {
                (P::Start, T::ObjectStart) => self.member_phase = P::KeyOrEnd,
                (P::KeyOrEnd | P::Key, T::String(key)) => {
                    self.expand_member = select(key)?;
                    self.member_phase = P::Colon;
                }
                (P::Colon, T::Colon) => self.member_phase = P::Value,
                (P::Value, T::ArrayStart) if expand => self.member_phase = P::Array,
                (P::Array | P::ElementEnd, T::ArrayEnd) => self.member_phase = P::End,
                (P::ElementEnd, T::Comma) => self.member_phase = P::Element,
                (P::End, T::Comma) => self.member_phase = P::Key,
                (P::KeyOrEnd | P::End, T::ObjectEnd) => self.member_phase = P::Closed,
                _ => return Err(JsonDecodeError::InvalidToken),
            }
        }
    }

    fn collect_object<'input>(
        &mut self,
        input: &mut &'input [u8],
        eof: bool,
    ) -> Result<DecoderStatus<Option<&'input [u8]>>, JsonDecodeError> {
        if !self.object_active {
            while input.first().is_some_and(u8::is_ascii_whitespace) {
                *input = &input[1..];
            }
            let Some(&first) = input.first() else {
                return if eof {
                    Err(JsonDecodeError::IncompleteToken)
                } else {
                    Ok(DecoderStatus::NeedInput)
                };
            };
            if !matches!(
                first,
                b'{' | b'[' | b'"' | b'-' | b'0'..=b'9' | b't' | b'f' | b'n'
            ) {
                return Err(JsonDecodeError::InvalidToken);
            }
            if matches!(first, b'"' | b'{' | b'[') {
                let source = *input;
                let mut values = serde_json::Deserializer::from_slice(source)
                    .into_iter::<&serde_json::value::RawValue>();
                if let Some(Ok(value)) = values.next() {
                    let bytes = value.get().as_bytes();
                    Self::validate_depth(bytes)?;
                    *input = &source[values.byte_offset()..];
                    return Ok(DecoderStatus::Component(Some(bytes)));
                }
            }
            self.object.clear();
            self.nesting.clear();
            self.quoted = false;
            self.escaped = false;
            self.object_string = first == b'"';
            self.object_container = matches!(first, b'{' | b'[');
            self.object_active = true;
        }
        while let Some(&byte) = input.first() {
            if !self.object_container
                && !self.object_string
                && (byte.is_ascii_whitespace()
                    || matches!(byte, b',' | b']' | b'}' | b':' | b'[' | b'{' | b'"'))
            {
                return self.complete_object();
            }
            *input = &input[1..];
            self.object.push(byte);
            if self.quoted {
                if self.escaped {
                    self.escaped = false;
                } else if byte == b'\\' {
                    self.escaped = true;
                } else if byte == b'"' {
                    self.quoted = false;
                    if self.object_string {
                        return self.complete_object();
                    }
                }
            } else {
                match byte {
                    b'"' => self.quoted = true,
                    b'{' | b'[' => {
                        if self.nesting.len() >= 64 {
                            return Err(JsonDecodeError::InvalidToken);
                        }
                        self.nesting.push(byte);
                    }
                    b'}' | b']' => {
                        let expected = if byte == b'}' { b'{' } else { b'[' };
                        if self.nesting.pop() != Some(expected) {
                            return Err(JsonDecodeError::InvalidToken);
                        }
                        if self.nesting.is_empty() {
                            return self.complete_object();
                        }
                    }
                    _ => {}
                }
            }
        }
        if eof {
            if self.object_container || self.quoted {
                return Err(JsonDecodeError::IncompleteToken);
            }
            self.complete_object()
        } else {
            Ok(DecoderStatus::NeedInput)
        }
    }

    fn complete_object<'input>(
        &mut self,
    ) -> Result<DecoderStatus<Option<&'input [u8]>>, JsonDecodeError> {
        let mut deserializer = serde_json::Deserializer::from_slice(&self.object);
        serde::Deserialize::deserialize(&mut deserializer)
            .map(|_: serde::de::IgnoredAny| ())
            .map_err(|_| JsonDecodeError::InvalidToken)?;
        deserializer
            .end()
            .map_err(|_| JsonDecodeError::InvalidToken)?;
        self.object_active = false;
        Ok(DecoderStatus::Component(None))
    }

    fn validate_depth(bytes: &[u8]) -> Result<(), JsonDecodeError> {
        let (mut depth, mut quoted, mut escaped) = (0_u8, false, false);
        for &byte in bytes {
            if quoted {
                if escaped {
                    escaped = false;
                } else if byte == b'\\' {
                    escaped = true;
                } else if byte == b'"' {
                    quoted = false;
                }
            } else {
                match byte {
                    b'"' => quoted = true,
                    b'{' | b'[' => {
                        if depth == 64 {
                            return Err(JsonDecodeError::InvalidToken);
                        }
                        depth += 1;
                    }
                    b'}' | b']' => depth -= 1,
                    _ => {}
                }
            }
        }
        Ok(())
    }

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
