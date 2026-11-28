use super::component::{MessageInput, Stops};
use crate::decoder::{Decoder, DecoderError, DecoderStatus};
use crate::json::{JsonDecoder, JsonToken};
use crate::llm::text::component::{Content, Message, Role, TextRequestComponent};
use crate::llm::text::request::decoder::{CaptureMode, Ready, RequestStep, TextRequestDecoder};

#[derive(Clone, Copy, Debug)]
enum Field {
    Messages,
    Stop,
}

#[derive(Debug)]
pub struct AnthropicRequestDecoder {
    request: TextRequestDecoder<Field>,
    have_token_limit: bool,
    have_model: bool,
    have_messages: bool,
    messages_started: bool,
}

impl AnthropicRequestDecoder {
    pub fn new(json: JsonDecoder) -> Self {
        Self {
            request: TextRequestDecoder::new(json),
            have_token_limit: false,
            have_model: false,
            have_messages: false,
            messages_started: false,
        }
    }
}

impl Decoder for AnthropicRequestDecoder {
    type Component<'a> = TextRequestComponent<&'a str>;
    type Error = DecoderError;

    fn next<'a, 'input: 'a>(
        &'a mut self,
        input: &mut &'input [u8],
        eof: bool,
    ) -> Result<DecoderStatus<Self::Component<'a>>, Self::Error> {
        use TextRequestComponent as C;

        self.request.begin()?;
        let (ready, source) = loop {
            let (step, source) = self.request.advance(input, eof)?;
            match step {
                RequestStep::NeedInput => {
                    self.request.complete();
                    return Ok(DecoderStatus::NeedInput);
                }
                RequestStep::Finished => {
                    self.request.complete();
                    return Ok(DecoderStatus::Finished);
                }
                RequestStep::Closed => {
                    if !self.have_model || !self.have_messages || !self.have_token_limit {
                        return Err(DecoderError::InvalidInput(
                            "model and messages are required",
                        ));
                    }
                }
                RequestStep::Value(field) => break (Ready::Value(field), source),
                RequestStep::Token => {
                    let field = match self.request.current(source)? {
                        JsonToken::ArrayStart(Some(key)) => match key {
                            "messages" => Field::Messages,
                            "stop_sequences" => Field::Stop,
                            _ => return Err(DecoderError::Unsupported("request array")),
                        },
                        _ => {
                            break (Ready::Scalar, source);
                        }
                    };
                    let mode = if matches!(field, Field::Messages) {
                        self.messages_started = true;
                        CaptureMode::Items
                    } else {
                        CaptureMode::Value
                    };
                    self.request
                        .enter(field, mode, matches!(field, Field::Messages))?;
                }
            }
        };
        let json = &self.request;
        let component = match ready {
            Ready::Scalar => {
                let component = match json.current(source)? {
                    JsonToken::ValueString(Some("model"), value) if !value.is_empty() => {
                        C::Model(value)
                    }
                    JsonToken::ValueString(Some("system"), text) if !self.messages_started => {
                        C::Message(Message {
                            role: Role::System,
                            content: Some(Content::Text(text)),
                            tool_calls: None,
                            tool_call_id: None,
                            reasoning: None,
                            reasoning_content: None,
                            reasoning_details: None,
                        })
                    }
                    JsonToken::ValueBoolean(Some("stream"), value) => C::Stream(value),
                    JsonToken::ValueNumber(Some("max_tokens"), value) => {
                        if self.have_token_limit {
                            return Err(DecoderError::InvalidInput("conflicting token limits"));
                        }
                        let value = value
                            .as_u64()
                            .and_then(|value| u32::try_from(value).ok())
                            .filter(|value| *value > 0)
                            .ok_or(DecoderError::InvalidInput("invalid token limit"))?;
                        self.have_token_limit = true;
                        C::MaxTokens(value)
                    }
                    _ => return Err(DecoderError::Unsupported("request scalar")),
                };
                if matches!(component, TextRequestComponent::Model(_)) {
                    self.have_model = true;
                }
                component
            }
            Ready::Value(field) => match field {
                Field::Messages => {
                    let value = json.deserialize_current::<MessageInput<'_>>(source)?;
                    let message = value.into();
                    self.have_messages = true;
                    C::Message(message)
                }
                Field::Stop => C::Stop(json.deserialize_current::<Stops<'_>>(source)?.0),
            },
        };
        self.request.complete();
        Ok(DecoderStatus::Component(component))
    }

    fn finish(self) -> Result<(), Self::Error> {
        self.request.finish()
    }
}
