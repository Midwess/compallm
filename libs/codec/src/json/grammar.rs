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
}

impl Default for Grammar {
    fn default() -> Self {
        Self {
            stack: [Position::ObjectFirst; 64],
            depth: 0,
            root: false,
        }
    }
}

impl Grammar {
    pub(super) fn complete(&self) -> bool {
        self.root && self.depth == 0
    }

    pub(super) fn accept(&mut self, token: &JsonToken<'_>) -> Result<(), JsonDecodeError> {
        use JsonToken as T;
        use Position as P;
        if self.depth > 0 {
            let position = &mut self.stack[self.depth - 1];
            match (*position, token) {
                (P::ObjectFirst | P::Key, T::String(_)) => {
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
                self.depth += 1;
            }
            T::String(_) | T::Number(_) | T::Boolean(_) | T::Null => {}
            _ => return Err(JsonDecodeError::InvalidToken),
        }
        self.root = true;
        Ok(())
    }
}
