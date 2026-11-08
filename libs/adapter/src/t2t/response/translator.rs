use std::{future::poll_fn, task::Poll};

use bytes::Bytes;
use futures_core::Stream;
use futures_sink::Sink;
use thiserror::Error;

use super::{decoder::ResponseDecoder, encoder::ResponseEncoder, event::ResponseEvent};
use crate::{
    codec::{DecodeStep, EncodeStep},
    error::{CodecError, StreamError},
    stream::{Codecs, Driver},
};

#[derive(Debug, Error)]
pub enum ResponseForwardError<I, T> {
    #[error("response translation failed: {0}")]
    Stream(StreamError<I>),
    #[error("response target failed: {0}")]
    Target(T),
}

pub struct Text2TextResponseTranslator {
    output_chunk_bytes: usize,
    steps_before_yield: usize,
    output: Vec<u8>,
}

impl Text2TextResponseTranslator {
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
    ) -> Result<(), ResponseForwardError<I, T::Error>>
    where
        S: Stream<Item = Result<Bytes, I>>,
        F: FnOnce() -> D,
        D: ResponseDecoder,
        T: Sink<Bytes>,
        E: ResponseEncoder,
    {
        let steps_before_yield = self.steps_before_yield;
        let mut driver = std::pin::pin!(Driver::with_output(
            source,
            ResponseCodecs {
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
            let chunk = chunk.map_err(ResponseForwardError::Stream)?;
            poll_fn(|cx| target.as_mut().poll_ready(cx))
                .await
                .map_err(ResponseForwardError::Target)?;
            target
                .as_mut()
                .start_send(chunk)
                .map_err(ResponseForwardError::Target)?;
            chunks += 1;
            if chunks == steps_before_yield {
                Self::yield_once().await;
                chunks = 0;
            }
        }
        poll_fn(|cx| target.as_mut().poll_close(cx))
            .await
            .map_err(ResponseForwardError::Target)
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
