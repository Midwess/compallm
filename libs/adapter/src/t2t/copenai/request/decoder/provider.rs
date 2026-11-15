use crate::decoder::DecoderError;
use crate::json::JsonToken;
use crate::t2t::component::{Provider, TextRequestComponent};

use super::CompatibleOpenAIRequestDecoder;

/// The provider block carries the per-provider request requirements.
#[derive(Debug, Default)]
pub(super) struct ProviderState {
    phase: Phase,
    key: u8,
    fields: u8,
    require_parameter: Option<bool>,
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
    pub(super) fn provider(
        state: &mut ProviderState,
        token: JsonToken<'_>,
    ) -> Result<Option<TextRequestComponent>, DecoderError> {
        use JsonToken as T;
        let invalid = || DecoderError::InvalidInput("invalid provider block");
        match (state.phase, token) {
            (Phase::Start, T::ObjectStart) => state.phase = Phase::KeyOrEnd,
            (Phase::KeyOrEnd | Phase::Key, T::String(key)) => {
                state.key = match key {
                    "require_parameter" => 1,
                    _ => return Err(DecoderError::Unsupported("provider field")),
                };
                if state.fields & state.key != 0 {
                    return Err(invalid());
                }
                state.fields |= state.key;
                state.phase = Phase::Colon;
            }
            (Phase::Colon, T::Colon) => state.phase = Phase::Value,
            (Phase::Value, T::Boolean(value)) => {
                state.require_parameter = Some(value);
                state.phase = Phase::CommaOrEnd;
            }
            (Phase::CommaOrEnd, T::Comma) => state.phase = Phase::Key,
            (Phase::KeyOrEnd | Phase::CommaOrEnd, T::ObjectEnd) => {
                return Ok(Some(TextRequestComponent::Provider(
                    Provider::RequireParameter(state.require_parameter.unwrap_or(false)),
                )));
            }
            _ => return Err(invalid()),
        }
        Ok(None)
    }
}
