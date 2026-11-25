use super::component::{
    MessageOutput, ResponseFormatOutput, StopOutput, StreamOptionsOutput, ToolChoiceOutput,
    ToolOutput, effort,
};
use crate::encoder::{Encoder, EncoderStatus};
use crate::json::{JsonEncodeError, JsonEncoder, JsonToken};
use crate::llm::text::component::*;

#[derive(Debug, thiserror::Error)]
pub enum RequestEncodeError {
    #[error(transparent)]
    Json(#[from] JsonEncodeError),
    #[error("invalid request encoder state")]
    InvalidState,
    #[error("model and messages are required")]
    Incomplete,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct OpenAICompletionRequestEncoderConfiguration {
    pub reasoning_effort: Option<ReasoningEffort>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Field {
    Model,
    Messages,
    Tools,
    ToolChoice,
    ParallelToolCalls,
    MaxTokens,
    Reasoning,
    TopP,
    Stop,
    ResponseFormat,
    Stream,
    StreamOptions,
}

#[derive(Debug)]
pub struct OpenAICompletionRequestEncoder {
    json: JsonEncoder,
    started: bool,
    tools_open: bool,
    messages_open: bool,
    messages_closed: bool,
    has_model: bool,
    has_messages: bool,
    active: Option<Field>,
    finalizing: bool,
    phase: u8,
    failed: bool,
    reasoning_effort: Option<ReasoningEffort>,
    effort: Option<&'static str>,
}

impl OpenAICompletionRequestEncoder {
    pub fn new(
        json: JsonEncoder,
        configuration: OpenAICompletionRequestEncoderConfiguration,
    ) -> Self {
        Self {
            json,
            started: false,
            tools_open: false,
            messages_open: false,
            messages_closed: false,
            has_model: false,
            has_messages: false,
            active: None,
            finalizing: false,
            phase: 0,
            failed: false,
            reasoning_effort: configuration.reasoning_effort,
            effort: None,
        }
    }

    fn encode(
        &mut self,
        component: Option<&mut TextRequestComponent>,
        output: &mut [u8],
    ) -> Result<EncoderStatus, RequestEncodeError> {
        use TextRequestComponent as C;
        if self.failed || (self.finalizing && component.is_some()) {
            return Err(RequestEncodeError::InvalidState);
        }
        let component = component.as_deref();
        if self.tools_open && !matches!(component, Some(C::Tool(_) | C::ToolsEnded)) {
            return Err(RequestEncodeError::InvalidState);
        }
        if !self.tools_open && matches!(component, Some(C::Tool(_) | C::ToolsEnded)) {
            return Err(RequestEncodeError::InvalidState);
        }
        if self.phase == 2 {
            return Ok(EncoderStatus::Finished { written: 0 });
        }
        if component.is_none() {
            if self.active.is_some() || !self.has_model || !self.has_messages {
                return Err(RequestEncodeError::Incomplete);
            }
            self.finalizing = true;
        }
        if self.active.is_none()
            && let Some(component) = component
        {
            let field = match component {
                C::Model(model) => {
                    if self.has_model || model.is_empty() {
                        return Err(RequestEncodeError::InvalidState);
                    }
                    self.has_model = true;
                    Field::Model
                }
                C::Message(message) => {
                    if self.messages_closed {
                        return Err(RequestEncodeError::InvalidState);
                    }
                    if message.role == Role::Tool && message.tool_call_id.is_none() {
                        return Err(RequestEncodeError::InvalidState);
                    }
                    Field::Messages
                }
                C::ToolsStarted | C::Tool(_) | C::ToolsEnded => Field::Tools,
                C::ToolChoice(_) => Field::ToolChoice,
                C::ParallelToolCalls(_) => Field::ParallelToolCalls,
                C::MaxTokens(limit) => {
                    if *limit == 0 {
                        return Err(RequestEncodeError::InvalidState);
                    }
                    Field::MaxTokens
                }
                C::Reasoning(reasoning) => {
                    let effort = self.reasoning_effort.or(reasoning.effort).and_then(effort);
                    match effort {
                        Some(value) => {
                            self.effort = Some(value);
                            Field::Reasoning
                        }
                        None => return Ok(EncoderStatus::ComponentFinished { written: 0 }),
                    }
                }
                C::TopP(value) => {
                    if !value.is_finite() || !(0.0..=1.0).contains(value) {
                        return Err(RequestEncodeError::InvalidState);
                    }
                    Field::TopP
                }
                C::Stop(_) => Field::Stop,
                C::ResponseFormat(_) => Field::ResponseFormat,
                C::Stream(_) => Field::Stream,
                C::StreamOptions(options) => match options {
                    Some(_) => Field::StreamOptions,
                    None => return Ok(EncoderStatus::ComponentFinished { written: 0 }),
                },
                C::TopK(_) | C::Provider(_) => {
                    return Ok(EncoderStatus::ComponentFinished { written: 0 });
                }
            };
            self.active = Some(field);
        }
        let message = self.active == Some(Field::Messages);
        let mut written = 0;
        loop {
            let mut token = if !self.started {
                JsonToken::ObjectStart(None)
            } else if self.messages_open && !message {
                JsonToken::ArrayEnd
            } else if message && !self.messages_open {
                JsonToken::ArrayStart(Some("messages"))
            } else if matches!(component, Some(C::ToolsStarted | C::ToolsEnded)) {
                let opening = matches!(component, Some(C::ToolsStarted));
                let mut token = if opening {
                    JsonToken::ArrayStart(Some("tools"))
                } else {
                    JsonToken::ArrayEnd
                };
                let status = self.json.next(Some(&mut token), &mut output[written..])?;
                return match status {
                    EncoderStatus::NeedOutput { written: n } => Ok(EncoderStatus::NeedOutput {
                        written: written + n,
                    }),
                    EncoderStatus::ComponentFinished { written: n } => {
                        self.tools_open = opening;
                        self.active = None;
                        Ok(EncoderStatus::ComponentFinished {
                            written: written + n,
                        })
                    }
                    _ => Err(RequestEncodeError::InvalidState),
                };
            } else if let Some(component) = component {
                let output = &mut output[written..];
                let status = match (self.active, component) {
                    (Some(Field::Model), C::Model(value)) => {
                        self.json.serialize_next(Some("model"), value, output)
                    }
                    (Some(Field::Messages), C::Message(value)) => {
                        self.json
                            .serialize_next(None, &MessageOutput::from(value), output)
                    }
                    (Some(Field::Tools), C::Tool(value)) => {
                        self.json
                            .serialize_next(None, &ToolOutput::from(value), output)
                    }
                    (Some(Field::ToolChoice), C::ToolChoice(value)) => self.json.serialize_next(
                        Some("tool_choice"),
                        &ToolChoiceOutput::from(value),
                        output,
                    ),
                    (Some(Field::ParallelToolCalls), C::ParallelToolCalls(value)) => self
                        .json
                        .serialize_next(Some("parallel_tool_calls"), value, output),
                    (Some(Field::MaxTokens), C::MaxTokens(value)) => {
                        self.json
                            .serialize_next(Some("max_completion_tokens"), value, output)
                    }
                    (Some(Field::Reasoning), C::Reasoning(_)) => self.json.serialize_next(
                        Some("reasoning_effort"),
                        &self.effort.ok_or(RequestEncodeError::InvalidState)?,
                        output,
                    ),
                    (Some(Field::TopP), C::TopP(value)) => {
                        self.json.serialize_next(Some("top_p"), value, output)
                    }
                    (Some(Field::Stop), C::Stop(value)) => {
                        self.json
                            .serialize_next(Some("stop"), &StopOutput::from(value), output)
                    }
                    (Some(Field::ResponseFormat), C::ResponseFormat(value)) => {
                        self.json.serialize_next(
                            Some("response_format"),
                            &ResponseFormatOutput::from(value),
                            output,
                        )
                    }
                    (Some(Field::Stream), C::Stream(value)) => {
                        self.json.serialize_next(Some("stream"), value, output)
                    }
                    (Some(Field::StreamOptions), C::StreamOptions(Some(value))) => {
                        self.json.serialize_next(
                            Some("stream_options"),
                            &StreamOptionsOutput::from(value),
                            output,
                        )
                    }
                    _ => return Err(RequestEncodeError::InvalidState),
                }?;
                return match status {
                    EncoderStatus::NeedOutput { written: n } => Ok(EncoderStatus::NeedOutput {
                        written: written + n,
                    }),
                    EncoderStatus::ComponentFinished { written: n } => {
                        self.active = None;
                        Ok(EncoderStatus::ComponentFinished {
                            written: written + n,
                        })
                    }
                    _ => Err(RequestEncodeError::InvalidState),
                };
            } else if self.phase == 0 {
                JsonToken::ObjectEnd
            } else {
                return match self.json.next(None, &mut output[written..])? {
                    EncoderStatus::Finished { written: n } => {
                        self.phase = 2;
                        Ok(EncoderStatus::Finished {
                            written: written + n,
                        })
                    }
                    _ => Err(RequestEncodeError::InvalidState),
                };
            };
            match self.json.next(Some(&mut token), &mut output[written..])? {
                EncoderStatus::NeedOutput { written: n } => {
                    return Ok(EncoderStatus::NeedOutput {
                        written: written + n,
                    });
                }
                EncoderStatus::ComponentFinished { written: n } => written += n,
                _ => return Err(RequestEncodeError::InvalidState),
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
                self.phase = 1;
            }
        }
    }
}

impl Encoder<TextRequestComponent> for OpenAICompletionRequestEncoder {
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
        if self.phase != 2 {
            return Err(RequestEncodeError::Incomplete);
        }
        self.json.finish().map_err(Into::into)
    }
}
