use super::grammar::{Grammar, Position};
use super::token::{JsonToken, TokenDecoder};
use super::{JsonDecodeError, JsonToken as Event};
use crate::decoder::{Decoder, DecoderStatus};

#[derive(Debug, Default)]
pub struct JsonDecoder {
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
    pub fn new() -> Self {
        Self::default()
    }

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
                    JsonToken::ObjectStart => Event::ValueObject(key),
                    JsonToken::ArrayStart => Event::ValueArray(key),
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
