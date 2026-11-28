use super::component::{
    ChatTools, EffortWire, FormatInput, FunctionChoice, MessageInput, Stops, StreamOptionsWire,
};
use crate::decoder::{Decoder, DecoderError, DecoderStatus};
use crate::json::{JsonDecoder, JsonToken};
use crate::llm::text::component::{
    Reasoning, Stop, StreamOptions, TextRequestComponent, ToolChoice,
};
use crate::llm::text::request::decoder::{CaptureMode, Ready, RequestStep, TextRequestDecoder};
use serde_with::de::DeserializeAsWrap as Input;

#[derive(Clone, Copy, Debug)]
enum Field {
    Messages,
    Tools,
    Stop,
    ResponseFormat,
    ToolChoice,
    StreamOptions,
}

#[derive(Debug)]
pub struct OpenAICompletionRequestDecoder {
    request: TextRequestDecoder<Field>,
    have_token_limit: bool,
    stream: bool,
    stream_options: bool,
    have_model: bool,
    have_messages: bool,
}

impl OpenAICompletionRequestDecoder {
    pub fn new(json: JsonDecoder) -> Self {
        Self {
            request: TextRequestDecoder::new(json),
            have_token_limit: false,
            stream: false,
            stream_options: false,
            have_model: false,
            have_messages: false,
        }
    }
}

impl Decoder for OpenAICompletionRequestDecoder {
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
                    if self.stream_options && !self.stream {
                        return Err(DecoderError::InvalidInput(
                            "stream options require streaming",
                        ));
                    }
                    if !self.have_model || !self.have_messages {
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
                            "tools" => Field::Tools,
                            "stop" => Field::Stop,
                            _ => return Err(DecoderError::Unsupported("request array")),
                        },
                        JsonToken::ObjectStart(Some(key)) => match key {
                            "response_format" => Field::ResponseFormat,
                            "stream_options" => Field::StreamOptions,
                            "tool_choice" => Field::ToolChoice,
                            _ => return Err(DecoderError::Unsupported("request object")),
                        },
                        _ => {
                            if matches!(
                                self.request.current(source)?,
                                JsonToken::ValueNull(Some("top_p"))
                            ) {
                                continue;
                            }
                            break (Ready::Scalar, source);
                        }
                    };
                    let mode = if matches!(field, Field::Messages) {
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
                    JsonToken::ValueString(Some("reasoning_effort"), value) => {
                        C::Reasoning(Reasoning {
                            effort: Some(
                                EffortWire::parse(value).ok_or(DecoderError::InvalidInput(
                                    "invalid reasoning effort",
                                ))?,
                            ),
                            ..Default::default()
                        })
                    }
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
            Ready::Value(field) => match field {
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
                Field::StreamOptions => C::StreamOptions(Some(
                    json.deserialize_current::<Input<StreamOptions, StreamOptionsWire>>(source)?
                        .into_inner(),
                )),
            },
        };
        match &component {
            C::Stream(value) => self.stream = *value,
            C::StreamOptions(value) => self.stream_options = value.is_some(),
            _ => (),
        }
        self.request.complete();
        Ok(DecoderStatus::Component(component))
    }

    fn finish(self) -> Result<(), Self::Error> {
        self.request.finish()
    }
}
