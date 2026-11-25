use super::grammar::{Grammar, Position};
use super::state::ParseState;
use super::token::{JsonToken, TokenDecoder};
use super::{JsonDecodeError, JsonToken as Event};
use crate::decoder::{Decoder, DecoderStatus};

#[derive(Debug, Default)]
pub struct JsonDecoder {
    reader: JsonReader,
    parse: ParseState,
    container_start: Option<bool>,
}

#[derive(Debug, Default)]
struct JsonReader {
    tokens: TokenDecoder,
    grammar: Grammar,
    failed: bool,
}

enum Prepared {
    Value,
    ObjectEnd,
    ArrayEnd,
}

impl JsonDecoder {
    pub(crate) fn string_layout(&self) -> super::StringLayout {
        self.parse.string_layout()
    }

    pub(crate) fn resolve_retained(
        &self,
        reference: super::RetainedString,
    ) -> Result<&str, JsonDecodeError> {
        self.parse.resolve_retained(reference)
    }

    pub(crate) fn resolve_string<'a>(
        &'a self,
        source: &'a [u8],
        reference: super::StringReference,
    ) -> Result<&'a str, JsonDecodeError> {
        match reference {
            super::StringReference::Input(range) => {
                std::str::from_utf8(source.get(range).ok_or(JsonDecodeError::InvalidState)?)
                    .map_err(|_| JsonDecodeError::InvalidState)
            }
            super::StringReference::Buffered => self.reader.tokens.buffered_string(),
        }
    }

    pub(super) fn container_start(&self) -> Result<bool, JsonDecodeError> {
        self.container_start.ok_or(JsonDecodeError::InvalidState)
    }

    pub(super) fn enter_value(&mut self, array: bool) -> Result<(), JsonDecodeError> {
        self.enter_container(array)?;
        if array {
            self.parse.active = true;
            self.parse.push(Event::ArrayStart(None))?;
            self.parse.buffer(b"[");
        }
        Ok(())
    }
    pub(super) fn enter_container(&mut self, array: bool) -> Result<(), JsonDecodeError> {
        if self.reader.failed || self.parse.active || self.container_start != Some(array) {
            return Err(JsonDecodeError::InvalidState);
        }
        self.container_start = None;
        self.parse.reset();
        if !array {
            self.parse.active = true;
            self.parse.push(Event::ObjectStart(None))?;
            self.parse.buffer(b"{");
        }
        Ok(())
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_capacity(buffer_bytes: usize) -> Self {
        let bytes = if buffer_bytes == 0 {
            16 * 1024
        } else {
            buffer_bytes
        };
        Self {
            reader: JsonReader {
                tokens: TokenDecoder::with_capacity(bytes),
                grammar: Grammar::with_capacity(bytes),
                failed: false,
            },
            parse: ParseState::with_capacity(bytes),
            container_start: None,
        }
    }

    pub fn parse_next<'de, 'input: 'de, T: serde::Deserialize<'de>>(
        &'de mut self,
        input: &mut &'input [u8],
        eof: bool,
    ) -> Result<DecoderStatus<T>, JsonDecodeError> {
        let source = *input;
        match self.capture_next(input, eof)? {
            DecoderStatus::Component(range) => self
                .deserialize_captured(source, range)
                .map(DecoderStatus::Component),
            DecoderStatus::NeedInput => Ok(DecoderStatus::NeedInput),
            DecoderStatus::Finished => Ok(DecoderStatus::Finished),
        }
    }

    pub(crate) fn retain_captured(
        &mut self,
        source: &[u8],
        range: std::ops::Range<usize>,
    ) -> Result<(), JsonDecodeError> {
        self.parse
            .retain(source.get(range).ok_or(JsonDecodeError::InvalidState)?);
        Ok(())
    }

    pub(crate) fn deserialize_captured<'de, T: serde::Deserialize<'de>>(
        &'de mut self,
        source: &'de [u8],
        range: std::ops::Range<usize>,
    ) -> Result<T, JsonDecodeError> {
        let bytes = source.get(range).ok_or(JsonDecodeError::InvalidState)?;
        self.parse
            .deserialize(bytes)
            .inspect_err(|_| self.reader.failed = true)
    }

    pub(crate) fn capture_next(
        &mut self,
        input: &mut &[u8],
        eof: bool,
    ) -> Result<DecoderStatus<std::ops::Range<usize>>, JsonDecodeError> {
        if self.reader.failed {
            return Err(JsonDecodeError::InvalidState);
        }
        let original_length = input.len();
        self.container_start = None;
        if !self.parse.active {
            self.parse.reset();
            // Exclude parent keys and separators from the captured value.
            match self.reader.prepare(input, eof) {
                Ok(DecoderStatus::Component(Prepared::Value)) => self.parse.active = true,
                Ok(DecoderStatus::NeedInput) => return Ok(DecoderStatus::NeedInput),
                Ok(DecoderStatus::Component(Prepared::ArrayEnd)) => {
                    return Ok(DecoderStatus::Finished);
                }
                Ok(_) => {
                    self.reader.failed = true;
                    return Err(JsonDecodeError::InvalidState);
                }
                Err(error) => {
                    self.reader.failed = true;
                    return Err(error);
                }
            }
        }
        let start = original_length - input.len();
        let source = *input;
        loop {
            match self.reader.next(input, eof)? {
                DecoderStatus::Component(token) => {
                    if matches!(token, Event::ArrayEnd) && self.parse.depth == 0 {
                        // The enclosing array ended before it held a value.
                        self.parse.discard();
                        return Ok(DecoderStatus::Finished);
                    }
                    let complete = self
                        .parse
                        .push(token)
                        .inspect_err(|_| self.reader.failed = true)?;
                    if complete {
                        let length = source.len() - input.len();
                        self.parse.complete_capture(&source[..length]);
                        return Ok(DecoderStatus::Component(start..start + length));
                    }
                }
                DecoderStatus::NeedInput => {
                    self.parse.buffer(&source[..source.len() - input.len()]);
                    return Ok(DecoderStatus::NeedInput);
                }
                DecoderStatus::Finished => {
                    self.reader.failed = true;
                    return Err(JsonDecodeError::IncompleteToken);
                }
            }
        }
    }
}

impl JsonReader {
    fn prepare(
        &mut self,
        input: &mut &[u8],
        eof: bool,
    ) -> Result<DecoderStatus<Prepared>, JsonDecodeError> {
        loop {
            if self.grammar.depth == 0 {
                return Ok(DecoderStatus::Component(Prepared::Value));
            }
            let position = self.grammar.stack[self.grammar.depth - 1];
            if matches!(
                position,
                Position::ObjectValue | Position::ArrayFirst | Position::ArrayValue
            ) {
                return Ok(DecoderStatus::Component(Prepared::Value));
            }
            let token = match self.tokens.next(input, eof)? {
                DecoderStatus::Component(token) => token,
                DecoderStatus::NeedInput => return Ok(DecoderStatus::NeedInput),
                DecoderStatus::Finished => return Err(JsonDecodeError::IncompleteToken),
            };
            self.grammar.accept(&token)?;
            match token {
                JsonToken::ObjectEnd => return Ok(DecoderStatus::Component(Prepared::ObjectEnd)),
                JsonToken::ArrayEnd => return Ok(DecoderStatus::Component(Prepared::ArrayEnd)),
                _ => {}
            }
        }
    }
}

impl Decoder for JsonDecoder {
    type Component<'a> = Event<'a>;
    type Error = JsonDecodeError;

    fn next<'a, 'input: 'a>(
        &'a mut self,
        input: &mut &'input [u8],
        eof: bool,
    ) -> Result<DecoderStatus<Event<'a>>, JsonDecodeError> {
        if self.parse.active {
            return Err(JsonDecodeError::InvalidState);
        }
        self.container_start = None;
        let status = self.reader.next(input, eof)?;
        self.container_start = match status {
            DecoderStatus::Component(Event::ObjectStart(_)) => Some(false),
            DecoderStatus::Component(Event::ArrayStart(_)) => Some(true),
            _ => None,
        };
        Ok(status)
    }

    fn finish(self) -> Result<(), JsonDecodeError> {
        if self.reader.failed {
            return Err(JsonDecodeError::InvalidState);
        }
        if self.parse.active {
            return Err(JsonDecodeError::IncompleteToken);
        }
        self.reader.finish()
    }
}

impl Decoder for JsonReader {
    type Component<'a> = Event<'a>;
    type Error = JsonDecodeError;

    fn next<'a, 'input: 'a>(
        &'a mut self,
        input: &mut &'input [u8],
        eof: bool,
    ) -> Result<DecoderStatus<Event<'a>>, JsonDecodeError> {
        if self.failed {
            return Err(JsonDecodeError::InvalidState);
        }
        match self.prepare(input, eof) {
            Ok(DecoderStatus::NeedInput) => return Ok(DecoderStatus::NeedInput),
            Ok(DecoderStatus::Component(Prepared::ObjectEnd)) => {
                return Ok(DecoderStatus::Component(Event::ObjectEnd));
            }
            Ok(DecoderStatus::Component(Prepared::ArrayEnd)) => {
                return Ok(DecoderStatus::Component(Event::ArrayEnd));
            }
            Ok(DecoderStatus::Component(Prepared::Value)) => {}
            Ok(DecoderStatus::Finished) => unreachable!(),
            Err(error) => {
                self.failed = true;
                return Err(error);
            }
        }
        let has_key = self.grammar.depth > 0
            && matches!(
                self.grammar.stack[self.grammar.depth - 1],
                Position::ObjectValue
            );
        match self.tokens.next(input, eof) {
            Ok(DecoderStatus::Component(token)) => {
                if let Err(error) = self.grammar.accept(&token) {
                    self.failed = true;
                    return Err(error);
                }
                let key = if has_key { self.grammar.key() } else { None };
                let event = match token {
                    JsonToken::String(value) => Event::ValueString(key, value),
                    JsonToken::Number(value) => Event::ValueNumber(key, value),
                    JsonToken::Boolean(value) => Event::ValueBoolean(key, value),
                    JsonToken::Null => Event::ValueNull(key),
                    JsonToken::ObjectStart => Event::ObjectStart(key),
                    JsonToken::ArrayStart => Event::ArrayStart(key),
                    JsonToken::ObjectEnd => Event::ObjectEnd,
                    JsonToken::ArrayEnd => Event::ArrayEnd,
                    _ => {
                        self.failed = true;
                        return Err(JsonDecodeError::InvalidToken);
                    }
                };
                Ok(DecoderStatus::Component(event))
            }
            Ok(DecoderStatus::Finished) if !self.grammar.complete() => {
                self.failed = true;
                Err(JsonDecodeError::IncompleteToken)
            }
            Ok(DecoderStatus::NeedInput) => Ok(DecoderStatus::NeedInput),
            Ok(DecoderStatus::Finished) => Ok(DecoderStatus::Finished),
            Err(error) => {
                self.failed = true;
                Err(error)
            }
        }
    }

    fn finish(self) -> Result<(), JsonDecodeError> {
        if self.failed {
            return Err(JsonDecodeError::InvalidState);
        }
        if !self.grammar.complete() {
            return Err(JsonDecodeError::IncompleteToken);
        }
        self.tokens.finish()
    }
}
