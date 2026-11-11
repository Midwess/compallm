use crate::error::CodecError;

/// Caller-owned decoded fragment and role-late storage. Never resized by codecs.
pub struct RequestScratch {
    bytes: Vec<u8>,
    fragment_bytes: usize,
    deferred_bytes: usize,
    pub(crate) fragment_len: usize,
    pub(crate) deferred_len: usize,
}

impl RequestScratch {
    pub fn new(fragment_bytes: usize, deferred_bytes: usize) -> Result<Self, CodecError> {
        if fragment_bytes == 0 {
            return Err(CodecError::InvalidInput("fragment_bytes must exceed zero"));
        }
        let len = fragment_bytes
            .checked_add(deferred_bytes)
            .ok_or(CodecError::LimitExceeded("scratch capacity overflow"))?;
        let mut bytes = Vec::new();
        bytes
            .try_reserve_exact(len)
            .map_err(|_| CodecError::LimitExceeded("scratch capacity unavailable"))?;
        bytes.resize(len, 0);
        Ok(Self {
            bytes,
            fragment_bytes,
            deferred_bytes,
            fragment_len: 0,
            deferred_len: 0,
        })
    }

    /// Payload window for decoded text or raw schema bytes, identified by the event.
    pub fn text(&self) -> &[u8] {
        &self.bytes[..self.fragment_bytes]
    }

    pub fn text_mut(&mut self) -> &mut [u8] {
        &mut self.bytes[..self.fragment_bytes]
    }

    pub fn deferred_capacity(&self) -> usize {
        self.deferred_bytes
    }

    /// Role-late window for custom codecs; valid-byte counts belong to the codec.
    pub fn deferred_mut(&mut self) -> &mut [u8] {
        &mut self.bytes[self.fragment_bytes..self.fragment_bytes + self.deferred_bytes]
    }

    pub(crate) fn defer(&mut self, byte: u8) -> Result<(), CodecError> {
        if self.deferred_len == self.deferred_bytes {
            return Err(CodecError::LimitExceeded(
                "text before role exceeds scratch capacity",
            ));
        }
        self.bytes[self.fragment_bytes + self.deferred_len] = byte;
        self.deferred_len += 1;
        Ok(())
    }

    pub(crate) fn copy_deferred(&mut self, offset: usize, len: usize) {
        self.bytes.copy_within(
            self.fragment_bytes + offset..self.fragment_bytes + offset + len,
            0,
        );
        self.fragment_len = len;
    }

    /// Reset counters before a fresh attempt; never reset while text is pending.
    pub fn reset(&mut self) {
        self.fragment_len = 0;
        self.deferred_len = 0;
    }
}
