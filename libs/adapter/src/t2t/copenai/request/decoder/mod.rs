mod max_completion_tokens;
mod message;
mod model;
mod parallel_tool_calls;
mod provider;
mod response_format;
mod stop;
mod stream;
mod stream_options;
mod tool_choice;
mod tools;
mod top_k;
mod top_p;

use crate::decoder::{Decoder, DecoderError, DecoderStatus};
use crate::json::{JsonDecoder, JsonStep, JsonToken};
use crate::t2t::component::TextRequestComponent;
use message::MessageState;
use provider::ProviderState;
use response_format::ResponseFormatState;
use stop::StopState;
use stream_options::StreamOptionsState;
use tool_choice::ToolChoiceState;
use tools::ToolsState;

#[derive(Debug, Default)]
pub struct CompatibleOpenAIRequestDecoder {
    json: JsonDecoder,
    phase: Phase,
    field: Field,
    fields: u16,
    message: MessageState,
    message_count: usize,
    tools: ToolsState,
    tool_choice: ToolChoiceState,
    stop: StopState,
    response_format: ResponseFormatState,
    provider: ProviderState,
    stream_options: StreamOptionsState,
}

#[derive(Clone, Copy, Debug, Default)]
enum Phase {
    #[default]
    Start,
    KeyOrEnd,
    Key,
    Colon,
    Value,
    CommaOrEnd,
    MessageOrEnd,
    Message,
    MessageValue,
    MessageCommaOrEnd,
    Tools,
    ToolChoice,
    Stop,
    ResponseFormat,
    Provider,
    StreamOptions,
    Closed,
    Finished,
    Failed,
}

#[derive(Clone, Copy, Debug, Default)]
enum Field {
    #[default]
    Model,
    Messages,
    Stream,
    ParallelToolCalls,
    MaxCompletionTokens,
    TopP,
    TopK,
    StreamOptions,
    Tools,
    ToolChoice,
    Stop,
    ResponseFormat,
    Provider,
}

impl CompatibleOpenAIRequestDecoder {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Decoder<TextRequestComponent> for CompatibleOpenAIRequestDecoder {
    fn next(
        &mut self,
        input: &mut &[u8],
    ) -> Result<DecoderStatus<TextRequestComponent>, DecoderError> {
        if matches!(self.phase, Phase::Failed) {
            return Err(DecoderError::InvalidState);
        }
        if matches!(self.phase, Phase::Finished) {
            return if input.is_empty() {
                Ok(DecoderStatus::Finished)
            } else {
                Err(DecoderError::InvalidState)
            };
        }
        let result = (|| {
            loop {
                let token = match self
                    .json
                    .next(input)
                    .map_err(|_| DecoderError::InvalidInput("invalid JSON"))?
                {
                    JsonStep::Event(token) => token,
                    JsonStep::NeedInput => return Ok(DecoderStatus::NeedInput),
                    JsonStep::Finished => return Err(DecoderError::InvalidState),
                };
                use JsonToken as T;
                let invalid = || DecoderError::InvalidInput("invalid compatible request");
                match (self.phase, token) {
                    (Phase::Start, T::ObjectStart) => self.phase = Phase::KeyOrEnd,
                    (Phase::KeyOrEnd | Phase::Key, T::String(key)) => {
                        let (field, bit) = match key {
                            "model" => (Field::Model, 1),
                            "messages" => (Field::Messages, 2),
                            "stream" => (Field::Stream, 4),
                            "parallel_tool_calls" => (Field::ParallelToolCalls, 8),
                            "max_completion_tokens" | "max_tokens" => {
                                (Field::MaxCompletionTokens, 16)
                            }
                            "top_p" => (Field::TopP, 32),
                            "top_k" => (Field::TopK, 64),
                            "stream_options" => (Field::StreamOptions, 128),
                            "tools" => (Field::Tools, 256),
                            "tool_choice" => (Field::ToolChoice, 512),
                            "stop" => (Field::Stop, 1024),
                            "response_format" => (Field::ResponseFormat, 2048),
                            "provider" => (Field::Provider, 4096),
                            _ => return Err(DecoderError::Unsupported("request field")),
                        };
                        if self.fields & bit != 0 {
                            return Err(invalid());
                        }
                        self.fields |= bit;
                        self.field = field;
                        self.phase = Phase::Colon;
                    }
                    (Phase::Colon, T::Colon) => self.phase = Phase::Value,
                    (Phase::Value, token) => {
                        self.phase = Phase::CommaOrEnd;
                        match self.field {
                            Field::Model => {
                                return Self::model(token).map(DecoderStatus::Component);
                            }
                            Field::Messages => match token {
                                T::ArrayStart => self.phase = Phase::MessageOrEnd,
                                _ => return Err(invalid()),
                            },
                            Field::Tools => match token {
                                T::ArrayStart => {
                                    self.tools = ToolsState::default();
                                    self.phase = Phase::Tools;
                                }
                                _ => return Err(invalid()),
                            },
                            Field::ToolChoice => {
                                self.tool_choice = ToolChoiceState::default();
                                if let Some(component) =
                                    Self::tool_choice(&mut self.tool_choice, token)?
                                {
                                    return Ok(DecoderStatus::Component(component));
                                }
                                self.phase = Phase::ToolChoice;
                            }
                            Field::Stop => {
                                self.stop = StopState::default();
                                if let Some(component) = Self::stop(&mut self.stop, token)? {
                                    return Ok(DecoderStatus::Component(component));
                                }
                                self.phase = Phase::Stop;
                            }
                            Field::ResponseFormat => {
                                self.response_format = ResponseFormatState::default();
                                if let Some(component) =
                                    Self::response_format(&mut self.response_format, token)?
                                {
                                    return Ok(DecoderStatus::Component(component));
                                }
                                self.phase = Phase::ResponseFormat;
                            }
                            Field::Provider => {
                                self.provider = ProviderState::default();
                                if let Some(component) = Self::provider(&mut self.provider, token)?
                                {
                                    return Ok(DecoderStatus::Component(component));
                                }
                                self.phase = Phase::Provider;
                            }
                            Field::Stream => {
                                return Self::stream(token).map(DecoderStatus::Component);
                            }
                            Field::ParallelToolCalls => {
                                return Self::parallel_tool_calls(token)
                                    .map(DecoderStatus::Component);
                            }
                            Field::MaxCompletionTokens => {
                                return Self::max_completion_tokens(token)
                                    .map(DecoderStatus::Component);
                            }
                            Field::TopK => return Self::top_k(token).map(DecoderStatus::Component),
                            Field::TopP => {
                                if let Some(component) = Self::top_p(token)? {
                                    return Ok(DecoderStatus::Component(component));
                                }
                            }
                            Field::StreamOptions => {
                                self.stream_options = StreamOptionsState::default();
                                if let Some(component) =
                                    Self::stream_options(&mut self.stream_options, token)?
                                {
                                    return Ok(DecoderStatus::Component(component));
                                }
                                self.phase = Phase::StreamOptions;
                            }
                        }
                    }
                    (Phase::StreamOptions, token) => {
                        if let Some(component) =
                            Self::stream_options(&mut self.stream_options, token)?
                        {
                            self.phase = Phase::CommaOrEnd;
                            return Ok(DecoderStatus::Component(component));
                        }
                    }
                    (Phase::Tools, token) => {
                        if let Some(component) = Self::tools(&mut self.tools, token)? {
                            self.phase = Phase::CommaOrEnd;
                            return Ok(DecoderStatus::Component(component));
                        }
                    }
                    (Phase::ToolChoice, token) => {
                        if let Some(component) = Self::tool_choice(&mut self.tool_choice, token)? {
                            self.phase = Phase::CommaOrEnd;
                            return Ok(DecoderStatus::Component(component));
                        }
                    }
                    (Phase::Stop, token) => {
                        if let Some(component) = Self::stop(&mut self.stop, token)? {
                            self.phase = Phase::CommaOrEnd;
                            return Ok(DecoderStatus::Component(component));
                        }
                    }
                    (Phase::ResponseFormat, token) => {
                        if let Some(component) =
                            Self::response_format(&mut self.response_format, token)?
                        {
                            self.phase = Phase::CommaOrEnd;
                            return Ok(DecoderStatus::Component(component));
                        }
                    }
                    (Phase::Provider, token) => {
                        if let Some(component) = Self::provider(&mut self.provider, token)? {
                            self.phase = Phase::CommaOrEnd;
                            return Ok(DecoderStatus::Component(component));
                        }
                    }
                    (Phase::MessageOrEnd | Phase::Message, T::ObjectStart) => {
                        self.message = MessageState::default();
                        self.phase = Phase::MessageValue;
                    }
                    (Phase::MessageValue, token) => {
                        if let Some(component) = Self::message(&mut self.message, token)? {
                            self.message_count += 1;
                            self.phase = Phase::MessageCommaOrEnd;
                            return Ok(DecoderStatus::Component(component));
                        }
                    }
                    (Phase::MessageCommaOrEnd, T::Comma) => self.phase = Phase::Message,
                    (Phase::MessageOrEnd | Phase::MessageCommaOrEnd, T::ArrayEnd) => {
                        self.phase = Phase::CommaOrEnd
                    }
                    (Phase::CommaOrEnd, T::Comma) => self.phase = Phase::Key,
                    (Phase::KeyOrEnd | Phase::CommaOrEnd, T::ObjectEnd) => {
                        if self.fields & 3 != 3 || self.message_count == 0 {
                            return Err(invalid());
                        }
                        self.phase = Phase::Closed;
                    }
                    _ => return Err(invalid()),
                }
            }
        })();
        if result.is_err() {
            self.phase = Phase::Failed;
        }
        result
    }

    fn finish(&mut self) -> Result<(), DecoderError> {
        if matches!(self.phase, Phase::Failed) {
            return Err(DecoderError::InvalidState);
        }
        if matches!(self.phase, Phase::Closed | Phase::Finished)
            && matches!(self.json.finish(), Ok(JsonStep::Finished))
        {
            self.phase = Phase::Finished;
            Ok(())
        } else {
            self.phase = Phase::Failed;
            Err(DecoderError::IncompleteInput)
        }
    }
}
