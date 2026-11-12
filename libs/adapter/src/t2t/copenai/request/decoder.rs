use crate::t2t::request::structure::{Kind, Machine, invalid};
use crate::t2t::request::{
    RequestScratch,
    decoder::RequestDecoder,
    event::{RequestEvent, RequestField, Structure as S},
    json::{JsonString, Piece},
};
use crate::{codec::DecodeStep, error::CodecError};

#[derive(Clone, Copy, PartialEq, Eq)]
enum StringKind {
    Field,
    Key,
    Value,
}

/// Incremental compatible Chat decoder. All variable payloads use supplied scratch.
pub struct CompatibleOpenAIRequestDecoder {
    machine: Machine,
    string: Option<JsonString>,
    string_kind: StringKind,
    key: [u8; 64],
    key_len: usize,
    colon: bool,
    number: bool,
    literal: Option<(&'static [u8], usize, S)>,
    piece: [u8; 4],
    piece_pos: usize,
    piece_len: usize,
    pending: Option<S>,
    emitted: bool,
    failed: bool,
    completed: bool,
}
impl Default for CompatibleOpenAIRequestDecoder {
    fn default() -> Self {
        Self::new()
    }
}
impl CompatibleOpenAIRequestDecoder {
    /// Start a fresh request; fragment capacity comes from supplied RequestScratch.
    pub fn new() -> Self {
        Self {
            machine: Machine::new(false),
            string: None,
            string_kind: StringKind::Value,
            key: [0; 64],
            key_len: 0,
            colon: false,
            number: false,
            literal: None,
            piece: [0; 4],
            piece_pos: 0,
            piece_len: 0,
            pending: None,
            emitted: false,
            failed: false,
            completed: false,
        }
    }
    fn emit(&mut self, event: S) -> Result<DecodeStep<RequestEvent>, CodecError> {
        self.machine.event(event)?;
        Ok(DecodeStep::Event(RequestEvent::Structure(event)))
    }
    fn fragment(&mut self, scratch: &RequestScratch) -> DecodeStep<RequestEvent> {
        self.emitted = true;
        DecodeStep::Event(RequestEvent::TextFragment(scratch.fragment_len))
    }
    fn end(
        &mut self,
        event: S,
        scratch: &RequestScratch,
    ) -> Result<DecodeStep<RequestEvent>, CodecError> {
        if scratch.fragment_len > 0 {
            self.pending = Some(event);
            Ok(self.fragment(scratch))
        } else {
            self.emit(event)
        }
    }
    fn step(
        &mut self,
        input: &mut &[u8],
        eof: bool,
        scratch: &mut RequestScratch,
    ) -> Result<DecodeStep<RequestEvent>, CodecError> {
        if self.completed {
            return if input.is_empty() && eof {
                Ok(DecodeStep::Finished)
            } else {
                Err(invalid())
            };
        }
        if self.emitted {
            scratch.fragment_len = 0;
            self.emitted = false;
        }
        if let Some(event) = self.pending.take() {
            return self.emit(event);
        }
        let mut consumed = 0;
        while consumed < 4096 {
            if self.piece_pos < self.piece_len {
                let byte = self.piece[self.piece_pos];
                self.piece_pos += 1;
                if self.string_kind == StringKind::Field && !self.number {
                    if self.key_len == self.key.len() {
                        return Err(crate::t2t::request::structure::unsupported());
                    }
                    self.key[self.key_len] = byte;
                    self.key_len += 1;
                } else {
                    self.machine.bytes(&[byte])?;
                    let index = scratch.fragment_len;
                    scratch.text_mut()[index] = byte;
                    scratch.fragment_len += 1;
                    if scratch.fragment_len == scratch.text().len() {
                        return Ok(self.fragment(scratch));
                    }
                }
                continue;
            }
            if self.number {
                let end = input
                    .first()
                    .is_none_or(|b| matches!(b, b' ' | b'\n' | b'\r' | b'\t' | b',' | b']' | b'}'));
                if end {
                    if input.is_empty() && !eof {
                        break;
                    }
                    self.number = false;
                    return self.end(S::NumberEnded, scratch);
                }
            }
            let Some((&byte, rest)) = input.split_first() else {
                break;
            };
            *input = rest;
            consumed += 1;
            if let Some(string) = &mut self.string {
                match string.push(byte)? {
                    Some(Piece::Bytes(bytes, len)) => {
                        self.piece = bytes;
                        self.piece_pos = 0;
                        self.piece_len = len;
                    }
                    Some(Piece::End) => {
                        self.string = None;
                        return match self.string_kind {
                            StringKind::Field => {
                                self.colon = true;
                                let field = RequestField::try_from(&self.key[..self.key_len])?;
                                self.emit(S::Field(field))
                            }
                            StringKind::Key => {
                                self.colon = true;
                                self.end(S::KeyEnded, scratch)
                            }
                            StringKind::Value => self.end(S::StringEnded, scratch),
                        };
                    }
                    None => {}
                }
                continue;
            }
            if self.number {
                self.piece = [byte, 0, 0, 0];
                self.piece_pos = 0;
                self.piece_len = 1;
                continue;
            }
            if let Some((literal, pos, event)) = self.literal {
                if byte != literal[pos] {
                    return Err(invalid());
                }
                if pos + 1 == literal.len() {
                    self.literal = None;
                    return self.emit(event);
                }
                self.literal = Some((literal, pos + 1, event));
                continue;
            }
            if matches!(byte, b' ' | b'\n' | b'\r' | b'\t') {
                continue;
            }
            if self.colon {
                if byte != b':' {
                    return Err(invalid());
                }
                self.colon = false;
                continue;
            }
            match byte {
                b',' => self.machine.comma()?,
                b'{' => return self.emit(S::ObjectStarted),
                b'}' => return self.emit(S::ObjectEnded),
                b'[' => return self.emit(S::ArrayStarted),
                b']' => return self.emit(S::ArrayEnded),
                b'"' => {
                    self.string = Some(JsonString::default());
                    self.key_len = 0;
                    self.string_kind = if self.machine.key_expected() {
                        if self.machine.kind() == Some(Kind::JsonObject) {
                            StringKind::Key
                        } else {
                            StringKind::Field
                        }
                    } else {
                        StringKind::Value
                    };
                    match self.string_kind {
                        StringKind::Field => {}
                        StringKind::Key => return self.emit(S::KeyStarted),
                        StringKind::Value => return self.emit(S::StringStarted),
                    }
                }
                b'-' | b'0'..=b'9' => {
                    self.number = true;
                    self.string_kind = StringKind::Value;
                    self.piece = [byte, 0, 0, 0];
                    self.piece_pos = 0;
                    self.piece_len = 1;
                    return self.emit(S::NumberStarted);
                }
                b't' => self.literal = Some((b"true", 1, S::Boolean(true))),
                b'f' => self.literal = Some((b"false", 1, S::Boolean(false))),
                b'n' => self.literal = Some((b"null", 1, S::Null)),
                _ => return Err(invalid()),
            }
        }
        if input.is_empty() && eof {
            if self.string.is_some()
                || self.number
                || self.literal.is_some()
                || self.colon
                || !self.machine.done
            {
                return Err(invalid());
            }
            self.completed = true;
            Ok(DecodeStep::Finished)
        } else if consumed > 0 {
            Ok(DecodeStep::Progress)
        } else {
            Ok(DecodeStep::NeedInput)
        }
    }
}
impl RequestDecoder for CompatibleOpenAIRequestDecoder {
    fn advance(
        &mut self,
        input: &mut &[u8],
        eof: bool,
        scratch: &mut RequestScratch,
    ) -> Result<DecodeStep<RequestEvent>, CodecError> {
        if self.failed {
            return Err(CodecError::Contract("decoder previously failed"));
        }
        let result = self.step(input, eof, scratch);
        if result.is_err() {
            self.failed = true;
        }
        result
    }
}
