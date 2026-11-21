use super::component::Request;
use crate::encoder::{Encoder, EncoderStatus};
use crate::json::{JsonEncodeError, JsonEncoder, JsonToken};
use crate::llm::text::component::TextRequestComponent;

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
pub struct CompatibleOpenAIRequestEncoder {
    json: JsonEncoder,
    started: bool,
    messages_open: bool,
    messages_closed: bool,
    has_model: bool,
    has_messages: bool,
    field: Option<Field>,
    finalizing: bool,
    closed: bool,
    finished: bool,
    failed: bool,
}

impl CompatibleOpenAIRequestEncoder {
    pub fn new(json: JsonEncoder) -> Self {
        Self {
            json,
            started: false,
            messages_open: false,
            messages_closed: false,
            has_model: false,
            has_messages: false,
            field: None,
            finalizing: false,
            closed: false,
            finished: false,
            failed: false,
        }
    }

    fn encode(
        &mut self,
        component: Option<&mut TextRequestComponent>,
        output: &mut [u8],
    ) -> Result<EncoderStatus, RequestEncodeError> {
        if self.failed || (self.finalizing && component.is_some()) {
            return Err(RequestEncodeError::InvalidState);
        }
        if self.finished {
            return Ok(EncoderStatus::Finished { written: 0 });
        }
        if component.is_none() {
            if self.field.is_some() || !self.has_model || !self.has_messages {
                return Err(RequestEncodeError::Incomplete);
            }
            self.finalizing = true;
        } else if self.field.is_none() {
            let component = component
                .as_deref()
                .ok_or(RequestEncodeError::InvalidState)?;
            match component {
                TextRequestComponent::Message(_) if self.messages_closed => {
                    return Err(RequestEncodeError::InvalidState);
                }
                TextRequestComponent::Model(value) => {
                    if self.has_model || value.is_empty() {
                        return Err(RequestEncodeError::InvalidState);
                    }
                    self.has_model = true;
                }
                TextRequestComponent::MaxCompletionTokens(0) => {
                    return Err(RequestEncodeError::InvalidState);
                }
                TextRequestComponent::TopP(value)
                    if !value.is_finite() || !(0.0..=1.0).contains(value) =>
                {
                    return Err(RequestEncodeError::InvalidState);
                }
                _ => {}
            }
            self.field = Some(Field::from(component));
        }
        if let Some(component) = component.as_deref()
            && self.field != Some(Field::from(component))
        {
            return Err(RequestEncodeError::InvalidState);
        }
        let message = self.field == Some(Field::Messages);
        let mut written = 0;
        loop {
            let mut token = if !self.started {
                JsonToken::ObjectStart(None)
            } else if self.messages_open && !message {
                JsonToken::ArrayEnd
            } else if message && !self.messages_open {
                JsonToken::ArrayStart(Some("messages"))
            } else if let Some(component) = component.as_deref() {
                let key = self.field.ok_or(RequestEncodeError::InvalidState)?.key();
                let status =
                    self.json
                        .serialize_next(key, &Request(component), &mut output[written..])?;
                return match status {
                    EncoderStatus::ComponentFinished { written: count } => {
                        self.field = None;
                        Ok(EncoderStatus::ComponentFinished {
                            written: written + count,
                        })
                    }
                    EncoderStatus::NeedOutput { written: count } => Ok(EncoderStatus::NeedOutput {
                        written: written + count,
                    }),
                    _ => Err(RequestEncodeError::InvalidState),
                };
            } else if !self.closed {
                JsonToken::ObjectEnd
            } else {
                return match self.json.next(None, &mut output[written..])? {
                    EncoderStatus::Finished { written: count } => {
                        self.finished = true;
                        Ok(EncoderStatus::Finished {
                            written: written + count,
                        })
                    }
                    _ => Err(RequestEncodeError::InvalidState),
                };
            };
            match self.json.next(Some(&mut token), &mut output[written..])? {
                EncoderStatus::NeedOutput { written: count } => {
                    return Ok(EncoderStatus::NeedOutput {
                        written: written + count,
                    });
                }
                EncoderStatus::ComponentFinished { written: count } => written += count,
                EncoderStatus::Finished { .. } => return Err(RequestEncodeError::InvalidState),
            }
            if !self.started {
                self.started = true;
            } else if self.messages_open && !message {
                self.messages_open = false;
                self.messages_closed = true;
            } else if message && !self.messages_open {
                self.messages_open = true;
                self.has_messages = true;
            } else {
                self.closed = true;
            }
        }
    }
}

impl Encoder<TextRequestComponent> for CompatibleOpenAIRequestEncoder {
    type Error = RequestEncodeError;

    fn next(
        &mut self,
        component: Option<&mut TextRequestComponent>,
        output: &mut [u8],
    ) -> Result<EncoderStatus, Self::Error> {
        let result = self.encode(component, output);
        if result.is_err() {
            self.failed = true;
        }
        result
    }

    fn finish(self) -> Result<(), Self::Error> {
        if self.failed {
            return Err(RequestEncodeError::InvalidState);
        }
        if !self.finished {
            return Err(RequestEncodeError::Incomplete);
        }
        self.json.finish().map_err(Into::into)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Field {
    Model,
    Messages,
    Tools,
    ToolChoice,
    ParallelToolCalls,
    MaxCompletionTokens,
    TopP,
    TopK,
    Stop,
    ResponseFormat,
    Provider,
    Stream,
    StreamOptions,
}

impl From<&TextRequestComponent> for Field {
    fn from(component: &TextRequestComponent) -> Self {
        use TextRequestComponent as C;
        match component {
            C::Model(_) => Self::Model,
            C::Message(_) => Self::Messages,
            C::Tools(_) => Self::Tools,
            C::ToolChoice(_) => Self::ToolChoice,
            C::ParallelToolCalls(_) => Self::ParallelToolCalls,
            C::MaxCompletionTokens(_) => Self::MaxCompletionTokens,
            C::TopP(_) => Self::TopP,
            C::TopK(_) => Self::TopK,
            C::Stop(_) => Self::Stop,
            C::ResponseFormat(_) => Self::ResponseFormat,
            C::Provider(_) => Self::Provider,
            C::Stream(_) => Self::Stream,
            C::StreamOptions(_) => Self::StreamOptions,
        }
    }
}

impl Field {
    fn key(self) -> Option<&'static str> {
        Some(match self {
            Self::Messages => return None,
            Self::Model => "model",
            Self::Tools => "tools",
            Self::ToolChoice => "tool_choice",
            Self::ParallelToolCalls => "parallel_tool_calls",
            Self::MaxCompletionTokens => "max_completion_tokens",
            Self::TopP => "top_p",
            Self::TopK => "top_k",
            Self::Stop => "stop",
            Self::ResponseFormat => "response_format",
            Self::Provider => "provider",
            Self::Stream => "stream",
            Self::StreamOptions => "stream_options",
        })
    }
}
