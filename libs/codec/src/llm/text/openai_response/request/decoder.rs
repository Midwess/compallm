use super::component::{
    EffortWire, FunctionChoice, MessageInput, ReasoningInput, ResponseTools, TextInput,
};
use crate::decoder::{Decoder, DecoderError, DecoderStatus};
use crate::json::{JsonDecoder, JsonToken};
use crate::llm::text::component::{
    Content, Message, Reasoning, Role, TextRequestComponent, ToolChoice,
};
use crate::llm::text::request::decoder::{CaptureMode, Ready, RequestStep, TextRequestDecoder};
#[derive(Clone, Copy, Debug)]
enum Field {
    Messages,
    Tools,
    ResponseFormat,
    Reasoning,
    ToolChoice,
}

#[derive(Debug)]
pub struct OpenAIResponseRequestDecoder {
    request: TextRequestDecoder<Field>,
    have_token_limit: bool,
    have_model: bool,
    have_messages: bool,
}

impl OpenAIResponseRequestDecoder {
    pub fn new(json: JsonDecoder) -> Self {
        Self {
            request: TextRequestDecoder::new(json),
            have_token_limit: false,
            have_model: false,
            have_messages: false,
        }
    }
}

impl Decoder for OpenAIResponseRequestDecoder {
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
                            "input" => Field::Messages,
                            "tools" => Field::Tools,
                            _ => return Err(DecoderError::Unsupported("request array")),
                        },
                        JsonToken::ObjectStart(Some(key)) => match key {
                            "text" => Field::ResponseFormat,
                            "reasoning" => Field::Reasoning,
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
                    JsonToken::ValueString(Some("input"), text) => {
                        self.have_messages = true;
                        C::Message(Message {
                            role: Role::User,
                            content: Some(Content::Text(text)),
                            tool_calls: None,
                            tool_call_id: None,
                            reasoning: None,
                            reasoning_content: None,
                            reasoning_details: None,
                        })
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
                    JsonToken::ValueNumber(Some("max_output_tokens"), value) => {
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
                    JsonToken::ValueString(Some("model"), _) => {
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
                Field::Tools => C::Tools(json.deserialize_current::<ResponseTools<'_>>(source)?.0),
                Field::ResponseFormat => C::ResponseFormat(
                    json.deserialize_current::<TextInput<'_>>(source)?
                        .try_into()
                        .map_err(DecoderError::InvalidInput)?,
                ),
                Field::Reasoning => {
                    let value = json.deserialize_current::<ReasoningInput<'_>>(source)?;
                    C::Reasoning(Reasoning {
                        effort: Some(
                            EffortWire::parse(value.effort)
                                .ok_or(DecoderError::InvalidInput("invalid reasoning effort"))?,
                        ),
                        ..Default::default()
                    })
                }
                Field::ToolChoice => C::ToolChoice(
                    json.deserialize_current::<FunctionChoice<'_>>(source)?
                        .into(),
                ),
            },
        };
        self.request.complete();
        Ok(DecoderStatus::Component(component))
    }

    fn finish(self) -> Result<(), Self::Error> {
        self.request.finish()
    }
}
