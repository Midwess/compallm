use super::super::{
    Format,
    decoder::SchemaValue,
    event::{OutputFormatEvent as E, OutputFormatKind as K, RequestEvent},
    json::Utf8,
};
use crate::{codec::EncodeStep, error::CodecError};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Payload {
    Name,
    Description,
    Schema,
}

pub(crate) struct FormatWriter {
    target: Format,
    pub active: bool,
    processing: bool,
    seen: bool,
    mode: Option<K>,
    pending_close: bool,
    pub closed_messages: bool,
    opened: bool,
    fields: usize,
    name: bool,
    description: bool,
    strict: bool,
    schema: bool,
    payload: Option<Payload>,
    payload_bytes: usize,
    remaining: usize,
    validator: SchemaValue,
    utf8: Utf8,
    syntax: [u8; 192],
    syntax_len: usize,
    syntax_pos: usize,
    escape: [u8; 6],
    escape_len: usize,
    escape_pos: usize,
}
fn invalid() -> CodecError {
    CodecError::InvalidInput("invalid output format events")
}
impl FormatWriter {
    pub fn new(target: Format) -> Self {
        Self {
            target,
            active: false,
            processing: false,
            seen: false,
            mode: None,
            pending_close: false,
            closed_messages: false,
            opened: false,
            fields: 0,
            name: false,
            description: false,
            strict: false,
            schema: false,
            payload: None,
            payload_bytes: 0,
            remaining: 0,
            validator: SchemaValue::default(),
            utf8: Utf8::default(),
            syntax: [0; 192],
            syntax_len: 0,
            syntax_pos: 0,
            escape: [0; 6],
            escape_len: 0,
            escape_pos: 0,
        }
    }
    pub fn processing(&self) -> bool {
        self.processing
    }
    fn syntax(&mut self, bytes: &[u8]) {
        let end = self.syntax_len + bytes.len();
        self.syntax[self.syntax_len..end].copy_from_slice(bytes);
        self.syntax_len = end;
    }
    fn open(&mut self) {
        if self.opened {
            return;
        }
        self.opened = true;
        self.close_messages();
        self.syntax(match self.target {
            Format::Chat => b",\"response_format\":{",
            Format::Responses => b",\"text\":{\"format\":{",
            Format::Anthropic => b",\"output_config\":{\"format\":{",
        });
        if self.target == Format::Chat {
            self.syntax(b"\"json_schema\":{");
        }
    }
    fn close_messages(&mut self) {
        if self.pending_close {
            self.syntax(b"]");
            self.closed_messages = true;
            self.pending_close = false;
        }
    }
    fn field(&mut self, name: &[u8]) {
        self.open();
        if self.fields > 0 {
            self.syntax(b",");
        }
        self.fields += 1;
        self.syntax(b"\"");
        self.syntax(name);
        self.syntax(b"\":");
    }
    fn escaping(&mut self, byte: u8) {
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
    pub fn start(&mut self, close_messages: bool) -> Result<(), CodecError> {
        if self.seen || self.processing {
            return Err(invalid());
        }
        self.seen = true;
        self.active = true;
        self.processing = true;
        self.pending_close = close_messages;
        Ok(())
    }
    pub fn accept(&mut self, event: RequestEvent) -> Result<(), CodecError> {
        if !self.active {
            return Err(invalid());
        }
        match event {
            RequestEvent::SchemaFragment(len) if self.payload == Some(Payload::Schema) => {
                self.remaining = len
            }
            RequestEvent::TextFragment(len)
                if matches!(self.payload, Some(Payload::Name | Payload::Description)) =>
            {
                if len > 256 {
                    return Err(CodecError::LimitExceeded(
                        "format metadata exceeds control capacity",
                    ));
                }
                self.remaining = len;
            }
            RequestEvent::OutputFormat(E::Mode(mode)) if self.payload.is_none() => {
                if self.mode.is_some() {
                    return Err(invalid());
                }
                if mode == K::JsonObject && self.target == Format::Anthropic {
                    return Err(CodecError::Unsupported(
                        "Anthropic has no JSON-object mode without a schema",
                    ));
                }
                self.mode = Some(mode);
            }
            RequestEvent::OutputFormat(E::NameStarted | E::DescriptionStarted)
                if self.payload.is_none() =>
            {
                let name = matches!(event, RequestEvent::OutputFormat(E::NameStarted));
                if if name { self.name } else { self.description } {
                    return Err(invalid());
                }
                if name {
                    self.name = true;
                } else {
                    self.description = true;
                }
                self.payload = Some(if name {
                    Payload::Name
                } else {
                    Payload::Description
                });
                self.payload_bytes = 0;
                self.utf8 = Utf8::default();
                if self.target != Format::Anthropic {
                    self.field(if name { b"name" } else { b"description" });
                    self.syntax(b"\"");
                }
            }
            RequestEvent::OutputFormat(E::NameEnded | E::DescriptionEnded) => {
                let name = matches!(event, RequestEvent::OutputFormat(E::NameEnded));
                if self.payload
                    != Some(if name {
                        Payload::Name
                    } else {
                        Payload::Description
                    })
                    || !self.utf8.complete()
                    || name && self.payload_bytes == 0
                {
                    return Err(invalid());
                }
                if self.target != Format::Anthropic {
                    self.syntax(b"\"");
                }
                self.payload = None;
            }
            RequestEvent::OutputFormat(E::Strict(value)) if self.payload.is_none() => {
                if self.strict {
                    return Err(invalid());
                }
                self.strict = true;
                if self.target != Format::Anthropic {
                    self.field(b"strict");
                    self.syntax(match value {
                        Some(true) => b"true",
                        Some(false) => b"false",
                        None => b"null",
                    });
                }
            }
            RequestEvent::OutputFormat(E::SchemaStarted) if self.payload.is_none() => {
                if self.schema {
                    return Err(invalid());
                }
                self.schema = true;
                self.payload = Some(Payload::Schema);
                self.field(b"schema");
            }
            RequestEvent::OutputFormat(E::SchemaEnded) => {
                if self.payload != Some(Payload::Schema) || !self.validator.done {
                    return Err(invalid());
                }
                self.payload = None;
            }
            RequestEvent::OutputFormat(E::Ended) if self.payload.is_none() => {
                let mode = self.mode.ok_or_else(invalid)?;
                if mode != K::JsonSchema
                    && (self.name || self.description || self.strict || self.schema)
                {
                    return Err(invalid());
                }
                if mode == K::JsonSchema {
                    if !self.schema {
                        return Err(invalid());
                    }
                    if self.target != Format::Anthropic && !self.name {
                        self.field(b"name");
                        self.syntax(b"\"response\"");
                    }
                    self.open();
                    if self.target == Format::Chat {
                        self.syntax(b"},\"type\":\"json_schema\"}");
                    } else {
                        if self.fields > 0 {
                            self.syntax(b",");
                        }
                        self.syntax(b"\"type\":\"json_schema\"}}");
                    }
                } else if mode == K::JsonObject && self.target != Format::Anthropic {
                    self.close_messages();
                    self.opened = true;
                    self.syntax(match self.target {
                        Format::Chat => b",\"response_format\":{",
                        _ => b",\"text\":{\"format\":{",
                    });
                    self.syntax(if mode == K::Text {
                        b"\"type\":\"text\"}"
                    } else {
                        b"\"type\":\"json_object\"}"
                    });
                    if self.target == Format::Responses {
                        self.syntax(b"}");
                    }
                }
                self.active = false;
            }
            _ => return Err(invalid()),
        }
        Ok(())
    }
    pub fn advance(
        &mut self,
        text: &mut &[u8],
        output: &mut [u8],
    ) -> Result<EncodeStep, CodecError> {
        if text.len() != self.remaining {
            return Err(CodecError::Contract("format payload length mismatch"));
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
                output[written] = self.escape[self.escape_pos];
                self.escape_pos += 1;
                written += 1;
                continue;
            }
            let Some((&byte, remaining)) = text.split_first() else {
                break;
            };
            match self.payload {
                Some(Payload::Schema) => {
                    self.validator.push(byte)?;
                    output[written] = byte;
                    written += 1;
                }
                Some(payload) => {
                    self.payload_bytes += 1;
                    let limit = if payload == Payload::Name { 64 } else { 256 };
                    if self.payload_bytes > limit {
                        return Err(CodecError::LimitExceeded(
                            "format metadata exceeds control capacity",
                        ));
                    }
                    if payload == Payload::Name
                        && !matches!(byte,b'a'..=b'z'|b'A'..=b'Z'|b'0'..=b'9'|b'_'|b'-')
                    {
                        return Err(invalid());
                    }
                    self.utf8.push(byte)?;
                    if self.target != Format::Anthropic {
                        self.escaping(byte);
                    }
                }
                None => return Err(invalid()),
            }
            *text = remaining;
            self.remaining -= 1;
        }
        if written > 0 {
            return Ok(EncodeStep::Wrote(written));
        }
        self.syntax_len = 0;
        self.syntax_pos = 0;
        self.escape_len = 0;
        self.escape_pos = 0;
        if !self.active {
            self.processing = false;
        }
        Ok(EncodeStep::NeedEvent)
    }
}
