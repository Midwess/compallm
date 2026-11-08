//! Initial text-only vocabulary; tool and provider-specific semantics are pending.

use bytes::Bytes;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Role {
    System,
    User,
    Assistant,
}

/// Fragments are decoded text bytes, not JSON-escaped text or entire messages.
/// Codecs must bound fragment sizes and handle UTF-8 split across fragments.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RequestEvent {
    MessageStarted { role: Role },
    TextStarted,
    TextFragment(Bytes),
    TextEnded,
    MessageEnded,
}
