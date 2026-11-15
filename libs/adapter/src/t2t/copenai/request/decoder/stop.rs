use crate::decoder::DecoderError;
use crate::json::JsonToken;
use crate::t2t::component::{Stop, TextRequestComponent};

use super::CompatibleOpenAIRequestDecoder;

/// Stop sequences arrive as a string, an array of strings, or null.
#[derive(Debug, Default)]
pub(super) struct StopState {
    phase: Phase,
    values: Vec<String>,
}

#[derive(Clone, Copy, Debug, Default)]
enum Phase {
    #[default]
    Start,
    Item,
    ItemCommaOrEnd,
}

impl CompatibleOpenAIRequestDecoder {
    pub(super) fn stop(
        state: &mut StopState,
        token: JsonToken<'_>,
    ) -> Result<Option<TextRequestComponent>, DecoderError> {
        use JsonToken as T;
        let invalid = || DecoderError::InvalidInput("invalid stop sequences");
        match (state.phase, token) {
            (Phase::Start, T::String(text)) => {
                return Ok(Some(TextRequestComponent::Stop(Stop::Text(
                    text.to_owned(),
                ))));
            }
            (Phase::Start, T::Null) => {
                return Ok(Some(TextRequestComponent::Stop(Stop::Null)));
            }
            (Phase::Start, T::ArrayStart) => state.phase = Phase::Item,
            (Phase::Item, T::String(text)) => {
                state.values.push(text.to_owned());
                state.phase = Phase::ItemCommaOrEnd;
            }
            (Phase::ItemCommaOrEnd, T::Comma) => state.phase = Phase::Item,
            (Phase::Item | Phase::ItemCommaOrEnd, T::ArrayEnd) => {
                return Ok(Some(TextRequestComponent::Stop(Stop::Array(
                    std::mem::take(&mut state.values),
                ))));
            }
            _ => return Err(invalid()),
        }
        Ok(None)
    }
}
