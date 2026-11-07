use std::{
    pin::Pin,
    task::{Context, Poll},
};

use bytes::{Bytes, BytesMut};
use futures_core::Stream;
use pin_project_lite::pin_project;

use crate::{
    codec::{DecodeStep, EncodeStep},
    error::{CodecError, StreamError},
    limits::TranslationLimits,
};

/// Private bridge sharing the driver without unifying request/response contracts.
pub(crate) trait Codecs {
    type Event;

    fn decode(
        &mut self,
        input: &mut Bytes,
        eof: bool,
    ) -> Result<DecodeStep<Self::Event>, CodecError>;
    fn accept(&mut self, event: Self::Event) -> Result<(), CodecError>;
    fn encode(&mut self, output: &mut [u8]) -> Result<EncodeStep, CodecError>;
    fn finish(&mut self) -> Result<(), CodecError>;
}

pin_project! {
    pub(crate) struct Driver<S, C> {
        #[pin]
        input: S,
        codecs: C,
        current: Bytes,
        output: BytesMut,
        eof: bool,
        finishing: bool,
        done: bool,
        limits: TranslationLimits,
    }
}

impl<S, C> Driver<S, C> {
    pub(crate) fn new(input: S, codecs: C, limits: TranslationLimits) -> Self {
        Self {
            input,
            codecs,
            current: Bytes::new(),
            output: BytesMut::new(),
            eof: false,
            finishing: false,
            done: false,
            limits,
        }
    }
}

impl<S, C, E> Stream for Driver<S, C>
where
    S: Stream<Item = Result<Bytes, E>>,
    C: Codecs,
{
    type Item = Result<Bytes, StreamError<E>>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.project();
        if *this.done {
            return Poll::Ready(None);
        }

        // Mark terminal before propagating a failure, including input errors.
        macro_rules! fail {
            ($error:expr) => {{
                *this.done = true;
                *this.current = Bytes::new();
                this.output.clear();
                return Poll::Ready(Some(Err($error)));
            }};
        }
        macro_rules! codec {
            ($result:expr) => {
                match $result {
                    Ok(value) => value,
                    Err(error) => fail!(StreamError::Codec(error)),
                }
            };
        }

        for _ in 0..this.limits.steps_per_poll.get() {
            this.output.resize(this.limits.output_chunk_bytes.get(), 0);
            match codec!(this.codecs.encode(this.output.as_mut())) {
                EncodeStep::Wrote(len) => {
                    if len == 0 || len > this.output.len() {
                        fail!(StreamError::Codec(CodecError::Contract(
                            "encoder reported invalid output length"
                        )));
                    }
                    this.output.truncate(len);
                    return Poll::Ready(Some(Ok(this.output.split().freeze())));
                }
                EncodeStep::Finished => {
                    if !*this.finishing {
                        fail!(StreamError::Codec(CodecError::Contract(
                            "encoder finished before source validation"
                        )));
                    }
                    *this.done = true;
                    this.output.clear();
                    return Poll::Ready(None);
                }
                EncodeStep::NeedEvent => {
                    if *this.finishing {
                        fail!(StreamError::Codec(CodecError::Contract(
                            "encoder requested an event after finish"
                        )));
                    }
                }
            }
            this.output.clear();

            let input_len = this.current.len();
            match codec!(this.codecs.decode(this.current, *this.eof)) {
                DecodeStep::Event(event) => codec!(this.codecs.accept(event)),
                DecodeStep::Progress => {
                    if this.current.len() >= input_len {
                        fail!(StreamError::Codec(CodecError::Contract(
                            "decoder reported progress without consuming bytes"
                        )));
                    }
                }
                DecodeStep::Finished => {
                    if !*this.eof || !this.current.is_empty() {
                        fail!(StreamError::Codec(CodecError::Contract(
                            "decoder finished before consuming transport EOF"
                        )));
                    }
                    codec!(this.codecs.finish());
                    *this.finishing = true;
                }
                DecodeStep::NeedInput => {
                    if !this.current.is_empty() || *this.eof {
                        fail!(StreamError::Codec(CodecError::Contract(
                            "decoder requested input with unconsumed bytes or at EOF"
                        )));
                    }
                    match this.input.as_mut().poll_next(cx) {
                        Poll::Ready(Some(Ok(bytes))) => *this.current = bytes,
                        Poll::Ready(Some(Err(error))) => fail!(StreamError::Input(error)),
                        Poll::Ready(None) => *this.eof = true,
                        Poll::Pending => return Poll::Pending,
                    }
                }
            }
        }

        cx.waker().wake_by_ref();
        Poll::Pending
    }
}
