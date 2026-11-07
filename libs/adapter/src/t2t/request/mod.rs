use std::{
    pin::Pin,
    task::{Context, Poll},
};

use bytes::Bytes;
use futures_core::Stream;
use pin_project_lite::pin_project;

use crate::{
    codec::{DecodeStep, EncodeStep},
    error::{CodecError, StreamError},
    limits::TranslationLimits,
    stream::{Codecs, Driver},
};
use decoder::RequestDecoder;
use encoder::RequestEncoder;
use event::RequestEvent;

pub mod decoder;
pub mod encoder;
pub mod event;
pub mod translator;

pub use translator::{RequestAttemptStream, RequestTranslatorError, Text2TextRequestTranslator};

pin_project! {
    /// Pull-driven request translation with injected incremental codecs.
    /// Construction does not poll input. Replay and provider selection are external.
    pub struct Text2TextRequestStream<S, D, E> {
        #[pin]
        driver: Driver<S, RequestCodecs<D, E>>,
    }
}

impl<S, D, E, InputError> Text2TextRequestStream<S, D, E>
where
    S: Stream<Item = Result<Bytes, InputError>>,
    D: RequestDecoder,
    E: RequestEncoder,
{
    pub fn new(input: S, decoder: D, encoder: E) -> Self {
        Self::with_limits(input, decoder, encoder, TranslationLimits::default())
    }

    pub fn with_limits(input: S, decoder: D, encoder: E, limits: TranslationLimits) -> Self {
        Self {
            driver: Driver::new(input, RequestCodecs { decoder, encoder }, limits),
        }
    }
}

impl<S, D, E, InputError> Stream for Text2TextRequestStream<S, D, E>
where
    S: Stream<Item = Result<Bytes, InputError>>,
    D: RequestDecoder,
    E: RequestEncoder,
{
    type Item = Result<Bytes, StreamError<InputError>>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.project().driver.poll_next(cx)
    }
}

struct RequestCodecs<D, E> {
    decoder: D,
    encoder: E,
}

impl<D: RequestDecoder, E: RequestEncoder> Codecs for RequestCodecs<D, E> {
    type Event = RequestEvent;

    fn decode(
        &mut self,
        input: &mut Bytes,
        eof: bool,
    ) -> Result<DecodeStep<RequestEvent>, CodecError> {
        self.decoder.advance(input, eof)
    }

    fn accept(&mut self, event: RequestEvent) -> Result<(), CodecError> {
        self.encoder.accept(event)
    }

    fn encode(&mut self, output: &mut [u8]) -> Result<EncodeStep, CodecError> {
        self.encoder.advance(output)
    }

    fn finish(&mut self) -> Result<(), CodecError> {
        self.encoder.finish()
    }
}
