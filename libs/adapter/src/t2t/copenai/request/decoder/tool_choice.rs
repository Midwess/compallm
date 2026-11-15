use crate::decoder::DecoderError;
use crate::json::JsonToken;
use crate::t2t::component::{TextRequestComponent, ToolChoice};

use super::CompatibleOpenAIRequestDecoder;

/// The tool choice is a keyword or an object naming a function.
#[derive(Debug, Default)]
pub(super) struct ToolChoiceState {
    phase: Phase,
    function: bool,
    key: u8,
    fields: u8,
    function_fields: u8,
    name: Option<String>,
}

#[derive(Clone, Copy, Debug, Default)]
enum Phase {
    #[default]
    Start,
    KeyOrEnd,
    Key,
    Colon,
    Value,
    CommaOrEnd,
}

impl CompatibleOpenAIRequestDecoder {
    pub(super) fn tool_choice(
        state: &mut ToolChoiceState,
        token: JsonToken<'_>,
    ) -> Result<Option<TextRequestComponent>, DecoderError> {
        use JsonToken as T;
        let invalid = || DecoderError::InvalidInput("invalid tool choice");
        match (state.phase, token) {
            (Phase::Start, T::String(keyword)) => {
                let choice = match keyword {
                    "auto" => ToolChoice::Auto,
                    "none" => ToolChoice::None,
                    "required" => ToolChoice::Required,
                    _ => return Err(DecoderError::Unsupported("tool choice")),
                };
                return Ok(Some(TextRequestComponent::ToolChoice(choice)));
            }
            (Phase::Start, T::ObjectStart) => state.phase = Phase::KeyOrEnd,
            (Phase::KeyOrEnd | Phase::Key, T::String(key)) => {
                state.key = if state.function {
                    match key {
                        "name" => 1,
                        _ => return Err(DecoderError::Unsupported("tool choice function field")),
                    }
                } else {
                    match key {
                        "type" => 1,
                        "function" => 2,
                        _ => return Err(DecoderError::Unsupported("tool choice field")),
                    }
                };
                let fields = if state.function {
                    &mut state.function_fields
                } else {
                    &mut state.fields
                };
                if *fields & state.key != 0 {
                    return Err(invalid());
                }
                *fields |= state.key;
                state.phase = Phase::Colon;
            }
            (Phase::Colon, T::Colon) => state.phase = Phase::Value,
            (Phase::Value, T::ObjectStart) if !state.function && state.key == 2 => {
                state.function = true;
                state.phase = Phase::KeyOrEnd;
            }
            (Phase::Value, T::String(text)) => {
                if state.function {
                    if state.key != 1 || text.is_empty() {
                        return Err(invalid());
                    }
                    state.name = Some(text.to_owned());
                } else if state.key != 1 {
                    return Err(invalid());
                } else if text != "function" {
                    return Err(DecoderError::Unsupported("tool choice type"));
                }
                state.phase = Phase::CommaOrEnd;
            }
            (Phase::CommaOrEnd, T::Comma) => state.phase = Phase::Key,
            (Phase::KeyOrEnd | Phase::CommaOrEnd, T::ObjectEnd) => {
                if state.function {
                    if state.function_fields & 1 == 0 {
                        return Err(invalid());
                    }
                    state.function = false;
                    state.phase = Phase::CommaOrEnd;
                } else {
                    let name = state.name.take().ok_or_else(invalid)?;
                    return Ok(Some(TextRequestComponent::ToolChoice(
                        ToolChoice::Function(name),
                    )));
                }
            }
            _ => return Err(invalid()),
        }
        Ok(None)
    }
}
