use super::super::component::CompatibleOpenAIRequestComponent;
use crate::decoder::DecoderError;
use crate::json::JsonToken;
use crate::llm::text::component::TextRequestComponent;

#[derive(Debug, Default)]
pub(super) struct RequestState {
    phase: Phase,
    field: &'static str,
    fields: u16,
    message_count: usize,
    buffer: Vec<u8>,
    containers: Vec<Container>,
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

#[derive(Debug, Clone, Copy)]
enum ValuePhase {
    First,
    Key,
    Colon,
    Value,
    End,
}

#[derive(Debug)]
struct Container {
    object: bool,
    phase: ValuePhase,
    keys: Vec<String>,
}

impl RequestState {
    pub(super) fn invalid() -> DecoderError {
        DecoderError::InvalidInput("invalid request component")
    }

    pub(super) fn ensure_active(&self) -> Result<(), DecoderError> {
        if matches!(self.phase, Phase::Failed | Phase::Finished) {
            return Err(DecoderError::InvalidState);
        }
        Ok(())
    }

    pub(super) fn ensure_complete(&self) -> Result<(), DecoderError> {
        match self.phase {
            Phase::Closed | Phase::Finished => Ok(()),
            Phase::Failed => Err(DecoderError::InvalidState),
            _ => Err(DecoderError::IncompleteInput),
        }
    }

    pub(super) fn finish(&mut self) -> Result<(), DecoderError> {
        self.ensure_complete()?;
        self.phase = Phase::Finished;
        Ok(())
    }

    pub(super) fn fail(&mut self) {
        self.phase = Phase::Failed;
    }

    fn field(&mut self, key: &str) -> Result<(), DecoderError> {
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
            return Err(Self::invalid());
        }
        self.fields |= bit;
        self.field = field;
        self.phase = Phase::Colon;
        Ok(())
    }

    pub(super) fn accept(
        &mut self,
        token: JsonToken<'_>,
    ) -> Result<Option<TextRequestComponent>, DecoderError> {
        use JsonToken as T;
        use Phase as P;
        match (self.phase, token) {
            (P::Start, T::ObjectStart) => self.phase = P::KeyOrEnd,
            (P::KeyOrEnd | P::Key, T::String(key)) => self.field(key)?,
            (P::Colon, T::Colon) => self.phase = P::Value,
            (P::Value, T::ArrayStart) if self.field == "messages" => self.phase = P::Messages,
            (P::Value, T::Null) if self.field == "top_p" && self.buffer.is_empty() => {
                self.phase = P::End
            }
            (P::Value, token) if self.field != "messages" => {
                if self.collect(self.field, token)? {
                    self.phase = P::End;
                    return self.component().map(Some);
                }
            }
            (P::Messages | P::Message, T::ObjectStart) => {
                self.collect("message", T::ObjectStart)?;
                self.phase = P::MessageValue;
            }
            (P::MessageValue, token) => {
                if self.collect("message", token)? {
                    self.message_count += 1;
                    self.phase = P::MessageEnd;
                    return self.component().map(Some);
                }
            }
            (P::Messages | P::MessageEnd, T::ArrayEnd) => self.phase = P::End,
            (P::MessageEnd, T::Comma) => self.phase = P::Message,
            (P::End, T::Comma) => self.phase = P::Key,
            (P::KeyOrEnd | P::End, T::ObjectEnd) => {
                if self.fields & 3 != 3 || self.message_count == 0 {
                    return Err(Self::invalid());
                }
                self.phase = P::Closed;
            }
            _ => return Err(Self::invalid()),
        }
        Ok(None)
    }

    fn collect(&mut self, field: &str, token: JsonToken<'_>) -> Result<bool, DecoderError> {
        if self.buffer.is_empty() {
            self.buffer.push(b'{');
            serde_json::to_writer(&mut self.buffer, field).map_err(|_| Self::invalid())?;
            self.buffer.push(b':');
        }
        self.track(&token)?;
        self.append(token)?;
        if !self.containers.is_empty() {
            return Ok(false);
        }
        self.buffer.push(b'}');
        Ok(true)
    }

    fn component(&mut self) -> Result<TextRequestComponent, DecoderError> {
        let component = CompatibleOpenAIRequestComponent::parse(&self.buffer)?;
        self.buffer.clear();
        Ok(component.into())
    }

    fn track(&mut self, token: &JsonToken<'_>) -> Result<(), DecoderError> {
        use JsonToken as T;
        use ValuePhase as P;
        let mut value = self.containers.is_empty();
        if let Some(container) = self.containers.last_mut() {
            match (token, container.object, container.phase) {
                (T::String(key), true, P::First | P::Key) => {
                    if container.keys.iter().any(|seen| seen == key) {
                        return Err(Self::invalid());
                    }
                    container.keys.push((*key).to_owned());
                    container.phase = P::Colon;
                }
                (T::Colon, true, P::Colon) => container.phase = P::Value,
                (T::Comma, object, P::End) => {
                    container.phase = if object { P::Key } else { P::Value }
                }
                (T::ObjectEnd, true, P::First | P::End)
                | (T::ArrayEnd, false, P::First | P::End) => {
                    self.containers.pop();
                }
                (_, true, P::Value) | (_, false, P::First | P::Value) => {
                    container.phase = P::End;
                    value = true;
                }
                _ => return Err(Self::invalid()),
            }
        }
        if value {
            match token {
                T::ObjectStart | T::ArrayStart => {
                    if self.containers.len() >= 64 {
                        return Err(DecoderError::LimitExceeded("JSON nesting"));
                    }
                    self.containers.push(Container {
                        object: matches!(token, T::ObjectStart),
                        phase: P::First,
                        keys: Vec::new(),
                    });
                }
                T::String(_) | T::Number(_) | T::Boolean(_) | T::Null => {}
                _ => return Err(Self::invalid()),
            }
        }
        Ok(())
    }

    fn append(&mut self, token: JsonToken<'_>) -> Result<(), DecoderError> {
        use JsonToken as T;
        match token {
            T::String(text) => {
                serde_json::to_writer(&mut self.buffer, text).map_err(|_| Self::invalid())?
            }
            T::Number(text) => self.buffer.extend_from_slice(text.as_bytes()),
            T::Boolean(value) => {
                self.buffer
                    .extend_from_slice(if value { b"true" } else { b"false" })
            }
            T::Null => self.buffer.extend_from_slice(b"null"),
            T::ObjectStart => self.buffer.push(b'{'),
            T::ObjectEnd => self.buffer.push(b'}'),
            T::ArrayStart => self.buffer.push(b'['),
            T::ArrayEnd => self.buffer.push(b']'),
            T::Colon => self.buffer.push(b':'),
            T::Comma => self.buffer.push(b','),
        }
        Ok(())
    }
}
