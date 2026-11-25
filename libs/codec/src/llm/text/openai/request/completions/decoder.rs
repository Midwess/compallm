use super::component::{
    EffortInput, FormatInput, MessageInput, StopInput, StreamOptionsInput, ToolChoiceInput,
    ToolsWire,
};
use crate::decoder::{Decoder, DecoderError, DecoderStatus};
use crate::json::{JsonContainer, JsonDecoder, JsonToken};
use crate::llm::text::component::*;
use serde_with::de::DeserializeAsWrap as Input;

#[derive(Debug, Clone, Copy)]
enum Field {
    Messages,
    ResponseFormat,
    Stop,
    StreamOptions,
    ToolChoice,
    Tools,
}

#[derive(Debug)]
pub struct OpenAICompletionRequestDecoder {
    json: Option<JsonContainer>,
    field: Option<Field>,
    started: bool,
    closed: bool,
    failed: bool,
    has_model: bool,
    has_messages: bool,
    has_token_limit: bool,
}

impl OpenAICompletionRequestDecoder {
    pub fn new(json: JsonDecoder) -> Self {
        Self {
            json: Some(JsonContainer::new(json)),
            field: None,
            started: false,
            closed: false,
            failed: false,
            has_model: false,
            has_messages: false,
            has_token_limit: false,
        }
    }

    fn proceed_scalar(
        &mut self,
        token: JsonToken<'_>,
    ) -> Result<Option<TextRequestComponent>, DecoderError> {
        use TextRequestComponent as C;
        let component = match token {
            JsonToken::ValueBoolean(Some("stream"), value) => C::Stream(value),
            JsonToken::ValueBoolean(Some("parallel_tool_calls"), value) => {
                C::ParallelToolCalls(value)
            }
            JsonToken::ValueNumber(Some("max_completion_tokens" | "max_tokens"), value) => {
                if self.has_token_limit {
                    return Err(DecoderError::InvalidInput("conflicting token limits"));
                }
                let limit = value
                    .as_u64()
                    .and_then(|value| u32::try_from(value).ok())
                    .filter(|value| *value > 0)
                    .ok_or(DecoderError::InvalidInput("invalid token limit"))?;
                self.has_token_limit = true;
                C::MaxTokens(limit)
            }
            JsonToken::ValueNumber(Some("top_p"), value) => {
                let value = value
                    .as_f64()
                    .filter(|value| value.is_finite() && (0.0..=1.0).contains(value))
                    .ok_or(DecoderError::InvalidInput("invalid top_p"))?;
                C::TopP(value)
            }
            JsonToken::ValueNull(Some("top_p")) => return Ok(None),
            JsonToken::ValueString(Some("tool_choice"), value) => C::ToolChoice(match value {
                "auto" => ToolChoice::Auto,
                "none" => ToolChoice::None,
                "required" => ToolChoice::Required,
                _ => return Err(DecoderError::InvalidInput("invalid tool choice")),
            }),
            JsonToken::ValueString(Some("stop"), value) => {
                if value.is_empty() || value.len() > 4096 {
                    return Err(DecoderError::InvalidInput("invalid stop sequence"));
                }
                C::Stop(Stop::Text(value.to_owned()))
            }
            JsonToken::ValueNull(Some("stop")) => C::Stop(Stop::Null),
            JsonToken::ValueString(Some("reasoning_effort"), value) => {
                let effort = match value {
                    "minimal" => EffortInput::Minimal,
                    "low" => EffortInput::Low,
                    "medium" => EffortInput::Medium,
                    "high" => EffortInput::High,
                    _ => return Err(DecoderError::InvalidInput("invalid reasoning effort")),
                };
                C::Reasoning(Reasoning {
                    effort: Some(effort.into()),
                    ..Reasoning::default()
                })
            }
            JsonToken::ValueString(Some("model"), model) => {
                if model.is_empty() {
                    return Err(DecoderError::InvalidInput("invalid model"));
                }
                self.has_model = true;
                C::Model(model.to_owned())
            }
            _ => return Err(DecoderError::Unsupported("request field")),
        };
        Ok(Some(component))
    }
}

impl Decoder for OpenAICompletionRequestDecoder {
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
                let status = match field {
                    Field::Messages => json
                        .parse_next::<Input<Message, MessageInput>>(input, eof)
                        .map(|status| DecodedField(status, TextRequestComponent::Message).into()),
                    Field::ResponseFormat => json
                        .parse_next::<Input<ResponseFormat, FormatInput>>(input, eof)
                        .map(|status| {
                            DecodedField(status, TextRequestComponent::ResponseFormat).into()
                        }),
                    Field::Stop => json
                        .parse_next::<Input<Stop, StopInput>>(input, eof)
                        .map(|status| DecodedField(status, TextRequestComponent::Stop).into()),
                    Field::StreamOptions => json
                        .parse_next::<Input<StreamOptions, StreamOptionsInput>>(input, eof)
                        .map(|status| {
                            DecodedField(status, |value| {
                                TextRequestComponent::StreamOptions(Some(value))
                            })
                            .into()
                        }),
                    Field::ToolChoice => json
                        .parse_next::<Input<ToolChoice, ToolChoiceInput>>(input, eof)
                        .map(|status| {
                            DecodedField(status, TextRequestComponent::ToolChoice).into()
                        }),
                    Field::Tools => json
                        .parse_next::<Input<ToolDefinition, ToolsWire>>(input, eof)
                        .map(|status| DecodedField(status, TextRequestComponent::Tool).into()),
                };
                match status {
                    Ok(DecoderStatus::Component(component)) => {
                        if !matches!(field, Field::Messages | Field::Tools) {
                            self.field = None;
                        }
                        break Ok(DecoderStatus::Component(component));
                    }
                    Ok(DecoderStatus::NeedInput) => break Ok(DecoderStatus::NeedInput),
                    Ok(DecoderStatus::Finished) => {
                        self.field = None;
                        if matches!(field, Field::Tools) {
                            break Ok(DecoderStatus::Component(TextRequestComponent::ToolsEnded));
                        }
                        continue;
                    }
                    Err(error) => break Err(error.into()),
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
                Ok(DecoderStatus::Component(JsonToken::ArrayStart(Some(key)))) => {
                    let field = match key {
                        "stop" => Field::Stop,
                        "tools" => Field::Tools,
                        _ => break Err(DecoderError::Unsupported("request array")),
                    };
                    json = match if matches!(field, Field::Tools) {
                        json.enter()
                    } else {
                        json.enter_value()
                    } {
                        Ok(array) => array,
                        Err(error) => {
                            self.failed = true;
                            return Err(error.into());
                        }
                    };
                    self.field = Some(field);
                    if matches!(field, Field::Tools) {
                        break Ok(DecoderStatus::Component(TextRequestComponent::ToolsStarted));
                    }
                }
                Ok(DecoderStatus::Component(JsonToken::ObjectStart(Some(key)))) => {
                    let field = match key {
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

struct DecodedField<T, A>(DecoderStatus<Input<T, A>>, fn(T) -> TextRequestComponent);

impl<T, A> From<DecodedField<T, A>> for DecoderStatus<TextRequestComponent> {
    fn from(DecodedField(status, component): DecodedField<T, A>) -> Self {
        match status {
            DecoderStatus::Component(value) => Self::Component(component(value.into_inner())),
            DecoderStatus::NeedInput => Self::NeedInput,
            DecoderStatus::Finished => Self::Finished,
        }
    }
}
