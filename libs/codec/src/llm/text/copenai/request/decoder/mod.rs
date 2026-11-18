mod message;
mod stop;

use super::component::CompatibleOpenAIRequestComponent;
use crate::decoder::{Decoder, DecoderError, DecoderStatus};
use crate::json::{JsonDecoder, JsonToken};
use crate::llm::text::component::{Stop, TextRequestComponent, ToolChoice};
use message::MessageState;
use stop::StopState;

#[derive(Debug)]
pub struct CompatibleOpenAIRequestDecoder {
    json: Option<JsonDecoder>,
    failed: bool,
    messages: MessageState,
    stop: StopState,
}

impl CompatibleOpenAIRequestDecoder {
    pub fn new(json: JsonDecoder) -> Self {
        Self {
            json: Some(json),
            failed: false,
            messages: MessageState::default(),
            stop: StopState::default(),
        }
    }

    fn proceed(
        &mut self,
        token: JsonToken<'_>,
    ) -> Result<Option<CompatibleOpenAIRequestComponent>, DecoderError> {
        if self.messages.active {
            return self.messages.proceed(token);
        }
        if self.stop.active {
            return self.stop.proceed(token);
        }
        let (key, value) = match token {
            JsonToken::ValueString(key, value) => (key, JsonToken::ValueString(None, value)),
            JsonToken::ValueNumber(key, value) => (key, JsonToken::ValueNumber(None, value)),
            JsonToken::ValueBoolean(key, value) => (key, JsonToken::ValueBoolean(None, value)),
            JsonToken::ValueNull(key) => (key, JsonToken::ValueNull(None)),
            JsonToken::ValueObject(key) => (key, JsonToken::ValueObject(None)),
            JsonToken::ValueArray(key) => (key, JsonToken::ValueArray(None)),
            JsonToken::ObjectEnd => return Ok(None),
            JsonToken::ArrayEnd => return Err(DecoderError::Unsupported("request array")),
        };
        let component = match (key, value) {
            (Some("messages"), value) => return self.messages.proceed(value),
            (Some("tools"), JsonToken::ValueArray(None)) => {
                return Err(DecoderError::Unsupported("tools array decoding"));
            }
            (Some("tools"), _) => return Err(DecoderError::InvalidInput("tools must be an array")),
            (Some("tool_choice"), JsonToken::ValueObject(None)) => {
                return Err(DecoderError::Unsupported("tool choice object decoding"));
            }
            (Some("stop"), value @ JsonToken::ValueArray(None)) => {
                return self.stop.proceed(value);
            }
            (Some("response_format"), JsonToken::ValueObject(None)) => {
                return Err(DecoderError::Unsupported("response format object decoding"));
            }
            (Some("response_format"), _) => {
                return Err(DecoderError::InvalidInput(
                    "response format must be an object",
                ));
            }
            (Some("provider"), JsonToken::ValueObject(None)) => {
                return Err(DecoderError::Unsupported("provider object decoding"));
            }
            (Some("provider"), _) => {
                return Err(DecoderError::InvalidInput("provider must be an object"));
            }
            (Some("stream_options"), JsonToken::ValueObject(None)) => {
                return Err(DecoderError::Unsupported("stream options object decoding"));
            }
            (Some("model"), JsonToken::ValueString(None, value)) => {
                CompatibleOpenAIRequestComponent::Model(value.to_owned())
            }
            (Some("tool_choice"), JsonToken::ValueString(None, value)) => {
                let choice = match value {
                    "auto" => ToolChoice::Auto,
                    "none" => ToolChoice::None,
                    "required" => ToolChoice::Required,
                    _ => return Err(DecoderError::InvalidInput("invalid tool choice")),
                };
                CompatibleOpenAIRequestComponent::ToolChoice(choice)
            }
            (Some("parallel_tool_calls"), JsonToken::ValueBoolean(None, value)) => {
                CompatibleOpenAIRequestComponent::ParallelToolCalls(value)
            }
            (Some("max_completion_tokens" | "max_tokens"), JsonToken::ValueNumber(None, value)) => {
                CompatibleOpenAIRequestComponent::MaxCompletionTokens(
                    value
                        .parse()
                        .map_err(|_| DecoderError::InvalidInput("invalid token limit"))?,
                )
            }
            (Some("top_p"), JsonToken::ValueNumber(None, value)) => {
                CompatibleOpenAIRequestComponent::TopP(
                    value
                        .parse()
                        .map_err(|_| DecoderError::InvalidInput("invalid top_p"))?,
                )
            }
            (Some("top_p"), JsonToken::ValueNull(None)) => return Ok(None),
            (Some("top_k"), JsonToken::ValueNumber(None, value)) => {
                CompatibleOpenAIRequestComponent::TopK(
                    value
                        .parse()
                        .map_err(|_| DecoderError::InvalidInput("invalid top_k"))?,
                )
            }
            (Some("stop"), JsonToken::ValueString(None, value)) => {
                if value.is_empty() || value.len() > 4096 {
                    return Err(DecoderError::InvalidInput("invalid stop sequence"));
                }
                CompatibleOpenAIRequestComponent::Stop(Stop::Text(value.to_owned()))
            }
            (Some("stop"), JsonToken::ValueNull(None)) => {
                CompatibleOpenAIRequestComponent::Stop(Stop::Null)
            }
            (Some("stream"), JsonToken::ValueBoolean(None, value)) => {
                CompatibleOpenAIRequestComponent::Stream(value)
            }
            (Some("stream_options"), JsonToken::ValueNull(None)) => {
                CompatibleOpenAIRequestComponent::StreamOptions(None)
            }
            (None, JsonToken::ValueObject(None)) => return Ok(None),
            _ => return Err(DecoderError::Unsupported("request field")),
        };
        Ok(Some(component.validate()?))
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
            match json.next(input, eof) {
                Ok(DecoderStatus::Component(token)) => match self.proceed(token) {
                    Ok(Some(component)) => break Ok(DecoderStatus::Component(component.into())),
                    Ok(None) => continue,
                    Err(error) => {
                        break Err(error);
                    }
                },
                Ok(DecoderStatus::NeedInput) => break Ok(DecoderStatus::NeedInput),
                Ok(DecoderStatus::Finished) => break Ok(DecoderStatus::Finished),
                Err(_) => {
                    break Err(DecoderError::InvalidInput("invalid request JSON"));
                }
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
        self.json
            .ok_or(DecoderError::InvalidState)?
            .finish()
            .map_err(|_| DecoderError::IncompleteInput)
    }
}
