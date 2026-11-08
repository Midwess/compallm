//! Incremental codec steps shared by request and response contracts.

/// A decoder may consume input while producing a single bounded event.
pub enum DecodeStep<T> {
    Event(T),
    /// Consumed a nonempty prefix without producing an event. Allows bounded
    /// parsing work even when the transport supplies a very large chunk.
    Progress,
    /// All supplied bytes were consumed; more transport input is required.
    NeedInput,
    /// The entire body, including transport EOF, has been validated.
    Finished,
}

/// Encoders write only into the provided output slice.
pub enum EncodeStep {
    /// The preceding event has been consumed; another event can be accepted.
    NeedEvent,
    /// A nonempty prefix of the output slice was initialized.
    Wrote(usize),
    /// Final output has been drained after `finish` was called.
    Finished,
}

/// Result of one bounded, synchronous translator step.
#[must_use = "handle output, input, progress, or completion before the next step"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TranslateStep {
    /// Flush this output before reading or translating again.
    OutputReady(usize),
    /// Call next to read more input or record source EOF.
    NeedInput,
    /// More codec work is possible. Async callers should yield before retrying.
    Progress,
    /// Codec finalization is complete; the caller still owns writer closure.
    Finished,
}
