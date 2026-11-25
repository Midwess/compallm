use super::component::{Input, MessageInput, ProviderWire, ReasoningWire, StreamOptionsWire};
use crate::decoder::{Decoder, DecoderError, DecoderStatus};
use crate::json::StringReference;
use crate::json::{JsonContainer, JsonDecoder, JsonToken};
use crate::llm::text::component::request::ScalarText;
use crate::llm::text::component::{
    Provider, Reasoning, Stop, StreamOptions, TextRequestComponent, ToolChoice,
};
use std::borrow::Cow;
type RequestJson = JsonContainer;

struct DecodedField<T, A, S>(DecoderStatus<Input<T, A>>, fn(T) -> TextRequestComponent<S>);

impl<T, A, S> From<DecodedField<T, A, S>> for DecoderStatus<TextRequestComponent<S>> {
    fn from(DecodedField(status, component): DecodedField<T, A, S>) -> Self {
        match status {
            DecoderStatus::Component(value) => Self::Component(component(value.into_inner())),
            DecoderStatus::NeedInput => Self::NeedInput,
            DecoderStatus::Finished => Self::Finished,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Field {
    Messages,
    Provider,
    Reasoning,
    ResponseFormat,
    StreamOptions,
    ToolChoice,
    Tools,
    Stop,
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

    fn proceed_scalar<'s>(
        &mut self,
        token: JsonToken<'_>,
    ) -> Result<Option<TextRequestComponent<Cow<'s, str>>>, DecoderError> {
        use TextRequestComponent as C;
        let component = match token {
            JsonToken::ValueBoolean(Some("stream"), value) => C::Stream(value),
            JsonToken::ValueNull(Some("stream_options")) => C::StreamOptions(None),
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
                    .ok_or(DecoderError::InvalidInput("invalid token limit"))?;
                self.has_token_limit = true;
                if limit == 0 {
                    return Err(DecoderError::InvalidInput("invalid token limit"));
                }
                C::MaxTokens(limit)
            }
            JsonToken::ValueNumber(Some("top_p"), value) => {
                let value = value
                    .as_f64()
                    .ok_or(DecoderError::InvalidInput("invalid top_p"))?;
                if !value.is_finite() || !(0.0..=1.0).contains(&value) {
                    return Err(DecoderError::InvalidInput("invalid top_p"));
                }
                C::TopP(value)
            }
            JsonToken::ValueNull(Some("top_p")) => return Ok(None),
            JsonToken::ValueNumber(Some("top_k"), value) => {
                let value = value
                    .as_u64()
                    .and_then(|value| u32::try_from(value).ok())
                    .ok_or(DecoderError::InvalidInput("invalid top_k"))?;
                C::TopK(value)
            }
            JsonToken::ValueString(Some("tool_choice"), value) => {
                let choice = match value {
                    "auto" => ToolChoice::Auto,
                    "none" => ToolChoice::None,
                    "required" => ToolChoice::Required,
                    _ => return Err(DecoderError::InvalidInput("invalid tool choice")),
                };
                C::ToolChoice(choice)
            }

            JsonToken::ValueNull(Some("stop")) => C::Stop(Stop::Null),

            _ => return Err(DecoderError::Unsupported("request field")),
        };
        Ok(Some(component))
    }
}

impl Decoder for CompatibleOpenAIRequestDecoder {
    type Component<'a> = TextRequestComponent<Cow<'a, str>>;
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
        let mut captured_tool = None;
        let mut captured_scalar = None;
        let mut captured_choice = None;
        let mut captured_message = None;
        let mut captured_format = None;
        let mut captured_stop = None;
        let result = loop {
            if let Some(field) = self.field {
                if matches!(field, Field::Tools) {
                    let source = *input;
                    match json.capture_next(input, eof) {
                        Ok(DecoderStatus::Component(range)) => {
                            captured_tool = Some((source, range));
                            break Ok(DecoderStatus::NeedInput);
                        }
                        Ok(DecoderStatus::NeedInput) => break Ok(DecoderStatus::NeedInput),
                        Ok(DecoderStatus::Finished) => {
                            self.field = None;
                            break Ok(DecoderStatus::Component(TextRequestComponent::ToolsEnded));
                        }
                        Err(error) => break Err(error.into()),
                    }
                }
                if matches!(field, Field::Stop) {
                    let source = *input;
                    match json.capture_next(input, eof) {
                        Ok(DecoderStatus::Component(range)) => {
                            captured_stop = Some((source, range));
                            self.field = None;
                            break Ok(DecoderStatus::NeedInput);
                        }
                        Ok(DecoderStatus::NeedInput) => break Ok(DecoderStatus::NeedInput),
                        Ok(DecoderStatus::Finished) => break Err(DecoderError::IncompleteInput),
                        Err(error) => break Err(error.into()),
                    }
                }
                if matches!(field, Field::ResponseFormat) {
                    let source = *input;
                    match json.capture_next(input, eof) {
                        Ok(DecoderStatus::Component(range)) => {
                            captured_format = Some((source, range));
                            self.field = None;
                            break Ok(DecoderStatus::NeedInput);
                        }
                        Ok(DecoderStatus::NeedInput) => break Ok(DecoderStatus::NeedInput),
                        Ok(DecoderStatus::Finished) => break Err(DecoderError::IncompleteInput),
                        Err(error) => break Err(error.into()),
                    }
                }
                if matches!(field, Field::Messages) {
                    let source = *input;
                    match json.capture_next(input, eof) {
                        Ok(DecoderStatus::Component(range)) => {
                            captured_message = Some((source, range));
                            break Ok(DecoderStatus::NeedInput);
                        }
                        Ok(DecoderStatus::NeedInput) => break Ok(DecoderStatus::NeedInput),
                        Ok(DecoderStatus::Finished) => {
                            self.field = None;
                            continue;
                        }
                        Err(error) => break Err(error.into()),
                    }
                }
                if matches!(field, Field::ToolChoice) {
                    let source = *input;
                    match json.capture_next(input, eof) {
                        Ok(DecoderStatus::Component(range)) => {
                            captured_choice = Some((source, range));
                            self.field = None;
                            break Ok(DecoderStatus::NeedInput);
                        }
                        Ok(DecoderStatus::NeedInput) => break Ok(DecoderStatus::NeedInput),
                        Ok(DecoderStatus::Finished) => break Err(DecoderError::IncompleteInput),
                        Err(error) => break Err(error.into()),
                    }
                }
                let status = match field {
                    Field::Messages => unreachable!(),
                    Field::Provider => json
                        .parse_next::<Input<Provider, ProviderWire>>(input, eof)
                        .map(|status| DecodedField(status, TextRequestComponent::Provider).into()),
                    Field::Reasoning => json
                        .parse_next::<Input<Reasoning, ReasoningWire>>(input, eof)
                        .map(|status| DecodedField(status, TextRequestComponent::Reasoning).into()),
                    Field::ResponseFormat => unreachable!(),
                    Field::StreamOptions => json
                        .parse_next::<Input<StreamOptions, StreamOptionsWire>>(input, eof)
                        .map(|status| {
                            DecodedField(status, |value| {
                                TextRequestComponent::StreamOptions(Some(value))
                            })
                            .into()
                        }),
                    Field::ToolChoice => unreachable!(),
                    Field::Tools => unreachable!(),
                    Field::Stop => unreachable!(),
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
            let source = *input;
            match json.next(input, eof) {
                Ok(DecoderStatus::Component(
                    token @ (JsonToken::ValueString(..)
                    | JsonToken::ValueNumber(..)
                    | JsonToken::ValueBoolean(..)
                    | JsonToken::ValueNull(..)),
                )) => {
                    if let JsonToken::ValueString(Some(key), value) = &token {
                        let kind = match *key {
                            "model" => {
                                if value.is_empty() {
                                    break Err(DecoderError::InvalidInput("invalid model"));
                                }
                                self.has_model = true;
                                Some(ScalarText::Model)
                            }
                            "stop" => {
                                if value.is_empty() || value.len() > 4096 {
                                    break Err(DecoderError::InvalidInput("invalid stop sequence"));
                                }
                                Some(ScalarText::Stop)
                            }

                            _ => None,
                        };
                        if let Some(kind) = kind {
                            captured_scalar =
                                Some((kind, source, StringReference::new(value, source)));
                            break Ok(DecoderStatus::NeedInput);
                        }
                    }
                    match self.proceed_scalar(token) {
                        Ok(Some(component)) => break Ok(DecoderStatus::Component(component)),
                        Ok(None) => continue,
                        Err(error) => break Err(error),
                    }
                }
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
                        "reasoning" => Field::Reasoning,
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
        if let Some((source, range)) = captured_stop {
            let value = self
                .json
                .as_mut()
                .ok_or(DecoderError::InvalidState)?
                .deserialize_captured::<crate::llm::text::copenai::request::component::Stops<'_>>(
                    source, range,
                )
                .map_err(DecoderError::from)
                .inspect_err(|_| self.failed = true)?;
            return Ok(DecoderStatus::Component(TextRequestComponent::Stop(
                value.0,
            )));
        }
        if let Some((source, range)) = captured_format {
            let value = self.json.as_mut().ok_or(DecoderError::InvalidState)?
                .deserialize_captured::<crate::llm::text::copenai::request::component::FormatInput<'_>>(source, range)
                .map_err(DecoderError::from).inspect_err(|_| self.failed = true)?;
            let format = value
                .try_into()
                .map_err(|_| DecoderError::InvalidInput("invalid response format"))
                .inspect_err(|_| self.failed = true)?;
            return Ok(DecoderStatus::Component(
                TextRequestComponent::ResponseFormat(format),
            ));
        }
        if let Some((source, range)) = captured_message {
            let value = self
                .json
                .as_mut()
                .ok_or(DecoderError::InvalidState)?
                .deserialize_captured::<MessageInput<'_>>(source, range)
                .map_err(DecoderError::from)
                .inspect_err(|_| self.failed = true)?;
            let message = value
                .try_into()
                .map_err(|_| DecoderError::InvalidInput("invalid message"))
                .inspect_err(|_| self.failed = true)?;
            return Ok(DecoderStatus::Component(TextRequestComponent::Message(
                message,
            )));
        }
        if let Some((source, range)) = captured_choice {
            let value=self.json.as_mut().ok_or(DecoderError::InvalidState)?
                .deserialize_captured::<crate::llm::text::copenai::request::component::FunctionChoice<'_>>(source,range)
                .map_err(DecoderError::from).inspect_err(|_|self.failed=true)?;
            let choice = value.into();
            return Ok(DecoderStatus::Component(TextRequestComponent::ToolChoice(
                choice,
            )));
        }
        if let Some((kind, source, reference)) = captured_scalar {
            let text = self
                .json
                .as_ref()
                .ok_or(DecoderError::InvalidState)?
                .resolve_string(source, reference)
                .map_err(DecoderError::from)?;
            return Ok(DecoderStatus::Component((kind, text).into()));
        }
        if let Some((source, range)) = captured_tool {
            let tool = self.json.as_mut().ok_or(DecoderError::InvalidState)?
                .deserialize_captured::<crate::llm::text::copenai::request::component::ChatTool<'_>>(source, range)
                .map_err(DecoderError::from).inspect_err(|_| self.failed = true)?;
            return Ok(DecoderStatus::Component(TextRequestComponent::Tool(
                tool.into(),
            )));
        }
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
