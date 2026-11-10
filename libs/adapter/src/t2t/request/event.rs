//! Initial text-only vocabulary; tool and provider-specific semantics are pending.

use crate::t2t::Text2TextTranslatorConfiguration;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Role {
    System,
    User,
    Assistant,
}

/// Fragments are decoded text bytes, not JSON-escaped text or entire messages.
/// Codecs must bound fragment sizes and handle UTF-8 split across fragments.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RequestEvent<T = usize> {
    /// Explicit source settings, emitted at most once and outside messages.
    Configuration(Text2TextTranslatorConfiguration),
    MessageStarted {
        role: Role,
    },
    TextStarted,
    /// Production decoders return a length into caller-owned decoded scratch.
    TextFragment(T),
    TextEnded,
    MessageEnded,
}

impl<T> RequestEvent<T> {
    pub fn map_text<U>(self, map: impl FnOnce(T) -> U) -> RequestEvent<U> {
        match self {
            Self::Configuration(value) => RequestEvent::Configuration(value),
            Self::MessageStarted { role } => RequestEvent::MessageStarted { role },
            Self::TextStarted => RequestEvent::TextStarted,
            Self::TextFragment(text) => RequestEvent::TextFragment(map(text)),
            Self::TextEnded => RequestEvent::TextEnded,
            Self::MessageEnded => RequestEvent::MessageEnded,
        }
    }
}
