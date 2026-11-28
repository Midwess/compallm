use crate::encoder::{Encoder, EncoderStatus};
use crate::json::{JsonEncodeError, JsonEncoder, JsonToken};

#[derive(Debug, thiserror::Error)]
pub enum RequestEncodeError {
    #[error(transparent)]
    Json(#[from] JsonEncodeError),
    #[error("invalid request encoder state")]
    InvalidState,
    #[error("model and messages are required")]
    Incomplete,
}

#[derive(Debug)]
pub(in crate::llm::text) struct TextRequestEncoder<F> {
    json: JsonEncoder,
    message_field: F,
    message_key: &'static str,
    started: bool,
    messages_open: bool,
    messages_closed: bool,
    field: Option<F>,
    finalizing: bool,
    closed: bool,
    finished: bool,
    failed: bool,
}

impl<F: Copy + PartialEq> TextRequestEncoder<F> {
    pub(in crate::llm::text) fn new(
        json: JsonEncoder,
        message_field: F,
        message_key: &'static str,
    ) -> Self {
        Self {
            json,
            message_field,
            message_key,
            started: false,
            messages_open: false,
            messages_closed: false,
            field: None,
            finalizing: false,
            closed: false,
            finished: false,
            failed: false,
        }
    }
    pub(in crate::llm::text) fn check(
        &self,
        has_component: bool,
    ) -> Result<bool, RequestEncodeError> {
        if self.failed || (self.finalizing && has_component) {
            return Err(RequestEncodeError::InvalidState);
        }
        Ok(self.finished)
    }
    pub(in crate::llm::text) fn field(&self) -> Option<F> {
        self.field
    }
    pub(in crate::llm::text) fn set_field(&mut self, field: F) -> Result<(), RequestEncodeError> {
        if self.field.is_some_and(|pending| pending != field) {
            self.failed = true;
            return Err(RequestEncodeError::InvalidState);
        }
        self.field = Some(field);
        Ok(())
    }
    pub(in crate::llm::text) fn clear_field(&mut self) {
        self.field = None;
    }
    pub(in crate::llm::text) fn poison(&mut self) {
        self.failed = true;
    }
    pub(in crate::llm::text) fn messages_started(&self) -> bool {
        self.messages_open || self.messages_closed
    }
    pub(in crate::llm::text) fn complete_component(
        &mut self,
        result: &Result<EncoderStatus, RequestEncodeError>,
    ) {
        match result {
            Ok(EncoderStatus::ComponentFinished { .. }) => self.field = None,
            Err(_) => self.failed = true,
            _ => (),
        }
    }
    pub(in crate::llm::text) fn start_finalizing(&mut self) {
        self.finalizing = true;
    }
    pub(in crate::llm::text) fn prepare_output(
        &mut self,
        output: &mut [u8],
    ) -> Result<EncoderStatus, RequestEncodeError> {
        let mut written = 0;
        if !self.started {
            let mut opening = JsonToken::ObjectStart(None);
            match self.json.next(Some(&mut opening), output)? {
                EncoderStatus::NeedOutput { written } => {
                    return Ok(EncoderStatus::NeedOutput { written });
                }
                EncoderStatus::ComponentFinished { written: count } => {
                    written = count;
                    self.started = true;
                }
                EncoderStatus::Finished { .. } => return Err(RequestEncodeError::InvalidState),
            }
        }
        if self.messages_open && self.field != Some(self.message_field) {
            let mut closing = JsonToken::ArrayEnd;
            match self.json.next(Some(&mut closing), &mut output[written..])? {
                EncoderStatus::NeedOutput { written: count } => {
                    return Ok(EncoderStatus::NeedOutput {
                        written: written + count,
                    });
                }
                EncoderStatus::ComponentFinished { written: count } => {
                    written += count;
                    self.messages_open = false;
                    self.messages_closed = true;
                }
                EncoderStatus::Finished { .. } => return Err(RequestEncodeError::InvalidState),
            }
        }
        Ok(EncoderStatus::ComponentFinished { written })
    }

    pub(in crate::llm::text) fn encode_token(
        &mut self,
        token: &mut JsonToken<'_>,
        output: &mut [u8],
    ) -> Result<EncoderStatus, RequestEncodeError> {
        let written = match self.prepare_output(output)? {
            EncoderStatus::ComponentFinished { written } => written,
            status => return Ok(status),
        };
        match self.json.next(Some(token), &mut output[written..])? {
            EncoderStatus::NeedOutput { written: count } => Ok(EncoderStatus::NeedOutput {
                written: written + count,
            }),
            EncoderStatus::ComponentFinished { written: count } => {
                Ok(EncoderStatus::ComponentFinished {
                    written: written + count,
                })
            }
            EncoderStatus::Finished { .. } => Err(RequestEncodeError::InvalidState),
        }
    }

    pub(in crate::llm::text) fn encode_value<T: serde::Serialize + ?Sized>(
        &mut self,
        key: Option<&str>,
        value: &T,
        output: &mut [u8],
    ) -> Result<EncoderStatus, RequestEncodeError> {
        let written = match self.prepare_output(output)? {
            EncoderStatus::ComponentFinished { written } => written,
            status => return Ok(status),
        };
        match self
            .json
            .serialize_next(key, value, &mut output[written..])?
        {
            EncoderStatus::NeedOutput { written: count } => Ok(EncoderStatus::NeedOutput {
                written: written + count,
            }),
            EncoderStatus::ComponentFinished { written: count } => {
                Ok(EncoderStatus::ComponentFinished {
                    written: written + count,
                })
            }
            EncoderStatus::Finished { .. } => Err(RequestEncodeError::InvalidState),
        }
    }

    pub(in crate::llm::text) fn prepare_messages(
        &mut self,
        output: &mut [u8],
    ) -> Result<EncoderStatus, RequestEncodeError> {
        if self.messages_closed {
            return Err(RequestEncodeError::InvalidState);
        }
        if self.messages_open {
            return Ok(EncoderStatus::ComponentFinished { written: 0 });
        }
        let mut token = JsonToken::ArrayStart(Some(self.message_key));
        let status = self.encode_token(&mut token, output)?;
        if matches!(status, EncoderStatus::ComponentFinished { .. }) {
            self.messages_open = true;
        }
        Ok(status)
    }
    pub(in crate::llm::text) fn close(
        &mut self,
        output: &mut [u8],
    ) -> Result<EncoderStatus, RequestEncodeError> {
        let mut written = match self.prepare_output(output)? {
            EncoderStatus::ComponentFinished { written } => written,
            status => return Ok(status),
        };
        if !self.closed {
            let mut closing = JsonToken::ObjectEnd;
            match self.json.next(Some(&mut closing), &mut output[written..])? {
                EncoderStatus::NeedOutput { written: count } => {
                    return Ok(EncoderStatus::NeedOutput {
                        written: written + count,
                    });
                }
                EncoderStatus::ComponentFinished { written: count } => {
                    written += count;
                    self.closed = true;
                }
                EncoderStatus::Finished { .. } => return Err(RequestEncodeError::InvalidState),
            }
        }
        match self.json.next(None, &mut output[written..])? {
            EncoderStatus::Finished { written: count } => {
                self.finished = true;
                Ok(EncoderStatus::Finished {
                    written: written + count,
                })
            }
            _ => Err(RequestEncodeError::InvalidState),
        }
    }

    pub(in crate::llm::text) fn finish(self) -> Result<(), RequestEncodeError> {
        if self.failed {
            return Err(RequestEncodeError::InvalidState);
        }
        if !self.finished {
            return Err(RequestEncodeError::Incomplete);
        }
        self.json.finish().map_err(Into::into)
    }
}
