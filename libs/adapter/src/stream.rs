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
    pub(crate) struct Driver<'a, S, C> {
        #[pin]
        input: S,
        codecs: C,
        current: Bytes,
        output: &'a mut [u8],
        eof: bool,
        finishing: bool,
        done: bool,
        output_chunk_bytes: usize,
        steps_before_yield: usize,
    }
}

impl<'a, S, C> Driver<'a, S, C> {
    pub(crate) fn with_output(
        input: S,
        codecs: C,
        output_chunk_bytes: usize,
        steps_before_yield: usize,
        output: &'a mut [u8],
    ) -> Self {
        Self {
            input,
            codecs,
            current: Bytes::new(),
            output,
            eof: false,
            finishing: false,
            done: false,
            output_chunk_bytes,
            steps_before_yield,
        }
    }
}

impl<S, C, E> Stream for Driver<'_, S, C>
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

        for _ in 0..*this.steps_before_yield {
            debug_assert_eq!(this.output.len(), *this.output_chunk_bytes);
            this.output.fill(0);
            match codec!(this.codecs.encode(this.output)) {
                EncodeStep::Wrote(len) => {
                    if len == 0 || len > *this.output_chunk_bytes {
                        fail!(StreamError::Codec(CodecError::Contract(
                            "encoder reported invalid output length"
                        )));
                    }
                    return Poll::Ready(Some(Ok(Bytes::copy_from_slice(&this.output[..len]))));
                }
                EncodeStep::Finished => {
                    if !*this.finishing {
                        fail!(StreamError::Codec(CodecError::Contract(
                            "encoder finished before source validation"
                        )));
                    }
                    *this.done = true;
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
