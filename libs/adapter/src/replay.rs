//! Generic byte retention, independent of codecs, HTTP, and runtimes.

use std::{
    future::{Future, poll_fn},
    pin::Pin,
    task::Poll,
};

use bytes::Bytes;
use futures_core::Stream;
use thiserror::Error;

/// Source, storage, or lifecycle failure while preparing or releasing replay.
#[derive(Debug, Error)]
pub enum ReplayError<I, R> {
    #[error("request input failed: {0}")]
    Input(I),
    #[error("replay storage failed: {0}")]
    Store(R),
    #[error("retention failed or was canceled; close this replay")]
    RetentionUnavailable,
    #[error("the replay is closing or closed")]
    Closed,
}

#[derive(Clone, Copy)]
enum ReplayState {
    Fresh,
    Retaining,
    Ready,
    Closing,
    Closed,
}

enum ReplayEvent {
    BeginRetention,
    RetentionCompleted,
    BeginClose,
    CloseCompleted,
}

enum ReplayAction {
    Proceed,
    AlreadyComplete,
}

enum ReplayStateError {
    RetentionUnavailable,
    Closed,
}

impl ReplayState {
    fn proceed(&mut self, event: ReplayEvent) -> Result<ReplayAction, ReplayStateError> {
        let next = match (*self, event) {
            (Self::Fresh, ReplayEvent::BeginRetention) => Self::Retaining,
            (Self::Ready, ReplayEvent::BeginRetention)
            | (Self::Closed, ReplayEvent::BeginClose) => {
                return Ok(ReplayAction::AlreadyComplete);
            }
            (Self::Retaining, ReplayEvent::BeginRetention) => {
                return Err(ReplayStateError::RetentionUnavailable);
            }
            (Self::Closing | Self::Closed, ReplayEvent::BeginRetention) => {
                return Err(ReplayStateError::Closed);
            }
            (Self::Retaining, ReplayEvent::RetentionCompleted) => Self::Ready,
            (_, ReplayEvent::BeginClose) => Self::Closing,
            (Self::Closing, ReplayEvent::CloseCompleted) => Self::Closed,

            _ => unreachable!("replay completion requires an active operation"),
        };
        *self = next;
        Ok(ReplayAction::Proceed)
    }
}

impl<I, R> From<ReplayStateError> for ReplayError<I, R> {
    fn from(error: ReplayStateError) -> Self {
        match error {
            ReplayStateError::RetentionUnavailable => Self::RetentionUnavailable,
            ReplayStateError::Closed => Self::Closed,
        }
    }
}

pub struct Replay<S, R> {
    source: Option<Pin<Box<S>>>,
    store: R,
    state: ReplayState,
}

impl<S, R> Replay<S, R> {
    /// Own the source and backend without polling either.
    pub fn new(source: S, store: R) -> Self {
        Self {
            source: Some(Box::pin(source)),
            store,
            state: ReplayState::Fresh,
        }
    }
}

impl<S, R, I> Replay<S, R>
where
    S: Stream<Item = Result<Bytes, I>>,
    R: ReplayStore,
{
    pub async fn retain(&mut self) -> Result<(), ReplayError<I, R::Error>> {
        if matches!(
            self.state.proceed(ReplayEvent::BeginRetention)?,
            ReplayAction::AlreadyComplete
        ) {
            return Ok(());
        }
        let source = self.source.as_mut().expect("fresh replay owns its source");
        let mut chunks = 0;
        while let Some(chunk) = poll_fn(|cx| source.as_mut().poll_next(cx)).await {
            self.store
                .append(chunk.map_err(ReplayError::Input)?)
                .await
                .map_err(ReplayError::Store)?;
            chunks += 1;
            if chunks == 64 {
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
                .await;
                chunks = 0;
            }
        }
        self.store.seal().await.map_err(ReplayError::Store)?;
        self.source = None;
        self.state.proceed(ReplayEvent::RetentionCompleted)?;
        Ok(())
    }

    /// Ensure retention and open an owned reader at byte zero. Reader creation
    /// failure/cancellation does not invalidate already sealed retention.
    pub async fn open_reader(&mut self) -> Result<R::Reader, ReplayError<I, R::Error>> {
        self.retain().await?;
        self.store.open_reader().await.map_err(ReplayError::Store)
    }

    /// Release the source and storage. Failed/canceled cleanup blocks readers
    /// but remains retryable; successful cleanup is idempotent.
    pub async fn close(&mut self) -> Result<(), ReplayError<I, R::Error>> {
        if matches!(
            self.state.proceed(ReplayEvent::BeginClose)?,
            ReplayAction::AlreadyComplete
        ) {
            return Ok(());
        }
        self.source = None;
        self.store.abort().await.map_err(ReplayError::Store)?;
        self.state.proceed(ReplayEvent::CloseCompleted)?;
        Ok(())
    }
}

/// One request's ordered retention. Implementations enforce their own capacity
/// limits and must not block the executor during storage operations.
///
/// Readers are available only after successful seal, start at byte zero, and do
/// not borrow the store. Report eviction/truncation as an error, not clean EOF.
/// Append cancellation may leave partial storage; abort must remain possible.
/// Reader creation must be cancellation-safe. No Send/static bounds are imposed.
pub trait ReplayStore {
    type Error;
    type Reader: Stream<Item = Result<Bytes, Self::Error>>;

    /// Accept original request bytes in order under a bounded buffering policy.
    fn append(&mut self, chunk: Bytes) -> impl Future<Output = Result<(), Self::Error>>;

    /// Drain pending writes and make the complete body replayable. This does not
    /// imply crash durability unless the backend explicitly guarantees it.
    fn seal(&mut self) -> impl Future<Output = Result<(), Self::Error>>;

    fn open_reader(&mut self) -> impl Future<Output = Result<Self::Reader, Self::Error>>;

    /// Invalidate retention and release storage, including after a canceled or
    /// failed append/seal. Must tolerate retries and partially completed cleanup.
    /// Backend drop behavior is best effort; asynchronous cleanup is explicit.
    fn abort(&mut self) -> impl Future<Output = Result<(), Self::Error>>;
}
