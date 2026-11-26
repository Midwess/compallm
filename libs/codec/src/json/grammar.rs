use super::JsonDecodeError;
use super::token::JsonToken;

#[derive(Debug, Clone, Copy)]
pub(super) enum Position {
    ObjectFirst,
    Key,
    Colon,
    ObjectValue,
    ObjectEnd,
    ArrayFirst,
    ArrayValue,
    ArrayEnd,
}

#[derive(Debug)]
pub(super) struct Grammar {
    pub(super) stack: [Position; 64],
    pub(super) depth: usize,
    root: bool,
    key_bytes: String,
    keys: Vec<std::ops::Range<usize>>,
    key_starts: [usize; 64],
    byte_starts: [usize; 64],
}

impl Default for Grammar {
    fn default() -> Self {
        Self::with_capacity(super::INTERNAL_BUFFER_BYTES)
    }
}

impl Grammar {
    pub(super) fn reset(&mut self) {
        self.depth = 0;
        self.root = false;
        self.key_bytes.clear();
        self.keys.clear();
        self.key_starts.fill(0);
        self.byte_starts.fill(0);
    }

    pub(super) fn with_capacity(bytes: usize) -> Self {
        Self {
            stack: [Position::ObjectFirst; 64],
            depth: 0,
            root: false,
            key_bytes: String::with_capacity(bytes),
            keys: Vec::with_capacity(bytes.div_ceil(std::mem::size_of::<std::ops::Range<usize>>())),
            key_starts: [0; 64],
            byte_starts: [0; 64],
        }
    }
}

impl Grammar {
    pub(super) fn key(&self) -> Option<&str> {
        self.keys.last().map(|range| &self.key_bytes[range.clone()])
    }

    pub(super) fn complete(&self) -> bool {
        self.root && self.depth == 0
    }

    pub(super) fn accept(&mut self, token: &JsonToken<'_>) -> Result<(), JsonDecodeError> {
        use JsonToken as T;
        use Position as P;
        if self.depth > 0 {
            let position = &mut self.stack[self.depth - 1];
            match (*position, token) {
                (P::ObjectFirst | P::Key, T::String(key)) => {
                    let start = self.key_starts[self.depth - 1];
                    if self.keys[start..]
                        .iter()
                        .any(|range| &self.key_bytes[range.clone()] == *key)
                    {
                        return Err(JsonDecodeError::InvalidToken);
                    }
                    let start = self.key_bytes.len();
                    self.key_bytes.push_str(key);
                    self.keys.push(start..self.key_bytes.len());
                    *position = P::Colon;
                    return Ok(());
                }
                (P::Colon, T::Colon) => {
                    *position = P::ObjectValue;
                    return Ok(());
                }
                (P::ObjectFirst | P::ObjectEnd, T::ObjectEnd)
                | (P::ArrayFirst | P::ArrayEnd, T::ArrayEnd) => {
                    self.depth -= 1;
                    self.keys.truncate(self.key_starts[self.depth]);
                    self.key_bytes.truncate(self.byte_starts[self.depth]);
                    return Ok(());
                }
                (P::ObjectEnd, T::Comma) => {
                    *position = P::Key;
                    return Ok(());
                }
                (P::ArrayEnd, T::Comma) => {
                    *position = P::ArrayValue;
                    return Ok(());
                }
                (P::ObjectValue, _) => *position = P::ObjectEnd,
                (P::ArrayFirst | P::ArrayValue, _) => *position = P::ArrayEnd,
                _ => return Err(JsonDecodeError::InvalidToken),
            }
        } else if self.root {
            return Err(JsonDecodeError::InvalidToken);
        }
        match token {
            T::ObjectStart | T::ArrayStart => {
                if self.depth == self.stack.len() {
                    return Err(JsonDecodeError::InvalidToken);
                }
                self.stack[self.depth] = if matches!(token, T::ObjectStart) {
                    P::ObjectFirst
                } else {
                    P::ArrayFirst
                };
                self.key_starts[self.depth] = self.keys.len();
                self.byte_starts[self.depth] = self.key_bytes.len();
                self.depth += 1;
            }
            T::String(_) | T::Number(_) | T::Boolean(_) | T::Null => {}
            _ => return Err(JsonDecodeError::InvalidToken),
        }
        self.root = true;
        Ok(())
    }
}
