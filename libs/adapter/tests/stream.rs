use std::{
    cell::Cell,
    convert::Infallible,
    marker::PhantomPinned,
    num::NonZeroUsize,
    pin::Pin,
    rc::Rc,
    sync::Arc,
    task::{Context, Poll, Wake, Waker},
};

use adapter::{
    codec::{DecodeStep, EncodeStep},
    error::{CodecError, StreamError},
    limits::TranslationLimits,
    t2t::{
        Text2TextRequestStream, Text2TextResponseStream,
        request::{decoder::RequestDecoder, encoder::RequestEncoder, event::RequestEvent},
        response::{decoder::ResponseDecoder, encoder::ResponseEncoder, event::ResponseEvent},
    },
};
use bytes::{Buf, Bytes};
use futures_core::Stream;

#[derive(Default)]
struct TextDecoder;

macro_rules! text_decoder {
    ($contract:ident, $event:ident) => {
        impl $contract for TextDecoder {
            fn advance(
                &mut self,
                input: &mut Bytes,
                eof: bool,
            ) -> Result<DecodeStep<$event>, CodecError> {
                if !input.is_empty() {
                    Ok(DecodeStep::Event($event::TextFragment(
                        input.split_to(input.len().min(4)),
                    )))
                } else if eof {
                    Ok(DecodeStep::Finished)
                } else {
                    Ok(DecodeStep::NeedInput)
                }
            }
        }
    };
}

text_decoder!(RequestDecoder, RequestEvent);
text_decoder!(ResponseDecoder, ResponseEvent);

#[derive(Default)]
struct TextEncoder {
    pending: Bytes,
    finished: bool,
}

macro_rules! text_encoder {
    ($contract:ident, $event:ident) => {
        impl $contract for TextEncoder {
            fn accept(&mut self, event: $event) -> Result<(), CodecError> {
                let $event::TextFragment(bytes) = event else {
                    return Err(CodecError::Unsupported("text fragments only"));
                };
                assert!(self.pending.is_empty());
                self.pending = bytes;
                Ok(())
            }

            fn advance(&mut self, output: &mut [u8]) -> Result<EncodeStep, CodecError> {
                if !self.pending.is_empty() {
                    let len = output.len().min(self.pending.len());
                    output[..len].copy_from_slice(&self.pending[..len]);
                    self.pending.advance(len);
                    Ok(EncodeStep::Wrote(len))
                } else if self.finished {
                    Ok(EncodeStep::Finished)
                } else {
                    Ok(EncodeStep::NeedEvent)
                }
            }

            fn finish(&mut self) -> Result<(), CodecError> {
                assert!(!self.finished);
                self.finished = true;
                self.pending = Bytes::from_static(b"!");
                Ok(())
            }
        }
    };
}

text_encoder!(RequestEncoder, RequestEvent);
text_encoder!(ResponseEncoder, ResponseEvent);

fn limits() -> TranslationLimits {
    TranslationLimits {
        output_chunk_bytes: NonZeroUsize::new(2).unwrap(),
        steps_per_poll: NonZeroUsize::new(3).unwrap(),
    }
}

fn collect<S, E>(stream: S) -> Vec<u8>
where
    S: Stream<Item = Result<Bytes, E>>,
    E: std::fmt::Debug,
{
    let mut stream = std::pin::pin!(stream);
    let mut cx = Context::from_waker(Waker::noop());
    let mut result = Vec::new();
    for _ in 0..100 {
        match stream.as_mut().poll_next(&mut cx) {
            Poll::Ready(Some(Ok(bytes))) => {
                assert!(bytes.len() <= 2);
                result.extend_from_slice(&bytes);
            }
            Poll::Ready(Some(Err(error))) => panic!("unexpected error: {error:?}"),
            Poll::Ready(None) => return result,
            Poll::Pending => {}
        }
    }
    panic!("stream did not terminate");
}

/// Non-Send, non-Unpin input with state outside the pinned value.
struct LocalInput<'a> {
    polls: &'a Cell<usize>,
    next: Rc<Cell<usize>>,
    _pinned: PhantomPinned,
}

impl Stream for LocalInput<'_> {
    type Item = Result<Bytes, Infallible>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.polls.set(self.polls.get() + 1);
        let index = self.next.get();
        self.next.set(index + 1);
        match index {
            0 => Poll::Ready(Some(Ok(Bytes::from_static(b"hello")))),
            1 => {
                cx.waker().wake_by_ref();
                Poll::Pending
            }
            2 => Poll::Ready(Some(Ok(Bytes::from_static(b" world")))),
            _ => Poll::Ready(None),
        }
    }
}

#[test]
fn request_drains_partial_events_and_finishes_without_restricting_input() {
    let polls = Cell::new(0);
    let input = LocalInput {
        polls: &polls,
        next: Rc::new(Cell::new(0)),
        _pinned: PhantomPinned,
    };
    let stream =
        Text2TextRequestStream::with_limits(input, TextDecoder, TextEncoder::default(), limits());
    assert_eq!(polls.get(), 0, "construction must not consume input");
    assert_eq!(collect(stream), b"hello world!");
    assert_eq!(polls.get(), 4);
}

#[test]
fn response_uses_its_own_contracts_and_drains_final_output() {
    let stream = Text2TextResponseStream::with_limits(
        OneChunk(Some(Ok(Bytes::from_static(b"response")))),
        TextDecoder,
        TextEncoder::default(),
        limits(),
    );
    assert_eq!(collect(stream), b"response!");
}

struct OneChunk(Option<Result<Bytes, &'static str>>);
impl Stream for OneChunk {
    type Item = Result<Bytes, &'static str>;

    fn poll_next(mut self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Poll::Ready(self.0.take())
    }
}

#[test]
fn transport_error_is_emitted_once_then_the_stream_is_terminal() {
    let mut stream = std::pin::pin!(Text2TextRequestStream::new(
        OneChunk(Some(Err("disconnected"))),
        TextDecoder,
        TextEncoder::default(),
    ));
    let mut cx = Context::from_waker(Waker::noop());
    assert!(matches!(
        stream.as_mut().poll_next(&mut cx),
        Poll::Ready(Some(Err(StreamError::Input("disconnected"))))
    ));
    assert!(matches!(
        stream.as_mut().poll_next(&mut cx),
        Poll::Ready(None)
    ));
}

struct BadEof;
impl RequestDecoder for BadEof {
    fn advance(&mut self, _: &mut Bytes, _: bool) -> Result<DecodeStep<RequestEvent>, CodecError> {
        Ok(DecodeStep::Finished)
    }
}

#[test]
fn premature_decoder_completion_is_rejected() {
    let mut stream = std::pin::pin!(Text2TextRequestStream::new(
        OneChunk(None),
        BadEof,
        TextEncoder::default(),
    ));
    let mut cx = Context::from_waker(Waker::noop());
    assert!(matches!(
        stream.as_mut().poll_next(&mut cx),
        Poll::Ready(Some(Err(StreamError::Codec(CodecError::Contract(_)))))
    ));
    assert!(matches!(
        stream.as_mut().poll_next(&mut cx),
        Poll::Ready(None)
    ));
}

struct EmptyForever(Rc<Cell<usize>>);
impl Stream for EmptyForever {
    type Item = Result<Bytes, Infallible>;

    fn poll_next(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.0.set(self.0.get() + 1);
        Poll::Ready(Some(Ok(Bytes::new())))
    }
}

struct WakeCount(std::sync::atomic::AtomicUsize);
impl Wake for WakeCount {
    fn wake(self: Arc<Self>) {
        self.0.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }
}

#[test]
fn empty_chunks_cannot_monopolize_a_poll_and_exhaustion_self_wakes() {
    let polls = Rc::new(Cell::new(0));
    let mut stream = std::pin::pin!(Text2TextRequestStream::with_limits(
        EmptyForever(polls.clone()),
        TextDecoder,
        TextEncoder::default(),
        limits(),
    ));
    let wakes = Arc::new(WakeCount(std::sync::atomic::AtomicUsize::new(0)));
    let waker = Waker::from(wakes.clone());
    let mut cx = Context::from_waker(&waker);
    assert!(stream.as_mut().poll_next(&mut cx).is_pending());
    assert_eq!(polls.get(), 3);
    assert_eq!(wakes.0.load(std::sync::atomic::Ordering::Relaxed), 1);
}

#[test]
fn pending_encoder_output_is_drained_before_polling_more_input() {
    let polls = Cell::new(0);
    let mut stream = std::pin::pin!(Text2TextRequestStream::with_limits(
        LocalInput {
            polls: &polls,
            next: Rc::new(Cell::new(0)),
            _pinned: PhantomPinned,
        },
        TextDecoder,
        TextEncoder::default(),
        TranslationLimits {
            steps_per_poll: NonZeroUsize::new(64).unwrap(),
            ..limits()
        },
    ));
    let mut cx = Context::from_waker(Waker::noop());
    for expected in [b"he".as_slice(), b"ll", b"o"] {
        let Poll::Ready(Some(Ok(bytes))) = stream.as_mut().poll_next(&mut cx) else {
            panic!("expected pending output");
        };
        assert_eq!(bytes.as_ref(), expected);
        assert_eq!(polls.get(), 1);
    }
}

struct Incomplete;
impl RequestDecoder for Incomplete {
    fn advance(
        &mut self,
        input: &mut Bytes,
        eof: bool,
    ) -> Result<DecodeStep<RequestEvent>, CodecError> {
        if !input.is_empty() {
            Ok(DecodeStep::Event(RequestEvent::TextFragment(
                input.split_to(input.len()),
            )))
        } else if eof {
            Err(CodecError::InvalidInput("incomplete document"))
        } else {
            Ok(DecodeStep::NeedInput)
        }
    }
}

#[test]
fn late_eof_failure_does_not_emit_successful_encoder_finalization() {
    let mut stream = std::pin::pin!(Text2TextRequestStream::new(
        OneChunk(Some(Ok(Bytes::from_static(b"partial")))),
        Incomplete,
        TextEncoder::default(),
    ));
    let mut cx = Context::from_waker(Waker::noop());
    let Poll::Ready(Some(Ok(bytes))) = stream.as_mut().poll_next(&mut cx) else {
        panic!("expected partial output");
    };
    assert_eq!(bytes.as_ref(), b"partial");
    assert!(matches!(
        stream.as_mut().poll_next(&mut cx),
        Poll::Ready(Some(Err(StreamError::Codec(CodecError::InvalidInput(_)))))
    ));
    assert!(matches!(
        stream.as_mut().poll_next(&mut cx),
        Poll::Ready(None)
    ));
}

struct ZeroOutput;
impl RequestEncoder for ZeroOutput {
    fn accept(&mut self, _: RequestEvent) -> Result<(), CodecError> {
        unreachable!()
    }

    fn advance(&mut self, _: &mut [u8]) -> Result<EncodeStep, CodecError> {
        Ok(EncodeStep::Wrote(0))
    }

    fn finish(&mut self) -> Result<(), CodecError> {
        unreachable!()
    }
}

#[test]
fn invalid_encoder_output_length_terminates_the_stream() {
    let mut stream = std::pin::pin!(Text2TextRequestStream::new(
        OneChunk(None),
        TextDecoder,
        ZeroOutput,
    ));
    let mut cx = Context::from_waker(Waker::noop());
    assert!(matches!(
        stream.as_mut().poll_next(&mut cx),
        Poll::Ready(Some(Err(StreamError::Codec(CodecError::Contract(_)))))
    ));
    assert!(matches!(
        stream.as_mut().poll_next(&mut cx),
        Poll::Ready(None)
    ));
}

struct ConsumeOnly;
impl RequestDecoder for ConsumeOnly {
    fn advance(
        &mut self,
        input: &mut Bytes,
        eof: bool,
    ) -> Result<DecodeStep<RequestEvent>, CodecError> {
        if !input.is_empty() {
            input.advance(1);
            Ok(DecodeStep::Progress)
        } else if eof {
            Ok(DecodeStep::Finished)
        } else {
            Ok(DecodeStep::NeedInput)
        }
    }
}

#[test]
fn decoder_can_consume_input_in_bounded_steps_without_emitting_events() {
    assert_eq!(
        collect(Text2TextRequestStream::with_limits(
            OneChunk(Some(Ok(Bytes::from_static(b"ignored")))),
            ConsumeOnly,
            TextEncoder::default(),
            limits(),
        )),
        b"!"
    );
}

// Test-only replay backend. Production backends must impose capacity limits.
#[derive(Default)]
struct StoreState {
    bytes: Vec<u8>,
    sealed: bool,
    opens: usize,
    aborts: usize,
    fail_append: bool,
    fail_open: bool,
    fail_abort: bool,
}

struct TestStore(Rc<std::cell::RefCell<StoreState>>);

impl adapter::replay::ReplayStore for TestStore {
    type Error = &'static str;
    type Reader = OneChunk;

    async fn append(&mut self, chunk: Bytes) -> Result<(), Self::Error> {
        let mut state = self.0.borrow_mut();
        if state.fail_append {
            return Err("write failed");
        }
        assert!(!state.sealed);
        state.bytes.extend_from_slice(&chunk);
        Ok(())
    }

    async fn seal(&mut self) -> Result<(), Self::Error> {
        self.0.borrow_mut().sealed = true;
        Ok(())
    }

    async fn open_reader(&mut self) -> Result<Self::Reader, Self::Error> {
        let mut state = self.0.borrow_mut();
        assert!(state.sealed);
        if state.fail_open {
            return Err("read failed");
        }
        state.opens += 1;
        Ok(OneChunk(Some(Ok(Bytes::copy_from_slice(&state.bytes)))))
    }

    async fn abort(&mut self) -> Result<(), Self::Error> {
        let mut state = self.0.borrow_mut();
        state.aborts += 1;
        if state.fail_abort {
            return Err("cleanup failed");
        }
        state.bytes.clear();
        Ok(())
    }
}

fn run<F: std::future::Future>(future: F) -> F::Output {
    let mut future = std::pin::pin!(future);
    let mut cx = Context::from_waker(Waker::noop());
    for _ in 0..100 {
        if let Poll::Ready(value) = future.as_mut().poll(&mut cx) {
            return value;
        }
    }
    panic!("future did not finish");
}

// A second concrete target type, representing a provider protocol switch.
struct OtherEncoder(TextEncoder);
impl RequestEncoder for OtherEncoder {
    fn accept(&mut self, event: RequestEvent) -> Result<(), CodecError> {
        RequestEncoder::accept(&mut self.0, event)
    }

    fn advance(&mut self, output: &mut [u8]) -> Result<EncodeStep, CodecError> {
        RequestEncoder::advance(&mut self.0, output)
    }

    fn finish(&mut self) -> Result<(), CodecError> {
        RequestEncoder::finish(&mut self.0)
    }
}

#[test]
fn translator_replays_original_bytes_with_fresh_codecs_after_partial_attempt() {
    use adapter::t2t::{Text2TextRequestTranslator, request::RequestTranslatorError};

    let state = Rc::new(std::cell::RefCell::new(StoreState::default()));
    let decoder_count = Cell::new(0);
    let mut translator = Text2TextRequestTranslator::with_limits(
        OneChunk(Some(Ok(Bytes::from_static(b"original")))),
        || {
            decoder_count.set(decoder_count.get() + 1);
            TextDecoder
        },
        TestStore(state.clone()),
        limits(),
    );
    assert!(state.borrow().bytes.is_empty());

    let mut first = Box::pin(run(translator.forward(TextEncoder::default())).unwrap());
    assert!(state.borrow().sealed);
    let mut cx = Context::from_waker(Waker::noop());
    while first.as_mut().poll_next(&mut cx).is_pending() {}
    assert!(matches!(
        run(translator.forward(OtherEncoder(TextEncoder::default()))),
        Err(RequestTranslatorError::AttemptActive)
    ));
    assert!(matches!(
        run(translator.close()),
        Err(RequestTranslatorError::AttemptActive)
    ));
    drop(first);

    let second = run(translator.forward(OtherEncoder(TextEncoder::default()))).unwrap();
    assert_eq!(collect(second), b"original!");
    assert_eq!(decoder_count.get(), 2);
    assert_eq!(state.borrow().opens, 2);
    run(translator.close()).unwrap();
    run(translator.close()).unwrap();
    assert_eq!(state.borrow().aborts, 1);
    assert!(state.borrow().bytes.is_empty());
    assert!(matches!(
        run(translator.forward(TextEncoder::default())),
        Err(RequestTranslatorError::Closed)
    ));
}

#[test]
fn failed_retention_cannot_be_reused_and_cleanup_failure_can_be_retried() {
    use adapter::t2t::{Text2TextRequestTranslator, request::RequestTranslatorError};

    let state = Rc::new(std::cell::RefCell::new(StoreState {
        fail_append: true,
        fail_abort: true,
        ..StoreState::default()
    }));
    let mut translator = Text2TextRequestTranslator::new(
        OneChunk(Some(Ok(Bytes::from_static(b"original")))),
        || TextDecoder,
        TestStore(state.clone()),
    );
    assert!(matches!(
        run(translator.forward(TextEncoder::default())),
        Err(RequestTranslatorError::Store("write failed"))
    ));
    assert!(!state.borrow().sealed);
    assert!(matches!(
        run(translator.forward(TextEncoder::default())),
        Err(RequestTranslatorError::RetentionUnavailable)
    ));
    assert!(matches!(
        run(translator.close()),
        Err(RequestTranslatorError::Store("cleanup failed"))
    ));
    assert!(matches!(
        run(translator.forward(TextEncoder::default())),
        Err(RequestTranslatorError::Closed)
    ));
    state.borrow_mut().fail_abort = false;
    run(translator.close()).unwrap();
    assert_eq!(state.borrow().aborts, 2);
}

#[test]
fn canceled_retention_is_not_mistaken_for_a_complete_request() {
    use adapter::t2t::{Text2TextRequestTranslator, request::RequestTranslatorError};

    let polls = Cell::new(0);
    let state = Rc::new(std::cell::RefCell::new(StoreState::default()));
    let mut translator = Text2TextRequestTranslator::new(
        LocalInput {
            polls: &polls,
            next: Rc::new(Cell::new(0)),
            _pinned: PhantomPinned,
        },
        || TextDecoder,
        TestStore(state.clone()),
    );
    {
        let mut attempt = std::pin::pin!(translator.forward(TextEncoder::default()));
        let mut cx = Context::from_waker(Waker::noop());
        assert!(attempt.as_mut().poll(&mut cx).is_pending());
    }
    assert_eq!(state.borrow().bytes, b"hello");
    assert!(!state.borrow().sealed);
    assert!(matches!(
        run(translator.forward(TextEncoder::default())),
        Err(RequestTranslatorError::RetentionUnavailable)
    ));
    run(translator.close()).unwrap();
}

#[test]
fn reader_open_failure_does_not_poison_successfully_sealed_retention() {
    use adapter::t2t::{Text2TextRequestTranslator, request::RequestTranslatorError};

    let state = Rc::new(std::cell::RefCell::new(StoreState {
        fail_open: true,
        ..StoreState::default()
    }));
    let mut translator = Text2TextRequestTranslator::with_limits(
        OneChunk(Some(Ok(Bytes::from_static(b"original")))),
        || TextDecoder,
        TestStore(state.clone()),
        limits(),
    );
    assert!(matches!(
        run(translator.forward(TextEncoder::default())),
        Err(RequestTranslatorError::Store("read failed"))
    ));
    state.borrow_mut().fail_open = false;
    assert_eq!(
        collect(run(translator.forward(TextEncoder::default())).unwrap()),
        b"original!"
    );
    run(translator.close()).unwrap();
}

#[test]
fn response_translator_creates_a_fresh_client_encoder_for_each_provider_body() {
    use adapter::t2t::Text2TextResponseTranslator;

    let count = Cell::new(0);
    let mut translator = Text2TextResponseTranslator::with_limits(
        || {
            count.set(count.get() + 1);
            TextEncoder::default()
        },
        limits(),
    );
    for body in [b"first".as_slice(), b"second"] {
        let stream = translator.start_attempt(
            OneChunk(Some(Ok(Bytes::copy_from_slice(body)))),
            TextDecoder,
        );
        let mut expected = body.to_vec();
        expected.push(b'!');
        assert_eq!(collect(stream), expected);
    }
    assert_eq!(count.get(), 2);
}
