mod value;

use crate::decoder::{Decoder, DecoderError, DecoderStatus};
use crate::json::{JsonDecoder, JsonToken};
use crate::llm::text::component::TextRequestComponent;
use serde::de::DeserializeOwned;
use serde_json::Value;
use value::ValueBuilder;

#[derive(Debug)]
pub struct CompatibleOpenAIRequestDecoder {
    json: JsonDecoder,
    phase: Phase,
    field: &'static str,
    fields: u16,
    value: ValueBuilder,
    message_count: usize,
}

#[derive(Debug, Default, Clone, Copy)]
enum Phase {
    #[default]
    Start,
    KeyOrEnd,
    Key,
    Colon,
    Value,
    End,
    Messages,
    Message,
    MessageValue,
    MessageEnd,
    Closed,
    Finished,
    Failed,
}

impl CompatibleOpenAIRequestDecoder {
    fn decode<T: DeserializeOwned>(value: Value) -> Result<T, DecoderError> {
        serde_json::from_value(value).map_err(|_| ValueBuilder::invalid())
    }

    fn component(field: &str, value: Value) -> Result<Option<TextRequestComponent>, DecoderError> {
        use TextRequestComponent as C;
        Ok(Some(match field {
            "model" => {
                let model: String = Self::decode(value)?;
                if model.is_empty() {
                    return Err(ValueBuilder::invalid());
                }
                C::Model(model)
            }
            "message" => C::Message(Self::decode(value)?),
            "tools" => C::Tools(Self::decode(value)?),
            "tool_choice" => C::ToolChoice(Self::decode(value)?),
            "parallel_tool_calls" => C::ParallelToolCalls(Self::decode(value)?),
            "max_completion_tokens" => {
                let count = Self::decode(value)?;
                if count == 0 {
                    return Err(ValueBuilder::invalid());
                }
                C::MaxCompletionTokens(count)
            }
            "top_k" => C::TopK(Self::decode(value)?),
            "top_p" => {
                if value.is_null() {
                    return Ok(None);
                }
                let p: f64 = Self::decode(value)?;
                if !p.is_finite() || !(0.0..=1.0).contains(&p) {
                    return Err(ValueBuilder::invalid());
                }
                C::TopP(p)
            }
            "stream" => C::Stream(Self::decode(value)?),
            "stream_options" => C::StreamOptions(Self::decode(value)?),
            "provider" => C::Provider(Self::decode(value)?),
            "stop" => C::Stop(Self::decode(value)?),
            "response_format" => C::ResponseFormat(Self::decode(value)?),
            _ => return Err(DecoderError::Unsupported("request field")),
        }))
    }

    pub fn new(json: JsonDecoder) -> Self {
        Self {
            json,
            phase: Phase::Start,
            field: "",
            fields: 0,
            value: ValueBuilder::default(),
            message_count: 0,
        }
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
        if matches!(self.phase, Phase::Failed | Phase::Finished) {
            return Err(DecoderError::InvalidState);
        }
        let result = (|| loop {
            let token = match self
                .json
                .next(input, eof)
                .map_err(|_| ValueBuilder::invalid())?
            {
                DecoderStatus::Component(token) => token,
                DecoderStatus::NeedInput => return Ok(DecoderStatus::NeedInput),
                DecoderStatus::Finished => {
                    if !matches!(self.phase, Phase::Closed) {
                        return Err(DecoderError::IncompleteInput);
                    }
                    self.phase = Phase::Finished;
                    return Ok(DecoderStatus::Finished);
                }
            };
            use JsonToken as T;
            use Phase as P;
            match (self.phase, token) {
                (P::Start, T::ObjectStart) => self.phase = P::KeyOrEnd,
                (P::KeyOrEnd | P::Key, T::String(key)) => {
                    let (field, bit) = match key {
                        "model" => ("model", 1),
                        "messages" => ("messages", 2),
                        "tools" => ("tools", 4),
                        "tool_choice" => ("tool_choice", 8),
                        "parallel_tool_calls" => ("parallel_tool_calls", 16),
                        "max_completion_tokens" | "max_tokens" => ("max_completion_tokens", 32),
                        "top_p" => ("top_p", 64),
                        "top_k" => ("top_k", 128),
                        "stop" => ("stop", 256),
                        "response_format" => ("response_format", 512),
                        "provider" => ("provider", 1024),
                        "stream" => ("stream", 2048),
                        "stream_options" => ("stream_options", 4096),
                        _ => return Err(DecoderError::Unsupported("request field")),
                    };
                    if self.fields & bit != 0 {
                        return Err(ValueBuilder::invalid());
                    }
                    self.fields |= bit;
                    self.field = field;
                    self.phase = P::Colon;
                }
                (P::Colon, T::Colon) => self.phase = P::Value,
                (P::Value, T::ArrayStart) if self.field == "messages" => self.phase = P::Messages,
                (P::Value, token) if self.field != "messages" => {
                    if let Some(value) = self.value.push(token)? {
                        self.phase = P::End;
                        if let Some(component) = Self::component(self.field, value)? {
                            return Ok(DecoderStatus::Component(component));
                        }
                    }
                }
                (P::Messages | P::Message, T::ObjectStart) => {
                    self.value.push(T::ObjectStart)?;
                    self.phase = P::MessageValue;
                }
                (P::MessageValue, token) => {
                    if let Some(value) = self.value.push(token)? {
                        let component =
                            Self::component("message", value)?.ok_or_else(ValueBuilder::invalid)?;
                        self.message_count += 1;
                        self.phase = P::MessageEnd;
                        return Ok(DecoderStatus::Component(component));
                    }
                }
                (P::Messages | P::MessageEnd, T::ArrayEnd) => self.phase = P::End,
                (P::MessageEnd, T::Comma) => self.phase = P::Message,
                (P::End, T::Comma) => self.phase = P::Key,
                (P::KeyOrEnd | P::End, T::ObjectEnd) => {
                    if self.fields & 3 != 3 || self.message_count == 0 {
                        return Err(ValueBuilder::invalid());
                    }
                    self.phase = P::Closed;
                }
                _ => return Err(ValueBuilder::invalid()),
            }
        })();
        if result.is_err() {
            self.phase = Phase::Failed;
        }
        result
    }

    fn finish(self) -> Result<(), Self::Error> {
        if matches!(self.phase, Phase::Failed) {
            return Err(DecoderError::InvalidState);
        }
        if !matches!(self.phase, Phase::Closed | Phase::Finished) {
            return Err(DecoderError::IncompleteInput);
        }
        self.json.finish().map_err(|_| ValueBuilder::invalid())
    }
}
