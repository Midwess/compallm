use super::super::json::{JsonString, Number, Piece};
use crate::error::CodecError;

#[derive(Clone, Copy)]
enum State {
    ObjectKey(bool),
    Colon,
    Value(bool),
    After,
}
#[derive(Clone, Copy)]
struct Frame {
    array: bool,
    state: State,
}
enum Token {
    String { parser: JsonString, key: bool },
    Number(Number),
    Literal(&'static [u8], usize),
}

pub(crate) struct SchemaValue {
    frames: [Frame; 64],
    depth: usize,
    token: Option<Token>,
    started: bool,
    pub done: bool,
}
impl Default for SchemaValue {
    fn default() -> Self {
        Self {
            frames: [Frame {
                array: false,
                state: State::ObjectKey(true),
            }; 64],
            depth: 0,
            token: None,
            started: false,
            done: false,
        }
    }
}
fn invalid() -> CodecError {
    CodecError::InvalidInput("invalid JSON schema object")
}
impl SchemaValue {
    fn value(&mut self) -> Result<(), CodecError> {
        if self.depth == 0 {
            if self.started {
                return Err(invalid());
            }
            self.started = true;
        } else {
            let f = &mut self.frames[self.depth - 1];
            if !matches!(f.state, State::Value(_)) {
                return Err(invalid());
            }
            f.state = State::After;
        }
        Ok(())
    }
    pub fn push(&mut self, byte: u8) -> Result<(), CodecError> {
        if self.done {
            return Err(invalid());
        }
        if let Some(token) = &mut self.token {
            match token {
                Token::String { parser, key } => {
                    if matches!(parser.push(byte)?, Some(Piece::End)) {
                        let key = *key;
                        self.token = None;
                        if key {
                            self.frames[self.depth - 1].state = State::Colon;
                        }
                    }
                    return Ok(());
                }
                Token::Literal(literal, position) => {
                    if byte != literal[*position] {
                        return Err(invalid());
                    }
                    *position += 1;
                    if *position == literal.len() {
                        self.token = None;
                    }
                    return Ok(());
                }
                Token::Number(number) => {
                    if matches!(byte, b' ' | b'\n' | b'\r' | b'\t' | b',' | b']' | b'}') {
                        if !number.complete() {
                            return Err(invalid());
                        }
                        self.token = None;
                    } else {
                        return number.push(byte);
                    }
                }
            }
        }
        if matches!(byte, b' ' | b'\n' | b'\r' | b'\t') {
            return Ok(());
        }
        if !self.started && byte != b'{' {
            return Err(invalid());
        }
        match byte {
            b'{' | b'[' => {
                self.value()?;
                if self.depth == self.frames.len() {
                    return Err(CodecError::LimitExceeded(
                        "schema nesting exceeds 64 containers",
                    ));
                }
                let array = byte == b'[';
                self.frames[self.depth] = Frame {
                    array,
                    state: if array {
                        State::Value(true)
                    } else {
                        State::ObjectKey(true)
                    },
                };
                self.depth += 1;
            }
            b'}' | b']' => {
                if self.depth == 0 {
                    return Err(invalid());
                }
                let f = self.frames[self.depth - 1];
                if f.array != (byte == b']')
                    || !matches!(
                        f.state,
                        State::After | State::ObjectKey(true) | State::Value(true)
                    )
                {
                    return Err(invalid());
                }
                self.depth -= 1;
                if self.depth == 0 {
                    self.done = true;
                }
            }
            b',' => {
                if self.depth == 0 {
                    return Err(invalid());
                }
                let f = &mut self.frames[self.depth - 1];
                if !matches!(f.state, State::After) {
                    return Err(invalid());
                }
                f.state = if f.array {
                    State::Value(false)
                } else {
                    State::ObjectKey(false)
                };
            }
            b':' => {
                if self.depth == 0 || !matches!(self.frames[self.depth - 1].state, State::Colon) {
                    return Err(invalid());
                }
                self.frames[self.depth - 1].state = State::Value(false);
            }
            b'"' => {
                let key = self.depth > 0
                    && matches!(self.frames[self.depth - 1].state, State::ObjectKey(_));
                if !key {
                    self.value()?;
                }
                self.token = Some(Token::String {
                    parser: JsonString::default(),
                    key,
                });
            }
            b't' | b'f' | b'n' => {
                self.value()?;
                self.token = Some(Token::Literal(
                    match byte {
                        b't' => b"true",
                        b'f' => b"false",
                        _ => b"null",
                    },
                    1,
                ));
            }
            b'-' | b'0'..=b'9' => {
                self.value()?;
                let mut number = Number::Start;
                number.push(byte)?;
                self.token = Some(Token::Number(number));
            }
            _ => return Err(invalid()),
        }
        Ok(())
    }
}
