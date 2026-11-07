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
