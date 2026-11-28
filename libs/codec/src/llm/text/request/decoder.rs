use crate::decoder::{DecoderError, DecoderStatus};
use crate::json::{JsonContainer, JsonDecoder, JsonToken};
use serde::Deserialize;
use std::cell::Cell;

#[derive(Clone, Copy, Debug)]
pub(in crate::llm::text) enum CaptureMode {
    Items,
    Value,
}

#[derive(Clone, Copy, Debug)]
pub(in crate::llm::text) enum RequestStep<F> {
    Token,
    Value(F),
    Closed,
    NeedInput,
    Finished,
}

pub(in crate::llm::text) enum Ready<F> {
    Scalar,
    Value(F),
}

#[derive(Clone, Copy, Debug)]
struct Pending<F> {
    field: F,
    mode: CaptureMode,
}

#[derive(Debug)]
pub(in crate::llm::text) struct TextRequestDecoder<F> {
    json: JsonContainer,
    pending: Option<Pending<F>>,
    started: bool,
    closed: bool,
    failed: Cell<bool>,
}

impl<F: Copy> TextRequestDecoder<F> {
    pub(in crate::llm::text) fn new(json: JsonDecoder) -> Self {
        Self {
            json: JsonContainer::new(json),
            pending: None,
            started: false,
            closed: false,
            failed: Cell::new(false),
        }
    }

    pub(in crate::llm::text) fn begin(&mut self) -> Result<(), DecoderError> {
        if self.failed.replace(true) {
            return Err(DecoderError::InvalidState);
        }
        Ok(())
    }

    pub(in crate::llm::text) fn complete(&self) {
        self.failed.set(false);
    }

    pub(in crate::llm::text) fn advance<'input>(
        &mut self,
        input: &mut &'input [u8],
        eof: bool,
    ) -> Result<(RequestStep<F>, &'input [u8]), DecoderError> {
        loop {
            let source = *input;
            let status = if self.pending.is_some() {
                self.json.advance_value(input, eof)?
            } else {
                self.json.advance(input, eof)?
            };
            match status {
                DecoderStatus::NeedInput => return Ok((RequestStep::NeedInput, source)),
                DecoderStatus::Finished => {
                    if let Some(pending) = self.pending.take() {
                        if matches!(pending.mode, CaptureMode::Items) {
                            continue;
                        }
                        return Err(DecoderError::IncompleteInput);
                    }
                    if !self.closed {
                        return Err(DecoderError::IncompleteInput);
                    }
                    return Ok((RequestStep::Finished, source));
                }
                DecoderStatus::Component(()) => {}
            }
            if let Some(pending) = self.pending {
                if matches!(pending.mode, CaptureMode::Value) {
                    self.pending = None;
                }
                return Ok((RequestStep::Value(pending.field), source));
            }
            match self.json.current(source)? {
                JsonToken::ObjectStart(None) if !self.started => self.started = true,
                JsonToken::ObjectEnd if self.started && !self.closed => {
                    self.closed = true;
                    return Ok((RequestStep::Closed, source));
                }
                JsonToken::ArrayStart(Some(_))
                | JsonToken::ObjectStart(Some(_))
                | JsonToken::ValueString(..)
                | JsonToken::ValueNumber(..)
                | JsonToken::ValueBoolean(..)
                | JsonToken::ValueNull(..)
                    if self.started && !self.closed =>
                {
                    return Ok((RequestStep::Token, source));
                }
                _ => return Err(DecoderError::InvalidInput("invalid request container")),
            }
        }
    }

    pub(in crate::llm::text) fn enter(
        &mut self,
        field: F,
        mode: CaptureMode,
        nonempty: bool,
    ) -> Result<(), DecoderError> {
        if self.pending.is_some() {
            return Err(DecoderError::InvalidState);
        }
        match mode {
            CaptureMode::Items => {
                self.json.enter()?;
            }
            CaptureMode::Value => {
                self.json.enter_value()?;
            }
        }
        if nonempty {
            self.json.nonempty();
        }
        self.pending = Some(Pending { field, mode });
        Ok(())
    }

    pub(in crate::llm::text) fn current<'a>(
        &'a self,
        source: &'a [u8],
    ) -> Result<JsonToken<'a>, DecoderError> {
        self.json.current(source).map_err(Into::into)
    }

    pub(in crate::llm::text) fn deserialize_current<'a, T: Deserialize<'a>>(
        &'a self,
        source: &'a [u8],
    ) -> Result<T, DecoderError> {
        self.json.deserialize_current(source).map_err(Into::into)
    }

    pub(in crate::llm::text) fn finish(self) -> Result<(), DecoderError> {
        if self.failed.get() {
            return Err(DecoderError::InvalidState);
        }
        if !self.closed {
            return Err(DecoderError::IncompleteInput);
        }
        self.json.finish().map_err(Into::into)
    }
}
