//! Storage contract for original request bytes, independent of HTTP and runtimes.

use std::future::Future;

use bytes::Bytes;
use futures_core::Stream;

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

    /// Accept original bytes in order under a bounded buffering policy.
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
