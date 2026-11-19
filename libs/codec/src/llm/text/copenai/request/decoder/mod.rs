use super::component::Decoded;
use crate::decoder::{Decoder, DecoderError, DecoderStatus};
use crate::json::{JsonContainer, JsonDecoder, JsonToken};
use crate::llm::text::component::{
    Message, Provider, ResponseFormat, Stop, StreamOptions, TextRequestComponent, ToolChoice,
    ToolDefinition,
};
type RequestJson = JsonContainer;

#[derive(Debug, Clone, Copy)]
enum Field {
    Messages,
    Provider,
    ResponseFormat,
    StreamOptions,
    ToolChoice,
    Tools,
    Stop,
}

impl Field {
    fn parse(
        self,
        json: &mut RequestJson,
        input: &mut &[u8],
        eof: bool,
    ) -> Result<DecoderStatus<TextRequestComponent>, DecoderError> {
        Ok(match self {
            Self::Messages => match json.parse_next::<Decoded<Message>>(input, eof)? {
                DecoderStatus::Component(value) => DecoderStatus::Component(value.into()),
                DecoderStatus::NeedInput => DecoderStatus::NeedInput,
                DecoderStatus::Finished => DecoderStatus::Finished,
            },
            Self::Provider => match json.parse_next::<Decoded<Provider>>(input, eof)? {
                DecoderStatus::Component(value) => DecoderStatus::Component(value.into()),
                DecoderStatus::NeedInput => DecoderStatus::NeedInput,
                DecoderStatus::Finished => DecoderStatus::Finished,
            },
            Self::ResponseFormat => match json.parse_next::<Decoded<ResponseFormat>>(input, eof)? {
                DecoderStatus::Component(value) => DecoderStatus::Component(value.into()),
                DecoderStatus::NeedInput => DecoderStatus::NeedInput,
                DecoderStatus::Finished => DecoderStatus::Finished,
            },
            Self::StreamOptions => match json.parse_next::<Decoded<StreamOptions>>(input, eof)? {
                DecoderStatus::Component(value) => DecoderStatus::Component(value.into()),
                DecoderStatus::NeedInput => DecoderStatus::NeedInput,
                DecoderStatus::Finished => DecoderStatus::Finished,
            },
            Self::ToolChoice => match json.parse_next::<Decoded<ToolChoice>>(input, eof)? {
                DecoderStatus::Component(value) => DecoderStatus::Component(value.into()),
                DecoderStatus::NeedInput => DecoderStatus::NeedInput,
                DecoderStatus::Finished => DecoderStatus::Finished,
            },
            Self::Tools => match json.parse_next::<Decoded<Vec<ToolDefinition>>>(input, eof)? {
                DecoderStatus::Component(value) => DecoderStatus::Component(value.into()),
                DecoderStatus::NeedInput => DecoderStatus::NeedInput,
                DecoderStatus::Finished => DecoderStatus::Finished,
            },
            Self::Stop => match json.parse_next::<Decoded<Stop>>(input, eof)? {
                DecoderStatus::Component(value) => DecoderStatus::Component(value.into()),
                DecoderStatus::NeedInput => DecoderStatus::NeedInput,
                DecoderStatus::Finished => DecoderStatus::Finished,
            },
        })
    }
}

#[derive(Debug)]
pub struct CompatibleOpenAIRequestDecoder {
    json: Option<RequestJson>,
    field: Option<Field>,
    failed: bool,
    started: bool,
    closed: bool,
    has_model: bool,
    has_messages: bool,
    has_token_limit: bool,
}

impl CompatibleOpenAIRequestDecoder {
    pub fn new(json: JsonDecoder) -> Self {
        Self {
            json: Some(RequestJson::new(json)),
            field: None,
            failed: false,
            started: false,
            closed: false,
            has_model: false,
            has_messages: false,
            has_token_limit: false,
        }
    }

    fn proceed_scalar(
        &mut self,
        token: JsonToken<'_>,
    ) -> Result<Option<TextRequestComponent>, DecoderError> {
        let component = match token {
            JsonToken::ValueBoolean(Some("stream"), value) => TextRequestComponent::Stream(value),
            JsonToken::ValueNull(Some("stream_options")) => {
                TextRequestComponent::StreamOptions(None)
            }
            JsonToken::ValueBoolean(Some("parallel_tool_calls"), value) => {
                TextRequestComponent::ParallelToolCalls(value)
            }
            JsonToken::ValueNumber(Some("max_completion_tokens" | "max_tokens"), value) => {
                if self.has_token_limit {
                    return Err(DecoderError::InvalidInput("conflicting token limits"));
                }
                let limit = value
                    .as_u64()
                    .and_then(|value| u32::try_from(value).ok())
                    .ok_or(DecoderError::InvalidInput("invalid token limit"))?;
                self.has_token_limit = true;
                if limit == 0 {
                    return Err(DecoderError::InvalidInput("invalid token limit"));
                }
                TextRequestComponent::MaxCompletionTokens(limit)
            }
            JsonToken::ValueNumber(Some("top_p"), value) => {
                let value = value
                    .as_f64()
                    .ok_or(DecoderError::InvalidInput("invalid top_p"))?;
                if !value.is_finite() || !(0.0..=1.0).contains(&value) {
                    return Err(DecoderError::InvalidInput("invalid top_p"));
                }
                TextRequestComponent::TopP(value)
            }
            JsonToken::ValueNull(Some("top_p")) => return Ok(None),
            JsonToken::ValueNumber(Some("top_k"), value) => {
                let value = value
                    .as_u64()
                    .and_then(|value| u32::try_from(value).ok())
                    .ok_or(DecoderError::InvalidInput("invalid top_k"))?;
                TextRequestComponent::TopK(value)
            }
            JsonToken::ValueString(Some("tool_choice"), value) => {
                let choice = match value {
                    "auto" => ToolChoice::Auto,
                    "none" => ToolChoice::None,
                    "required" => ToolChoice::Required,
                    _ => return Err(DecoderError::InvalidInput("invalid tool choice")),
                };
                TextRequestComponent::ToolChoice(choice)
            }
            JsonToken::ValueString(Some("stop"), value) => {
                if value.is_empty() || value.len() > 4096 {
                    return Err(DecoderError::InvalidInput("invalid stop sequence"));
                }
                TextRequestComponent::Stop(Stop::Text(value.to_owned()))
            }
            JsonToken::ValueNull(Some("stop")) => TextRequestComponent::Stop(Stop::Null),
            JsonToken::ValueString(Some("model"), model) => {
                if model.is_empty() {
                    return Err(DecoderError::InvalidInput("invalid model"));
                }
                self.has_model = true;
                TextRequestComponent::Model(model.to_owned())
            }
            _ => return Err(DecoderError::Unsupported("request field")),
        };
        Ok(Some(component))
    }
}

impl Decoder for CompatibleOpenAIRequestDecoder {
    type Component<'a> = TextRequestComponent;
    type Error = DecoderError;

    fn next<'a, 'input: 'a>(
        &'a mut self,
        input: &mut &'input [u8],
        eof: bool,
    ) -> Result<DecoderStatus<Self::Component<'a>>, Self::Error> {
        if self.failed {
            return Err(DecoderError::InvalidState);
        }
        let mut json = self.json.take().ok_or(DecoderError::InvalidState)?;
        let result = loop {
            if let Some(field) = self.field {
                match field.parse(&mut json, input, eof) {
                    Ok(DecoderStatus::Component(component)) => {
                        if !matches!(field, Field::Messages) {
                            self.field = None;
                        }
                        break Ok(DecoderStatus::Component(component));
                    }
                    Ok(DecoderStatus::NeedInput) => break Ok(DecoderStatus::NeedInput),
                    Ok(DecoderStatus::Finished) => {
                        self.field = None;
                        continue;
                    }
                    Err(error) => break Err(error),
                }
            }
            match json.next(input, eof) {
                Ok(DecoderStatus::Component(
                    token @ (JsonToken::ValueString(..)
                    | JsonToken::ValueNumber(..)
                    | JsonToken::ValueBoolean(..)
                    | JsonToken::ValueNull(..)),
                )) => match self.proceed_scalar(token) {
                    Ok(Some(component)) => break Ok(DecoderStatus::Component(component)),
                    Ok(None) => continue,
                    Err(error) => break Err(error),
                },
                Ok(DecoderStatus::Component(JsonToken::ObjectStart(None))) if !self.started => {
                    self.started = true;
                }
                Ok(DecoderStatus::Component(JsonToken::ArrayStart(Some("messages")))) => {
                    json = match json.enter() {
                        Ok(array) => array.nonempty(),
                        Err(error) => {
                            self.failed = true;
                            return Err(error.into());
                        }
                    };
                    self.has_messages = true;
                    self.field = Some(Field::Messages);
                }
                Ok(DecoderStatus::Component(JsonToken::ObjectStart(Some(key)))) => {
                    let field = match key {
                        "provider" => Field::Provider,
                        "response_format" => Field::ResponseFormat,
                        "stream_options" => Field::StreamOptions,
                        "tool_choice" => Field::ToolChoice,
                        _ => break Err(DecoderError::Unsupported("request object")),
                    };
                    json = match json.enter() {
                        Ok(object) => object,
                        Err(error) => {
                            self.failed = true;
                            return Err(error.into());
                        }
                    };
                    self.field = Some(field);
                }
                Ok(DecoderStatus::Component(JsonToken::ArrayStart(Some(key)))) => {
                    let field = match key {
                        "tools" => Field::Tools,
                        "stop" => Field::Stop,
                        _ => break Err(DecoderError::Unsupported("request array")),
                    };
                    json = match json.enter_value() {
                        Ok(array) => array,
                        Err(error) => {
                            self.failed = true;
                            return Err(error.into());
                        }
                    };
                    self.field = Some(field);
                }
                Ok(DecoderStatus::Component(JsonToken::ObjectEnd)) => {
                    if !self.started || !self.has_model || !self.has_messages {
                        break Err(DecoderError::InvalidInput(
                            "model and messages are required",
                        ));
                    }
                    self.closed = true;
                }
                Ok(DecoderStatus::Component(_)) => {
                    break Err(DecoderError::Unsupported("request container"));
                }
                Ok(DecoderStatus::NeedInput) => break Ok(DecoderStatus::NeedInput),
                Ok(DecoderStatus::Finished) => {
                    break if self.closed {
                        Ok(DecoderStatus::Finished)
                    } else {
                        Err(DecoderError::IncompleteInput)
                    };
                }
                Err(error) => break Err(error.into()),
            }
        };
        self.json = Some(json);
        self.failed = result.is_err();
        result
    }

    fn finish(self) -> Result<(), Self::Error> {
        if self.failed {
            return Err(DecoderError::InvalidState);
        }
        if !self.closed {
            return Err(DecoderError::IncompleteInput);
        }
        self.json
            .ok_or(DecoderError::InvalidState)?
            .finish()
            .map_err(|_| DecoderError::IncompleteInput)
    }
}
