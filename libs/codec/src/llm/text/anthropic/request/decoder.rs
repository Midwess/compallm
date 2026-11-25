use super::component::*;
use crate::decoder::{Decoder, DecoderError, DecoderStatus};
use crate::json::{JsonContainer, JsonDecoder, JsonToken};
use crate::llm::text::component::*;
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
    pending: VecDeque<TextRequestComponent>,
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
            pending: VecDeque::new(),
            failed: false,
            started: false,
            closed: false,
            has_model: false,
            has_messages: false,
            has_max_tokens: false,
        }
    }

    fn scalar(
        &mut self,
        token: JsonToken<'_>,
    ) -> Result<Option<TextRequestComponent>, DecoderError> {
        let component = match token {
            JsonToken::ValueString(Some("model"), value) if !value.is_empty() => {
                self.has_model = true;
                TextRequestComponent::Model(value.to_owned())
            }
            JsonToken::ValueString(Some("system"), value) => {
                TextRequestComponent::Message(Message {
                    role: Role::System,
                    content: Some(Content::Text(value.to_owned())),
                    tool_calls: None,
                    tool_call_id: None,
                    reasoning: None,
                    reasoning_content: None,
                    reasoning_details: None,
                })
            }
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

    fn queue_output_config(&mut self, value: OutputConfigInput) {
        if let Some(effort) = value.effort {
            self.pending
                .push_back(TextRequestComponent::Reasoning(Reasoning {
                    effort: Some(effort.into()),
                    ..Reasoning::default()
                }));
        }
        if let Some(format) = value.format {
            self.pending.push_back(TextRequestComponent::ResponseFormat(
                ResponseFormat::JsonSchema(JsonSchema {
                    name: "response".to_owned(),
                    description: None,
                    strict: Some(true),
                    schema: format.schema,
                }),
            ));
        }
    }
}

impl Decoder for AnthropicRequestDecoder {
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
        if let Some(component) = self.pending.pop_front() {
            return Ok(DecoderStatus::Component(component));
        }
        let mut json = self.json.take().ok_or(DecoderError::InvalidState)?;
        let result = loop {
            if let Some(field) = self.field {
                let status = match field {
                    Field::Messages => {
                        match json.parse_next::<Input<Vec<Message>, MessagesWire>>(input, eof) {
                            Ok(DecoderStatus::Component(value)) => {
                                self.pending.extend(
                                    value
                                        .into_inner()
                                        .into_iter()
                                        .map(TextRequestComponent::Message),
                                );
                                Ok(DecoderStatus::Finished)
                            }
                            Ok(DecoderStatus::NeedInput) => Ok(DecoderStatus::NeedInput),
                            Ok(DecoderStatus::Finished) => Ok(DecoderStatus::Finished),
                            Err(e) => Err(e),
                        }
                    }
                    Field::System => json
                        .parse_next::<Input<Message, SystemWire>>(input, eof)
                        .map(|s| map_status(s, |v| TextRequestComponent::Message(v.into_inner()))),
                    Field::Tools => json
                        .parse_next::<ToolInput>(input, eof)
                        .map(|s| map_status(s, |v| TextRequestComponent::Tool(v.into()))),
                    Field::ToolChoice => json.parse_next::<ChoiceInput>(input, eof).map(|s| {
                        map_status(s, |v| {
                            let (choice, parallel) = v.into_components();
                            if let Some(parallel) = parallel {
                                self.pending
                                    .push_back(TextRequestComponent::ParallelToolCalls(parallel));
                            }
                            TextRequestComponent::ToolChoice(choice)
                        })
                    }),
                    Field::Thinking => json
                        .parse_next::<ThinkingInput>(input, eof)
                        .map(|s| map_status(s, |v| TextRequestComponent::Reasoning(v.into()))),
                    Field::Stop => json
                        .parse_next::<Vec<String>>(input, eof)
                        .map(|s| map_status(s, |v| TextRequestComponent::Stop(Stop::Array(v)))),
                    Field::OutputConfig => {
                        json.parse_next::<OutputConfigInput>(input, eof)
                            .map(|s| match s {
                                DecoderStatus::Component(v) => {
                                    self.queue_output_config(v);
                                    DecoderStatus::Finished
                                }
                                DecoderStatus::NeedInput => DecoderStatus::NeedInput,
                                DecoderStatus::Finished => DecoderStatus::Finished,
                            })
                    }
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
                        if matches!(field, Field::Messages) && !self.pending.is_empty() {
                            break Ok(DecoderStatus::Component(
                                self.pending.pop_front().expect("pending message"),
                            ));
                        }
                        self.field = None;
                        if matches!(field, Field::Tools) {
                            break Ok(DecoderStatus::Component(TextRequestComponent::ToolsEnded));
                        }
                        if let Some(component) = self.pending.pop_front() {
                            break Ok(DecoderStatus::Component(component));
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
                    | JsonToken::ValueBoolean(..)),
                )) => match self.scalar(token) {
                    Ok(Some(c)) => break Ok(DecoderStatus::Component(c)),
                    Ok(None) => continue,
                    Err(e) => break Err(e),
                },
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
        result
    }

    fn finish(self) -> Result<(), Self::Error> {
        if self.failed {
            return Err(DecoderError::InvalidState);
        }
        if !self.closed || !self.pending.is_empty() {
            return Err(DecoderError::IncompleteInput);
        }
        self.json
            .ok_or(DecoderError::InvalidState)?
            .finish()
            .map_err(|_| DecoderError::IncompleteInput)
    }
}
