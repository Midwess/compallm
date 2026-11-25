use super::{JsonDecodeError, JsonToken};

#[derive(Clone, Copy, Debug, Default)]
pub(super) enum JsonState {
    #[default]
    Idle,
    String {
        escaped: bool,
    },
    Scalar,
    Finished,
    Failed,
}

#[derive(Debug)]
pub(super) struct ParseState {
    bytes: Vec<u8>,
    decoded: Vec<u8>,
    pub(super) depth: usize,
    pub(super) active: bool,
}

impl Default for ParseState {
    fn default() -> Self {
        Self::with_capacity(16 * 1024)
    }
}

impl ParseState {
    pub(super) fn with_capacity(bytes: usize) -> Self {
        Self {
            bytes: Vec::with_capacity(bytes),
            decoded: Vec::with_capacity(bytes),
            depth: 0,
            active: false,
        }
    }

    pub(super) fn push(&mut self, token: JsonToken<'_>) -> Result<bool, JsonDecodeError> {
        match token {
            JsonToken::ObjectStart(_) | JsonToken::ArrayStart(_) => self.depth += 1,
            JsonToken::ObjectEnd | JsonToken::ArrayEnd => {
                self.depth = self
                    .depth
                    .checked_sub(1)
                    .ok_or(JsonDecodeError::InvalidState)?;
            }
            _ => {}
        }
        Ok(self.depth == 0)
    }

    pub(super) fn buffer(&mut self, bytes: &[u8]) {
        self.bytes.extend_from_slice(bytes);
    }

    /// Discards a capture that ended without a value.
    pub(super) fn discard(&mut self) {
        self.bytes.clear();
        self.depth = 0;
        self.active = false;
    }

    pub(super) fn reset(&mut self) {
        self.bytes.clear();
    }

    pub(super) fn retain(&mut self, bytes: &[u8]) {
        if self.bytes.is_empty() {
            self.buffer(bytes);
        }
        self.prepare_decoded();
    }

    fn prepare_decoded(&mut self) {
        self.decoded.resize(
            if self.bytes.contains(&b'\\') {
                self.bytes.len()
            } else {
                0
            },
            0,
        );
    }

    pub(super) fn string_layout(&self) -> super::StringLayout {
        super::StringLayout::new(&self.bytes, &self.decoded)
    }

    pub(super) fn resolve_retained(
        &self,
        reference: super::RetainedString,
    ) -> Result<&str, JsonDecodeError> {
        let bytes = match reference {
            super::RetainedString::Capture(range) => self.bytes.get(range),
            super::RetainedString::Decoded(range) => self.decoded.get(range),
            super::RetainedString::Literal(text) => return Ok(text),
        }
        .ok_or(JsonDecodeError::InvalidState)?;
        std::str::from_utf8(bytes).map_err(|_| JsonDecodeError::InvalidState)
    }

    pub(super) fn complete_capture(&mut self, bytes: &[u8]) {
        if !self.bytes.is_empty() {
            self.buffer(bytes);
        }
        self.active = false;
    }

    pub(super) fn deserialize<'de, T: serde::Deserialize<'de>>(
        &'de mut self,
        bytes: &'de [u8],
    ) -> Result<T, JsonDecodeError> {
        let source = if self.bytes.is_empty() {
            bytes
        } else {
            &self.bytes
        };
        self.decoded.resize(
            if source.contains(&b'\\') {
                source.len()
            } else {
                0
            },
            0,
        );
        compallm_serde_rules::borrowed::from_slice(source, &mut self.decoded)
            .map_err(JsonDecodeError::Deserialize)
    }
}
