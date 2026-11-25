use super::component::{InputItem, ReasoningInput, Text, Tool, ToolChoiceInput};
use crate::decoder::{Decoder, DecoderError, DecoderStatus};
use crate::json::{JsonContainer, JsonDecoder, JsonToken};
use crate::llm::text::component::*;

#[derive(Debug, Clone, Copy)]
enum Field {
    Input,
    Tools,
    Text,
    ToolChoice,
    Reasoning,
}

#[derive(Debug)]
pub struct OpenAIResponseRequestDecoder {
    json: Option<JsonContainer>,
    field: Option<Field>,
    started: bool,
    closed: bool,
    failed: bool,
    has_model: bool,
    has_input: bool,
}

impl OpenAIResponseRequestDecoder {
    pub fn new(json: JsonDecoder) -> Self {
        Self {
            json: Some(JsonContainer::new(json)),
            field: None,
            started: false,
            closed: false,
            failed: false,
            has_model: false,
            has_input: false,
        }
    }

    fn scalar(&mut self, token: JsonToken<'_>) -> Result<TextRequestComponent, DecoderError> {
        Ok(match token {
            JsonToken::ValueString(Some("model"), value) if !value.is_empty() => {
                self.has_model = true;
                TextRequestComponent::Model(value.into())
            }
            JsonToken::ValueString(Some("input"), value) => {
                self.has_input = true;
                TextRequestComponent::Message(Message::from(InputItem::Message {
                    role: super::component::InputRole::User,
                    content: value.into(),
                }))
            }
            JsonToken::ValueBoolean(Some("stream"), value) => TextRequestComponent::Stream(value),
            JsonToken::ValueBoolean(Some("parallel_tool_calls"), value) => {
                TextRequestComponent::ParallelToolCalls(value)
            }
            JsonToken::ValueNumber(Some("max_output_tokens"), value) => {
                let value = value
                    .as_u64()
                    .and_then(|v| u32::try_from(v).ok())
                    .filter(|v| *v > 0)
                    .ok_or(DecoderError::InvalidInput("invalid token limit"))?;
                TextRequestComponent::MaxTokens(value)
            }
            JsonToken::ValueNumber(Some("top_p"), value) => {
                let value = value
                    .as_f64()
                    .filter(|v| v.is_finite() && (0.0..=1.0).contains(v))
                    .ok_or(DecoderError::InvalidInput("invalid top_p"))?;
                TextRequestComponent::TopP(value)
            }
            JsonToken::ValueString(Some("tool_choice"), value) => {
                TextRequestComponent::ToolChoice(match value {
                    "auto" => ToolChoice::Auto,
                    "none" => ToolChoice::None,
                    "required" => ToolChoice::Required,
                    _ => return Err(DecoderError::Unsupported("tool choice")),
                })
            }
            _ => return Err(DecoderError::Unsupported("Responses request field")),
        })
    }
}

impl Decoder for OpenAIResponseRequestDecoder {
    type Component<'a> = TextRequestComponent;
    type Error = DecoderError;

    fn next<'a, 'input: 'a>(
        &'a mut self,
        input: &mut &'input [u8],
        eof: bool,
    ) -> Result<DecoderStatus<TextRequestComponent>, DecoderError> {
        if self.failed {
            return Err(DecoderError::InvalidState);
        }
        let mut json = self.json.take().ok_or(DecoderError::InvalidState)?;
        let result = (|| loop {
            if let Some(field) = self.field {
                let status = match field {
                    Field::Input => json
                        .parse_next::<InputItem>(input, eof)?
                        .map_component(|v| TextRequestComponent::Message(v.into())),
                    Field::Tools => json
                        .parse_next::<Tool>(input, eof)?
                        .map_component(|v| TextRequestComponent::Tool(v.into())),
                    Field::Text => {
                        let status = json.parse_next::<Text>(input, eof)?;
                        match status {
                            DecoderStatus::Component(value) => {
                                let format = value.format.try_into().map_err(|_| {
                                    DecoderError::InvalidInput("invalid response format")
                                })?;
                                DecoderStatus::Component(TextRequestComponent::ResponseFormat(
                                    format,
                                ))
                            }
                            DecoderStatus::NeedInput => DecoderStatus::NeedInput,
                            DecoderStatus::Finished => DecoderStatus::Finished,
                        }
                    }
                    Field::ToolChoice => json
                        .parse_next::<ToolChoiceInput>(input, eof)?
                        .map_component(|v| TextRequestComponent::ToolChoice(v.into())),
                    Field::Reasoning => json
                        .parse_next::<ReasoningInput>(input, eof)?
                        .map_component(|v| TextRequestComponent::Reasoning(v.into())),
                };
                match status {
                    DecoderStatus::Finished => {
                        self.field = None;
                        if matches!(field, Field::Tools) {
                            return Ok((
                                DecoderStatus::Component(TextRequestComponent::ToolsEnded),
                                json,
                            ));
                        }
                        continue;
                    }
                    DecoderStatus::Component(_)
                        if !matches!(field, Field::Input | Field::Tools) =>
                    {
                        self.field = None
                    }
                    _ => {}
                }
                return Ok((status, json));
            }
            match json.next(input, eof)? {
                DecoderStatus::Component(JsonToken::ObjectStart(None)) if !self.started => {
                    self.started = true
                }
                DecoderStatus::Component(JsonToken::ArrayStart(Some("input"))) => {
                    self.has_input = true;
                    json = json.enter()?.nonempty();
                    self.field = Some(Field::Input);
                }
                DecoderStatus::Component(JsonToken::ArrayStart(Some("tools"))) => {
                    json = json.enter()?;
                    self.field = Some(Field::Tools);
                    return Ok((
                        DecoderStatus::Component(TextRequestComponent::ToolsStarted),
                        json,
                    ));
                }
                DecoderStatus::Component(JsonToken::ObjectStart(Some("text"))) => {
                    json = json.enter()?;
                    self.field = Some(Field::Text);
                }
                DecoderStatus::Component(JsonToken::ObjectStart(Some("tool_choice"))) => {
                    json = json.enter()?;
                    self.field = Some(Field::ToolChoice);
                }
                DecoderStatus::Component(JsonToken::ObjectStart(Some("reasoning"))) => {
                    json = json.enter()?;
                    self.field = Some(Field::Reasoning);
                }
                DecoderStatus::Component(JsonToken::ObjectEnd) => {
                    if !self.has_model || !self.has_input {
                        return Err(DecoderError::InvalidInput("model and input are required"));
                    }
                    self.closed = true;
                }
                DecoderStatus::Component(JsonToken::ValueNull(Some("top_p"))) => {}
                DecoderStatus::Component(token) => {
                    return self
                        .scalar(token)
                        .map(|value| (DecoderStatus::Component(value), json));
                }
                DecoderStatus::NeedInput => return Ok((DecoderStatus::NeedInput, json)),
                DecoderStatus::Finished if self.closed => {
                    return Ok((DecoderStatus::Finished, json));
                }
                DecoderStatus::Finished => return Err(DecoderError::IncompleteInput),
            }
        })();
        match result {
            Ok((status, json)) => {
                self.json = Some(json);
                Ok(status)
            }
            Err(error) => {
                self.failed = true;
                Err(error)
            }
        }
    }

    fn finish(self) -> Result<(), DecoderError> {
        if self.failed {
            return Err(DecoderError::InvalidState);
        }
        if !self.closed {
            return Err(DecoderError::IncompleteInput);
        }
        self.json
            .ok_or(DecoderError::InvalidState)?
            .finish()
            .map_err(Into::into)
    }
}

trait MapComponent<T> {
    fn map_component<U>(self, f: impl FnOnce(T) -> U) -> DecoderStatus<U>;
}

impl<T> MapComponent<T> for DecoderStatus<T> {
    fn map_component<U>(self, f: impl FnOnce(T) -> U) -> DecoderStatus<U> {
        match self {
            Self::Component(value) => DecoderStatus::Component(f(value)),
            Self::NeedInput => DecoderStatus::NeedInput,
            Self::Finished => DecoderStatus::Finished,
        }
    }
}
