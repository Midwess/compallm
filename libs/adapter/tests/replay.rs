use std::{
    cell::{Cell, RefCell},
    convert::Infallible,
    future::{Future, poll_fn},
    marker::PhantomPinned,
    pin::Pin,
    rc::Rc,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    task::{Context, Poll, Wake, Waker},
};

use adapter::replay::{Replay, ReplayError, ReplayStore};
#[path = "support/drive.rs"]
mod drive;
use adapter::{
    codec::{DecodeStep, EncodeStep},
    error::{CodecError, TranslationError},
    t2t::request::{
        RequestScratch, decoder::RequestDecoder, encoder::RequestEncoder, event::RequestEvent,
    },
};
use bytes::{Buf, Bytes};
use futures_core::Stream;
use futures_util::{StreamExt, TryStreamExt, io::Cursor, stream};

struct PartialWriter<'a> {
    bytes: &'a mut Vec<u8>,
    cancel: bool,
}
impl futures_io::AsyncWrite for PartialWriter<'_> {
    fn poll_write(
        mut self: Pin<&mut Self>,
        _: &mut Context<'_>,
        bytes: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        if !self.bytes.is_empty() {
            return if self.cancel {
                Poll::Pending
            } else {
                Poll::Ready(Err(std::io::Error::other("target failed")))
            };
        }
        self.bytes.extend_from_slice(bytes);
        Poll::Ready(Ok(bytes.len()))
    }
    fn poll_flush(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Poll::Ready(Ok(()))
    }
    fn poll_close(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        panic!("failed attempt must not close")
    }
}

struct TextDecoder;
impl RequestDecoder for TextDecoder {
    fn advance(
        &mut self,
        input: &mut &[u8],
        eof: bool,
        scratch: &mut RequestScratch,
    ) -> Result<DecodeStep<RequestEvent>, CodecError> {
        if !input.is_empty() {
            let len = input.len().min(4);
            scratch.text_mut()[..len].copy_from_slice(&input[..len]);
            input.advance(len);
            Ok(DecodeStep::Event(RequestEvent::TextFragment(len)))
        } else if eof {
            Ok(DecodeStep::Finished)
        } else {
            Ok(DecodeStep::NeedInput)
        }
    }
}

#[derive(Default)]
struct TextEncoder {
    pending: usize,
    finished: bool,
}
impl RequestEncoder for TextEncoder {
    fn accept(&mut self, event: RequestEvent) -> Result<(), CodecError> {
        let RequestEvent::TextFragment(bytes) = event else {
            return Err(CodecError::Unsupported("text only"));
        };
        assert_eq!(self.pending, 0);
        self.pending = bytes;
        Ok(())
    }
    fn advance(&mut self, text: &mut &[u8], output: &mut [u8]) -> Result<EncodeStep, CodecError> {
        if self.pending > 0 {
            let len = output.len().min(self.pending);
            output[..len].copy_from_slice(&text[..len]);
            text.advance(len);
            self.pending -= len;
            Ok(EncodeStep::Wrote(len))
        } else if self.finished {
            Ok(EncodeStep::Finished)
        } else {
            Ok(EncodeStep::NeedEvent)
        }
    }
    fn finish(&mut self) -> Result<(), CodecError> {
        self.finished = true;
        Ok(())
    }
}

#[test]
fn replay_preserves_original_request_bytes_without_framing_or_model_rewriting() {
    let chunks = [
        Bytes::from_static(b" {\n\"model\":\"client-model\","),
        Bytes::from_static(b"\"messages\":[{\"content\":\"hello\\nworld\"}]}\n"),
    ];
    let original: Vec<u8> = chunks
        .iter()
        .flat_map(|chunk| chunk.iter().copied())
        .collect();
    let state = Rc::new(RefCell::new(Storage::default()));
    let source = stream::iter(chunks.into_iter().map(Ok::<_, Infallible>));
    let mut replay = Replay::new(source, Store(state.clone()));
    for _ in 0..2 {
        let reader = run(replay.open_reader()).unwrap();
        let output = run(reader.collect::<Vec<_>>())
            .into_iter()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        let bytes: Vec<u8> = output
            .iter()
            .flat_map(|chunk| chunk.iter().copied())
            .collect();
        assert_eq!(bytes, original);
    }
    assert_eq!(state.borrow().bytes, original);
    assert_eq!(state.borrow().appends, 2);
    assert_eq!(state.borrow().seals, 1);
    run(replay.close()).unwrap();
}

#[test]
fn replay_readers_forward_after_target_failure_or_cancellation() {
    for canceled in [false, true] {
        let state = Rc::new(RefCell::new(Storage::default()));
        let mut replay = Replay::new(source(), Store(state.clone()));
        let decoder_count = Cell::new(0);
        let decoder_factory = || {
            decoder_count.set(decoder_count.get() + 1);
            TextDecoder
        };
        let reader = run(replay.open_reader())
            .unwrap()
            .map_err(std::io::Error::other)
            .into_async_read();
        let mut partial = Vec::new();
        let target = PartialWriter {
            bytes: &mut partial,
            cancel: canceled,
        };
        if canceled {
            cancel_pending(drive::translate_request(
                reader,
                decoder_factory(),
                target,
                TextEncoder::default(),
                16 * 1024,
                64,
            ));
        } else {
            assert!(matches!(
                run(drive::translate_request(reader, decoder_factory(), target, TextEncoder::default(), 16 * 1024, 64)),
                Err(TranslationError::Target(error)) if error.to_string() == "target failed"
            ));
        }
        assert_eq!(partial, b"orig");
        let reader = run(replay.open_reader())
            .unwrap()
            .map_err(std::io::Error::other)
            .into_async_read();
        let mut output = Cursor::new(Vec::new());
        run(drive::translate_request(
            reader,
            decoder_factory(),
            &mut output,
            TextEncoder::default(),
            16 * 1024,
            64,
        ))
        .unwrap();
        assert_eq!(output.into_inner(), b"original");
        assert_eq!(decoder_count.get(), 2);
        assert_eq!(state.borrow().appends, 1);
        assert_eq!(state.borrow().seals, 1);
        assert_eq!(state.borrow().opens, 2);
        run(replay.close()).unwrap();
        assert_eq!(state.borrow().aborts, 1);
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Operation {
    Append,
    Seal,
    Open,
    Abort,
}

#[derive(Default)]
struct Storage {
    bytes: Vec<u8>,
    sealed: bool,
    appends: usize,
    seals: usize,
    opens: usize,
    aborts: usize,
    fail: Option<Operation>,
    pending: Option<Operation>,
    fail_read: bool,
}

struct Store(Rc<RefCell<Storage>>);

impl Store {
    async fn operation(&self, operation: Operation) -> Result<(), &'static str> {
        poll_fn(|_| {
            let state = self.0.borrow();
            if state.pending == Some(operation) {
                Poll::Pending
            } else if state.fail == Some(operation) {
                Poll::Ready(Err("storage failed"))
            } else {
                Poll::Ready(Ok(()))
            }
        })
        .await
    }
}

impl ReplayStore for Store {
    type Error = &'static str;
    type Reader = stream::Iter<std::vec::IntoIter<Result<Bytes, &'static str>>>;

    async fn append(&mut self, chunk: Bytes) -> Result<(), Self::Error> {
        self.0.borrow_mut().appends += 1;
        self.operation(Operation::Append).await?;
        let mut state = self.0.borrow_mut();
        assert!(!state.sealed);
        // Small bounded fixtures only; not a production storage backend.
        assert!(state.bytes.len() + chunk.len() <= 128);
        state.bytes.extend_from_slice(&chunk);
        Ok(())
    }

    async fn seal(&mut self) -> Result<(), Self::Error> {
        self.0.borrow_mut().seals += 1;
        self.operation(Operation::Seal).await?;
        self.0.borrow_mut().sealed = true;
        Ok(())
    }

    async fn open_reader(&mut self) -> Result<Self::Reader, Self::Error> {
        self.0.borrow_mut().opens += 1;
        self.operation(Operation::Open).await?;
        let state = self.0.borrow();
        assert!(state.sealed);
        let chunk = if state.fail_read {
            Err("reader failed")
        } else {
            Ok(Bytes::copy_from_slice(&state.bytes))
        };
        Ok(stream::iter(vec![chunk]))
    }

    async fn abort(&mut self) -> Result<(), Self::Error> {
        self.0.borrow_mut().aborts += 1;
        self.operation(Operation::Abort).await?;
        self.0.borrow_mut().bytes.clear();
        Ok(())
    }
}

fn run<F: Future>(future: F) -> F::Output {
    let mut future = std::pin::pin!(future);
    for _ in 0..100 {
        if let Poll::Ready(result) = future
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop()))
        {
            return result;
        }
    }
    panic!("future did not complete");
}

fn cancel_pending<F: Future>(future: F) {
    let mut future = std::pin::pin!(future);
    assert!(
        future
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop()))
            .is_pending()
    );
}

fn source() -> impl Stream<Item = Result<Bytes, &'static str>> {
    stream::iter([Ok(Bytes::from_static(b"original"))])
}

#[test]
fn retention_is_once_and_readers_start_at_zero_and_own_their_lifetime() {
    let state = Rc::new(RefCell::new(Storage::default()));
    let mut replay = Replay::new(source(), Store(state.clone()));
    assert_eq!(state.borrow().appends, 0);
    drop(replay.retain());
    drop(replay.open_reader());
    drop(replay.close());
    assert_eq!(state.borrow().aborts, 0);
    let mut first = run(replay.open_reader()).unwrap();
    let mut second = run(replay.open_reader()).unwrap();
    run(replay.retain()).unwrap();
    assert_eq!(state.borrow().appends, 1);
    assert_eq!(state.borrow().seals, 1);
    assert_eq!(state.borrow().opens, 2);
    run(replay.close()).unwrap();
    run(replay.close()).unwrap();
    assert_eq!(state.borrow().aborts, 1);
    assert_eq!(run(first.next()).unwrap().unwrap(), b"original"[..]);
    assert_eq!(run(second.next()).unwrap().unwrap(), b"original"[..]);
    assert!(matches!(
        run(replay.open_reader()),
        Err(ReplayError::Closed)
    ));
    assert!(matches!(run(replay.retain()), Err(ReplayError::Closed)));
}

#[test]
fn input_failure_keeps_partial_retention_unavailable_until_cleanup() {
    let state = Rc::new(RefCell::new(Storage::default()));
    let input = stream::iter([Ok(Bytes::from_static(b"partial")), Err("input failed")]);
    let mut replay = Replay::new(input, Store(state.clone()));
    assert!(matches!(
        run(replay.retain()),
        Err(ReplayError::Input("input failed"))
    ));
    assert_eq!(state.borrow().bytes, b"partial");
    assert_eq!(state.borrow().seals, 0);
    assert!(matches!(
        run(replay.open_reader()),
        Err(ReplayError::RetentionUnavailable)
    ));
    run(replay.close()).unwrap();
    assert!(state.borrow().bytes.is_empty());
}

#[test]
fn append_and_seal_failure_or_cancellation_never_make_partial_replay_ready() {
    for operation in [Operation::Append, Operation::Seal] {
        for canceled in [false, true] {
            let state = Rc::new(RefCell::new(Storage::default()));
            if canceled {
                state.borrow_mut().pending = Some(operation);
            } else {
                state.borrow_mut().fail = Some(operation);
            }
            let mut replay = Replay::new(source(), Store(state.clone()));
            if canceled {
                cancel_pending(replay.open_reader());
            } else {
                assert!(matches!(
                    run(replay.open_reader()),
                    Err(ReplayError::Store("storage failed"))
                ));
            }
            state.borrow_mut().pending = None;
            state.borrow_mut().fail = None;
            assert!(!state.borrow().sealed);
            assert!(matches!(
                run(replay.retain()),
                Err(ReplayError::RetentionUnavailable)
            ));
            assert_eq!(state.borrow().opens, 0);
            run(replay.close()).unwrap();
            assert!(state.borrow().bytes.is_empty());
        }
    }
}

#[test]
fn reader_open_failure_or_cancellation_and_read_failure_preserve_ready_replay() {
    for canceled in [false, true] {
        let state = Rc::new(RefCell::new(Storage::default()));
        if canceled {
            state.borrow_mut().pending = Some(Operation::Open);
        } else {
            state.borrow_mut().fail = Some(Operation::Open);
        }
        let mut replay = Replay::new(source(), Store(state.clone()));
        if canceled {
            cancel_pending(replay.open_reader());
        } else {
            assert!(matches!(
                run(replay.open_reader()),
                Err(ReplayError::Store(_))
            ));
        }
        state.borrow_mut().pending = None;
        state.borrow_mut().fail = None;
        state.borrow_mut().fail_read = true;
        let mut reader = run(replay.open_reader()).unwrap();
        assert!(matches!(run(reader.next()), Some(Err("reader failed"))));
        state.borrow_mut().fail_read = false;
        let mut reader = run(replay.open_reader()).unwrap();
        assert_eq!(run(reader.next()).unwrap().unwrap(), b"original"[..]);
        assert_eq!(state.borrow().seals, 1);
        run(replay.close()).unwrap();
    }
}

#[test]
fn failed_or_canceled_cleanup_blocks_reads_but_can_be_retried() {
    for ready in [false, true] {
        for canceled in [false, true] {
            let state = Rc::new(RefCell::new(Storage::default()));
            let mut replay = Replay::new(source(), Store(state.clone()));
            if ready {
                run(replay.retain()).unwrap();
            }
            if canceled {
                state.borrow_mut().pending = Some(Operation::Abort);
            } else {
                state.borrow_mut().fail = Some(Operation::Abort);
            }
            if canceled {
                cancel_pending(replay.close());
            } else {
                assert!(matches!(run(replay.close()), Err(ReplayError::Store(_))));
            }
            assert!(matches!(
                run(replay.open_reader()),
                Err(ReplayError::Closed)
            ));
            state.borrow_mut().pending = None;
            state.borrow_mut().fail = None;
            run(replay.close()).unwrap();
            run(replay.close()).unwrap();
            assert_eq!(state.borrow().aborts, 2);
        }
    }
}

// Borrowed, non-Send, non-Unpin source; no restriction leaks from the wrapper.
struct LocalSource<'a> {
    polls: &'a Cell<usize>,
    drops: Rc<Cell<usize>>,
    _pin: PhantomPinned,
}

impl Stream for LocalSource<'_> {
    type Item = Result<Bytes, Infallible>;
    fn poll_next(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let polls = self.polls.get();
        self.polls.set(polls + 1);
        if polls == 0 {
            let chunk = Bytes::from_static(b"partial");
            Poll::Ready(Some(Ok(chunk)))
        } else {
            Poll::Pending
        }
    }
}

impl Drop for LocalSource<'_> {
    fn drop(&mut self) {
        self.drops.set(self.drops.get() + 1);
    }
}

#[test]
fn cancellation_waiting_for_source_blocks_reuse_and_close_releases_source() {
    let polls = Cell::new(0);
    let drops = Rc::new(Cell::new(0));
    let state = Rc::new(RefCell::new(Storage::default()));
    let mut replay = Replay::new(
        LocalSource {
            polls: &polls,
            drops: drops.clone(),
            _pin: PhantomPinned,
        },
        Store(state.clone()),
    );
    assert_eq!(polls.get(), 0);
    cancel_pending(replay.retain());
    assert_eq!(polls.get(), 2);
    assert_eq!(state.borrow().bytes, b"partial");
    assert!(matches!(
        run(replay.open_reader()),
        Err(ReplayError::RetentionUnavailable)
    ));
    assert_eq!(polls.get(), 2);
    run(replay.close()).unwrap();
    assert_eq!(drops.get(), 1);
}

struct WakeCount(AtomicUsize);
impl Wake for WakeCount {
    fn wake(self: Arc<Self>) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
}

#[test]
fn immediately_ready_empty_input_yields_and_self_wakes() {
    let state = Rc::new(RefCell::new(Storage::default()));
    let input = stream::repeat(Ok::<_, Infallible>(Bytes::new()));
    let mut replay = Replay::new(input, Store(state.clone()));
    let wakes = Arc::new(WakeCount(AtomicUsize::new(0)));
    let waker = Waker::from(wakes.clone());
    {
        let mut retain = std::pin::pin!(replay.retain());
        assert!(
            retain
                .as_mut()
                .poll(&mut Context::from_waker(&waker))
                .is_pending()
        );
    }
    assert_eq!(state.borrow().appends, 64);
    assert_eq!(wakes.0.load(Ordering::Relaxed), 1);
    assert!(matches!(
        run(replay.retain()),
        Err(ReplayError::RetentionUnavailable)
    ));
    run(replay.close()).unwrap();
}
