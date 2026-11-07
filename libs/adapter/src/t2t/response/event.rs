//! Initial text-only response vocabulary, not a complete provider contract.

use bytes::Bytes;

/// Fragments are decoded text bytes; boundaries need not align with UTF-8.
/// Response usage, finish reasons, tools, and reasoning remain to be specified.
#[derive(Debug)]
pub enum ResponseEvent {
    MessageStarted,
    TextStarted,
    TextFragment(Bytes),
    TextEnded,
    MessageEnded,
}
