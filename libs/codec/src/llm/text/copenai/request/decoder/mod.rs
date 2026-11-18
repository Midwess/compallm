mod content;
mod message;
mod provider;
mod reasoning;
mod response_format;
mod schema;
mod stop;
mod stream_options;
mod tool_calls;
mod tool_choice;
mod tools;

use super::component::CompatibleOpenAIRequestComponent;
use crate::decoder::{Decoder, DecoderError, DecoderStatus};
use crate::json::{JsonDecoder, JsonToken};
use crate::llm::text::component::{Stop, TextRequestComponent, ToolChoice};
use message::MessageState;
use provider::ProviderState;
use response_format::ResponseFormatState;
use stop::StopState;
use stream_options::StreamOptionsState;
use tool_choice::ToolChoiceState;
use tools::ToolsState;

#[derive(Debug)]
pub struct CompatibleOpenAIRequestDecoder {
    json: Option<JsonDecoder>,
    failed: bool,
    messages: MessageState,
    stop: StopState,
    provider: ProviderState,
    stream_options: StreamOptionsState,
    tool_choice: ToolChoiceState,
    tools: ToolsState,
    response_format: ResponseFormatState,
    started: bool,
    closed: bool,
    has_model: bool,
    has_messages: bool,
    has_token_limit: bool,
}

impl CompatibleOpenAIRequestDecoder {
    pub fn new(json: JsonDecoder) -> Self {
        Self {
            json: Some(json),
            failed: false,
            messages: MessageState::default(),
            stop: StopState::default(),
            provider: ProviderState::default(),
            stream_options: StreamOptionsState::default(),
            tool_choice: ToolChoiceState::default(),
            tools: ToolsState::default(),
            response_format: ResponseFormatState::default(),
            started: false,
            closed: false,
            has_model: false,
            has_messages: false,
            has_token_limit: false,
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
        if self.provider.active {
            return self.provider.proceed(token);
        }
        if self.stream_options.active {
            return self.stream_options.proceed(token);
        }
        if self.tool_choice.active {
            return self.tool_choice.proceed(token);
        }
        if self.tools.active {
            return self.tools.proceed(token);
        }
        if self.response_format.active {
            return self.response_format.proceed(token);
        }
        let (key, value) = match token {
            JsonToken::ValueString(key, value) => (key, JsonToken::ValueString(None, value)),
            JsonToken::ValueNumber(key, value) => (key, JsonToken::ValueNumber(None, value)),
            JsonToken::ValueBoolean(key, value) => (key, JsonToken::ValueBoolean(None, value)),
            JsonToken::ValueNull(key) => (key, JsonToken::ValueNull(None)),
            JsonToken::ValueObject(key) => (key, JsonToken::ValueObject(None)),
            JsonToken::ValueArray(key) => (key, JsonToken::ValueArray(None)),
            JsonToken::ObjectEnd => {
                if !self.started || !self.has_model || !self.has_messages {
                    return Err(DecoderError::InvalidInput(
                        "model and messages are required",
                    ));
                }
                self.closed = true;
                return Ok(None);
            }
            JsonToken::ArrayEnd => return Err(DecoderError::Unsupported("request array")),
        };
        let component = match (key, value) {
            (Some("messages"), value) => {
                self.has_messages = true;
                return self.messages.proceed(value);
            }
            (Some("tools"), value @ JsonToken::ValueArray(None)) => {
                return self.tools.proceed(value);
            }
            (Some("tools"), _) => return Err(DecoderError::InvalidInput("tools must be an array")),
            (Some("tool_choice"), value @ JsonToken::ValueObject(None)) => {
                return self.tool_choice.proceed(value);
            }
            (Some("stop"), value @ JsonToken::ValueArray(None)) => {
                return self.stop.proceed(value);
            }
            (Some("response_format"), value @ JsonToken::ValueObject(None)) => {
                return self.response_format.proceed(value);
            }
            (Some("response_format"), _) => {
                return Err(DecoderError::InvalidInput(
                    "response format must be an object",
                ));
            }
            (Some("provider"), value @ JsonToken::ValueObject(None)) => {
                return self.provider.proceed(value);
            }
            (Some("provider"), _) => {
                return Err(DecoderError::InvalidInput("provider must be an object"));
            }
            (Some("stream_options"), value @ JsonToken::ValueObject(None)) => {
                return self.stream_options.proceed(value);
            }
            (Some("model"), JsonToken::ValueString(None, value)) => {
                self.has_model = true;
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
                if self.has_token_limit {
                    return Err(DecoderError::InvalidInput("conflicting token limits"));
                }
                self.has_token_limit = true;
                CompatibleOpenAIRequestComponent::MaxCompletionTokens(
                    value
                        .as_u64()
                        .and_then(|value| u32::try_from(value).ok())
                        .ok_or(DecoderError::InvalidInput("invalid token limit"))?,
                )
            }
            (Some("top_p"), JsonToken::ValueNumber(None, value)) => {
                CompatibleOpenAIRequestComponent::TopP(
                    value
                        .as_f64()
                        .ok_or(DecoderError::InvalidInput("invalid top_p"))?,
                )
            }
            (Some("top_p"), JsonToken::ValueNull(None)) => return Ok(None),
            (Some("top_k"), JsonToken::ValueNumber(None, value)) => {
                CompatibleOpenAIRequestComponent::TopK(
                    value
                        .as_u64()
                        .and_then(|value| u32::try_from(value).ok())
                        .ok_or(DecoderError::InvalidInput("invalid top_k"))?,
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
            (None, JsonToken::ValueObject(None)) if !self.started => {
                self.started = true;
                return Ok(None);
            }
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
                Ok(DecoderStatus::Finished) => {
                    if !self.closed {
                        break Err(DecoderError::IncompleteInput);
                    }
                    break Ok(DecoderStatus::Finished);
                }
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
        if !self.closed {
            return Err(DecoderError::IncompleteInput);
        }
        self.json
            .ok_or(DecoderError::InvalidState)?
            .finish()
            .map_err(|_| DecoderError::IncompleteInput)
    }
}
