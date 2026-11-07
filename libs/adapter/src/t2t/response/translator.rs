use bytes::Bytes;
use futures_core::Stream;

use super::{Text2TextResponseStream, decoder::ResponseDecoder, encoder::ResponseEncoder};
use crate::limits::TranslationLimits;

/// Keeps client response encoding configuration across provider attempts. Each
/// call uses fresh state; responses are not retained or replayed. The caller
/// selects the source decoder using the actual provider format and owns HTTP
/// commitment, cancellation, and fallback eligibility.
pub struct Text2TextResponseTranslator<F> {
    encoder_factory: F,
    limits: TranslationLimits,
}

impl<F> Text2TextResponseTranslator<F> {
    pub fn new(encoder_factory: F) -> Self {
        Self::with_limits(encoder_factory, TranslationLimits::default())
    }

    pub fn with_limits(encoder_factory: F, limits: TranslationLimits) -> Self {
        Self {
            encoder_factory,
            limits,
        }
    }

    pub fn start_attempt<S, D, E, I>(
        &mut self,
        input: S,
        decoder: D,
    ) -> Text2TextResponseStream<S, D, E>
    where
        S: Stream<Item = Result<Bytes, I>>,
        D: ResponseDecoder,
        E: ResponseEncoder,
        F: FnMut() -> E,
    {
        Text2TextResponseStream::with_limits(input, decoder, (self.encoder_factory)(), self.limits)
    }
}
