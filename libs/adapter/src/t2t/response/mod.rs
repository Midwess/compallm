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
use decoder::ResponseDecoder;
use encoder::ResponseEncoder;
use event::ResponseEvent;

pub mod decoder;
pub mod encoder;
pub mod event;
pub mod translator;

pub use translator::Text2TextResponseTranslator;

pin_project! {
    /// Pull-driven response translation. HTTP commitment and fallback are external.
    /// The injected codecs determine source/target JSON or SSE framing.
    pub struct Text2TextResponseStream<S, D, E> {
        #[pin]
        driver: Driver<S, ResponseCodecs<D, E>>,
    }
}

impl<S, D, E, InputError> Text2TextResponseStream<S, D, E>
where
    S: Stream<Item = Result<Bytes, InputError>>,
    D: ResponseDecoder,
    E: ResponseEncoder,
{
    pub fn new(input: S, decoder: D, encoder: E) -> Self {
        Self::with_limits(input, decoder, encoder, TranslationLimits::default())
    }

    pub fn with_limits(input: S, decoder: D, encoder: E, limits: TranslationLimits) -> Self {
        Self {
            driver: Driver::new(input, ResponseCodecs { decoder, encoder }, limits),
        }
    }
}

impl<S, D, E, InputError> Stream for Text2TextResponseStream<S, D, E>
where
    S: Stream<Item = Result<Bytes, InputError>>,
    D: ResponseDecoder,
    E: ResponseEncoder,
{
    type Item = Result<Bytes, StreamError<InputError>>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.project().driver.poll_next(cx)
    }
}

struct ResponseCodecs<D, E> {
    decoder: D,
    encoder: E,
}

impl<D: ResponseDecoder, E: ResponseEncoder> Codecs for ResponseCodecs<D, E> {
    type Event = ResponseEvent;

    fn decode(
        &mut self,
        input: &mut Bytes,
        eof: bool,
    ) -> Result<DecodeStep<ResponseEvent>, CodecError> {
        self.decoder.advance(input, eof)
    }

    fn accept(&mut self, event: ResponseEvent) -> Result<(), CodecError> {
        self.encoder.accept(event)
    }

    fn encode(&mut self, output: &mut [u8]) -> Result<EncodeStep, CodecError> {
        self.encoder.advance(output)
    }

    fn finish(&mut self) -> Result<(), CodecError> {
        self.encoder.finish()
    }
}
