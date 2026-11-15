use crate::decoder::DecoderError;
use crate::json::JsonToken;
use crate::t2t::component::{StreamOptions, TextRequestComponent};

use super::CompatibleOpenAIRequestDecoder;

#[derive(Debug, Default)]
pub(super) struct StreamOptionsState {
    phase: Phase,
    include_usage: Option<bool>,
}

#[derive(Clone, Copy, Debug, Default)]
enum Phase {
    #[default]
    Start,
    KeyOrEnd,
    Colon,
    Value,
    End,
}

impl CompatibleOpenAIRequestDecoder {
    pub(super) fn stream_options(
        state: &mut StreamOptionsState,
        token: JsonToken<'_>,
    ) -> Result<Option<TextRequestComponent>, DecoderError> {
        use JsonToken as T;
        match (state.phase, token) {
            (Phase::Start, T::Null) => {
                return Ok(Some(TextRequestComponent::StreamOptions(None)));
            }
            (Phase::Start, T::ObjectStart) => state.phase = Phase::KeyOrEnd,
            (Phase::KeyOrEnd, T::String("include_usage")) => state.phase = Phase::Colon,
            (Phase::KeyOrEnd, T::String(_)) => {
                return Err(DecoderError::Unsupported("stream_options field"));
            }
            (Phase::Colon, T::Colon) => state.phase = Phase::Value,
            (Phase::Value, T::Boolean(value)) => {
                state.include_usage = Some(value);
                state.phase = Phase::End;
            }
            (Phase::KeyOrEnd | Phase::End, T::ObjectEnd) => {
                return Ok(Some(TextRequestComponent::StreamOptions(Some(
                    StreamOptions {
                        include_usage: state.include_usage,
                    },
                ))));
            }
            _ => return Err(DecoderError::InvalidInput("invalid stream_options")),
        }
        Ok(None)
    }
}
