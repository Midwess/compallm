use super::schema::{Kind, Machine, invalid};
use crate::t2t::request::{
    Format,
    encoder::{Encoder, RequestEncoder},
    event::{RequestEvent, RequestField as F, Structure as S},
    json::{JsonFloat, Utf8},
};
use crate::{codec::EncodeStep, error::CodecError, t2t::Text2TextTranslatorConfiguration};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Scalar {
    String,
    Key,
    Number,
}

/// Compatible Chat encoder. Target model is borrowed; payloads are never retained.
pub struct CompatibleOpenAIRequestEncoder<'model> {
    machine: Machine,
    model: &'model str,
    configuration: Text2TextTranslatorConfiguration,
    legacy: Encoder<'model>,
    legacy_first: Option<RequestEvent>,
    structured: Option<bool>,
    syntax: [u8; 128],
    syntax_len: usize,
    syntax_pos: usize,
    header: u8,
    model_pos: usize,
    escape: [u8; 6],
    escape_len: usize,
    escape_pos: usize,
    scalar: Option<Scalar>,
    utf8: Utf8,
    remaining: usize,
    skip: bool,
    ready: bool,
    finished: bool,
    failed: bool,
}
impl<'model> CompatibleOpenAIRequestEncoder<'model> {
    /// Select the target model and fallback token budget. Preserve supported
    /// reasoning fields as supplied; callers must verify opaque origin compatibility.
    pub fn new(
        model: &'model str,
        configuration: Text2TextTranslatorConfiguration,
    ) -> Result<Self, CodecError> {
        let legacy = Encoder::new(Format::Chat, model, configuration)?;
        Ok(Self {
            machine: Machine::new(true),
            model,
            configuration,
            legacy,
            legacy_first: None,
            structured: None,
            syntax: [0; 128],
            syntax_len: 0,
            syntax_pos: 0,
            header: 0,
            model_pos: 0,
            escape: [0; 6],
            escape_len: 0,
            escape_pos: 0,
            scalar: None,
            utf8: Utf8::default(),
            remaining: 0,
            skip: false,
            ready: false,
            finished: false,
            failed: false,
        })
    }
    fn syntax(&mut self, bytes: &[u8]) {
        let end = self.syntax_len + bytes.len();
        self.syntax[self.syntax_len..end].copy_from_slice(bytes);
        self.syntax_len = end;
    }
    fn set_escape(&mut self, byte: u8) {
        self.escape_pos = 0;
        self.escape_len = match byte {
            b'"' | b'\\' => {
                self.escape[0] = b'\\';
                self.escape[1] = byte;
                2
            }
            0..=31 => {
                self.escape[..4].copy_from_slice(b"\\u00");
                self.escape[4] = b"0123456789abcdef"[(byte >> 4) as usize];
                self.escape[5] = b"0123456789abcdef"[(byte & 15) as usize];
                6
            }
            _ => {
                self.escape[0] = byte;
                1
            }
        };
    }
    fn accept_inner(&mut self, event: RequestEvent) -> Result<(), CodecError> {
        if !self.ready || self.finished {
            return Err(CodecError::Contract("accept requires NeedEvent"));
        }
        self.ready = false;
        if self.structured.is_none() {
            self.structured = Some(matches!(event, RequestEvent::Structure(_)));
            if self.structured == Some(false) {
                self.legacy_first = Some(event);
                return Ok(());
            }
        }
        if self.structured == Some(false) {
            return self.legacy.accept(event);
        }
        if let RequestEvent::TextFragment(len) = event {
            if self.scalar.is_none() {
                return Err(invalid());
            }
            self.remaining = len;
            return Ok(());
        }
        let RequestEvent::Structure(event) = event else {
            return Err(CodecError::Unsupported(
                "cannot mix message and structured events",
            ));
        };
        let root = self.machine.kind() == Some(Kind::Root);
        let options = self.machine.kind() == Some(Kind::StreamOptions);
        let first = self.machine.kind().is_none();
        let comma = self.machine.after();
        let array = self.machine.kind().is_some_and(|k| {
            matches!(
                k,
                Kind::Messages
                    | Kind::Tools
                    | Kind::Calls
                    | Kind::Details
                    | Kind::Content
                    | Kind::JsonArray
                    | Kind::Stops
            )
        });
        if matches!(event, S::StringEnded | S::KeyEnded) && !self.utf8.complete() {
            return Err(invalid());
        }
        self.machine.event(event)?;
        if options || self.machine.kind() == Some(Kind::StreamOptions) {
            self.skip = self.machine.kind() == Some(Kind::StreamOptions);
            return Ok(());
        }
        if let S::Field(field) = event {
            self.skip = root
                && matches!(
                    field,
                    F::Model
                        | F::Stream
                        | F::StreamOptions
                        | F::MaxTokens
                        | F::MaxCompletionTokens
                        | F::TopP
                );
            if !self.skip {
                if comma || root {
                    self.syntax(b",");
                }
                self.syntax(b"\"");
                self.syntax(<&str>::from(field).as_bytes());
                self.syntax(b"\":");
            }
            return Ok(());
        }
        let start = matches!(
            event,
            S::ObjectStarted
                | S::ArrayStarted
                | S::StringStarted
                | S::NumberStarted
                | S::Boolean(_)
                | S::Null
        );
        if start && array && comma {
            self.syntax(b",");
        }
        match event {
            S::ObjectStarted if first => {
                self.syntax(b"{\"model\":\"");
                self.header = 1;
            }
            S::ObjectStarted => self.syntax(b"{"),
            S::ObjectEnded if root => {}
            S::ObjectEnded => self.syntax(b"}"),
            S::ArrayStarted => self.syntax(b"["),
            S::ArrayEnded => self.syntax(b"]"),
            S::StringStarted | S::NumberStarted | S::KeyStarted => {
                self.scalar = Some(match event {
                    S::KeyStarted => Scalar::Key,
                    S::NumberStarted => Scalar::Number,
                    _ => Scalar::String,
                });
                self.utf8 = Utf8::default();
                if !self.skip && event != S::NumberStarted {
                    if event == S::KeyStarted && comma {
                        self.syntax(b",");
                    }
                    self.syntax(b"\"");
                }
            }
            S::StringEnded | S::NumberEnded | S::KeyEnded => {
                if !self.skip {
                    match event {
                        S::StringEnded => self.syntax(b"\""),
                        S::KeyEnded => self.syntax(b"\":"),
                        _ => {}
                    }
                }
                self.scalar = None;
                self.skip = false;
            }
            S::Boolean(value) => {
                if !self.skip {
                    self.syntax(if value { b"true" } else { b"false" });
                }
                self.skip = false;
            }
            S::Null => {
                if !self.skip {
                    self.syntax(b"null");
                }
                self.skip = false;
            }
            S::Field(_) => unreachable!(),
        }
        Ok(())
    }
    fn advance_inner(
        &mut self,
        text: &mut &[u8],
        output: &mut [u8],
    ) -> Result<EncodeStep, CodecError> {
        if output.is_empty() {
            return Err(CodecError::Contract(
                "encoder output slice must not be empty",
            ));
        }
        if self.structured.is_none() {
            if !text.is_empty() {
                return Err(invalid());
            }
            self.ready = true;
            return Ok(EncodeStep::NeedEvent);
        }
        if self.structured == Some(false) {
            if self.legacy_first.is_some() {
                let step = self.legacy.advance(&mut &b""[..], output)?;
                if !matches!(step, EncodeStep::NeedEvent) {
                    return Ok(step);
                }
                self.legacy
                    .accept(self.legacy_first.take().expect("first event"))?;
            }
            let step = self.legacy.advance(text, output)?;
            self.ready = matches!(step, EncodeStep::NeedEvent);
            return Ok(step);
        }
        if text.len() != self.remaining {
            return Err(CodecError::Contract(
                "pending text length does not match event progress",
            ));
        }
        let max = output.len().min(4096);
        let mut written = 0;
        while written < max {
            if self.syntax_pos < self.syntax_len {
                let len = (max - written).min(self.syntax_len - self.syntax_pos);
                output[written..written + len]
                    .copy_from_slice(&self.syntax[self.syntax_pos..self.syntax_pos + len]);
                self.syntax_pos += len;
                written += len;
                continue;
            }
            if self.escape_pos < self.escape_len {
                let len = (max - written).min(self.escape_len - self.escape_pos);
                output[written..written + len]
                    .copy_from_slice(&self.escape[self.escape_pos..self.escape_pos + len]);
                self.escape_pos += len;
                written += len;
                continue;
            }
            if self.header == 1 {
                if self.model_pos < self.model.len() {
                    let byte = self.model.as_bytes()[self.model_pos];
                    self.model_pos += 1;
                    self.set_escape(byte);
                    continue;
                }
                self.header = 0;
                self.syntax_len = 0;
                self.syntax_pos = 0;
                self.syntax(b"\"");
                continue;
            }
            if let Some((&byte, rest)) = text.split_first() {
                self.machine.bytes(&[byte])?;
                self.utf8.push(byte)?;
                *text = rest;
                self.remaining -= 1;
                if !self.skip {
                    if self.scalar == Some(Scalar::Number) {
                        output[written] = byte;
                        written += 1;
                    } else {
                        self.set_escape(byte);
                    }
                }
                continue;
            }
            break;
        }
        if written > 0 {
            self.ready = false;
            return Ok(EncodeStep::Wrote(written));
        }
        self.syntax_len = 0;
        self.syntax_pos = 0;
        self.escape_len = 0;
        self.escape_pos = 0;
        self.ready = !self.finished;
        Ok(if self.finished {
            EncodeStep::Finished
        } else {
            EncodeStep::NeedEvent
        })
    }
}
impl RequestEncoder for CompatibleOpenAIRequestEncoder<'_> {
    fn accept(&mut self, event: RequestEvent) -> Result<(), CodecError> {
        if self.failed {
            return Err(CodecError::Contract("encoder previously failed"));
        }
        let result = self.accept_inner(event);
        if result.is_err() {
            self.failed = true;
        }
        result
    }
    fn advance(&mut self, text: &mut &[u8], output: &mut [u8]) -> Result<EncodeStep, CodecError> {
        if self.failed {
            return Err(CodecError::Contract("encoder previously failed"));
        }
        let result = self.advance_inner(text, output);
        if result.is_err() {
            self.failed = true;
        }
        result
    }
    fn finish(&mut self) -> Result<(), CodecError> {
        if self.failed {
            return Err(CodecError::Contract("encoder previously failed"));
        }
        let result = (|| {
            if self.structured == Some(false) {
                return self.legacy.finish();
            }
            if !self.ready || self.finished || !self.machine.done {
                return Err(invalid());
            }
            self.syntax(b",\"max_completion_tokens\":");
            let mut value = self
                .machine
                .tokens
                .unwrap_or(self.configuration.max_output_tokens);
            let mut digits = [0; 10];
            let mut start = digits.len();
            loop {
                start -= 1;
                digits[start] = b'0' + (value % 10) as u8;
                value /= 10;
                if value == 0 {
                    break;
                }
            }
            self.syntax(&digits[start..]);
            if let Some(value) = self.machine.top_p {
                self.syntax(b",\"top_p\":");
                self.syntax(JsonFloat::try_from(value)?.as_ref());
            }
            self.syntax(
                Format::Chat.streaming_suffix(self.machine.stream, self.machine.include_usage),
            );
            self.finished = true;
            self.ready = false;
            Ok(())
        })();
        if result.is_err() {
            self.failed = true;
        }
        result
    }
}
