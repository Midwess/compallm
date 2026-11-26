use super::component::{ConfiguredReasoning, ReasoningWire, Request};
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

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CompatibleOpenAIRequestEncoderConfiguration {
    pub reasoning_effort: Option<crate::llm::text::component::ReasoningEffort>,
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
    reasoning: Option<crate::llm::text::component::Reasoning>,
    reasoning_effort: Option<crate::llm::text::component::ReasoningEffort>,
    reasoning_written: bool,
}

impl CompatibleOpenAIRequestEncoder {
    pub fn new(
        json: JsonEncoder,
        configuration: CompatibleOpenAIRequestEncoderConfiguration,
    ) -> Self {
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
            reasoning: None,
            reasoning_effort: configuration.reasoning_effort,
            reasoning_written: false,
        }
    }

    fn prepare_output(&mut self, output: &mut [u8]) -> Result<EncoderStatus, RequestEncodeError> {
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
        if self.messages_open && self.field != Some(Field::Messages) {
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

    fn encode_token(
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

    fn encode_message<S: serde::Serialize + AsRef<str>>(
        &mut self,
        component: &TextRequestComponent<S>,
        output: &mut [u8],
    ) -> Result<EncoderStatus, RequestEncodeError> {
        if self.messages_closed {
            return Err(RequestEncodeError::InvalidState);
        }
        let mut written = 0;
        if !self.messages_open {
            let mut opening = JsonToken::ArrayStart(Some("messages"));
            match self.encode_token(&mut opening, output)? {
                EncoderStatus::NeedOutput { written } => {
                    return Ok(EncoderStatus::NeedOutput { written });
                }
                EncoderStatus::ComponentFinished { written: count } => {
                    written = count;
                    self.messages_open = true;
                }
                EncoderStatus::Finished { .. } => return Err(RequestEncodeError::InvalidState),
            }
        }
        match self
            .json
            .serialize_next(None, &Request(component), &mut output[written..])?
        {
            EncoderStatus::NeedOutput { written: count } => Ok(EncoderStatus::NeedOutput {
                written: written + count,
            }),
            EncoderStatus::ComponentFinished { written: count } => {
                self.has_messages = true;
                Ok(EncoderStatus::ComponentFinished {
                    written: written + count,
                })
            }
            EncoderStatus::Finished { .. } => Err(RequestEncodeError::InvalidState),
        }
    }

    fn encode_value<T: serde::Serialize + ?Sized>(
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

    fn configured_reasoning(&self) -> ConfiguredReasoning {
        let reasoning = self.reasoning.unwrap_or_default();
        ConfiguredReasoning {
            enabled: reasoning.enabled,
            effort: self.reasoning_effort,
            max_tokens: self
                .reasoning_effort
                .is_none()
                .then_some(reasoning.max_tokens)
                .flatten(),
            exclude: reasoning.exclude,
        }
    }

    fn complete_component(
        &mut self,
        result: Result<EncoderStatus, RequestEncodeError>,
    ) -> Result<EncoderStatus, RequestEncodeError> {
        match &result {
            Ok(EncoderStatus::ComponentFinished { .. }) => {
                self.has_model |= self.field == Some(Field::Model);
                self.reasoning_written |= self.field == Some(Field::Reasoning);
                self.field = None;
            }
            Err(_) => self.failed = true,
            _ => {}
        }
        result
    }

    fn finalize(&mut self, output: &mut [u8]) -> Result<EncoderStatus, RequestEncodeError> {
        if self.field.is_some() || !self.has_model || !self.has_messages {
            return Err(RequestEncodeError::Incomplete);
        }
        self.finalizing = true;
        let mut written = match self.prepare_output(output)? {
            EncoderStatus::ComponentFinished { written } => written,
            status => return Ok(status),
        };
        if !self.reasoning_written && self.reasoning_effort.is_some() {
            let value = self.configured_reasoning();
            match self
                .json
                .serialize_next(Some("reasoning"), &value, &mut output[written..])?
            {
                EncoderStatus::NeedOutput { written: count } => {
                    return Ok(EncoderStatus::NeedOutput {
                        written: written + count,
                    });
                }
                EncoderStatus::ComponentFinished { written: count } => {
                    written += count;
                    self.reasoning_written = true;
                }
                EncoderStatus::Finished { .. } => return Err(RequestEncodeError::InvalidState),
            }
        }
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

    pub fn finish(self) -> Result<(), RequestEncodeError> {
        <Self as Encoder<TextRequestComponent>>::finish(self)
    }
}

impl<S: serde::Serialize + AsRef<str>> Encoder<TextRequestComponent<S>>
    for CompatibleOpenAIRequestEncoder
{
    type Error = RequestEncodeError;

    fn next(
        &mut self,
        component: Option<&mut TextRequestComponent<S>>,
        output: &mut [u8],
    ) -> Result<EncoderStatus, Self::Error> {
        use crate::llm::text::component::{Stop, ToolChoice};
        use TextRequestComponent as C;

        if self.failed || (self.finalizing && component.is_some()) {
            return Err(RequestEncodeError::InvalidState);
        }
        if self.finished {
            return Ok(EncoderStatus::Finished { written: 0 });
        }
        if let Some(component) = component.as_deref() {
            let field = Field::from(component);
            if self.field.is_some_and(|pending| pending != field) {
                self.failed = true;
                return Err(RequestEncodeError::InvalidState);
            }
            if self.field.is_none() {
                match component {
                    C::Model(value) if self.has_model || value.as_ref().is_empty() => {
                        self.failed = true;
                        return Err(RequestEncodeError::InvalidState);
                    }
                    C::MaxTokens(0) => {
                        self.failed = true;
                        return Err(RequestEncodeError::InvalidState);
                    }
                    C::TopP(value) if !value.is_finite() || !(0.0..=1.0).contains(value) => {
                        self.failed = true;
                        return Err(RequestEncodeError::InvalidState);
                    }
                    _ => {}
                }
            }
            self.field = Some(field);
        }

        let mut token = match component.as_deref() {
            Some(C::Model(value)) => JsonToken::ValueString(Some("model"), value.as_ref()),
            Some(component @ C::Message(_)) => {
                let result = self.encode_message(component, output);
                return self.complete_component(result);
            }
            Some(component @ C::Tools(_)) => {
                let result = self.encode_value(Some("tools"), &Request(component), output);
                return self.complete_component(result);
            }
            Some(component @ C::ToolChoice(choice)) => JsonToken::ValueString(
                Some("tool_choice"),
                match choice {
                    ToolChoice::Auto => "auto",
                    ToolChoice::None => "none",
                    ToolChoice::Required => "required",
                    ToolChoice::Function(_) => {
                        let result =
                            self.encode_value(Some("tool_choice"), &Request(component), output);
                        return self.complete_component(result);
                    }
                },
            ),
            Some(C::ParallelToolCalls(value)) => {
                JsonToken::ValueBoolean(Some("parallel_tool_calls"), *value)
            }
            Some(C::MaxTokens(value)) => {
                JsonToken::ValueNumber(Some("max_tokens"), (*value).into())
            }
            Some(C::Reasoning(value)) => {
                let mut value = *value;
                value.effort = None;
                if ReasoningWire::validate(&value).is_err() {
                    self.failed = true;
                    return Err(RequestEncodeError::InvalidState);
                }
                self.reasoning = Some(value);
                let value = self.configured_reasoning();
                let result = self.encode_value(Some("reasoning"), &value, output);
                return self.complete_component(result);
            }
            Some(C::TopP(value)) => JsonToken::ValueNumber(
                Some("top_p"),
                serde_json::Number::from_f64(*value).ok_or(RequestEncodeError::InvalidState)?,
            ),
            Some(C::TopK(value)) => JsonToken::ValueNumber(Some("top_k"), (*value).into()),
            Some(C::Stop(Stop::Text(value))) => {
                JsonToken::ValueString(Some("stop"), value.as_ref())
            }
            Some(C::Stop(Stop::Null)) => JsonToken::ValueNull(Some("stop")),
            Some(
                component @ (C::Stop(Stop::Array(_) | Stop::ArrayView(_))
                | C::ResponseFormat(_)
                | C::Provider(_)
                | C::StreamOptions(Some(_))),
            ) => {
                let key = self.field.ok_or(RequestEncodeError::InvalidState)?.key();
                let result = self.encode_value(key, &Request(component), output);
                return self.complete_component(result);
            }
            Some(C::Stream(value)) => JsonToken::ValueBoolean(Some("stream"), *value),
            Some(C::StreamOptions(None)) => JsonToken::ValueNull(Some("stream_options")),
            None => {
                let result = self.finalize(output);
                if result.is_err() {
                    self.failed = true;
                }
                return result;
            }
        };
        let result = self.encode_token(&mut token, output);
        self.complete_component(result)
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
    MaxTokens,
    Reasoning,
    TopP,
    TopK,
    Stop,
    ResponseFormat,
    Provider,
    Stream,
    StreamOptions,
}

impl<S> From<&TextRequestComponent<S>> for Field {
    fn from(component: &TextRequestComponent<S>) -> Self {
        use TextRequestComponent as C;
        match component {
            C::Model(_) => Self::Model,
            C::Message(_) => Self::Messages,
            C::Tools(_) => Self::Tools,
            C::ToolChoice(_) => Self::ToolChoice,
            C::ParallelToolCalls(_) => Self::ParallelToolCalls,
            C::MaxTokens(_) => Self::MaxTokens,
            C::Reasoning(_) => Self::Reasoning,
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
            Self::MaxTokens => "max_tokens",
            Self::Reasoning => "reasoning",
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
