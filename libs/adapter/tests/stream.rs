use std::{
    cell::Cell,
    convert::Infallible,
    marker::PhantomPinned,
    pin::Pin,
    rc::Rc,
    sync::Arc,
    task::{Context, Poll, Wake, Waker},
};

use adapter::{
    codec::{DecodeStep, EncodeStep},
    error::{CodecError, StreamError},
    t2t::{
        Text2TextRequestTranslator, Text2TextResponseTranslator,
        request::{
            RequestForwardError, decoder::RequestDecoder, encoder::RequestEncoder,
            event::RequestEvent,
        },
        response::{
            ResponseForwardError, decoder::ResponseDecoder, encoder::ResponseEncoder,
            event::ResponseEvent,
        },
    },
};
use bytes::{Buf, Bytes};
use futures_core::Stream;
use futures_sink::Sink;

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
fn request_scratch_is_reused_after_success_target_failure_and_cancellation() {
    struct ObserveOutput {
        encoder: TextEncoder,
        buffers: Rc<std::cell::RefCell<Vec<(usize, usize)>>>,
    }
    impl RequestEncoder for ObserveOutput {
        fn accept(&mut self, event: RequestEvent) -> Result<(), CodecError> {
            RequestEncoder::accept(&mut self.encoder, event)
        }

        fn advance(&mut self, output: &mut [u8]) -> Result<EncodeStep, CodecError> {
            self.buffers
                .borrow_mut()
                .push((output.as_ptr() as usize, output.len()));
            RequestEncoder::advance(&mut self.encoder, output)
        }

        fn finish(&mut self) -> Result<(), CodecError> {
            RequestEncoder::finish(&mut self.encoder)
        }
    }
    let buffers = Rc::new(std::cell::RefCell::new(Vec::new()));
    let mut translator = Text2TextRequestTranslator::new(2, 3);
    for outcome in 0..4 {
        let target = TestSink::default();
        target.state.borrow_mut().fail_ready = outcome == 1;
        let forward = translator.forward(
            OneChunk(Some(Ok(Bytes::from_static(b"original")))),
            || TextDecoder,
            target.pinned(),
            ObserveOutput {
                encoder: TextEncoder::default(),
                buffers: buffers.clone(),
            },
        );
        if outcome == 2 {
            let mut forward = std::pin::pin!(forward);
            assert!(
                forward
                    .as_mut()
                    .poll(&mut Context::from_waker(Waker::noop()))
                    .is_pending()
            );
        } else if outcome == 1 {
            assert!(matches!(
                run(forward),
                Err(RequestForwardError::Target("ready failed"))
            ));
        } else {
            run(forward).unwrap();
            assert_eq!(target.bytes(), b"original!");
        }
    }
    let buffers = buffers.borrow();
    assert!(!buffers.is_empty());
    assert!(buffers.iter().all(|buffer| *buffer == buffers[0]));
    assert_eq!(buffers[0].1, 2);
}

#[test]
fn retained_output_is_independent_of_scratch_reuse() {
    let mut translator = Text2TextRequestTranslator::new(2, 3);
    let mut retained = Vec::new();
    for body in [b"first".as_slice(), b"second", b"third"] {
        let mut chunks = Vec::new();
        let target = futures_util::sink::unfold(&mut chunks, |chunks, chunk: Bytes| async move {
            chunks.push(chunk);
            Ok::<_, Infallible>(chunks)
        });
        run(translator.forward(
            OneChunk(Some(Ok(Bytes::copy_from_slice(body)))),
            || TextDecoder,
            target,
            TextEncoder::default(),
        ))
        .unwrap();
        assert!(chunks.iter().all(|chunk| chunk.len() <= 2));
        retained.push(chunks);
    }
    for (chunks, expected) in retained
        .iter()
        .zip([b"first!".as_slice(), b"second!", b"third!"])
    {
        let output: Vec<u8> = chunks
            .iter()
            .flat_map(|chunk| chunk.iter().copied())
            .collect();
        assert_eq!(output, expected);
    }
}

#[test]
fn forwarding_is_lazy_until_first_poll() {
    let polls = Cell::new(0);
    let decoders = Cell::new(0);
    let mut translator = Text2TextRequestTranslator::new(2, 3);
    let target = TestSink::default();
    let input = LocalInput {
        polls: &polls,
        next: Rc::new(Cell::new(0)),
        _pinned: PhantomPinned,
    };
    let forward = translator.forward(
        input,
        || {
            decoders.set(decoders.get() + 1);
            TextDecoder
        },
        target.pinned(),
        TextEncoder::default(),
    );
    assert_eq!(polls.get(), 0);
    assert_eq!(decoders.get(), 0);
    assert!(target.bytes().is_empty());
    assert!(!target.state.borrow().closed);
    run(forward).unwrap();
    assert_eq!(target.bytes(), b"hello world!");
    assert_eq!(decoders.get(), 1);
}

#[test]
fn zero_constructor_bounds_use_defaults_in_both_directions() {
    for (output_chunk_bytes, steps_before_yield, expected_chunks) in
        [(0, 3, 3), (2, 0, 4), (0, 0, 3)]
    {
        let mut translator =
            Text2TextRequestTranslator::new(output_chunk_bytes, steps_before_yield);
        let target = TestSink::default();
        run(translator.forward(
            OneChunk(Some(Ok(Bytes::from_static(b"source")))),
            || TextDecoder,
            target.pinned(),
            TextEncoder::default(),
        ))
        .unwrap();
        assert_eq!(target.bytes(), b"source!");
        assert_eq!(target.state.borrow().chunks, expected_chunks);
        let mut translator =
            Text2TextResponseTranslator::new(output_chunk_bytes, steps_before_yield);
        let target = TestSink::default();
        run(translator.forward(
            OneChunk(Some(Ok(Bytes::from_static(b"source")))),
            || TextDecoder,
            target.pinned(),
            TextEncoder::default(),
        ))
        .unwrap();
        assert_eq!(target.bytes(), b"source!");
        assert_eq!(target.state.borrow().chunks, expected_chunks);
    }
}

#[test]
fn constructor_configures_limits() {
    let mut translator = Text2TextRequestTranslator::new(2, 3);
    let target = TestSink::default();
    run(translator.forward(
        OneChunk(Some(Ok(Bytes::from_static(b"source")))),
        || TextDecoder,
        target.pinned(),
        TextEncoder::default(),
    ))
    .unwrap();
    assert_eq!(target.bytes(), b"source!");
    assert_eq!(target.state.borrow().chunks, 4);
}

#[test]
fn constructor_output_bound_controls_chunk_size() {
    let mut translator = Text2TextRequestTranslator::new(1, 3);
    let first = TestSink::default();
    run(translator.forward(
        OneChunk(Some(Ok(Bytes::from_static(b"first")))),
        || TextDecoder,
        first.pinned(),
        TextEncoder::default(),
    ))
    .unwrap();
    assert_eq!(first.bytes(), b"first!");
    assert_eq!(first.state.borrow().chunks, 6);
    let mut translator = Text2TextRequestTranslator::new(2, 3);
    let next = TestSink::default();
    run(translator.forward(
        OneChunk(Some(Ok(Bytes::from_static(b"next")))),
        || TextDecoder,
        next.pinned(),
        TextEncoder::default(),
    ))
    .unwrap();
    assert_eq!(next.bytes(), b"next!");
    assert_eq!(next.state.borrow().chunks, 3);
}

#[test]
fn request_drains_partial_events_and_finishes_without_restricting_input() {
    let polls = Cell::new(0);
    let input = LocalInput {
        polls: &polls,
        next: Rc::new(Cell::new(0)),
        _pinned: PhantomPinned,
    };
    let mut translator = Text2TextRequestTranslator::new(2, 3);
    assert_eq!(polls.get(), 0, "construction must not consume input");
    let target = TestSink::default();
    run(translator.forward(
        input,
        || TextDecoder,
        target.pinned(),
        TextEncoder::default(),
    ))
    .unwrap();
    assert_eq!(target.bytes(), b"hello world!");
    assert!(target.state.borrow().closed);
    assert_eq!(polls.get(), 4);
}

#[test]
fn response_uses_its_own_contracts_and_drains_final_output() {
    let mut translator = Text2TextResponseTranslator::new(2, 3);
    let target = TestSink::default();
    run(translator.forward(
        OneChunk(Some(Ok(Bytes::from_static(b"response")))),
        || TextDecoder,
        target.pinned(),
        TextEncoder::default(),
    ))
    .unwrap();
    assert_eq!(target.bytes(), b"response!");
    assert!(target.state.borrow().closed);
}

struct OneChunk(Option<Result<Bytes, &'static str>>);
impl Stream for OneChunk {
    type Item = Result<Bytes, &'static str>;

    fn poll_next(mut self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Poll::Ready(self.0.take())
    }
}

#[test]
fn response_transport_failure_does_not_close_target_and_allows_reuse() {
    let mut translator = Text2TextResponseTranslator::new(16 * 1024, 64);
    let target = TestSink::default();
    assert!(matches!(
        run(translator.forward(
            OneChunk(Some(Err("disconnected"))),
            || TextDecoder,
            target.pinned(),
            TextEncoder::default(),
        )),
        Err(ResponseForwardError::Stream(StreamError::Input(
            "disconnected"
        )))
    ));
    assert!(!target.state.borrow().closed);
    assert!(target.bytes().is_empty());
    run(translator.forward(
        OneChunk(Some(Ok(Bytes::from_static(b"fresh")))),
        || TextDecoder,
        target.pinned(),
        TextEncoder::default(),
    ))
    .unwrap();
    assert_eq!(target.bytes(), b"fresh!");
}

struct BadEof;
impl RequestDecoder for BadEof {
    fn advance(&mut self, _: &mut Bytes, _: bool) -> Result<DecodeStep<RequestEvent>, CodecError> {
        Ok(DecodeStep::Finished)
    }
}

#[test]
fn premature_decoder_completion_is_rejected() {
    let (result, target) = request_attempt(
        OneChunk(None),
        BadEof,
        TextEncoder::default(),
        16 * 1024,
        64,
    );
    assert!(matches!(
        result,
        Err(RequestForwardError::Stream(StreamError::Codec(
            CodecError::Contract(_)
        )))
    ));
    assert!(!target.state.borrow().closed);
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
    let mut translator = Text2TextResponseTranslator::new(2, 3);
    let mut forward = std::pin::pin!(translator.forward(
        EmptyForever(polls.clone()),
        || TextDecoder,
        TestSink::default().pinned(),
        TextEncoder::default(),
    ));
    let wakes = Arc::new(WakeCount(std::sync::atomic::AtomicUsize::new(0)));
    let waker = Waker::from(wakes.clone());
    let mut cx = Context::from_waker(&waker);
    assert!(forward.as_mut().poll(&mut cx).is_pending());
    assert_eq!(polls.get(), 3);
    assert_eq!(wakes.0.load(std::sync::atomic::Ordering::Relaxed), 1);
}

#[test]
fn pending_encoder_output_is_drained_before_polling_more_input() {
    let polls = Cell::new(0);
    let mut translator = Text2TextResponseTranslator::new(2, 64);
    let mut chunks = Vec::new();
    let target = futures_util::sink::unfold(&mut chunks, |chunks, chunk: Bytes| {
        if chunks.len() < 3 {
            assert_eq!(polls.get(), 1);
        }
        chunks.push(chunk);
        std::future::ready(Ok::<_, Infallible>(chunks))
    });
    run(translator.forward(
        LocalInput {
            polls: &polls,
            next: Rc::new(Cell::new(0)),
            _pinned: PhantomPinned,
        },
        || TextDecoder,
        target,
        TextEncoder::default(),
    ))
    .unwrap();
    for (chunk, expected) in chunks.iter().zip([b"he".as_slice(), b"ll", b"o"]) {
        assert_eq!(chunk.as_ref(), expected);
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
    let (result, target) = request_attempt(
        OneChunk(Some(Ok(Bytes::from_static(b"partial")))),
        Incomplete,
        TextEncoder::default(),
        16 * 1024,
        64,
    );
    assert_eq!(target.bytes(), b"partial");
    assert!(matches!(
        result,
        Err(RequestForwardError::Stream(StreamError::Codec(
            CodecError::InvalidInput(_)
        )))
    ));
    assert!(!target.state.borrow().closed);
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
    let (result, target) = request_attempt(OneChunk(None), TextDecoder, ZeroOutput, 16 * 1024, 64);
    assert!(matches!(
        result,
        Err(RequestForwardError::Stream(StreamError::Codec(
            CodecError::Contract(_)
        )))
    ));
    assert!(target.bytes().is_empty());
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
    let (result, target) = request_attempt(
        OneChunk(Some(Ok(Bytes::from_static(b"ignored")))),
        ConsumeOnly,
        TextEncoder::default(),
        2,
        3,
    );
    result.unwrap();
    assert_eq!(target.bytes(), b"!");
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

fn request_attempt<S, D, E, I>(
    input: S,
    decoder: D,
    encoder: E,
    output_chunk_bytes: usize,
    steps_before_yield: usize,
) -> (Result<(), RequestForwardError<I, &'static str>>, TestSink)
where
    S: Stream<Item = Result<Bytes, I>>,
    I: std::fmt::Debug,
    D: RequestDecoder,
    E: RequestEncoder,
{
    let mut translator = Text2TextRequestTranslator::new(output_chunk_bytes, steps_before_yield);
    let target = TestSink::default();
    let result = run(translator.forward(input, || decoder, target.pinned(), encoder));
    (result, target)
}

#[derive(Default)]
struct SinkState {
    bytes: Vec<u8>,
    chunks: usize,
    closed: bool,
    blocked: bool,
    waker: Option<Waker>,
    fail_ready: bool,
    fail_send: bool,
    fail_close: bool,
}

#[derive(Clone, Default)]
struct TestSink {
    state: Rc<std::cell::RefCell<SinkState>>,
    _pinned: PhantomPinned,
}

impl TestSink {
    fn pinned(&self) -> Pin<Box<Self>> {
        Box::pin(self.clone())
    }
    fn bytes(&self) -> Vec<u8> {
        self.state.borrow().bytes.clone()
    }

    fn unblock(&self) {
        let mut state = self.state.borrow_mut();
        state.blocked = false;
        if let Some(waker) = state.waker.take() {
            waker.wake();
        }
    }
}

impl Sink<Bytes> for TestSink {
    type Error = &'static str;

    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        let mut state = self.state.borrow_mut();
        if state.fail_ready {
            Poll::Ready(Err("ready failed"))
        } else if state.blocked {
            state.waker = Some(cx.waker().clone());
            Poll::Pending
        } else {
            Poll::Ready(Ok(()))
        }
    }

    fn start_send(self: Pin<&mut Self>, bytes: Bytes) -> Result<(), Self::Error> {
        let mut state = self.state.borrow_mut();
        if state.fail_send {
            return Err("send failed");
        }
        assert!(!state.closed);
        state.bytes.extend_from_slice(&bytes);
        state.chunks += 1;
        Ok(())
    }

    fn poll_flush(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn poll_close(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        let mut state = self.state.borrow_mut();
        if state.fail_close {
            return Poll::Ready(Err("close failed"));
        }
        state.closed = true;
        Poll::Ready(Ok(()))
    }
}

#[test]
fn blocked_target_holds_only_one_output_chunk_and_resumes() {
    let decoded = Rc::new(Cell::new(0));
    struct CountDecoder(Rc<Cell<usize>>);
    impl RequestDecoder for CountDecoder {
        fn advance(
            &mut self,
            input: &mut Bytes,
            eof: bool,
        ) -> Result<DecodeStep<RequestEvent>, CodecError> {
            let result = RequestDecoder::advance(&mut TextDecoder, input, eof)?;
            if matches!(result, DecodeStep::Event(_)) {
                self.0.set(self.0.get() + 1);
            }
            Ok(result)
        }
    }
    let mut translator = Text2TextRequestTranslator::new(2, 64);
    let target = TestSink::default();
    target.state.borrow_mut().blocked = true;
    {
        let mut forward = std::pin::pin!(translator.forward(
            OneChunk(Some(Ok(Bytes::from_static(b"original")))),
            || CountDecoder(decoded.clone()),
            target.pinned(),
            TextEncoder::default()
        ));
        let mut cx = Context::from_waker(Waker::noop());
        for _ in 0..3 {
            assert!(forward.as_mut().poll(&mut cx).is_pending());
        }
        assert_eq!(decoded.get(), 1);
        assert!(target.bytes().is_empty());
        target.unblock();
        run(forward).unwrap();
    }
    assert_eq!(target.bytes(), b"original!");
    assert!(target.state.borrow().closed);
}

#[test]
fn target_failures_are_reported_and_fresh_input_can_be_forwarded() {
    for (field, expected) in [(0, "ready failed"), (1, "send failed"), (2, "close failed")] {
        let mut translator = Text2TextRequestTranslator::new(2, 3);
        let target = TestSink::default();
        {
            let mut state = target.state.borrow_mut();
            match field {
                0 => state.fail_ready = true,
                1 => state.fail_send = true,
                _ => state.fail_close = true,
            }
        }
        assert!(
            matches!(run(translator.forward(OneChunk(Some(Ok(Bytes::from_static(b"original")))), || TextDecoder, target.pinned(), TextEncoder::default())),
            Err(RequestForwardError::Target(error)) if error == expected)
        );
        assert!(!target.state.borrow().closed);
        let retry = TestSink::default();
        run(translator.forward(
            OneChunk(Some(Ok(Bytes::from_static(b"retry")))),
            || TextDecoder,
            retry.pinned(),
            TextEncoder::default(),
        ))
        .unwrap();
        assert_eq!(retry.bytes(), b"retry!");
        assert!(retry.state.borrow().closed);
    }
}

#[tokio::test]
async fn bounded_channel_target_drains_concurrently_and_ends_on_success() {
    use futures_util::StreamExt;

    let mut translator = Text2TextRequestTranslator::new(2, 3);
    let (target, mut receiver) = futures_channel::mpsc::channel::<Bytes>(1);
    let forward = translator.forward(
        OneChunk(Some(Ok(Bytes::from_static(b"original")))),
        || TextDecoder,
        target,
        TextEncoder::default(),
    );
    let consume = async {
        let mut bytes = Vec::new();
        while let Some(chunk) = receiver.next().await {
            assert!(chunk.len() <= 2);
            bytes.extend_from_slice(&chunk);
        }
        bytes
    };
    let (result, bytes) = tokio::time::timeout(
        std::time::Duration::from_secs(2),
        futures_util::future::join(forward, consume),
    )
    .await
    .expect("forwarding must not deadlock a bounded target");
    result.unwrap();
    assert_eq!(bytes, b"original!");
}

#[test]
fn source_failure_is_reported_without_successful_close() {
    let mut translator = Text2TextRequestTranslator::new(2, 3);
    let target = TestSink::default();
    assert!(matches!(
        run(translator.forward(
            OneChunk(Some(Err("reader failed"))),
            || TextDecoder,
            target.pinned(),
            TextEncoder::default()
        )),
        Err(RequestForwardError::Stream(StreamError::Input(
            "reader failed"
        )))
    ));
    assert!(!target.state.borrow().closed);
}

#[test]
fn ready_source_and_target_still_yield_between_output_batches() {
    let mut translator = Text2TextRequestTranslator::new(1, 4);
    let target = TestSink::default();
    let wakes = Arc::new(WakeCount(std::sync::atomic::AtomicUsize::new(0)));
    let waker = Waker::from(wakes.clone());
    {
        let mut forward = std::pin::pin!(translator.forward(
            OneChunk(Some(Ok(Bytes::from_static(b"original")))),
            || TextDecoder,
            target.pinned(),
            TextEncoder::default()
        ));
        let mut cx = Context::from_waker(&waker);
        assert!(forward.as_mut().poll(&mut cx).is_pending());
        assert_eq!(target.state.borrow().chunks, 4);
        assert_eq!(wakes.0.load(std::sync::atomic::Ordering::Relaxed), 1);
        run(forward).unwrap();
    }
    assert!(target.state.borrow().closed);
}

#[test]
fn late_source_failure_preserves_partial_output_without_finalization() {
    let mut translator = Text2TextRequestTranslator::new(16 * 1024, 64);
    let input =
        futures_util::stream::iter([Ok(Bytes::from_static(b"partial")), Err("disconnected")]);
    let target = TestSink::default();
    assert!(matches!(
        run(translator.forward(
            input,
            || TextDecoder,
            target.pinned(),
            TextEncoder::default()
        )),
        Err(RequestForwardError::Stream(StreamError::Input(
            "disconnected"
        )))
    ));
    assert_eq!(target.bytes(), b"partial");
    assert!(!target.state.borrow().closed);
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

impl ResponseEncoder for OtherEncoder {
    fn accept(&mut self, event: ResponseEvent) -> Result<(), CodecError> {
        ResponseEncoder::accept(&mut self.0, event)
    }

    fn advance(&mut self, output: &mut [u8]) -> Result<EncodeStep, CodecError> {
        ResponseEncoder::advance(&mut self.0, output)
    }

    fn finish(&mut self) -> Result<(), CodecError> {
        ResponseEncoder::finish(&mut self.0)
    }
}

#[test]
fn canceled_partial_forward_allows_a_fresh_attempt() {
    let decoder_count = Cell::new(0);
    let mut translator = Text2TextRequestTranslator::new(2, 3);
    let decoder_factory = || {
        decoder_count.set(decoder_count.get() + 1);
        TextDecoder
    };
    assert_eq!(decoder_count.get(), 0);

    let first = TestSink::default();
    {
        let mut forward = std::pin::pin!(translator.forward(
            OneChunk(Some(Ok(Bytes::from_static(b"original")))),
            decoder_factory,
            first.pinned(),
            TextEncoder::default()
        ));
        let mut cx = Context::from_waker(Waker::noop());
        assert!(forward.as_mut().poll(&mut cx).is_pending());
    }
    assert!(!first.bytes().is_empty());
    assert!(!first.state.borrow().closed);
    let second = TestSink::default();
    run(translator.forward(
        OneChunk(Some(Ok(Bytes::from_static(b"retry")))),
        decoder_factory,
        second.pinned(),
        OtherEncoder(TextEncoder::default()),
    ))
    .unwrap();
    assert_eq!(second.bytes(), b"retry!");
    assert!(second.state.borrow().closed);
    assert_eq!(decoder_count.get(), 2);
}

#[test]
fn completed_forward_accepts_different_source_factory_decoder_and_sink_types() {
    let mut translator = Text2TextRequestTranslator::new(16 * 1024, 64);
    let target = TestSink::default();
    run(translator.forward(
        OneChunk(Some(Ok(Bytes::from_static(b"original")))),
        TextDecoder::default,
        target.pinned(),
        TextEncoder::default(),
    ))
    .unwrap();
    assert!(target.state.borrow().closed);
    assert_eq!(target.bytes(), b"original!");
    struct OtherDecoder(TextDecoder);
    impl RequestDecoder for OtherDecoder {
        fn advance(
            &mut self,
            input: &mut Bytes,
            eof: bool,
        ) -> Result<DecodeStep<RequestEvent>, CodecError> {
            RequestDecoder::advance(&mut self.0, input, eof)
        }
    }
    let mut second = Vec::new();
    let sink = futures_util::sink::unfold(&mut second, |bytes, chunk: Bytes| async move {
        bytes.extend_from_slice(&chunk);
        Ok::<_, Infallible>(bytes)
    });
    let decoder = OtherDecoder(TextDecoder);
    run(translator.forward(
        futures_util::stream::iter([Ok::<_, Infallible>(Bytes::from_static(b"second"))]),
        move || decoder,
        sink,
        OtherEncoder(TextEncoder::default()),
    ))
    .unwrap();
    assert_eq!(second, b"second!");
}

#[test]
fn unpolled_forward_drops_owned_source_without_polling_or_creating_decoder() {
    struct TrackedInput<'a> {
        polls: &'a Cell<usize>,
        drops: &'a Cell<usize>,
    }
    impl Stream for TrackedInput<'_> {
        type Item = Result<Bytes, Infallible>;
        fn poll_next(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Option<Self::Item>> {
            self.polls.set(self.polls.get() + 1);
            Poll::Pending
        }
    }
    impl Drop for TrackedInput<'_> {
        fn drop(&mut self) {
            self.drops.set(self.drops.get() + 1);
        }
    }
    let polls = Cell::new(0);
    let drops = Cell::new(0);
    let decoder_count = Cell::new(0);
    let mut translator = Text2TextRequestTranslator::new(16 * 1024, 64);
    let decoder_factory = || {
        decoder_count.set(decoder_count.get() + 1);
        TextDecoder
    };
    let unused = TestSink::default();
    let forward = translator.forward(
        TrackedInput {
            polls: &polls,
            drops: &drops,
        },
        decoder_factory,
        unused.pinned(),
        TextEncoder::default(),
    );
    assert_eq!(drops.get(), 0);
    drop(forward);
    assert_eq!(drops.get(), 1);
    assert_eq!(polls.get(), 0);
    assert_eq!(decoder_count.get(), 0);
    assert!(unused.bytes().is_empty());
    assert!(!unused.state.borrow().closed);
    let target = TestSink::default();
    run(translator.forward(
        OneChunk(Some(Ok(Bytes::from_static(b"original")))),
        decoder_factory,
        target.pinned(),
        TextEncoder::default(),
    ))
    .unwrap();
    assert_eq!(decoder_count.get(), 1);
    assert_eq!(target.bytes(), b"original!");
}

#[test]
fn forwarding_outputs_before_upload_completes_and_cancellation_consumes_source() {
    let polls = Cell::new(0);
    let mut translator = Text2TextRequestTranslator::new(16 * 1024, 64);
    let input = LocalInput {
        polls: &polls,
        next: Rc::new(Cell::new(0)),
        _pinned: PhantomPinned,
    };
    let target = TestSink::default();
    {
        let mut attempt = std::pin::pin!(translator.forward(
            input,
            || TextDecoder,
            target.pinned(),
            TextEncoder::default()
        ));
        let mut cx = Context::from_waker(Waker::noop());
        assert!(attempt.as_mut().poll(&mut cx).is_pending());
    }
    assert_eq!(polls.get(), 2);
    assert_eq!(target.bytes(), b"hello");
    assert!(!target.state.borrow().closed);
}

#[test]
fn cancellation_while_waiting_for_first_input_drops_source() {
    struct PendingInput(Rc<Cell<usize>>);
    impl Stream for PendingInput {
        type Item = Result<Bytes, Infallible>;

        fn poll_next(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Option<Self::Item>> {
            Poll::Pending
        }
    }
    impl Drop for PendingInput {
        fn drop(&mut self) {
            self.0.set(self.0.get() + 1);
        }
    }
    let drops = Rc::new(Cell::new(0));
    let mut translator = Text2TextRequestTranslator::new(2, 3);
    let target = TestSink::default();
    {
        let mut forward = std::pin::pin!(translator.forward(
            PendingInput(drops.clone()),
            || TextDecoder,
            target.pinned(),
            TextEncoder::default()
        ));
        assert!(
            forward
                .as_mut()
                .poll(&mut Context::from_waker(Waker::noop()))
                .is_pending()
        );
    }
    assert_eq!(drops.get(), 1);
    assert!(target.bytes().is_empty());
    assert!(!target.state.borrow().closed);
    run(translator.forward(
        OneChunk(Some(Ok(Bytes::from_static(b"fresh")))),
        || TextDecoder,
        target.pinned(),
        TextEncoder::default(),
    ))
    .unwrap();
    assert_eq!(target.bytes(), b"fresh!");
    assert_eq!(drops.get(), 1);
}

#[test]
fn codec_failure_does_not_prevent_a_fresh_attempt() {
    let mut translator = Text2TextRequestTranslator::new(2, 3);
    let failed = TestSink::default();
    assert!(matches!(
        run(translator.forward(
            OneChunk(Some(Ok(Bytes::from_static(b"original")))),
            || BadEof,
            failed.pinned(),
            TextEncoder::default()
        )),
        Err(RequestForwardError::Stream(StreamError::Codec(
            CodecError::Contract(_)
        )))
    ));
    assert!(!failed.state.borrow().closed);
    let target = TestSink::default();
    run(translator.forward(
        OneChunk(Some(Ok(Bytes::from_static(b"fresh")))),
        || TextDecoder,
        target.pinned(),
        TextEncoder::default(),
    ))
    .unwrap();
    assert_eq!(target.bytes(), b"fresh!");
    assert!(target.state.borrow().closed);
}

#[test]
fn response_forward_is_lazy_and_accepts_fresh_per_call_types() {
    let count = Cell::new(0);
    let polls = Cell::new(0);
    let mut translator = Text2TextResponseTranslator::new(2, 3);
    let target = TestSink::default();
    let forward = translator.forward(
        LocalInput {
            polls: &polls,
            next: Rc::new(Cell::new(0)),
            _pinned: PhantomPinned,
        },
        || {
            count.set(count.get() + 1);
            TextDecoder
        },
        target.pinned(),
        TextEncoder::default(),
    );
    assert_eq!(count.get(), 0);
    assert_eq!(polls.get(), 0);
    run(forward).unwrap();
    assert_eq!(target.bytes(), b"hello world!");
    assert_eq!(count.get(), 1);
    assert_eq!(polls.get(), 4);

    struct OtherDecoder(TextDecoder);
    impl ResponseDecoder for OtherDecoder {
        fn advance(
            &mut self,
            input: &mut Bytes,
            eof: bool,
        ) -> Result<DecodeStep<ResponseEvent>, CodecError> {
            ResponseDecoder::advance(&mut self.0, input, eof)
        }
    }
    let mut output = Vec::new();
    let sink = futures_util::sink::unfold(&mut output, |output, chunk: Bytes| async move {
        output.extend_from_slice(&chunk);
        Ok::<_, Infallible>(output)
    });
    let decoder = OtherDecoder(TextDecoder);
    run(translator.forward(
        futures_util::stream::iter([Ok::<_, Infallible>(Bytes::from_static(b"second"))]),
        move || decoder,
        sink,
        OtherEncoder(TextEncoder::default()),
    ))
    .unwrap();
    assert_eq!(output, b"second!");
}

#[test]
fn response_output_can_be_retained_across_scratch_reuse() {
    let mut translator = Text2TextResponseTranslator::new(2, 3);
    let mut retained = Vec::new();
    for body in [b"first".as_slice(), b"second"] {
        let mut chunks = Vec::new();
        let target = futures_util::sink::unfold(&mut chunks, |chunks, chunk: Bytes| async move {
            chunks.push(chunk);
            Ok::<_, Infallible>(chunks)
        });
        run(translator.forward(
            OneChunk(Some(Ok(Bytes::copy_from_slice(body)))),
            || TextDecoder,
            target,
            TextEncoder::default(),
        ))
        .unwrap();
        assert!(chunks.iter().all(|chunk| chunk.len() <= 2));
        retained.push(chunks);
    }
    for (chunks, expected) in retained.iter().zip([b"first!".as_slice(), b"second!"]) {
        let output: Vec<_> = chunks
            .iter()
            .flat_map(|chunk| chunk.iter().copied())
            .collect();
        assert_eq!(output, expected);
    }
}

#[test]
fn response_scratch_survives_success_target_failures_and_cancellation() {
    struct ObserveOutput {
        encoder: TextEncoder,
        buffers: Rc<std::cell::RefCell<Vec<(usize, usize)>>>,
    }
    impl ResponseEncoder for ObserveOutput {
        fn accept(&mut self, event: ResponseEvent) -> Result<(), CodecError> {
            ResponseEncoder::accept(&mut self.encoder, event)
        }

        fn advance(&mut self, output: &mut [u8]) -> Result<EncodeStep, CodecError> {
            self.buffers
                .borrow_mut()
                .push((output.as_ptr() as usize, output.len()));
            ResponseEncoder::advance(&mut self.encoder, output)
        }

        fn finish(&mut self) -> Result<(), CodecError> {
            ResponseEncoder::finish(&mut self.encoder)
        }
    }
    let buffers = Rc::new(std::cell::RefCell::new(Vec::new()));
    let mut translator = Text2TextResponseTranslator::new(2, 3);
    for outcome in 0..6 {
        let target = TestSink::default();
        {
            let mut state = target.state.borrow_mut();
            state.fail_ready = outcome == 1;
            state.fail_send = outcome == 2;
            state.fail_close = outcome == 3;
        }
        let forward = translator.forward(
            OneChunk(Some(Ok(Bytes::from_static(b"original")))),
            || TextDecoder,
            target.pinned(),
            ObserveOutput {
                encoder: TextEncoder::default(),
                buffers: buffers.clone(),
            },
        );
        if outcome == 4 {
            let mut forward = std::pin::pin!(forward);
            assert!(
                forward
                    .as_mut()
                    .poll(&mut Context::from_waker(Waker::noop()))
                    .is_pending()
            );
            assert_eq!(target.state.borrow().chunks, 3);
            assert!(!target.state.borrow().closed);
        } else if (1..=3).contains(&outcome) {
            let expected = ["ready failed", "send failed", "close failed"][outcome - 1];
            assert!(
                matches!(run(forward), Err(ResponseForwardError::Target(error)) if error == expected)
            );
            assert!(!target.state.borrow().closed);
        } else {
            run(forward).unwrap();
            assert_eq!(target.bytes(), b"original!");
            assert!(target.state.borrow().closed);
        }
    }
    let buffers = buffers.borrow();
    assert!(!buffers.is_empty());
    assert!(buffers.iter().all(|buffer| *buffer == buffers[0]));
    assert_eq!(buffers[0].1, 2);
}

#[test]
fn response_backpressure_stops_decoding_until_target_is_ready() {
    struct CountDecoder(Rc<Cell<usize>>);
    impl ResponseDecoder for CountDecoder {
        fn advance(
            &mut self,
            input: &mut Bytes,
            eof: bool,
        ) -> Result<DecodeStep<ResponseEvent>, CodecError> {
            let result = ResponseDecoder::advance(&mut TextDecoder, input, eof)?;
            if matches!(result, DecodeStep::Event(_)) {
                self.0.set(self.0.get() + 1);
            }
            Ok(result)
        }
    }
    let decoded = Rc::new(Cell::new(0));
    let mut translator = Text2TextResponseTranslator::new(2, 64);
    let target = TestSink::default();
    target.state.borrow_mut().blocked = true;
    let mut forward = std::pin::pin!(translator.forward(
        OneChunk(Some(Ok(Bytes::from_static(b"original")))),
        || CountDecoder(decoded.clone()),
        target.pinned(),
        TextEncoder::default(),
    ));
    for _ in 0..3 {
        assert!(
            forward
                .as_mut()
                .poll(&mut Context::from_waker(Waker::noop()))
                .is_pending()
        );
    }
    assert_eq!(decoded.get(), 1);
    assert!(target.bytes().is_empty());
    target.unblock();
    run(forward).unwrap();
    assert_eq!(target.bytes(), b"original!");
    assert!(target.state.borrow().closed);
}

#[test]
fn response_unpolled_and_pending_forward_drop_owned_input() {
    struct TrackedInput<'a> {
        polls: &'a Cell<usize>,
        drops: &'a Cell<usize>,
    }
    impl Stream for TrackedInput<'_> {
        type Item = Result<Bytes, Infallible>;

        fn poll_next(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Option<Self::Item>> {
            self.polls.set(self.polls.get() + 1);
            Poll::Pending
        }
    }
    impl Drop for TrackedInput<'_> {
        fn drop(&mut self) {
            self.drops.set(self.drops.get() + 1);
        }
    }
    let polls = Cell::new(0);
    let drops = Cell::new(0);
    let decoders = Cell::new(0);
    let mut translator = Text2TextResponseTranslator::new(2, 3);
    for polled in [false, true] {
        let target = TestSink::default();
        let forward = translator.forward(
            TrackedInput {
                polls: &polls,
                drops: &drops,
            },
            || {
                decoders.set(decoders.get() + 1);
                TextDecoder
            },
            target.pinned(),
            TextEncoder::default(),
        );
        if polled {
            let mut forward = std::pin::pin!(forward);
            assert!(
                forward
                    .as_mut()
                    .poll(&mut Context::from_waker(Waker::noop()))
                    .is_pending()
            );
        } else {
            drop(forward);
        }
        assert!(target.bytes().is_empty());
        assert!(!target.state.borrow().closed);
    }
    assert_eq!(drops.get(), 2);
    assert_eq!(polls.get(), 1);
    assert_eq!(decoders.get(), 1);
}

#[test]
fn response_late_codec_failure_preserves_partial_output_without_finalization() {
    struct IncompleteResponse;
    impl ResponseDecoder for IncompleteResponse {
        fn advance(
            &mut self,
            input: &mut Bytes,
            eof: bool,
        ) -> Result<DecodeStep<ResponseEvent>, CodecError> {
            if eof {
                Err(CodecError::InvalidInput("incomplete response"))
            } else {
                ResponseDecoder::advance(&mut TextDecoder, input, eof)
            }
        }
    }
    let mut translator = Text2TextResponseTranslator::new(2, 3);
    let target = TestSink::default();
    assert!(matches!(
        run(translator.forward(
            OneChunk(Some(Ok(Bytes::from_static(b"partial")))),
            || IncompleteResponse,
            target.pinned(),
            TextEncoder::default(),
        )),
        Err(ResponseForwardError::Stream(StreamError::Codec(
            CodecError::InvalidInput(_)
        )))
    ));
    assert_eq!(target.bytes(), b"partial");
    assert!(!target.state.borrow().closed);
    let retry = TestSink::default();
    run(translator.forward(
        OneChunk(Some(Ok(Bytes::from_static(b"fresh")))),
        || TextDecoder,
        retry.pinned(),
        TextEncoder::default(),
    ))
    .unwrap();
    assert_eq!(retry.bytes(), b"fresh!");
}

#[test]
fn response_ready_output_batches_yield_and_self_wake() {
    let mut translator = Text2TextResponseTranslator::new(1, 4);
    let target = TestSink::default();
    let wakes = Arc::new(WakeCount(std::sync::atomic::AtomicUsize::new(0)));
    let waker = Waker::from(wakes.clone());
    let mut forward = std::pin::pin!(translator.forward(
        OneChunk(Some(Ok(Bytes::from_static(b"original")))),
        || TextDecoder,
        target.pinned(),
        TextEncoder::default(),
    ));
    assert!(
        forward
            .as_mut()
            .poll(&mut Context::from_waker(&waker))
            .is_pending()
    );
    assert_eq!(target.state.borrow().chunks, 4);
    assert_eq!(wakes.0.load(std::sync::atomic::Ordering::Relaxed), 1);
    run(forward).unwrap();
    assert!(target.state.borrow().closed);
}

#[tokio::test]
async fn response_bounded_channel_drains_concurrently_and_closes_on_success() {
    use futures_util::StreamExt;

    let mut translator = Text2TextResponseTranslator::new(2, 3);
    let (target, mut receiver) = futures_channel::mpsc::channel::<Bytes>(1);
    let forward = translator.forward(
        OneChunk(Some(Ok(Bytes::from_static(b"response")))),
        || TextDecoder,
        target,
        TextEncoder::default(),
    );
    let consume = async {
        let mut bytes = Vec::new();
        while let Some(chunk) = receiver.next().await {
            assert!(chunk.len() <= 2);
            bytes.extend_from_slice(&chunk);
        }
        bytes
    };
    let (result, bytes) = tokio::time::timeout(
        std::time::Duration::from_secs(2),
        futures_util::future::join(forward, consume),
    )
    .await
    .expect("response forwarding must not deadlock a bounded target");
    result.unwrap();
    assert_eq!(bytes, b"response!");
}
