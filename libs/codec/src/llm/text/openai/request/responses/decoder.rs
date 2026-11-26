use super::component::{FunctionChoice, InputItem, ReasoningInput, Text, Tool};
use crate::decoder::{Decoder, DecoderError, DecoderStatus};
use crate::json::StringReference;
use crate::json::{JsonContainer, JsonDecoder, JsonToken};
use crate::llm::text::component::request::ScalarText;
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

    fn scalar<'s>(
        &mut self,
        token: JsonToken<'_>,
    ) -> Result<TextRequestComponent<&'s str>, DecoderError> {
        Ok(match token {
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
    type Component<'a> = TextRequestComponent<&'a str>;
    type Error = DecoderError;

    fn next<'a, 'input: 'a>(
        &'a mut self,
        input: &mut &'input [u8],
        eof: bool,
    ) -> Result<DecoderStatus<TextRequestComponent<&'a str>>, DecoderError> {
        if self.failed {
            return Err(DecoderError::InvalidState);
        }
        let mut json = self.json.take().ok_or(DecoderError::InvalidState)?;
        let mut captured_tool = None;
        let mut captured_scalar = None;
        let mut captured_choice = None;
        let mut captured_message = None;
        let mut captured_format = None;
        let result = (|| loop {
            if let Some(field) = self.field {
                if matches!(field, Field::Tools) {
                    let source = *input;
                    match json.capture_next(input, eof) {
                        Ok(DecoderStatus::Component(range)) => {
                            captured_tool = Some((source, range));
                            return Ok((DecoderStatus::NeedInput, json));
                        }
                        Ok(DecoderStatus::NeedInput) => {
                            return Ok((DecoderStatus::NeedInput, json));
                        }
                        Ok(DecoderStatus::Finished) => {
                            self.field = None;
                            return Ok((
                                DecoderStatus::Component(TextRequestComponent::ToolsEnded),
                                json,
                            ));
                        }
                        Err(error) => return Err(error.into()),
                    }
                }
                if matches!(field, Field::Text) {
                    let source = *input;
                    match json.capture_next(input, eof) {
                        Ok(DecoderStatus::Component(range)) => {
                            captured_format = Some((source, range));
                            self.field = None;
                            return Ok((DecoderStatus::NeedInput, json));
                        }
                        Ok(DecoderStatus::NeedInput) => {
                            return Ok((DecoderStatus::NeedInput, json));
                        }
                        Ok(DecoderStatus::Finished) => return Err(DecoderError::IncompleteInput),
                        Err(error) => return Err(error.into()),
                    }
                }
                if matches!(field, Field::Input) {
                    let source = *input;
                    match json.capture_next(input, eof) {
                        Ok(DecoderStatus::Component(range)) => {
                            captured_message = Some((source, range));
                            return Ok((DecoderStatus::NeedInput, json));
                        }
                        Ok(DecoderStatus::NeedInput) => {
                            return Ok((DecoderStatus::NeedInput, json));
                        }
                        Ok(DecoderStatus::Finished) => {
                            self.field = None;
                            continue;
                        }
                        Err(error) => return Err(error.into()),
                    }
                }
                if matches!(field, Field::ToolChoice) {
                    let source = *input;
                    match json.capture_next(input, eof) {
                        Ok(DecoderStatus::Component(range)) => {
                            captured_choice = Some((source, range));
                            self.field = None;
                            return Ok((DecoderStatus::NeedInput, json));
                        }
                        Ok(DecoderStatus::NeedInput) => {
                            return Ok((DecoderStatus::NeedInput, json));
                        }
                        Ok(DecoderStatus::Finished) => return Err(DecoderError::IncompleteInput),
                        Err(error) => return Err(error.into()),
                    }
                }
                let status = match field {
                    Field::Input => unreachable!(),
                    Field::Tools => unreachable!(),
                    Field::Text => unreachable!(),
                    Field::ToolChoice => unreachable!(),
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
            let source = *input;
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
                    if let JsonToken::ValueString(Some(key), value) = &token {
                        let kind = match *key {
                            "model" => {
                                if value.is_empty() {
                                    return Err(DecoderError::InvalidInput("invalid model"));
                                }
                                self.has_model = true;
                                Some(ScalarText::Model)
                            }
                            "input" => {
                                self.has_input = true;
                                Some(ScalarText::UserMessage)
                            }

                            _ => None,
                        };
                        if let Some(kind) = kind {
                            captured_scalar =
                                Some((kind, source, StringReference::new(value, source)));
                            return Ok((DecoderStatus::NeedInput, json));
                        }
                    }

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
                if let Some((source, range)) = captured_format {
                    let value = self
                        .json
                        .as_mut()
                        .ok_or(DecoderError::InvalidState)?
                        .deserialize_captured::<Text<'_>>(source, range)
                        .map_err(DecoderError::from)
                        .inspect_err(|_| self.failed = true)?;
                    let format = value
                        .format
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
                        .deserialize_captured::<InputItem<'_>>(source, range)
                        .map_err(DecoderError::from)
                        .inspect_err(|_| self.failed = true)?;
                    let message = value.into();
                    return Ok(DecoderStatus::Component(TextRequestComponent::Message(
                        message,
                    )));
                }
                if let Some((source, range)) = captured_choice {
                    let value = self
                        .json
                        .as_mut()
                        .ok_or(DecoderError::InvalidState)?
                        .deserialize_captured::<FunctionChoice<'_>>(source, range)
                        .map_err(DecoderError::from)
                        .inspect_err(|_| self.failed = true)?;
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
                    let tool = self
                        .json
                        .as_mut()
                        .ok_or(DecoderError::InvalidState)?
                        .deserialize_captured::<Tool<'_>>(source, range)
                        .map_err(DecoderError::from)
                        .inspect_err(|_| self.failed = true)?;
                    return Ok(DecoderStatus::Component(TextRequestComponent::Tool(
                        tool.into(),
                    )));
                }
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
