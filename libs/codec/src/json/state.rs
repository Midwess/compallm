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

#[derive(Debug, Default)]
pub(super) struct ParseState {
    bytes: Vec<u8>,
    depth: usize,
    pub(super) active: bool,
}

impl ParseState {
    pub(super) fn push(&mut self, token: JsonToken<'_>) -> Result<bool, JsonDecodeError> {
        match token {
            JsonToken::ValueObject(_) | JsonToken::ValueArray(_) => self.depth += 1,
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

    pub(super) fn deserialize<T: serde::de::DeserializeOwned>(
        &mut self,
        bytes: &[u8],
    ) -> Result<T, JsonDecodeError> {
        let value = if self.bytes.is_empty() {
            serde_json::from_slice(bytes)
        } else {
            self.buffer(bytes);
            serde_json::from_slice(&self.bytes)
        }
        .map_err(JsonDecodeError::Deserialize)?;
        self.bytes.clear();
        self.active = false;
        Ok(value)
    }
}
