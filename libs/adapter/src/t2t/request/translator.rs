use std::{
    future::poll_fn,
    pin::Pin,
    sync::{Arc, Weak},
    task::{Context, Poll},
};

use bytes::Bytes;
use futures_core::Stream;
use pin_project_lite::pin_project;
use thiserror::Error;

use super::{Text2TextRequestStream, decoder::RequestDecoder, encoder::RequestEncoder};
use crate::{error::StreamError, limits::TranslationLimits, replay::ReplayStore};

#[derive(Debug, Error)]
pub enum RequestTranslatorError<I, R> {
    #[error("request input failed: {0}")]
    Input(I),
    #[error("replay storage failed: {0}")]
    Store(R),
    #[error("the previous request attempt body is still alive")]
    AttemptActive,
    #[error("retention failed or was canceled; close this translator")]
    RetentionUnavailable,
    #[error("the request translator is closing or closed")]
    Closed,
}

enum State {
    Fresh,
    /// Also left in place by cancellation: incomplete retention is never reused.
    Retaining,
    Ready,
    Closing,
    Closed,
}

/// Owns original bytes through an injected replay store and creates fresh
/// per-provider bodies. The initial implementation retains before forwarding.
/// No background task is spawned. Call close explicitly for asynchronous cleanup.
pub struct Text2TextRequestTranslator<S, R, F> {
    input: Option<Pin<Box<S>>>,
    store: R,
    decoder_factory: F,
    limits: TranslationLimits,
    state: State,
    active_attempt: Weak<()>,
}

impl<S, R, F> Text2TextRequestTranslator<S, R, F> {
    /// Retain the source stream, source decoder factory, and replay backend
    /// without polling input. A fresh decoder is constructed for each forward.
    pub fn new(input: S, decoder_factory: F, store: R) -> Self {
        Self::with_limits(input, decoder_factory, store, TranslationLimits::default())
    }

    pub fn with_limits(input: S, decoder_factory: F, store: R, limits: TranslationLimits) -> Self {
        Self {
            input: Some(Box::pin(input)),
            store,
            decoder_factory,
            limits,
            state: State::Fresh,
            active_attempt: Weak::new(),
        }
    }
}

impl<S, R, F, D, I> Text2TextRequestTranslator<S, R, F>
where
    S: Stream<Item = Result<Bytes, I>>,
    R: ReplayStore,
    F: FnMut() -> D,
    D: RequestDecoder,
{
    /// Retain and seal once, then create a new decoder, reader at byte zero, and
    /// target encoder. Different calls may use different concrete encoder types.
    ///
    /// The caller must cancel/join the prior HTTP attempt and drop its body first.
    /// This method cannot decide retry eligibility or response commitment safety.
    /// The returned stream is the target HTTP body; this method does not send it.
    pub async fn forward<E: RequestEncoder>(
        &mut self,
        target_encoder: E,
    ) -> Result<RequestAttemptStream<R::Reader, D, E>, RequestTranslatorError<I, R::Error>> {
        if self.active_attempt.strong_count() != 0 {
            return Err(RequestTranslatorError::AttemptActive);
        }
        self.retain().await?;
        let reader = self
            .store
            .open_reader()
            .await
            .map_err(RequestTranslatorError::Store)?;
        let lease = Arc::new(());
        self.active_attempt = Arc::downgrade(&lease);
        Ok(RequestAttemptStream {
            stream: Text2TextRequestStream::with_limits(
                reader,
                (self.decoder_factory)(),
                target_encoder,
                self.limits,
            ),
            _lease: lease,
        })
    }

    async fn retain(&mut self) -> Result<(), RequestTranslatorError<I, R::Error>> {
        match self.state {
            State::Ready => return Ok(()),
            State::Retaining => return Err(RequestTranslatorError::RetentionUnavailable),
            State::Closing | State::Closed => return Err(RequestTranslatorError::Closed),
            State::Fresh => self.state = State::Retaining,
        }

        let input = self.input.as_mut().expect("fresh translator owns input");
        let mut chunks = 0;
        while let Some(result) = poll_fn(|cx| input.as_mut().poll_next(cx)).await {
            let chunk = result.map_err(RequestTranslatorError::Input)?;
            self.store
                .append(chunk)
                .await
                .map_err(RequestTranslatorError::Store)?;

            // Storage and input may both be immediately ready, even for empty
            // chunks. Cooperatively yield without requiring a runtime dependency.
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
        self.store
            .seal()
            .await
            .map_err(RequestTranslatorError::Store)?;
        self.input = None;
        self.state = State::Ready;
        Ok(())
    }

    /// Release retained data after dropping all attempt bodies. Failure or
    /// cancellation leaves close retryable but prevents any further attempts.
    pub async fn close(&mut self) -> Result<(), RequestTranslatorError<I, R::Error>> {
        if self.active_attempt.strong_count() != 0 {
            return Err(RequestTranslatorError::AttemptActive);
        }
        if matches!(self.state, State::Closed) {
            return Ok(());
        }
        self.state = State::Closing;
        self.input = None;
        self.store
            .abort()
            .await
            .map_err(RequestTranslatorError::Store)?;
        self.state = State::Closed;
        Ok(())
    }
}

pin_project! {
    /// A single attempt's translated body. Dropping it releases the translator's
    /// attempt lease; reaching EOF alone does not permit another attempt.
    pub struct RequestAttemptStream<S, D, E> {
        #[pin]
        stream: Text2TextRequestStream<S, D, E>,
        _lease: Arc<()>,
    }
}

impl<S, D, E, I> Stream for RequestAttemptStream<S, D, E>
where
    S: Stream<Item = Result<Bytes, I>>,
    D: RequestDecoder,
    E: RequestEncoder,
{
    type Item = Result<Bytes, StreamError<I>>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.project().stream.poll_next(cx)
    }
}
