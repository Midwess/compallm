use super::component::*;
use crate::encoder::{Encoder, EncoderStatus};
use crate::json::{JsonEncodeError, JsonEncoder, JsonToken};
use crate::llm::text::component::*;

#[derive(Debug, thiserror::Error)]
pub enum RequestEncodeError {
    #[error(transparent)]
    Json(#[from] JsonEncodeError),
    #[error("invalid request encoder state")]
    InvalidState,
    #[error("model and conversation messages are required")]
    Incomplete,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AnthropicRequestEncoderConfiguration {
    pub reasoning_effort: Option<ReasoningEffort>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Field {
    Model,
    System,
    Messages,
    Tools,
    ToolChoice,
    Thinking,
    TopP,
    TopK,
    StopSequences,
    Stream,
    OutputConfig,
}

#[derive(Debug)]
pub struct AnthropicRequestEncoder {
    json: JsonEncoder,
    started: bool,
    tools_open: bool,
    first_message: bool,
    system_written: bool,
    messages_open: bool,
    messages_closed: bool,
    has_messages: bool,
    has_model: bool,
    max_tokens: u32,
    effort: Option<&'static str>,
    effort_written: bool,
    active: Option<Field>,
    finalizing: bool,
    phase: u8,
    failed: bool,
}

impl AnthropicRequestEncoder {
    pub fn new(json: JsonEncoder, configuration: AnthropicRequestEncoderConfiguration) -> Self {
        Self {
            json,
            started: false,
            tools_open: false,
            first_message: true,
            system_written: false,
            messages_open: false,
            messages_closed: false,
            has_messages: false,
            has_model: false,
            max_tokens: 32768,
            effort: configuration.reasoning_effort.and_then(effort),
            effort_written: false,
            active: None,
            finalizing: false,
            phase: 0,
            failed: false,
        }
    }

    pub fn finish(self) -> Result<(), RequestEncodeError> {
        <Self as Encoder<TextRequestComponent>>::finish(self)
    }

    fn encode<S: serde::Serialize + AsRef<str>>(
        &mut self,
        component: Option<&mut TextRequestComponent<S>>,
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
        if self.phase == 5 {
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
                    if self.has_model || model.as_ref().is_empty() {
                        return Err(RequestEncodeError::InvalidState);
                    }
                    self.has_model = true;
                    Field::Model
                }
                C::Message(message) => {
                    let leading_system = self.first_message && message.role == Role::System;
                    self.first_message = false;
                    if message.role == Role::Developer {
                        return Ok(EncoderStatus::ComponentFinished { written: 0 });
                    }
                    if leading_system {
                        Field::System
                    } else {
                        if self.messages_closed {
                            return Err(RequestEncodeError::InvalidState);
                        }
                        if message.role == Role::Tool && message.tool_call_id.is_none() {
                            return Err(RequestEncodeError::InvalidState);
                        }
                        Field::Messages
                    }
                }
                C::ToolsStarted | C::Tool(_) | C::ToolsEnded => Field::Tools,
                C::ToolChoice(_) => Field::ToolChoice,
                C::MaxTokens(limit) => {
                    if *limit == 0 {
                        return Err(RequestEncodeError::InvalidState);
                    }
                    self.max_tokens = *limit;
                    return Ok(EncoderStatus::ComponentFinished { written: 0 });
                }
                C::Reasoning(reasoning) => {
                    if reasoning.enabled.is_none() && reasoning.max_tokens.is_none() {
                        return Ok(EncoderStatus::ComponentFinished { written: 0 });
                    }
                    Field::Thinking
                }
                C::TopP(value) => {
                    if !value.is_finite() || !(0.0..=1.0).contains(value) {
                        return Err(RequestEncodeError::InvalidState);
                    }
                    Field::TopP
                }
                C::TopK(_) => Field::TopK,
                C::Stop(Stop::Text(_) | Stop::Array(_)) => Field::StopSequences,
                C::Stream(_) => Field::Stream,
                C::ResponseFormat(ResponseFormat::JsonSchema(schema)) => {
                    if self.effort_written {
                        return Err(RequestEncodeError::InvalidState);
                    }
                    let _ = schema;
                    Field::OutputConfig
                }
                _ => return Ok(EncoderStatus::ComponentFinished { written: 0 }),
            };
            self.active = Some(field);
        }
        let message = self.active == Some(Field::Messages);
        let mut written = 0;
        loop {
            let mut token = if !self.started {
                JsonToken::ObjectStart(None)
            } else if message && !self.system_written {
                let status =
                    self.json
                        .serialize_next(Some("system"), "", &mut output[written..])?;
                match status {
                    EncoderStatus::NeedOutput { written: n } => {
                        return Ok(EncoderStatus::NeedOutput {
                            written: written + n,
                        });
                    }
                    EncoderStatus::ComponentFinished { written: n } => {
                        written += n;
                        self.system_written = true;
                        continue;
                    }
                    _ => return Err(RequestEncodeError::InvalidState),
                }
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
                    (Some(Field::System), C::Message(value)) => {
                        self.json
                            .serialize_next(Some("system"), &system(value), output)
                    }
                    (Some(Field::Messages), C::Message(value)) => {
                        self.json
                            .serialize_next(None, &MessageWire::from(value), output)
                    }
                    (Some(Field::Tools), C::Tool(value)) => {
                        self.json
                            .serialize_next(None, &ToolWire::from(value), output)
                    }
                    (Some(Field::ToolChoice), C::ToolChoice(value)) => {
                        self.json
                            .serialize_next(Some("tool_choice"), &Choice::from(value), output)
                    }
                    (Some(Field::Thinking), C::Reasoning(value)) => {
                        let mut value = *value;
                        if self.effort.is_some() {
                            value.max_tokens = None;
                        }
                        self.json
                            .serialize_next(Some("thinking"), &Thinking::from(&value), output)
                    }
                    (Some(Field::TopP), C::TopP(value)) => {
                        self.json.serialize_next(Some("top_p"), value, output)
                    }
                    (Some(Field::TopK), C::TopK(value)) => {
                        self.json.serialize_next(Some("top_k"), value, output)
                    }
                    (Some(Field::StopSequences), C::Stop(Stop::Text(value))) => self
                        .json
                        .serialize_next(Some("stop_sequences"), &[value], output),
                    (Some(Field::StopSequences), C::Stop(Stop::Array(value))) => self
                        .json
                        .serialize_next(Some("stop_sequences"), value, output),
                    (Some(Field::Stream), C::Stream(value)) => {
                        self.json.serialize_next(Some("stream"), value, output)
                    }
                    (
                        Some(Field::OutputConfig),
                        C::ResponseFormat(ResponseFormat::JsonSchema(schema)),
                    ) => self.json.serialize_next(
                        Some("output_config"),
                        &OutputConfig {
                            effort: self.effort,
                            format: Some(Format {
                                r#type: "json_schema",
                                schema: schema.schema.as_ref(),
                            }),
                        },
                        output,
                    ),
                    _ => return Err(RequestEncodeError::InvalidState),
                }?;
                return match status {
                    EncoderStatus::NeedOutput { written: n } => Ok(EncoderStatus::NeedOutput {
                        written: written + n,
                    }),
                    EncoderStatus::ComponentFinished { written: n } => {
                        if self.active == Some(Field::System) {
                            self.system_written = true;
                        }
                        if self.active == Some(Field::OutputConfig) {
                            self.effort_written = true;
                        }
                        self.active = None;
                        Ok(EncoderStatus::ComponentFinished {
                            written: written + n,
                        })
                    }
                    _ => Err(RequestEncodeError::InvalidState),
                };
            } else if self.phase < 2 {
                let status = if self.phase == 0 {
                    self.json.serialize_next(
                        Some("max_tokens"),
                        &self.max_tokens,
                        &mut output[written..],
                    )?
                } else if self.effort.is_some() && !self.effort_written {
                    self.json.serialize_next(
                        Some("output_config"),
                        &OutputConfig {
                            effort: self.effort,
                            format: None,
                        },
                        &mut output[written..],
                    )?
                } else {
                    self.phase = 2;
                    continue;
                };
                match status {
                    EncoderStatus::NeedOutput { written: n } => {
                        return Ok(EncoderStatus::NeedOutput {
                            written: written + n,
                        });
                    }
                    EncoderStatus::ComponentFinished { written: n } => {
                        written += n;
                        self.phase += 1;
                        continue;
                    }
                    _ => return Err(RequestEncodeError::InvalidState),
                }
            } else if self.phase == 2 {
                JsonToken::ObjectEnd
            } else {
                return match self.json.next(None, &mut output[written..])? {
                    EncoderStatus::Finished { written: n } => {
                        self.phase = 5;
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
                self.phase = 3;
            }
        }
    }
}

impl<S: serde::Serialize + AsRef<str>> Encoder<TextRequestComponent<S>>
    for AnthropicRequestEncoder
{
    type Error = RequestEncodeError;
    fn next(
        &mut self,
        component: Option<&mut TextRequestComponent<S>>,
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
        if self.phase != 5 {
            return Err(RequestEncodeError::Incomplete);
        }
        self.json.finish().map_err(Into::into)
    }
}
