use crate::{
    codec::EncodeStep,
    error::CodecError,
    t2t::{
        Text2TextTranslatorConfiguration,
        request::{
            encoder::Encoder,
            event::{RequestEvent as E, RequestField as F, Role, Structure as S},
            json::Utf8,
            structure::{Kind, Machine},
        },
    },
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Payload {
    Role,
    Content,
    Ignore,
}

pub(super) struct Structured {
    machine: Machine,
    payload: Option<Payload>,
    remaining: usize,
    role: [u8; 16],
    role_len: usize,
    role_seen: bool,
    content: bool,
    pending: [Option<E>; 3],
    utf8: Utf8,
}

impl Structured {
    pub fn new() -> Self {
        Self {
            machine: Machine::new(true),
            payload: None,
            remaining: 0,
            role: [0; 16],
            role_len: 0,
            role_seen: false,
            content: false,
            pending: [None, None, None],
            utf8: Utf8::default(),
        }
    }

    fn unsupported() -> CodecError {
        CodecError::Unsupported(
            "structured text encoding requires role-first text messages and supported settings",
        )
    }

    pub fn accept(&mut self, event: E, target: &mut Encoder<'_>) -> Result<(), CodecError> {
        if let E::TextFragment(len) = event {
            if self.payload.is_none() {
                return Err(Self::unsupported());
            }
            self.remaining = len;
            if self.payload == Some(Payload::Content) {
                target.accept(E::TextFragment(len))?;
            }
            return Ok(());
        }
        let E::Structure(event) = event else {
            return Err(Self::unsupported());
        };
        let kind = self.machine.kind();
        let field = self.machine.field();
        if matches!(event, S::StringStarted | S::NumberStarted | S::KeyStarted) {
            self.utf8 = Utf8::default();
        }
        if matches!(event, S::StringEnded | S::NumberEnded | S::KeyEnded) && !self.utf8.complete() {
            return Err(CodecError::InvalidInput(
                "incomplete UTF-8 structured payload",
            ));
        }
        self.machine.event(event)?;
        let mapped = match event {
            S::Field(field) if kind == Some(Kind::Root) => {
                if !matches!(
                    field,
                    F::Model
                        | F::Messages
                        | F::MaxTokens
                        | F::MaxCompletionTokens
                        | F::Stream
                        | F::StreamOptions
                        | F::TopP
                ) {
                    return Err(Self::unsupported());
                }
                None
            }
            S::ObjectStarted if self.machine.kind() == Some(Kind::Message) => {
                self.role_seen = false;
                self.content = false;
                None
            }
            S::Field(F::ToolCalls | F::ToolCallId) if kind == Some(Kind::Message) => {
                return Err(Self::unsupported());
            }
            S::StringStarted if kind == Some(Kind::Message) && field == Some(F::Role) => {
                self.role_len = 0;
                self.payload = Some(Payload::Role);
                None
            }
            S::StringEnded if self.payload == Some(Payload::Role) => {
                self.payload = None;
                self.role_seen = true;
                let role = match &self.role[..self.role_len] {
                    b"system" => Role::System,
                    b"developer" => Role::Developer,
                    b"user" => Role::User,
                    b"assistant" => Role::Assistant,
                    _ => return Err(Self::unsupported()),
                };
                Some(E::MessageStarted { role })
            }
            S::StringStarted if kind == Some(Kind::Message) && field == Some(F::Content) => {
                if !self.role_seen {
                    return Err(Self::unsupported());
                }
                self.content = true;
                self.payload = Some(Payload::Content);
                Some(E::TextStarted)
            }
            S::ArrayStarted if kind == Some(Kind::Message) && field == Some(F::Content) => {
                if !self.role_seen {
                    return Err(Self::unsupported());
                }
                self.content = true;
                Some(E::TextPartsStarted)
            }
            S::StringStarted if kind == Some(Kind::TextPart) && field == Some(F::Text) => {
                self.payload = Some(Payload::Content);
                Some(E::TextStarted)
            }
            S::ArrayEnded if kind == Some(Kind::Content) => Some(E::TextPartsEnded),
            S::StringEnded if self.payload == Some(Payload::Content) => {
                self.payload = None;
                Some(E::TextEnded)
            }
            S::Null if kind == Some(Kind::Message) && field == Some(F::Content) => {
                return Err(Self::unsupported());
            }
            S::ObjectEnded if kind == Some(Kind::Message) => {
                if !self.content {
                    return Err(Self::unsupported());
                }
                Some(E::MessageEnded)
            }
            S::StringStarted | S::NumberStarted | S::KeyStarted => {
                self.payload = Some(Payload::Ignore);
                None
            }
            S::StringEnded | S::NumberEnded | S::KeyEnded => {
                self.payload = None;
                None
            }
            S::ObjectEnded if kind == Some(Kind::Root) => {
                let mut index = 0;
                if let Some(tokens) = self.machine.tokens {
                    self.pending[index] =
                        Some(E::Configuration(Text2TextTranslatorConfiguration {
                            max_output_tokens: tokens,
                        }));
                    index += 1;
                }
                if let Some(top_p) = self.machine.top_p {
                    self.pending[index] = Some(E::TopP(top_p));
                    index += 1;
                }
                self.pending[index] = Some(E::Streaming {
                    stream: self.machine.stream,
                    include_usage: self.machine.include_usage,
                });
                None
            }
            _ => None,
        };
        if let Some(event) = mapped {
            target.accept(event)?;
        }
        Ok(())
    }

    pub fn advance(
        &mut self,
        text: &mut &[u8],
        output: &mut [u8],
        target: &mut Encoder<'_>,
    ) -> Result<EncodeStep, CodecError> {
        if self.remaining != text.len() {
            return Err(CodecError::Contract("structured payload length mismatch"));
        }
        if self.payload == Some(Payload::Content) {
            let before = *text;
            let result = target.advance(text, output)?;
            let consumed = before.len() - text.len();
            for &byte in &before[..consumed] {
                self.utf8.push(byte)?;
            }
            self.machine.bytes(&before[..consumed])?;
            self.remaining -= consumed;
            return Ok(result);
        }
        if !text.is_empty() {
            let len = text.len().min(4096);
            let bytes = &text[..len];
            self.machine.bytes(bytes)?;
            for &byte in bytes {
                self.utf8.push(byte)?;
            }
            if self.payload == Some(Payload::Role) {
                if len > self.role.len() - self.role_len {
                    return Err(Self::unsupported());
                }
                self.role[self.role_len..self.role_len + len].copy_from_slice(bytes);
                self.role_len += len;
            }
            *text = &text[len..];
            self.remaining -= len;
            if !text.is_empty() {
                output[0] = b' ';
                return Ok(EncodeStep::Wrote(1));
            }
        }
        loop {
            let step = target.advance(&mut &[][..], output)?;
            if !matches!(step, EncodeStep::NeedEvent) {
                return Ok(step);
            }
            let Some(event) = self.pending[0].take() else {
                return Ok(step);
            };
            self.pending.rotate_left(1);
            target.accept(event)?;
        }
    }

    pub fn finish(&self) -> Result<(), CodecError> {
        if !self.machine.done || self.remaining != 0 || self.pending.iter().any(Option::is_some) {
            return Err(CodecError::InvalidInput("incomplete structured request"));
        }
        Ok(())
    }
}
