use super::component::{MessageOutput, StopOutput};
use crate::encoder::{Encoder, EncoderStatus};
use crate::json::{JsonEncoder, JsonToken};
use crate::llm::text::component::TextRequestComponent;
pub use crate::llm::text::request::encoder::RequestEncodeError;
use crate::llm::text::request::encoder::TextRequestEncoder;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AnthropicRequestEncoderConfiguration {}

#[derive(Debug)]
pub struct AnthropicRequestEncoder {
    request: TextRequestEncoder<Field>,
    has_model: bool,
    has_messages: bool,
    has_token_limit: bool,
    has_system: bool,
}

impl AnthropicRequestEncoder {
    pub fn new(json: JsonEncoder, _configuration: AnthropicRequestEncoderConfiguration) -> Self {
        Self {
            request: TextRequestEncoder::new(json, Field::Messages, "messages"),
            has_model: false,
            has_messages: false,
            has_token_limit: false,
            has_system: false,
        }
    }

    fn encode_message<S: serde::Serialize + AsRef<str>>(
        &mut self,
        component: &TextRequestComponent<S>,
        output: &mut [u8],
    ) -> Result<EncoderStatus, RequestEncodeError> {
        let written = match self.request.prepare_messages(output)? {
            EncoderStatus::ComponentFinished { written } => written,
            status => return Ok(status),
        };
        match self.request.encode_value(
            None,
            &MessageOutput::try_from(match component {
                TextRequestComponent::Message(message) => message,
                _ => return Err(RequestEncodeError::InvalidState),
            })
            .map_err(|_| RequestEncodeError::InvalidState)?,
            &mut output[written..],
        )? {
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

    fn complete_component(
        &mut self,
        result: Result<EncoderStatus, RequestEncodeError>,
    ) -> Result<EncoderStatus, RequestEncodeError> {
        if matches!(&result, Ok(EncoderStatus::ComponentFinished { .. })) {
            self.has_model |= self.request.field() == Some(Field::Model);
            self.has_token_limit |= self.request.field() == Some(Field::MaxTokens);
            self.has_system |= self.request.field() == Some(Field::System);
        }
        self.request.complete_component(&result);
        result
    }

    fn finalize(&mut self, output: &mut [u8]) -> Result<EncoderStatus, RequestEncodeError> {
        if self.request.field().is_some()
            || !self.has_model
            || !self.has_messages
            || !self.has_token_limit
        {
            return Err(RequestEncodeError::Incomplete);
        }
        self.request.start_finalizing();
        self.request.close(output)
    }

    pub fn finish(self) -> Result<(), RequestEncodeError> {
        <Self as Encoder<TextRequestComponent>>::finish(self)
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
        use crate::llm::text::component::Role;
        use TextRequestComponent as C;

        if self.request.check(component.is_some())? {
            return Ok(EncoderStatus::Finished { written: 0 });
        }
        if let Some(component) = component.as_deref() {
            let field = Field::try_from(component).inspect_err(|_| {
                self.request.poison();
            })?;
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
            Some(C::Message(message)) if message.role == Role::System => {
                if self.has_system || self.request.messages_started() {
                    self.request.poison();
                    return Err(RequestEncodeError::InvalidState);
                }
                let value = MessageOutput::try_from(message).map_err(|_| {
                    self.request.poison();
                    RequestEncodeError::InvalidState
                })?;
                let mut token = JsonToken::ValueString(Some("system"), value.content);
                let result = self.request.encode_token(&mut token, output);
                return self.complete_component(result);
            }
            Some(component @ C::Message(_)) => {
                let result = self.encode_message(component, output);
                return self.complete_component(result);
            }
            Some(C::MaxTokens(value)) => {
                JsonToken::ValueNumber(Some("max_tokens"), (*value).into())
            }
            Some(C::Stop(value)) => {
                let value = StopOutput::try_from(value).map_err(|_| {
                    self.request.poison();
                    RequestEncodeError::InvalidState
                })?;
                let result = self
                    .request
                    .encode_value(Some("stop_sequences"), &value, output);
                return self.complete_component(result);
            }
            Some(C::TopP(_) | C::StreamOptions(_)) => {
                self.request.clear_field();
                return Ok(EncoderStatus::ComponentFinished { written: 0 });
            }
            Some(C::Stream(value)) => JsonToken::ValueBoolean(Some("stream"), *value),
            Some(_) => {
                self.request.poison();
                return Err(RequestEncodeError::InvalidState);
            }
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
    System,
    MaxTokens,
    Stop,
    Stream,
    Omitted,
}

impl<S> TryFrom<&TextRequestComponent<S>> for Field {
    type Error = RequestEncodeError;
    fn try_from(component: &TextRequestComponent<S>) -> Result<Self, Self::Error> {
        use crate::llm::text::component::Role;
        use TextRequestComponent as C;
        Ok(match component {
            C::Model(_) => Self::Model,
            C::Message(message) if message.role == Role::System => Self::System,
            C::Message(_) => Self::Messages,
            C::MaxTokens(_) => Self::MaxTokens,
            C::Stop(_) => Self::Stop,
            C::Stream(_) => Self::Stream,
            C::TopP(_) | C::StreamOptions(_) => Self::Omitted,
            _ => return Err(RequestEncodeError::InvalidState),
        })
    }
}
