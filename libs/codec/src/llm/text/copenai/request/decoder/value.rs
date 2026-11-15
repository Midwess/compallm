use crate::decoder::DecoderError;
use crate::json::JsonToken;
use serde_json::{Map, Value};

#[derive(Debug, Default)]
pub(super) struct ValueBuilder {
    stack: Vec<Frame>,
}

#[derive(Debug)]
struct Frame {
    value: Value,
    phase: Phase,
    key: Option<String>,
}

#[derive(Debug, Clone, Copy)]
enum Phase {
    First,
    Key,
    Colon,
    Value,
    End,
}

impl ValueBuilder {
    #[inline]
    pub(super) fn invalid() -> DecoderError {
        DecoderError::InvalidInput("invalid component JSON")
    }

    pub(super) fn push(&mut self, token: JsonToken<'_>) -> Result<Option<Value>, DecoderError> {
        use JsonToken as T;
        if let Some(frame) = self.stack.last_mut() {
            match (&frame.value, frame.phase, &token) {
                (Value::Object(_), Phase::First | Phase::Key, T::String(key)) => {
                    frame.key = Some((*key).to_owned());
                    frame.phase = Phase::Colon;
                    return Ok(None);
                }
                (Value::Object(_), Phase::Colon, T::Colon) => {
                    frame.phase = Phase::Value;
                    return Ok(None);
                }
                (_, Phase::End, T::Comma) => {
                    frame.phase = if frame.value.is_object() {
                        Phase::Key
                    } else {
                        Phase::Value
                    };
                    return Ok(None);
                }
                (Value::Object(_), Phase::First | Phase::End, T::ObjectEnd)
                | (Value::Array(_), Phase::First | Phase::End, T::ArrayEnd) => {
                    let value = self.stack.pop().ok_or_else(Self::invalid)?.value;
                    return self.attach(value);
                }
                (Value::Object(_), Phase::Value, _)
                | (Value::Array(_), Phase::First | Phase::Value, _) => {}
                _ => return Err(Self::invalid()),
            }
        }
        let value = match token {
            T::ObjectStart | T::ArrayStart => {
                if self.stack.len() >= 64 {
                    return Err(DecoderError::LimitExceeded("JSON nesting"));
                }
                self.stack.push(Frame {
                    value: if matches!(token, T::ObjectStart) {
                        Value::Object(Map::new())
                    } else {
                        Value::Array(Vec::new())
                    },
                    phase: Phase::First,
                    key: None,
                });
                return Ok(None);
            }
            T::String(text) => Value::String(text.to_owned()),
            T::Number(text) => Value::Number(text.parse().map_err(|_| Self::invalid())?),
            T::Boolean(value) => Value::Bool(value),
            T::Null => Value::Null,
            _ => return Err(Self::invalid()),
        };
        self.attach(value)
    }

    fn attach(&mut self, value: Value) -> Result<Option<Value>, DecoderError> {
        let Some(frame) = self.stack.last_mut() else {
            return Ok(Some(value));
        };
        match &mut frame.value {
            Value::Object(map) => {
                let key = frame.key.take().ok_or_else(Self::invalid)?;
                if map.insert(key, value).is_some() {
                    return Err(Self::invalid());
                }
            }
            Value::Array(array) => array.push(value),
            _ => return Err(Self::invalid()),
        }
        frame.phase = Phase::End;
        Ok(None)
    }
}
