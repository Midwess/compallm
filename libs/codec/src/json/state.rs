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
    pub(super) capture: super::capture::Capture,
    pub(super) depth: usize,
    pub(super) active: bool,
}

impl Default for ParseState {
    fn default() -> Self {
        Self::with_capacity(super::INTERNAL_BUFFER_BYTES)
    }
}

impl ParseState {
    pub(super) fn with_capacity(bytes: usize) -> Self {
        Self {
            bytes: Vec::with_capacity(bytes),
            capture: super::capture::Capture::with_capacity(bytes),
            depth: 0,
            active: false,
        }
    }

    pub(super) fn push(
        &mut self,
        token: JsonToken<'_>,
        end: usize,
        input: &[u8],
        offset: usize,
    ) -> Result<bool, JsonDecodeError> {
        let start = if matches!(token, JsonToken::ObjectStart(_) | JsonToken::ArrayStart(_)) {
            end.checked_sub(1).ok_or(JsonDecodeError::InvalidState)?
        } else {
            0
        };
        let nested = self.depth > 0;
        match &token {
            JsonToken::ObjectStart(_) | JsonToken::ArrayStart(_) => self.depth += 1,
            JsonToken::ObjectEnd | JsonToken::ArrayEnd => {
                self.depth = self
                    .depth
                    .checked_sub(1)
                    .ok_or(JsonDecodeError::InvalidState)?;
            }
            _ => {}
        }
        self.capture.push(token, start..end, nested, input, offset);
        Ok(self.depth == 0)
    }

    pub(super) fn buffer(&mut self, bytes: &[u8]) {
        self.bytes.extend_from_slice(bytes);
    }

    /// Discards a capture that ended without a value.
    pub(super) fn discard(&mut self) {
        self.bytes.clear();
        self.capture.clear();
        self.depth = 0;
        self.active = false;
    }

    pub(super) fn reset(&mut self) {
        self.bytes.clear();
        self.capture.clear();
    }

    pub(super) fn retain(&mut self, bytes: &[u8]) {
        if self.bytes.is_empty() {
            self.buffer(bytes);
        }
    }

    pub(super) fn string_layout(&self) -> super::StringLayout {
        super::StringLayout::new(&self.bytes, self.capture.text.as_bytes())
    }

    pub(super) fn resolve_retained(
        &self,
        reference: super::RetainedString,
    ) -> Result<&str, JsonDecodeError> {
        let bytes = match reference {
            super::RetainedString::Capture(range) => self.bytes.get(range),
            super::RetainedString::Decoded(range) => self.capture.text.as_bytes().get(range),
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
        inner: &mut super::decoder::JsonParser,
    ) -> Result<T, JsonDecodeError> {
        let source = if self.bytes.is_empty() {
            bytes
        } else {
            &self.bytes
        };
        let source = std::str::from_utf8(source).map_err(|_| JsonDecodeError::InvalidToken)?;
        self.capture
            .deserialize(source, inner)
            .map_err(JsonDecodeError::Deserialize)
    }

    pub(super) fn captured_bytes(&self) -> usize {
        self.bytes.len()
    }
}
