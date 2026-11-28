use super::component::{
    ChoiceOutput, EffortWire, MessageOutput, ReasoningOutput, TextOutput, ToolsOutput,
};
use crate::encoder::{Encoder, EncoderStatus};
use crate::json::{JsonEncoder, JsonToken};
use crate::llm::text::component::TextRequestComponent;
pub use crate::llm::text::request::encoder::RequestEncodeError;
use crate::llm::text::request::encoder::TextRequestEncoder;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct OpenAIResponseRequestEncoderConfiguration {
    pub reasoning_effort: Option<crate::llm::text::component::ReasoningEffort>,
}

#[derive(Debug)]
pub struct OpenAIResponseRequestEncoder {
    request: TextRequestEncoder<Field>,
    message_index: usize,
    has_model: bool,
    has_messages: bool,
    reasoning_effort: Option<crate::llm::text::component::ReasoningEffort>,
    reasoning_written: bool,
}

impl OpenAIResponseRequestEncoder {
    pub fn new(
        json: JsonEncoder,
        configuration: OpenAIResponseRequestEncoderConfiguration,
    ) -> Self {
        Self {
            request: TextRequestEncoder::new(json, Field::Messages, "input"),
            message_index: 0,
            has_model: false,
            has_messages: false,
            reasoning_effort: configuration.reasoning_effort,
            reasoning_written: false,
        }
    }

    fn encode_message<S: serde::Serialize + AsRef<str>>(
        &mut self,
        component: &TextRequestComponent<S>,
        output: &mut [u8],
    ) -> Result<EncoderStatus, RequestEncodeError> {
        let mut written = match self.request.prepare_messages(output)? {
            EncoderStatus::ComponentFinished { written } => written,
            status => return Ok(status),
        };
        let TextRequestComponent::Message(message) = component else {
            return Err(RequestEncodeError::InvalidState);
        };
        let count = super::component::message::item_count(message);
        if count == 0 {
            return Err(RequestEncodeError::InvalidState);
        }
        loop {
            let value = MessageOutput::try_from((message, self.message_index))
                .map_err(|_| RequestEncodeError::InvalidState)?;
            match self
                .request
                .encode_value(None, &value, &mut output[written..])?
            {
                EncoderStatus::NeedOutput { written: count } => {
                    return Ok(EncoderStatus::NeedOutput {
                        written: written + count,
                    });
                }
                EncoderStatus::ComponentFinished { written: count } => {
                    written += count;
                    self.message_index += 1;
                }
                _ => return Err(RequestEncodeError::InvalidState),
            }
            if self.message_index == count {
                self.message_index = 0;
                self.has_messages = true;
                return Ok(EncoderStatus::ComponentFinished { written });
            }
        }
    }

    fn complete_component(
        &mut self,
        result: Result<EncoderStatus, RequestEncodeError>,
    ) -> Result<EncoderStatus, RequestEncodeError> {
        if matches!(&result, Ok(EncoderStatus::ComponentFinished { .. })) {
            self.has_model |= self.request.field() == Some(Field::Model);
            self.reasoning_written |= self.request.field() == Some(Field::Reasoning);
        }
        self.request.complete_component(&result);
        result
    }

    fn finalize(&mut self, output: &mut [u8]) -> Result<EncoderStatus, RequestEncodeError> {
        if self.request.field().is_some() || !self.has_model || !self.has_messages {
            return Err(RequestEncodeError::Incomplete);
        }
        self.request.start_finalizing();
        let mut written = match self.request.prepare_output(output)? {
            EncoderStatus::ComponentFinished { written } => written,
            status => return Ok(status),
        };
        if !self.reasoning_written
            && let Some(effort) = self.reasoning_effort
        {
            let name = EffortWire::name(effort).ok_or_else(|| {
                self.request.poison();
                RequestEncodeError::InvalidState
            })?;
            let value = ReasoningOutput { effort: name };
            match self
                .request
                .encode_value(Some("reasoning"), &value, &mut output[written..])?
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
                _ => return Err(RequestEncodeError::InvalidState),
            }
        }
        match self.request.close(&mut output[written..])? {
            EncoderStatus::NeedOutput { written: count } => Ok(EncoderStatus::NeedOutput {
                written: written + count,
            }),
            EncoderStatus::Finished { written: count } => Ok(EncoderStatus::Finished {
                written: written + count,
            }),
            _ => Err(RequestEncodeError::InvalidState),
        }
    }

    pub fn finish(self) -> Result<(), RequestEncodeError> {
        <Self as Encoder<TextRequestComponent>>::finish(self)
    }
}

impl<S: serde::Serialize + AsRef<str>> Encoder<TextRequestComponent<S>>
    for OpenAIResponseRequestEncoder
{
    type Error = RequestEncodeError;

    fn next(
        &mut self,
        component: Option<&mut TextRequestComponent<S>>,
        output: &mut [u8],
    ) -> Result<EncoderStatus, Self::Error> {
        use crate::llm::text::component::ToolChoice;
        use TextRequestComponent as C;

        if self.request.check(component.is_some())? {
            return Ok(EncoderStatus::Finished { written: 0 });
        }
        if let Some(component) = component.as_deref() {
            let field = Field::from(component);
            if self.request.field().is_none() {
                match component {
                    C::Model(value) if self.has_model || value.as_ref().is_empty() => {
                        self.request.poison();
                        return Err(RequestEncodeError::InvalidState);
                    }
                    C::MaxTokens(0) => {
                        self.request.poison();
                        return Err(RequestEncodeError::InvalidState);
                    }
                    C::TopP(value) if !value.is_finite() || !(0.0..=1.0).contains(value) => {
                        self.request.poison();
                        return Err(RequestEncodeError::InvalidState);
                    }
                    _ => {}
                }
            }
            self.request.set_field(field)?;
        }

        let mut token = match component.as_deref() {
            Some(C::Model(value)) => JsonToken::ValueString(Some("model"), value.as_ref()),
            Some(component @ C::Message(_)) => {
                let result = self.encode_message(component, output);
                return self.complete_component(result);
            }
            Some(component @ C::Tools(_)) => {
                let result = self.request.encode_value(
                    Some("tools"),
                    &ToolsOutput(match component {
                        C::Tools(value) => value,
                        _ => unreachable!(),
                    }),
                    output,
                );
                return self.complete_component(result);
            }
            Some(C::ToolChoice(choice)) => JsonToken::ValueString(
                Some("tool_choice"),
                match choice {
                    ToolChoice::Auto => "auto",
                    ToolChoice::None => "none",
                    ToolChoice::Required => "required",
                    ToolChoice::Function(_) => {
                        let result = self.request.encode_value(
                            Some("tool_choice"),
                            &ChoiceOutput::from(choice),
                            output,
                        );
                        return self.complete_component(result);
                    }
                },
            ),
            Some(C::ParallelToolCalls(value)) => {
                JsonToken::ValueBoolean(Some("parallel_tool_calls"), *value)
            }
            Some(C::MaxTokens(value)) => {
                JsonToken::ValueNumber(Some("max_output_tokens"), (*value).into())
            }
            Some(C::Reasoning(value)) => {
                self.reasoning_effort = self.reasoning_effort.or(value.effort);
                if let Some(effort) = self.reasoning_effort {
                    let name = EffortWire::name(effort).ok_or_else(|| {
                        self.request.poison();
                        RequestEncodeError::InvalidState
                    })?;
                    let value = ReasoningOutput { effort: name };
                    let result = self.request.encode_value(Some("reasoning"), &value, output);
                    return self.complete_component(result);
                }
                self.request.clear_field();
                return Ok(EncoderStatus::ComponentFinished { written: 0 });
            }
            Some(C::TopP(value)) => JsonToken::ValueNumber(
                Some("top_p"),
                serde_json::Number::from_f64(*value).ok_or(RequestEncodeError::InvalidState)?,
            ),
            Some(C::TopK(_) | C::Provider(_)) => {
                self.request.poison();
                return Err(RequestEncodeError::InvalidState);
            }
            Some(C::Stop(_) | C::StreamOptions(_)) => {
                self.request.clear_field();
                return Ok(EncoderStatus::ComponentFinished { written: 0 });
            }
            Some(C::ResponseFormat(value)) => {
                let result =
                    self.request
                        .encode_value(Some("text"), &TextOutput::from(value), output);
                return self.complete_component(result);
            }
            Some(C::Stream(value)) => JsonToken::ValueBoolean(Some("stream"), *value),
            None => {
                let result = self.finalize(output);
                if result.is_err() {
                    self.request.poison();
                }
                return result;
            }
        };
        let result = self.request.encode_token(&mut token, output);
        self.complete_component(result)
    }

    fn finish(self) -> Result<(), Self::Error> {
        self.request.finish()
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
