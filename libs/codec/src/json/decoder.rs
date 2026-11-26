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
    inner: std::cell::RefCell<JsonParser>,
    current: Option<super::component::ReadyToken>,
    current_source: Option<(usize, usize)>,
    current_value: Option<std::ops::Range<usize>>,
}

#[derive(Debug, Default)]
pub(super) struct JsonParser {
    reader: JsonReader,
    parse: ParseState,
}

#[derive(Debug, Default)]
struct JsonReader {
    tokens: TokenDecoder,
    grammar: Grammar,
    failed: bool,
    // Input addresses are used only during one next call, before input can be reused.
    key_input: Option<std::ops::Range<usize>>,
}

enum Prepared {
    Value,
    ObjectEnd,
    ArrayEnd,
}

impl JsonDecoder {
    pub fn advance(
        &mut self,
        input: &mut &[u8],
        eof: bool,
    ) -> Result<DecoderStatus<()>, JsonDecodeError> {
        self.current = None;
        self.current_source = None;
        self.current_value = None;
        self.container_start = None;
        if self.parse.active {
            return Err(JsonDecodeError::InvalidState);
        }
        let source = *input;
        match self.reader.next(input, eof)? {
            DecoderStatus::Component(token) => {
                self.container_start = match token {
                    Event::ObjectStart(_) => Some(false),
                    Event::ArrayStart(_) => Some(true),
                    _ => None,
                };
                self.current = Some(super::component::ReadyToken::new(token, source));
                self.current_source = Some((source.as_ptr() as usize, source.len()));
                Ok(DecoderStatus::Component(()))
            }
            DecoderStatus::NeedInput => Ok(DecoderStatus::NeedInput),
            DecoderStatus::Finished => Ok(DecoderStatus::Finished),
        }
    }

    pub fn current<'a>(&'a self, source: &'a [u8]) -> Result<Event<'a>, JsonDecodeError> {
        use super::component::ReadyToken;
        if self.current_source != Some((source.as_ptr() as usize, source.len())) {
            return Err(JsonDecodeError::InvalidState);
        }
        let key = |reference: &Option<super::StringReference>| {
            reference
                .as_ref()
                .map(|reference| match reference {
                    super::StringReference::Input(range) => source
                        .get(range.clone())
                        .and_then(|bytes| std::str::from_utf8(bytes).ok())
                        .ok_or(JsonDecodeError::InvalidState),
                    super::StringReference::Buffered => self
                        .reader
                        .grammar
                        .key()
                        .ok_or(JsonDecodeError::InvalidState),
                })
                .transpose()
        };
        Ok(
            match self.current.as_ref().ok_or(JsonDecodeError::InvalidState)? {
                ReadyToken::ObjectStart(reference) => Event::ObjectStart(key(reference)?),
                ReadyToken::ObjectEnd => Event::ObjectEnd,
                ReadyToken::ArrayStart(reference) => Event::ArrayStart(key(reference)?),
                ReadyToken::ArrayEnd => Event::ArrayEnd,
                ReadyToken::String(reference, value) => {
                    let value = match value {
                        super::StringReference::Input(range) => source
                            .get(range.clone())
                            .and_then(|bytes| std::str::from_utf8(bytes).ok())
                            .ok_or(JsonDecodeError::InvalidState)?,
                        super::StringReference::Buffered => self.reader.tokens.buffered_string()?,
                    };
                    Event::ValueString(key(reference)?, value)
                }
                ReadyToken::Number(reference, value) => {
                    Event::ValueNumber(key(reference)?, value.clone())
                }
                ReadyToken::Boolean(reference, value) => {
                    Event::ValueBoolean(key(reference)?, *value)
                }
                ReadyToken::Null(reference) => Event::ValueNull(key(reference)?),
            },
        )
    }
    pub(super) fn container_start(&self) -> Result<bool, JsonDecodeError> {
        self.container_start.ok_or(JsonDecodeError::InvalidState)
    }

    pub(super) fn enter_value(&mut self, array: bool) -> Result<(), JsonDecodeError> {
        self.enter_container(array)?;
        if array {
            self.parse.active = true;
            self.parse.push(Event::ArrayStart(None), 1, &[], 0)?;
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
            self.parse.push(Event::ObjectStart(None), 1, &[], 0)?;
            self.parse.buffer(b"{");
        }
        Ok(())
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn advance_value(
        &mut self,
        input: &mut &[u8],
        eof: bool,
    ) -> Result<DecoderStatus<()>, JsonDecodeError> {
        let source = *input;
        match self.capture_next(input, eof)? {
            DecoderStatus::Component(range) => {
                self.set_current_value(source, range);
                Ok(DecoderStatus::Component(()))
            }
            DecoderStatus::NeedInput => Ok(DecoderStatus::NeedInput),
            DecoderStatus::Finished => Ok(DecoderStatus::Finished),
        }
    }

    pub(super) fn set_current_value(&mut self, source: &[u8], range: std::ops::Range<usize>) {
        self.current_source = Some((source.as_ptr() as usize, source.len()));
        self.current_value = Some(range);
    }

    pub fn deserialize_current<'de, T: serde::Deserialize<'de>>(
        &'de self,
        source: &'de [u8],
    ) -> Result<T, JsonDecodeError> {
        if self.current_source != Some((source.as_ptr() as usize, source.len())) {
            return Err(JsonDecodeError::InvalidState);
        }
        let range = self
            .current_value
            .as_ref()
            .ok_or(JsonDecodeError::InvalidState)?;
        let bytes = source
            .get(range.clone())
            .ok_or(JsonDecodeError::InvalidState)?;
        let mut inner = self
            .inner
            .try_borrow_mut()
            .map_err(|_| JsonDecodeError::InvalidState)?;
        self.parse.deserialize(bytes, &mut inner)
    }

    pub fn view_current<'a>(
        &'a self,
        source: &'a [u8],
    ) -> Result<super::JsonView<&'a str>, JsonDecodeError> {
        if self.current_source != Some((source.as_ptr() as usize, source.len())) {
            return Err(JsonDecodeError::InvalidState);
        }
        let range = self
            .current_value
            .as_ref()
            .ok_or(JsonDecodeError::InvalidState)?;
        let bytes = source
            .get(range.clone())
            .ok_or(JsonDecodeError::InvalidState)?;
        self.parse.source(bytes).map(super::JsonView::new)
    }

    pub fn deserialize_next<'de, 'input: 'de, T: serde::Deserialize<'de>>(
        &'de mut self,
        input: &mut &'input [u8],
        eof: bool,
    ) -> Result<DecoderStatus<T>, JsonDecodeError> {
        let source = *input;
        match self.advance_value(input, eof)? {
            DecoderStatus::Component(()) => self
                .deserialize_current(source)
                .map(DecoderStatus::Component),
            DecoderStatus::NeedInput => Ok(DecoderStatus::NeedInput),
            DecoderStatus::Finished => Ok(DecoderStatus::Finished),
        }
    }

    pub(super) fn capture_next(
        &mut self,
        input: &mut &[u8],
        eof: bool,
    ) -> Result<DecoderStatus<std::ops::Range<usize>>, JsonDecodeError> {
        self.container_start = None;
        self.current = None;
        self.current_source = None;
        self.current_value = None;
        JsonParser::capture(&mut self.reader, &mut self.parse, input, eof)
    }

    pub(super) fn validate_value(source: &str) -> Result<(), JsonDecodeError> {
        let mut parser = JsonParser::default();
        parser.capture_value(source)?;
        Ok(())
    }

    pub(super) fn decode_value(source: &str) -> Result<super::JsonValue<&str>, JsonDecodeError> {
        let mut parser = JsonParser::default();
        parser.capture_value(source)?;
        Ok(parser.parse.capture.into_value(source))
    }
}

impl JsonParser {
    pub(super) fn capture_value(
        &mut self,
        source: &str,
    ) -> Result<&super::capture::Capture, JsonDecodeError> {
        self.reader.tokens.reset();
        self.reader.grammar.reset();
        self.reader.failed = false;
        self.reader.key_input = None;
        self.parse.discard();
        let mut input = source.as_bytes();
        match Self::capture(&mut self.reader, &mut self.parse, &mut input, true)? {
            DecoderStatus::Component(_) => {}
            _ => return Err(JsonDecodeError::IncompleteToken),
        }
        if !matches!(self.reader.next(&mut input, true)?, DecoderStatus::Finished) {
            return Err(JsonDecodeError::InvalidToken);
        }
        Ok(&self.parse.capture)
    }

    fn capture(
        reader: &mut JsonReader,
        parse: &mut ParseState,
        input: &mut &[u8],
        eof: bool,
    ) -> Result<DecoderStatus<std::ops::Range<usize>>, JsonDecodeError> {
        if reader.failed {
            return Err(JsonDecodeError::InvalidState);
        }
        let original_length = input.len();
        if !parse.active {
            parse.reset();
            // Exclude parent keys and separators from the captured value.
            match reader.prepare(input, eof) {
                Ok(DecoderStatus::Component(Prepared::Value)) => parse.active = true,
                Ok(DecoderStatus::NeedInput) => return Ok(DecoderStatus::NeedInput),
                Ok(DecoderStatus::Component(Prepared::ArrayEnd)) => {
                    return Ok(DecoderStatus::Finished);
                }
                Ok(_) => {
                    reader.failed = true;
                    return Err(JsonDecodeError::InvalidState);
                }
                Err(error) => {
                    reader.failed = true;
                    return Err(error);
                }
            }
        }
        let start = original_length - input.len();
        let source = *input;
        loop {
            match reader.next(input, eof)? {
                DecoderStatus::Component(token) => {
                    if matches!(token, Event::ArrayEnd) && parse.depth == 0 {
                        // The enclosing array ended before it held a value.
                        parse.discard();
                        return Ok(DecoderStatus::Finished);
                    }
                    let offset = parse.captured_bytes();
                    let end = offset + source.len() - input.len();
                    let complete = parse
                        .push(token, end, source, offset)
                        .inspect_err(|_| reader.failed = true)?;
                    if complete {
                        let length = source.len() - input.len();
                        parse.complete_capture(&source[..length]);
                        return Ok(DecoderStatus::Component(start..start + length));
                    }
                }
                DecoderStatus::NeedInput => {
                    parse.buffer(&source[..source.len() - input.len()]);
                    return Ok(DecoderStatus::NeedInput);
                }
                DecoderStatus::Finished => {
                    reader.failed = true;
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
            let source = *input;
            let token = match self.tokens.next(input, eof)? {
                DecoderStatus::Component(token) => token,
                DecoderStatus::NeedInput => return Ok(DecoderStatus::NeedInput),
                DecoderStatus::Finished => return Err(JsonDecodeError::IncompleteToken),
            };
            if let JsonToken::String(value) = &token
                && matches!(position, Position::ObjectFirst | Position::Key)
                && let super::StringReference::Input(range) =
                    super::StringReference::new(value, source)
            {
                let base = source.as_ptr() as usize;
                self.key_input = Some(base + range.start..base + range.end);
            }
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
        let source = *input;
        match self.advance(input, eof)? {
            DecoderStatus::Component(()) => self.current(source).map(DecoderStatus::Component),
            DecoderStatus::NeedInput => Ok(DecoderStatus::NeedInput),
            DecoderStatus::Finished => Ok(DecoderStatus::Finished),
        }
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
        let source = *input;
        self.key_input = None;
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
                let key = if has_key {
                    self.key_input
                        .as_ref()
                        .and_then(|range| {
                            let base = source.as_ptr() as usize;
                            let start = range.start.checked_sub(base)?;
                            let end = range.end.checked_sub(base)?;
                            std::str::from_utf8(source.get(start..end)?).ok()
                        })
                        .or_else(|| self.grammar.key())
                } else {
                    None
                };
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
