use super::component::{
    FormatOutput, ReasoningOutput, TextOutput, ToolChoiceOutput, ToolOutput, effort, input_item,
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
    #[error("model and input are required")]
    Incomplete,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct OpenAIResponseRequestEncoderConfiguration {
    pub reasoning_effort: Option<ReasoningEffort>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Field {
    Model,
    Input,
    Tools,
    ToolChoice,
    ParallelToolCalls,
    MaxTokens,
    Reasoning,
    TopP,
    ResponseFormat,
    Stream,
}

#[derive(Debug)]
pub struct OpenAIResponseRequestEncoder {
    json: JsonEncoder,
    started: bool,
    tools_open: bool,
    input_open: bool,
    input_closed: bool,
    has_model: bool,
    has_input: bool,
    active: Option<Field>,
    item: usize,
    finalizing: bool,
    phase: u8,
    failed: bool,
    reasoning_effort: Option<ReasoningEffort>,
    effort: Option<&'static str>,
}

impl OpenAIResponseRequestEncoder {
    pub fn new(
        json: JsonEncoder,
        configuration: OpenAIResponseRequestEncoderConfiguration,
    ) -> Self {
        Self {
            json,
            started: false,
            tools_open: false,
            input_open: false,
            input_closed: false,
            has_model: false,
            has_input: false,
            active: None,
            item: 0,
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
            if self.active.is_some() || !self.has_model || !self.has_input {
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
                C::Message(_) => {
                    if self.input_closed {
                        return Err(RequestEncodeError::InvalidState);
                    }
                    Field::Input
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
                C::ResponseFormat(_) => Field::ResponseFormat,
                C::Stream(_) => Field::Stream,
                C::Stop(_) | C::TopK(_) | C::Provider(_) | C::StreamOptions(_) => {
                    return Ok(EncoderStatus::ComponentFinished { written: 0 });
                }
            };
            self.active = Some(field);
        }
        let message = self.active == Some(Field::Input);
        let mut written = 0;
        loop {
            let mut token = if !self.started {
                JsonToken::ObjectStart(None)
            } else if self.input_open && !message {
                JsonToken::ArrayEnd
            } else if message && !self.input_open {
                JsonToken::ArrayStart(Some("input"))
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
                    (Some(Field::Input), C::Message(value)) => {
                        let item = input_item(value, self.item)
                            .map_err(|_| RequestEncodeError::InvalidState)?
                            .ok_or(RequestEncodeError::InvalidState)?;
                        self.json.serialize_next(None, &item, output)
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
                            .serialize_next(Some("max_output_tokens"), value, output)
                    }
                    (Some(Field::Reasoning), C::Reasoning(_)) => self.json.serialize_next(
                        Some("reasoning"),
                        &ReasoningOutput {
                            effort: self.effort,
                        },
                        output,
                    ),
                    (Some(Field::TopP), C::TopP(value)) => {
                        self.json.serialize_next(Some("top_p"), value, output)
                    }
                    (Some(Field::ResponseFormat), C::ResponseFormat(value)) => {
                        self.json.serialize_next(
                            Some("text"),
                            &TextOutput {
                                format: FormatOutput::from(value),
                            },
                            output,
                        )
                    }
                    (Some(Field::Stream), C::Stream(value)) => {
                        self.json.serialize_next(Some("stream"), value, output)
                    }
                    _ => return Err(RequestEncodeError::InvalidState),
                }?;
                match status {
                    EncoderStatus::NeedOutput { written: n } => {
                        return Ok(EncoderStatus::NeedOutput {
                            written: written + n,
                        });
                    }
                    EncoderStatus::ComponentFinished { written: n } => {
                        written += n;
                        if let (Some(Field::Input), C::Message(value)) = (self.active, component) {
                            self.item += 1;
                            if input_item(value, self.item)
                                .map_err(|_| RequestEncodeError::InvalidState)?
                                .is_some()
                            {
                                continue;
                            }
                            self.item = 0;
                        }
                        self.active = None;
                        return Ok(EncoderStatus::ComponentFinished { written });
                    }
                    _ => return Err(RequestEncodeError::InvalidState),
                }
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
            } else if self.input_open && !message {
                self.input_open = false;
                self.input_closed = true;
            } else if message && !self.input_open {
                self.input_open = true;
                self.has_input = true;
            } else {
                self.phase = 1;
            }
        }
    }
}

impl Encoder<TextRequestComponent> for OpenAIResponseRequestEncoder {
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
