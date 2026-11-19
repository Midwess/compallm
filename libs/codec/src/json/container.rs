use super::{JsonDecodeError, JsonDecoder, JsonToken};
use crate::decoder::{Decoder, DecoderStatus};
use serde::de::DeserializeOwned;

#[derive(Debug)]
pub struct JsonContainer {
    decoder: Option<JsonDecoder>,
    parent: Option<Box<Self>>,
    array: bool,
    complete: bool,
    nonempty: bool,
    has_value: bool,
    failed: bool,
}

impl JsonContainer {
    pub fn new(decoder: JsonDecoder) -> Self {
        Self {
            decoder: Some(decoder),
            parent: None,
            array: false,
            complete: false,
            nonempty: false,
            has_value: false,
            failed: false,
        }
    }

    pub fn nonempty(mut self) -> Self {
        self.nonempty = true;
        self
    }

    pub fn enter(mut self) -> Result<Self, JsonDecodeError> {
        let array = self.decoder()?.container_start()?;
        self.decoder()?.enter_container(array)?;
        Ok(Self {
            decoder: None,
            parent: Some(Box::new(self)),
            array,
            complete: false,
            nonempty: false,
            has_value: false,
            failed: false,
        })
    }

    pub fn enter_value(mut self) -> Result<Self, JsonDecodeError> {
        let array = self.decoder()?.container_start()?;
        self.decoder()?.enter_value(array)?;
        Ok(Self {
            decoder: None,
            parent: Some(Box::new(self)),
            array: false,
            complete: false,
            nonempty: false,
            has_value: false,
            failed: false,
        })
    }

    fn decoder(&mut self) -> Result<&mut JsonDecoder, JsonDecodeError> {
        match &mut self.decoder {
            Some(decoder) => Ok(decoder),
            None => self
                .parent
                .as_mut()
                .ok_or(JsonDecodeError::InvalidState)?
                .decoder(),
        }
    }

    fn restore(&mut self) -> Result<(), JsonDecodeError> {
        if self.complete {
            *self = *self.parent.take().ok_or(JsonDecodeError::InvalidState)?;
        }
        Ok(())
    }

    pub fn parse_next<T: DeserializeOwned>(
        &mut self,
        input: &mut &[u8],
        eof: bool,
    ) -> Result<DecoderStatus<T>, JsonDecodeError> {
        if self.failed || self.parent.is_none() {
            return Err(JsonDecodeError::InvalidState);
        }
        if self.complete {
            self.restore()?;
            return Ok(DecoderStatus::Finished);
        }
        let status = self
            .decoder()?
            .parse_next::<T>(input, eof)
            .inspect_err(|_| self.failed = true)?;
        match &status {
            DecoderStatus::Component(_) => {
                self.has_value = true;
                self.complete = !self.array;
            }
            DecoderStatus::Finished => {
                if !self.array || (self.nonempty && !self.has_value) {
                    self.failed = true;
                    return Err(JsonDecodeError::InvalidToken);
                }
                self.complete = true;
                self.restore()?;
            }
            DecoderStatus::NeedInput => {}
        }
        Ok(status)
    }

    pub fn next<'a, 'input: 'a>(
        &'a mut self,
        input: &mut &'input [u8],
        eof: bool,
    ) -> Result<DecoderStatus<JsonToken<'a>>, JsonDecodeError> {
        if self.failed {
            return Err(JsonDecodeError::InvalidState);
        }
        self.restore()?;
        if self.parent.is_some() {
            return Err(JsonDecodeError::InvalidState);
        }
        self.decoder()?.next(input, eof)
    }

    pub fn finish(mut self) -> Result<(), JsonDecodeError> {
        if self.failed {
            return Err(JsonDecodeError::InvalidState);
        }
        self.restore()?;
        if self.parent.is_some() {
            return Err(JsonDecodeError::IncompleteToken);
        }
        self.decoder.ok_or(JsonDecodeError::InvalidState)?.finish()
    }
}
