use super::component::*;
use crate::decoder::{Decoder, DecoderError, DecoderStatus};
use crate::json::RetainedString;
use crate::json::StringReference;
use crate::json::{JsonContainer, JsonDecoder, JsonToken};
use crate::llm::text::component::request::ScalarText;
use crate::llm::text::component::*;
use std::borrow::Cow;
use std::collections::VecDeque;

fn map_status<T, U>(status: DecoderStatus<T>, map: impl FnOnce(T) -> U) -> DecoderStatus<U> {
    match status {
        DecoderStatus::Component(value) => DecoderStatus::Component(map(value)),
        DecoderStatus::NeedInput => DecoderStatus::NeedInput,
        DecoderStatus::Finished => DecoderStatus::Finished,
    }
}

#[derive(Debug, Clone, Copy)]
enum Field {
    Messages,
    System,
    Tools,
    ToolChoice,
    Thinking,
    Stop,
    OutputConfig,
}

#[derive(Debug)]
pub struct AnthropicRequestDecoder {
    json: Option<JsonContainer>,
    field: Option<Field>,
    pending_parallel: Option<bool>,
    pending_messages: VecDeque<Message<RetainedString>>,
    pending_format: bool,
    pending_system: bool,
    failed: bool,
    started: bool,
    closed: bool,
    has_model: bool,
    has_messages: bool,
    has_max_tokens: bool,
}

impl AnthropicRequestDecoder {
    pub fn new(json: JsonDecoder) -> Self {
        Self {
            json: Some(JsonContainer::new(json)),
            field: None,
            pending_parallel: None,
            pending_messages: VecDeque::new(),
            pending_format: false,
            pending_system: false,
            failed: false,
            started: false,
            closed: false,
            has_model: false,
            has_messages: false,
            has_max_tokens: false,
        }
    }

    fn scalar<'s>(
        &mut self,
        token: JsonToken<'_>,
    ) -> Result<Option<TextRequestComponent<Cow<'s, str>>>, DecoderError> {
        let component = match token {
            JsonToken::ValueNumber(Some("max_tokens"), value) => {
                let value = value
                    .as_u64()
                    .and_then(|v| u32::try_from(v).ok())
                    .filter(|v| *v > 0)
                    .ok_or(DecoderError::InvalidInput("invalid max_tokens"))?;
                self.has_max_tokens = true;
                TextRequestComponent::MaxTokens(value)
            }
            JsonToken::ValueNumber(Some("top_p"), value) => {
                let value = value
                    .as_f64()
                    .filter(|v| v.is_finite() && (0.0..=1.0).contains(v))
                    .ok_or(DecoderError::InvalidInput("invalid top_p"))?;
                TextRequestComponent::TopP(value)
            }
            JsonToken::ValueNumber(Some("top_k"), value) => TextRequestComponent::TopK(
                value
                    .as_u64()
                    .and_then(|v| u32::try_from(v).ok())
                    .ok_or(DecoderError::InvalidInput("invalid top_k"))?,
            ),
            JsonToken::ValueBoolean(Some("stream"), value) => TextRequestComponent::Stream(value),
            _ => return Err(DecoderError::Unsupported("request field")),
        };
        Ok(Some(component))
    }

    fn message_component(&mut self) -> Result<TextRequestComponent<Cow<'_, str>>, DecoderError> {
        if self.pending_system {
            let json = self.json.as_mut().ok_or(DecoderError::InvalidState)?;
            self.pending_system = false;
            let value = json
                .deserialize_captured::<SystemInput<'_>>(&[], 0..0)
                .map_err(DecoderError::from)
                .inspect_err(|_| self.failed = true)?;
            return Ok(TextRequestComponent::Message(value.0));
        }
        let json = self.json.as_mut().ok_or(DecoderError::InvalidState)?;
        let layout = json.string_layout().map_err(DecoderError::from)?;
        let value = json
            .deserialize_captured::<MessagesInput<'_>>(&[], 0..0)
            .map_err(DecoderError::from)
            .inspect_err(|_| self.failed = true)?;
        let mut invalid = false;
        self.pending_messages
            .extend(value.0.into_iter().map(|message| {
                message.map_text(|text| {
                    layout
                        .locate(&text)
                        .or_else(|| {
                            (text == "anthropic").then_some(RetainedString::Literal("anthropic"))
                        })
                        .unwrap_or_else(|| {
                            invalid = true;
                            RetainedString::Literal("")
                        })
                })
            }));
        if invalid {
            self.failed = true;
            return Err(DecoderError::InvalidState);
        }
        self.queued_message()
    }

    fn queued_message(&mut self) -> Result<TextRequestComponent<Cow<'_, str>>, DecoderError> {
        let message = self
            .pending_messages
            .pop_front()
            .ok_or(DecoderError::InvalidState)?;
        let json = self.json.as_ref().ok_or(DecoderError::InvalidState)?;
        let mut invalid = false;
        let message = message.map_text(|reference| match json.resolve_retained(reference) {
            Ok(text) => Cow::Borrowed(text),
            Err(_) => {
                invalid = true;
                Cow::Borrowed("")
            }
        });
        if invalid {
            self.failed = true;
            return Err(DecoderError::InvalidState);
        }
        Ok(TextRequestComponent::Message(message))
    }

    fn output_format(&mut self) -> Result<TextRequestComponent<Cow<'_, str>>, DecoderError> {
        self.pending_format = false;
        let value = self
            .json
            .as_mut()
            .ok_or(DecoderError::InvalidState)?
            .deserialize_captured::<OutputConfigInput<'_>>(&[], 0..0)
            .map_err(DecoderError::from)
            .inspect_err(|_| self.failed = true)?;
        let format = value.format.ok_or(DecoderError::InvalidState)?;
        Ok(TextRequestComponent::ResponseFormat(
            ResponseFormat::JsonSchema(JsonSchema {
                name: Cow::Borrowed("response"),
                description: None,
                strict: Some(true),
                schema: format.schema,
            }),
        ))
    }
}

impl Decoder for AnthropicRequestDecoder {
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
        if !self.pending_messages.is_empty() {
            return self.queued_message().map(DecoderStatus::Component);
        }
        if self.pending_system {
            return self.message_component().map(DecoderStatus::Component);
        }
        if self.pending_format {
            return self.output_format().map(DecoderStatus::Component);
        }
        if let Some(parallel) = self.pending_parallel.take() {
            return Ok(DecoderStatus::Component(
                TextRequestComponent::ParallelToolCalls(parallel),
            ));
        }
        let mut json = self.json.take().ok_or(DecoderError::InvalidState)?;
        let mut captured_tool = None;
        let mut captured_scalar = None;
        let mut captured_stop = None;
        let mut captured_choice = None;
        let mut captured_message = None;
        let mut captured_config = false;
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
                if matches!(field, Field::OutputConfig) {
                    match json.capture_next(input, eof) {
                        Ok(DecoderStatus::Component(_)) => {
                            captured_config = true;
                            self.field = None;
                            break Ok(DecoderStatus::NeedInput);
                        }
                        Ok(DecoderStatus::NeedInput) => break Ok(DecoderStatus::NeedInput),
                        Ok(DecoderStatus::Finished) => break Err(DecoderError::IncompleteInput),
                        Err(error) => break Err(error.into()),
                    }
                }
                if matches!(field, Field::Messages | Field::System) {
                    let source = *input;
                    match json.capture_next(input, eof) {
                        Ok(DecoderStatus::Component(range)) => {
                            captured_message =
                                Some((source, range, matches!(field, Field::System)));
                            if matches!(field, Field::System) {
                                self.field = None;
                            }
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
                let status = match field {
                    Field::Messages | Field::System => unreachable!(),
                    Field::Tools => unreachable!(),
                    Field::ToolChoice => unreachable!(),
                    Field::Thinking => json
                        .parse_next::<ThinkingInput>(input, eof)
                        .map(|s| map_status(s, |v| TextRequestComponent::Reasoning(v.into()))),
                    Field::Stop => unreachable!(),
                    Field::OutputConfig => unreachable!(),
                };
                match status {
                    Ok(DecoderStatus::Component(component)) => {
                        if !matches!(field, Field::Tools) {
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
                    | JsonToken::ValueBoolean(..)),
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
                            "system" => Some(ScalarText::SystemMessage),

                            _ => None,
                        };
                        if let Some(kind) = kind {
                            captured_scalar =
                                Some((kind, source, StringReference::new(value, source)));
                            break Ok(DecoderStatus::NeedInput);
                        }
                    }
                    match self.scalar(token) {
                        Ok(Some(c)) => break Ok(DecoderStatus::Component(c)),
                        Ok(None) => continue,
                        Err(e) => break Err(e),
                    }
                }
                Ok(DecoderStatus::Component(JsonToken::ObjectStart(None))) if !self.started => {
                    self.started = true
                }
                Ok(DecoderStatus::Component(JsonToken::ArrayStart(Some("messages")))) => {
                    json = json.enter().map_err(DecoderError::from)?.nonempty();
                    self.has_messages = true;
                    self.field = Some(Field::Messages);
                }
                Ok(DecoderStatus::Component(JsonToken::ArrayStart(Some(key)))) => {
                    let field = match key {
                        "system" => Field::System,
                        "tools" => Field::Tools,
                        "stop_sequences" => Field::Stop,
                        _ => break Err(DecoderError::Unsupported("request array")),
                    };
                    json = if matches!(field, Field::Tools) {
                        json.enter()
                    } else {
                        json.enter_value()
                    }
                    .map_err(DecoderError::from)?;
                    self.field = Some(field);
                    if matches!(field, Field::Tools) {
                        break Ok(DecoderStatus::Component(TextRequestComponent::ToolsStarted));
                    }
                }
                Ok(DecoderStatus::Component(JsonToken::ObjectStart(Some(key)))) => {
                    let field = match key {
                        "tool_choice" => Field::ToolChoice,
                        "thinking" => Field::Thinking,
                        "output_config" => Field::OutputConfig,
                        _ => break Err(DecoderError::Unsupported("request object")),
                    };
                    json = json.enter().map_err(DecoderError::from)?;
                    self.field = Some(field);
                }
                Ok(DecoderStatus::Component(JsonToken::ObjectEnd)) => {
                    if !self.has_model || !self.has_messages || !self.has_max_tokens {
                        break Err(DecoderError::InvalidInput(
                            "model, messages, and max_tokens are required",
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
        if captured_config {
            let value = self
                .json
                .as_mut()
                .ok_or(DecoderError::InvalidState)?
                .deserialize_captured::<OutputConfigInput<'_>>(&[], 0..0)
                .map_err(DecoderError::from)
                .inspect_err(|_| self.failed = true)?;
            self.pending_format = value.format.is_some();
            if let Some(effort) = value.effort {
                return Ok(DecoderStatus::Component(TextRequestComponent::Reasoning(
                    Reasoning {
                        effort: Some(effort.into()),
                        ..Reasoning::default()
                    },
                )));
            }
            if self.pending_format {
                return self.output_format().map(DecoderStatus::Component);
            }
            return self.next(input, eof);
        }
        if let Some((source, range, system)) = captured_message {
            self.json
                .as_mut()
                .ok_or(DecoderError::InvalidState)?
                .retain_captured(source, range)
                .map_err(DecoderError::from)?;
            self.pending_system = system;
            return self.message_component().map(DecoderStatus::Component);
        }
        if let Some((source, range)) = captured_choice {
            let value = self
                .json
                .as_mut()
                .ok_or(DecoderError::InvalidState)?
                .deserialize_captured::<ChoiceInput<'_>>(source, range)
                .map_err(DecoderError::from)
                .inspect_err(|_| self.failed = true)?;
            let (choice, parallel) = value.into_components();
            if let Some(parallel) = parallel {
                self.pending_parallel = Some(parallel);
            }
            return Ok(DecoderStatus::Component(TextRequestComponent::ToolChoice(
                choice,
            )));
        }
        if let Some((source, range)) = captured_stop {
            let value = self
                .json
                .as_mut()
                .ok_or(DecoderError::InvalidState)?
                .deserialize_captured::<StopSequences<'_>>(source, range)
                .map_err(DecoderError::from)
                .inspect_err(|_| self.failed = true)?;
            return Ok(DecoderStatus::Component(TextRequestComponent::Stop(
                value.into(),
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
            let tool = self
                .json
                .as_mut()
                .ok_or(DecoderError::InvalidState)?
                .deserialize_captured::<ToolInput<'_>>(source, range)
                .map_err(DecoderError::from)
                .inspect_err(|_| self.failed = true)?;
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
        if !self.closed
            || self.pending_parallel.is_some()
            || !self.pending_messages.is_empty()
            || self.pending_format
            || self.pending_system
        {
            return Err(DecoderError::IncompleteInput);
        }
        self.json
            .ok_or(DecoderError::InvalidState)?
            .finish()
            .map_err(|_| DecoderError::IncompleteInput)
    }
}
