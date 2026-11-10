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
        let mut scratch = Self {
            bytes: Vec::new(),
            fragment_bytes: 0,
            deferred_bytes: 0,
            fragment_len: 0,
            deferred_len: 0,
        };
        scratch.prepare(fragment_bytes, deferred_bytes)?;
        Ok(scratch)
    }

    pub(crate) fn prepare(
        &mut self,
        fragment_bytes: usize,
        deferred_bytes: usize,
    ) -> Result<(), CodecError> {
        if fragment_bytes == 0 {
            return Err(CodecError::InvalidInput("fragment_bytes must exceed zero"));
        }
        let len = fragment_bytes
            .checked_add(deferred_bytes)
            .ok_or(CodecError::LimitExceeded("scratch capacity overflow"))?;
        if len > self.bytes.len() {
            self.bytes
                .try_reserve_exact(len - self.bytes.len())
                .map_err(|_| CodecError::LimitExceeded("scratch capacity unavailable"))?;
            self.bytes.resize(len, 0);
        }
        self.fragment_bytes = fragment_bytes;
        self.deferred_bytes = deferred_bytes;
        self.reset();
        Ok(())
    }

    /// Decoded fragment window. Only the prefix reported by TextFragment is valid.
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
