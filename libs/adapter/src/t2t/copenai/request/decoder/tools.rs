use crate::decoder::DecoderError;
use crate::json::JsonToken;
use crate::t2t::component::{FunctionDefinition, TextRequestComponent, ToolDefinition, ToolType};

use super::CompatibleOpenAIRequestDecoder;

/// Tool definitions arrive as an array of objects holding a nested function object.
#[derive(Debug, Default)]
pub(super) struct ToolsState {
    phase: Phase,
    function: bool,
    key: u8,
    fields: u8,
    function_fields: u8,
    kind: Option<ToolType>,
    name: Option<String>,
    description: Option<String>,
    parameters: Option<String>,
    strict: Option<bool>,
    tools: Vec<ToolDefinition>,
}

#[derive(Clone, Copy, Debug, Default)]
enum Phase {
    #[default]
    ItemOrEnd,
    Item,
    KeyOrEnd,
    Key,
    Colon,
    Value,
    CommaOrEnd,
    ItemCommaOrEnd,
}

impl CompatibleOpenAIRequestDecoder {
    pub(super) fn tools(
        state: &mut ToolsState,
        token: JsonToken<'_>,
    ) -> Result<Option<TextRequestComponent>, DecoderError> {
        use JsonToken as T;
        let invalid = || DecoderError::InvalidInput("invalid tool definition");
        match (state.phase, token) {
            (Phase::ItemOrEnd | Phase::ItemCommaOrEnd, T::ArrayEnd) => {
                return Ok(Some(TextRequestComponent::Tools(std::mem::take(
                    &mut state.tools,
                ))));
            }
            (Phase::ItemOrEnd | Phase::Item, T::ObjectStart) => {
                state.fields = 0;
                state.function_fields = 0;
                state.phase = Phase::KeyOrEnd;
            }
            (Phase::KeyOrEnd | Phase::Key, T::String(key)) => {
                state.key = if state.function {
                    match key {
                        "name" => 1,
                        "description" => 2,
                        "parameters" => 4,
                        "strict" => 8,
                        _ => return Err(DecoderError::Unsupported("tool function field")),
                    }
                } else {
                    match key {
                        "type" => 1,
                        "function" => 2,
                        _ => return Err(DecoderError::Unsupported("tool field")),
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
                    match state.key {
                        1 if !text.is_empty() => state.name = Some(text.to_owned()),
                        2 => state.description = Some(text.to_owned()),
                        4 => state.parameters = Some(text.to_owned()),
                        _ => return Err(invalid()),
                    }
                } else if state.key == 1 {
                    if text != "function" {
                        return Err(DecoderError::Unsupported("tool type"));
                    }
                    state.kind = Some(ToolType::Function);
                } else {
                    return Err(invalid());
                }
                state.phase = Phase::CommaOrEnd;
            }
            (Phase::Value, T::Boolean(strict)) if state.function && state.key == 8 => {
                state.strict = Some(strict);
                state.phase = Phase::CommaOrEnd;
            }
            (Phase::Value, T::Null) if state.function && matches!(state.key, 2 | 4 | 8) => {
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
                    if state.fields & 3 != 3 {
                        return Err(invalid());
                    }
                    let kind = state.kind.take().ok_or_else(invalid)?;
                    state.tools.push(ToolDefinition {
                        kind,
                        function: FunctionDefinition {
                            name: state.name.take().ok_or_else(invalid)?,
                            description: state.description.take(),
                            parameters: state.parameters.take(),
                            strict: state.strict.take(),
                        },
                    });
                    state.phase = Phase::ItemCommaOrEnd;
                }
            }
            (Phase::ItemCommaOrEnd, T::Comma) => state.phase = Phase::Item,
            _ => return Err(invalid()),
        }
        Ok(None)
    }
}
