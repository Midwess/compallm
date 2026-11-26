use super::component::{
    ChatTools, FormatInput, FunctionChoice, MessageInput, ProviderWire, ReasoningWire, Stops,
    StreamOptionsWire,
};
use crate::decoder::{Decoder, DecoderError, DecoderStatus};
use crate::json::{JsonContainer, JsonDecoder, JsonToken};
use crate::llm::text::component::{
    Provider, Reasoning, Stop, StreamOptions, TextRequestComponent, ToolChoice,
};
use serde_with::de::DeserializeAsWrap as Input;

#[derive(Clone, Copy, Debug)]
enum Field {
    Messages,
    Tools,
    Stop,
    ResponseFormat,
    ToolChoice,
    Provider,
    Reasoning,
    StreamOptions,
}

enum Ready {
    Scalar,
    Value(Field),
}

enum Action {
    Start,
    Close,
    Scalar,
    Enter(Field),
}

#[derive(Debug)]
pub struct CompatibleOpenAIRequestDecoder {
    json: JsonContainer,
    field: Option<Field>,
    have_token_limit: bool,
    have_model: bool,
    have_messages: bool,
    started: bool,
    closed: bool,
    failed: bool,
}

impl CompatibleOpenAIRequestDecoder {
    pub fn new(json: JsonDecoder) -> Self {
        Self {
            json: JsonContainer::new(json),
            field: None,
            have_token_limit: false,
            have_model: false,
            have_messages: false,
            started: false,
            closed: false,
            failed: false,
        }
    }
}

impl Decoder for CompatibleOpenAIRequestDecoder {
    type Component<'a> = TextRequestComponent<&'a str>;
    type Error = DecoderError;

    fn next<'a, 'input: 'a>(
        &'a mut self,
        input: &mut &'input [u8],
        eof: bool,
    ) -> Result<DecoderStatus<Self::Component<'a>>, Self::Error> {
        use TextRequestComponent as C;

        if self.failed {
            return Err(DecoderError::InvalidState);
        }
        self.failed = true;
        let (ready, source) = loop {
            let source = *input;
            let status = if self.field.is_some() {
                self.json.advance_value(input, eof)?
            } else {
                self.json.advance(input, eof)?
            };
            match status {
                DecoderStatus::NeedInput => {
                    self.failed = false;
                    return Ok(DecoderStatus::NeedInput);
                }
                DecoderStatus::Component(()) => {
                    if let Some(field) = self.field {
                        break (Ready::Value(field), source);
                    }
                }
                DecoderStatus::Finished => match self.field.take() {
                    Some(Field::Messages) => continue,
                    Some(_) => return Err(DecoderError::IncompleteInput),
                    None => {
                        if !self.closed {
                            return Err(DecoderError::IncompleteInput);
                        }
                        self.failed = false;
                        return Ok(DecoderStatus::Finished);
                    }
                },
            }
            let action = match self.json.current(source)? {
                JsonToken::ObjectStart(None) if !self.started => Action::Start,
                JsonToken::ObjectEnd if self.started && !self.closed => Action::Close,
                JsonToken::ArrayStart(Some(key)) if self.started && !self.closed => {
                    Action::Enter(match key {
                        "messages" => Field::Messages,
                        "tools" => Field::Tools,
                        "stop" => Field::Stop,
                        _ => return Err(DecoderError::Unsupported("request array")),
                    })
                }
                JsonToken::ObjectStart(Some(key)) if self.started && !self.closed => {
                    Action::Enter(match key {
                        "provider" => Field::Provider,
                        "reasoning" => Field::Reasoning,
                        "response_format" => Field::ResponseFormat,
                        "stream_options" => Field::StreamOptions,
                        "tool_choice" => Field::ToolChoice,
                        _ => return Err(DecoderError::Unsupported("request object")),
                    })
                }
                token @ (JsonToken::ValueString(..)
                | JsonToken::ValueNumber(..)
                | JsonToken::ValueBoolean(..)
                | JsonToken::ValueNull(..))
                    if self.started && !self.closed =>
                {
                    if matches!(token, JsonToken::ValueNull(Some("top_p"))) {
                        continue;
                    }
                    Action::Scalar
                }
                _ => return Err(DecoderError::InvalidInput("invalid request container")),
            };
            match action {
                Action::Start => self.started = true,
                Action::Close => {
                    if !self.have_model || !self.have_messages {
                        return Err(DecoderError::InvalidInput(
                            "model and messages are required",
                        ));
                    }
                    self.closed = true;
                }
                Action::Scalar => break (Ready::Scalar, source),
                Action::Enter(field) => {
                    match field {
                        Field::Messages => {
                            self.json.enter()?.nonempty();
                        }
                        Field::Stop | Field::Tools => {
                            self.json.enter_value()?;
                        }
                        _ => {
                            self.json.enter()?;
                        }
                    }
                    self.field = Some(field);
                }
            }
        };
        let json = &self.json;
        let component = match ready {
            Ready::Scalar => {
                let component = match json.current(source)? {
                    JsonToken::ValueString(Some("model"), value) if !value.is_empty() => {
                        C::Model(value)
                    }
                    JsonToken::ValueString(Some("stop"), value)
                        if !value.is_empty() && value.len() <= 4096 =>
                    {
                        C::Stop(Stop::Text(value))
                    }
                    JsonToken::ValueString(Some("tool_choice"), value) => {
                        C::ToolChoice(match value {
                            "auto" => ToolChoice::Auto,
                            "none" => ToolChoice::None,
                            "required" => ToolChoice::Required,
                            _ => return Err(DecoderError::InvalidInput("invalid tool choice")),
                        })
                    }
                    JsonToken::ValueBoolean(Some("stream"), value) => C::Stream(value),
                    JsonToken::ValueBoolean(Some("parallel_tool_calls"), value) => {
                        C::ParallelToolCalls(value)
                    }
                    JsonToken::ValueNull(Some("stop")) => C::Stop(Stop::Null),
                    JsonToken::ValueNull(Some("stream_options")) => C::StreamOptions(None),
                    JsonToken::ValueNumber(Some("max_tokens" | "max_completion_tokens"), value) => {
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
                    JsonToken::ValueNumber(Some("top_p"), value) => {
                        let value = value
                            .as_f64()
                            .filter(|value| value.is_finite() && (0.0..=1.0).contains(value))
                            .ok_or(DecoderError::InvalidInput("invalid top_p"))?;
                        C::TopP(value)
                    }
                    JsonToken::ValueNumber(Some("top_k"), value) => C::TopK(
                        value
                            .as_u64()
                            .and_then(|value| u32::try_from(value).ok())
                            .ok_or(DecoderError::InvalidInput("invalid top_k"))?,
                    ),
                    JsonToken::ValueString(Some("model" | "stop"), _) => {
                        return Err(DecoderError::InvalidInput("invalid request text"));
                    }
                    _ => return Err(DecoderError::Unsupported("request scalar")),
                };
                if matches!(component, TextRequestComponent::Model(_)) {
                    self.have_model = true;
                }
                component
            }
            Ready::Value(field) => {
                if !matches!(field, Field::Messages) {
                    self.field = None;
                }
                match field {
                    Field::Messages => {
                        let value = json.deserialize_current::<MessageInput<'_>>(source)?;
                        let message = value.try_into().map_err(DecoderError::InvalidInput)?;
                        self.have_messages = true;
                        C::Message(message)
                    }
                    Field::Tools => C::Tools(json.deserialize_current::<ChatTools<'_>>(source)?.0),
                    Field::Stop => C::Stop(json.deserialize_current::<Stops<'_>>(source)?.0),
                    Field::ResponseFormat => C::ResponseFormat(
                        json.deserialize_current::<FormatInput<'_>>(source)?
                            .try_into()
                            .map_err(DecoderError::InvalidInput)?,
                    ),
                    Field::ToolChoice => C::ToolChoice(
                        json.deserialize_current::<FunctionChoice<'_>>(source)?
                            .into(),
                    ),
                    Field::Provider => C::Provider(
                        json.deserialize_current::<Input<Provider, ProviderWire>>(source)?
                            .into_inner(),
                    ),
                    Field::Reasoning => C::Reasoning(
                        json.deserialize_current::<Input<Reasoning, ReasoningWire>>(source)?
                            .into_inner(),
                    ),
                    Field::StreamOptions => C::StreamOptions(Some(
                        json.deserialize_current::<Input<StreamOptions, StreamOptionsWire>>(
                            source,
                        )?
                        .into_inner(),
                    )),
                }
            }
        };
        self.failed = false;
        Ok(DecoderStatus::Component(component))
    }

    fn finish(self) -> Result<(), Self::Error> {
        if self.failed {
            return Err(DecoderError::InvalidState);
        }
        if !self.closed {
            return Err(DecoderError::IncompleteInput);
        }
        self.json.finish().map_err(Into::into)
    }
}
