use super::{JsonDecodeError, JsonDecoder, JsonToken};
use crate::decoder::{Decoder, DecoderStatus};
use serde::Deserialize;

#[derive(Clone, Copy, Debug, Default)]
struct Frame {
    array: bool,
    complete: bool,
    nonempty: bool,
    has_value: bool,
}

#[derive(Debug)]
pub struct JsonContainer {
    decoder: JsonDecoder,
    frames: [Frame; 65],
    depth: usize,
    failed: bool,
}

impl JsonContainer {
    pub fn new(decoder: JsonDecoder) -> Self {
        Self {
            decoder,
            frames: [Frame::default(); 65],
            depth: 0,
            failed: false,
        }
    }

    pub fn nonempty(&mut self) -> &mut Self {
        self.frames[self.depth].nonempty = true;
        self
    }

    pub fn enter(&mut self) -> Result<&mut Self, JsonDecodeError> {
        if self.failed || self.depth + 1 == self.frames.len() {
            return Err(JsonDecodeError::InvalidState);
        }
        let array = self.decoder.container_start()?;
        self.decoder.enter_container(array)?;
        self.depth += 1;
        self.frames[self.depth] = Frame {
            array,
            ..Frame::default()
        };
        Ok(self)
    }

    pub fn enter_value(&mut self) -> Result<&mut Self, JsonDecodeError> {
        if self.failed || self.depth + 1 == self.frames.len() {
            return Err(JsonDecodeError::InvalidState);
        }
        let array = self.decoder.container_start()?;
        self.decoder.enter_value(array)?;
        self.depth += 1;
        self.frames[self.depth] = Frame::default();
        Ok(self)
    }

    fn restore(&mut self) -> Result<(), JsonDecodeError> {
        if self.frames[self.depth].complete {
            self.depth = self
                .depth
                .checked_sub(1)
                .ok_or(JsonDecodeError::InvalidState)?;
        }
        Ok(())
    }

    pub fn deserialize_next<'de, 'input: 'de, T: Deserialize<'de>>(
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

    pub fn advance_value(
        &mut self,
        input: &mut &[u8],
        eof: bool,
    ) -> Result<DecoderStatus<()>, JsonDecodeError> {
        let source = *input;
        match self.capture_next(input, eof)? {
            DecoderStatus::Component(range) => {
                self.decoder.set_current_value(source, range);
                Ok(DecoderStatus::Component(()))
            }
            DecoderStatus::NeedInput => Ok(DecoderStatus::NeedInput),
            DecoderStatus::Finished => Ok(DecoderStatus::Finished),
        }
    }

    pub fn deserialize_current<'de, T: Deserialize<'de>>(
        &'de self,
        source: &'de [u8],
    ) -> Result<T, JsonDecodeError> {
        if self.failed {
            return Err(JsonDecodeError::InvalidState);
        }
        self.decoder.deserialize_current(source)
    }

    pub fn view_current<'a>(
        &'a self,
        source: &'a [u8],
    ) -> Result<super::JsonView<&'a str>, JsonDecodeError> {
        if self.failed {
            return Err(JsonDecodeError::InvalidState);
        }
        self.decoder.view_current(source)
    }

    pub(super) fn capture_next(
        &mut self,
        input: &mut &[u8],
        eof: bool,
    ) -> Result<DecoderStatus<std::ops::Range<usize>>, JsonDecodeError> {
        if self.failed || self.depth == 0 {
            return Err(JsonDecodeError::InvalidState);
        }
        if self.frames[self.depth].complete {
            self.restore()?;
            return Ok(DecoderStatus::Finished);
        }
        let status = self
            .decoder
            .capture_next(input, eof)
            .inspect_err(|_| self.failed = true)?;
        let frame = &mut self.frames[self.depth];
        match &status {
            DecoderStatus::Component(_) => {
                frame.has_value = true;
                frame.complete = !frame.array;
            }
            DecoderStatus::Finished => {
                if !frame.array || (frame.nonempty && !frame.has_value) {
                    self.failed = true;
                    return Err(JsonDecodeError::InvalidToken);
                }
                frame.complete = true;
            }
            DecoderStatus::NeedInput => {}
        }
        Ok(status)
    }

    pub fn advance(
        &mut self,
        input: &mut &[u8],
        eof: bool,
    ) -> Result<DecoderStatus<()>, JsonDecodeError> {
        if self.failed {
            return Err(JsonDecodeError::InvalidState);
        }
        self.restore()?;
        if self.depth != 0 {
            return Err(JsonDecodeError::InvalidState);
        }
        self.decoder.advance(input, eof)
    }

    pub fn current<'a>(&'a self, source: &'a [u8]) -> Result<JsonToken<'a>, JsonDecodeError> {
        if self.failed || self.depth != 0 {
            return Err(JsonDecodeError::InvalidState);
        }
        self.decoder.current(source)
    }

    pub fn next<'a, 'input: 'a>(
        &'a mut self,
        input: &mut &'input [u8],
        eof: bool,
    ) -> Result<DecoderStatus<JsonToken<'a>>, JsonDecodeError> {
        let source = *input;
        match self.advance(input, eof)? {
            DecoderStatus::Component(()) => self.current(source).map(DecoderStatus::Component),
            DecoderStatus::NeedInput => Ok(DecoderStatus::NeedInput),
            DecoderStatus::Finished => Ok(DecoderStatus::Finished),
        }
    }

    pub fn finish(mut self) -> Result<(), JsonDecodeError> {
        if self.failed {
            return Err(JsonDecodeError::InvalidState);
        }
        self.restore()?;
        if self.depth != 0 {
            return Err(JsonDecodeError::IncompleteToken);
        }
        self.decoder.finish()
    }
}
