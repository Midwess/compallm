use std::{future::poll_fn, task::Poll};

use bytes::Bytes;
use futures_core::Stream;
use futures_sink::Sink;
use thiserror::Error;

use super::{decoder::RequestDecoder, encoder::RequestEncoder, event::RequestEvent};
use crate::{
    codec::{DecodeStep, EncodeStep},
    error::{CodecError, StreamError},
    stream::{Codecs, Driver},
};

#[derive(Debug, Error)]
pub enum RequestForwardError<I, T> {
    #[error("request translation failed: {0}")]
    Stream(StreamError<I>),
    #[error("request target failed: {0}")]
    Target(T),
}

pub struct Text2TextRequestTranslator {
    output_chunk_bytes: usize,
    steps_before_yield: usize,
    output: Vec<u8>,
}

impl Text2TextRequestTranslator {
    pub fn new(mut output_chunk_bytes: usize, mut steps_before_yield: usize) -> Self {
        if output_chunk_bytes == 0 {
            output_chunk_bytes = 16 * 1024;
        }
        if steps_before_yield == 0 {
            steps_before_yield = 64;
        }
        Self {
            output_chunk_bytes,
            steps_before_yield,
            output: vec![0; output_chunk_bytes],
        }
    }

    pub async fn forward<S, F, D, I, T, E>(
        &mut self,
        source: S,
        decoder_factory: F,
        target: T,
        target_encoder: E,
    ) -> Result<(), RequestForwardError<I, T::Error>>
    where
        S: Stream<Item = Result<Bytes, I>>,
        F: FnOnce() -> D,
        D: RequestDecoder,
        T: Sink<Bytes>,
        E: RequestEncoder,
    {
        let steps_before_yield = self.steps_before_yield;
        let mut driver = std::pin::pin!(Driver::with_output(
            source,
            RequestCodecs {
                decoder: decoder_factory(),
                encoder: target_encoder,
            },
            self.output_chunk_bytes,
            steps_before_yield,
            &mut self.output,
        ));
        let mut target = std::pin::pin!(target);
        let mut chunks = 0;
        while let Some(chunk) = poll_fn(|cx| driver.as_mut().poll_next(cx)).await {
            let chunk = chunk.map_err(RequestForwardError::Stream)?;
            poll_fn(|cx| target.as_mut().poll_ready(cx))
                .await
                .map_err(RequestForwardError::Target)?;
            target
                .as_mut()
                .start_send(chunk)
                .map_err(RequestForwardError::Target)?;
            chunks += 1;
            if chunks == steps_before_yield {
                Self::yield_once().await;
                chunks = 0;
            }
        }
        poll_fn(|cx| target.as_mut().poll_close(cx))
            .await
            .map_err(RequestForwardError::Target)
    }

    async fn yield_once() {
        let mut yielded = false;
        poll_fn(|cx| {
            if yielded {
                Poll::Ready(())
            } else {
                yielded = true;
                cx.waker().wake_by_ref();
                Poll::Pending
            }
        })
        .await
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
