use crate::decoder::DecoderError;
use crate::json::JsonToken;

#[derive(Debug)]
pub(super) struct SchemaState {
    pub(super) active: bool,
    bytes: Vec<u8>,
    first: [bool; 64],
    depth: usize,
}

impl Default for SchemaState {
    fn default() -> Self {
        Self {
            active: false,
            bytes: Vec::new(),
            first: [true; 64],
            depth: 0,
        }
    }
}

impl SchemaState {
    pub(super) fn begin(&mut self) {
        self.active = true;
        self.bytes.clear();
        self.bytes.push(b'{');
        self.first[0] = true;
        self.depth = 1;
    }

    pub(super) fn proceed(&mut self, token: JsonToken<'_>) -> Result<Option<String>, DecoderError> {
        if !self.active {
            return Err(DecoderError::InvalidState);
        }
        if matches!(token, JsonToken::ObjectEnd | JsonToken::ArrayEnd) {
            self.bytes.push(if matches!(token, JsonToken::ObjectEnd) {
                b'}'
            } else {
                b']'
            });
            self.depth -= 1;
            if self.depth == 0 {
                self.active = false;
                return String::from_utf8(std::mem::take(&mut self.bytes))
                    .map(Some)
                    .map_err(|_| DecoderError::InvalidInput("invalid schema text"));
            }
            return Ok(None);
        }
        if !self.first[self.depth - 1] {
            self.bytes.push(b',');
        }
        self.first[self.depth - 1] = false;
        let key = match &token {
            JsonToken::ValueString(key, _)
            | JsonToken::ValueNumber(key, _)
            | JsonToken::ValueBoolean(key, _)
            | JsonToken::ValueNull(key)
            | JsonToken::ValueObject(key)
            | JsonToken::ValueArray(key) => *key,
            _ => unreachable!(),
        };
        if let Some(key) = key {
            serde_json::to_writer(&mut self.bytes, key).map_err(|_| DecoderError::InvalidState)?;
            self.bytes.push(b':');
        }
        match token {
            JsonToken::ValueString(_, value) => serde_json::to_writer(&mut self.bytes, value)
                .map_err(|_| DecoderError::InvalidState)?,
            JsonToken::ValueNumber(_, value) => serde_json::to_writer(&mut self.bytes, &value)
                .map_err(|_| DecoderError::InvalidState)?,
            JsonToken::ValueBoolean(_, value) => {
                self.bytes
                    .extend_from_slice(if value { b"true" } else { b"false" })
            }
            JsonToken::ValueNull(_) => self.bytes.extend_from_slice(b"null"),
            JsonToken::ValueObject(_) | JsonToken::ValueArray(_) => {
                if self.depth == self.first.len() {
                    return Err(DecoderError::LimitExceeded("schema nesting"));
                }
                self.bytes
                    .push(if matches!(token, JsonToken::ValueObject(_)) {
                        b'{'
                    } else {
                        b'['
                    });
                self.first[self.depth] = true;
                self.depth += 1;
            }
            _ => unreachable!(),
        }
        Ok(None)
    }
}
