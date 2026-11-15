use crate::decoder::DecoderError;
use crate::json::JsonToken;
use crate::t2t::component::{JsonSchema, ResponseFormat, TextRequestComponent};

use super::CompatibleOpenAIRequestDecoder;

/// The response format is an object naming a format and, for schemas, its definition.
#[derive(Debug, Default)]
pub(super) struct ResponseFormatState {
    phase: Phase,
    schema: bool,
    key: u8,
    fields: u8,
    schema_fields: u8,
    kind: Option<Kind>,
    name: Option<String>,
    description: Option<String>,
    strict: Option<bool>,
    schema_text: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Kind {
    Text,
    JsonObject,
    JsonSchema,
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
    pub(super) fn response_format(
        state: &mut ResponseFormatState,
        token: JsonToken<'_>,
    ) -> Result<Option<TextRequestComponent>, DecoderError> {
        use JsonToken as T;
        let invalid = || DecoderError::InvalidInput("invalid response format");
        match (state.phase, token) {
            (Phase::Start, T::ObjectStart) => state.phase = Phase::KeyOrEnd,
            (Phase::KeyOrEnd | Phase::Key, T::String(key)) => {
                state.key = if state.schema {
                    match key {
                        "name" => 1,
                        "description" => 2,
                        "strict" => 4,
                        "schema" => 8,
                        _ => return Err(DecoderError::Unsupported("response schema field")),
                    }
                } else {
                    match key {
                        "type" => 1,
                        "json_schema" => 2,
                        _ => return Err(DecoderError::Unsupported("response format field")),
                    }
                };
                let fields = if state.schema {
                    &mut state.schema_fields
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
            (Phase::Value, T::ObjectStart) if !state.schema && state.key == 2 => {
                state.schema = true;
                state.phase = Phase::KeyOrEnd;
            }
            (Phase::Value, T::String(text)) => {
                if state.schema {
                    match state.key {
                        1 if !text.is_empty() => state.name = Some(text.to_owned()),
                        2 => state.description = Some(text.to_owned()),
                        8 => state.schema_text = Some(text.to_owned()),
                        _ => return Err(invalid()),
                    }
                } else if state.key != 1 {
                    return Err(invalid());
                } else {
                    state.kind = Some(match text {
                        "text" => Kind::Text,
                        "json_object" => Kind::JsonObject,
                        "json_schema" => Kind::JsonSchema,
                        _ => return Err(DecoderError::Unsupported("response format type")),
                    });
                }
                state.phase = Phase::CommaOrEnd;
            }
            (Phase::Value, T::Boolean(strict)) if state.schema && state.key == 4 => {
                state.strict = Some(strict);
                state.phase = Phase::CommaOrEnd;
            }
            (Phase::Value, T::Null) if state.schema && matches!(state.key, 2 | 4) => {
                state.phase = Phase::CommaOrEnd;
            }
            (Phase::CommaOrEnd, T::Comma) => state.phase = Phase::Key,
            (Phase::KeyOrEnd | Phase::CommaOrEnd, T::ObjectEnd) => {
                if state.schema {
                    if state.schema_fields & 9 != 9 {
                        return Err(invalid());
                    }
                    state.schema = false;
                    state.phase = Phase::CommaOrEnd;
                } else {
                    let format = match state.kind.take().ok_or_else(invalid)? {
                        Kind::Text => ResponseFormat::Text,
                        Kind::JsonObject => ResponseFormat::JsonObject,
                        Kind::JsonSchema => ResponseFormat::JsonSchema(JsonSchema {
                            name: state.name.take().ok_or_else(invalid)?,
                            description: state.description.take(),
                            strict: state.strict.take(),
                            schema: state.schema_text.take().ok_or_else(invalid)?,
                        }),
                    };
                    return Ok(Some(TextRequestComponent::ResponseFormat(format)));
                }
            }
            _ => return Err(invalid()),
        }
        Ok(None)
    }
}
