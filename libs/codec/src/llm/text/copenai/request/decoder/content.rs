use crate::json::{JsonDecodeError, JsonToken};
use crate::llm::text::component::{Content, ObjectMessage};

#[derive(Debug, Default)]
pub(super) struct ContentState {
    pub(super) active: bool,
    array: bool,
    current: Option<TextPart>,
    parts: Vec<ObjectMessage>,
}

#[derive(Debug, Default)]
struct TextPart {
    has_type: bool,
    text: Option<String>,
}

impl ContentState {
    pub(super) fn proceed(
        &mut self,
        token: JsonToken<'_>,
    ) -> Result<Option<Content>, JsonDecodeError> {
        let state = self;
        if !state.active {
            match token {
                JsonToken::ValueString(_, value) => {
                    return Ok(Some(Content::Text(value.to_owned())));
                }
                JsonToken::ValueNull(_) => return Ok(Some(Content::Null)),
                JsonToken::ValueArray(_) => state.array = true,
                JsonToken::ValueObject(_) => {
                    state.array = false;
                    state.current = Some(TextPart::default());
                }
                _ => return Err(JsonDecodeError::InvalidToken),
            }
            state.active = true;
            return Ok(None);
        }
        if let Some(part) = &mut state.current {
            match token {
                JsonToken::ValueString(Some("type"), "text") => {
                    if part.has_type {
                        return Err(JsonDecodeError::InvalidToken);
                    }
                    part.has_type = true;
                }
                JsonToken::ValueString(Some("text"), text) => {
                    if part.text.is_some() {
                        return Err(JsonDecodeError::InvalidToken);
                    }
                    part.text = Some(text.to_owned());
                }
                JsonToken::ObjectEnd => {
                    let part = state.current.take().ok_or(JsonDecodeError::InvalidState)?;
                    if !part.has_type {
                        return Err(JsonDecodeError::InvalidToken);
                    }
                    let part = ObjectMessage::Text {
                        text: part.text.ok_or(JsonDecodeError::InvalidToken)?,
                    };
                    if state.array {
                        state.parts.push(part);
                    } else {
                        state.active = false;
                        return Ok(Some(Content::Object(part)));
                    }
                }
                _ => return Err(JsonDecodeError::InvalidToken),
            }
            return Ok(None);
        }
        match token {
            JsonToken::ValueObject(None) if state.array => {
                state.current = Some(TextPart::default());
                Ok(None)
            }
            JsonToken::ArrayEnd if state.array => {
                state.active = false;
                Ok(Some(Content::Array(std::mem::take(&mut state.parts))))
            }
            _ => Err(JsonDecodeError::InvalidToken),
        }
    }
}
