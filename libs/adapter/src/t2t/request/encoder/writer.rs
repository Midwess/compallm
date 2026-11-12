use super::super::decoder::stop::{CHAT_STOP_COUNT, STOP_BYTES, STOP_COUNT};
use super::super::event::OutputFormatEvent;
use super::super::{
    Format, MODEL_BYTES,
    json::{JsonFloat, Utf8},
};
use super::FormatWriter;
use super::{StopWriter, Stops};
use crate::{
    codec::EncodeStep,
    error::CodecError,
    t2t::{
        Text2TextTranslatorConfiguration,
        request::event::{RequestEvent, Role},
    },
};

enum TextSource {
    Model,
    Fragment,
    Number,
}

enum Segment {
    Raw(usize),
    Literal(&'static [u8], usize),
    Number(JsonFloat, usize),
    Stops(StopWriter),
    Text {
        source: TextSource,
        len: usize,
        offset: usize,
        escape: [u8; 6],
        escaped: usize,
        escape_len: usize,
    },
}

impl Segment {
    fn text(source: TextSource, len: usize) -> Self {
        Self::Text {
            source,
            len,
            offset: 0,
            escape: [0; 6],
            escaped: 0,
            escape_len: 0,
        }
    }
    fn done(&self) -> bool {
        match self {
            Self::Raw(remaining) => *remaining == 0,
            Self::Literal(bytes, offset) => *offset == bytes.len(),
            Self::Number(bytes, offset) => *offset == bytes.as_ref().len(),
            Self::Stops(writer) => writer.done(),
            Self::Text {
                len,
                offset,
                escaped,
                escape_len,
                ..
            } => offset == len && escaped == escape_len,
        }
    }
    fn write(
        &mut self,
        text: &mut &[u8],
        model: &[u8],
        number: &[u8],
        stops: &Stops,
        output: &mut [u8],
        utf8: &mut Utf8,
    ) -> Result<usize, CodecError> {
        match self {
            Self::Raw(remaining) => {
                let len = output.len().min(*remaining).min(text.len());
                output[..len].copy_from_slice(&text[..len]);
                *text = &text[len..];
                *remaining -= len;
                Ok(len)
            }
            Self::Stops(writer) => Ok(writer.write(output, stops)),
            Self::Number(bytes, offset) => {
                let bytes = bytes.as_ref();
                let len = output.len().min(bytes.len() - *offset);
                output[..len].copy_from_slice(&bytes[*offset..*offset + len]);
                *offset += len;
                Ok(len)
            }
            Self::Literal(bytes, offset) => {
                let len = output.len().min(bytes.len() - *offset);
                output[..len].copy_from_slice(&bytes[*offset..*offset + len]);
                *offset += len;
                Ok(len)
            }
            Self::Text {
                source,
                len,
                offset,
                escape,
                escaped,
                escape_len,
            } => {
                let mut written = 0;
                while written < output.len() {
                    if *escaped < *escape_len {
                        let len = (output.len() - written).min(*escape_len - *escaped);
                        output[written..written + len]
                            .copy_from_slice(&escape[*escaped..*escaped + len]);
                        *escaped += len;
                        written += len;
                        continue;
                    }
                    if *offset == *len {
                        break;
                    }
                    let byte = match source {
                        TextSource::Model => model[*offset],
                        TextSource::Number => number[*offset],
                        TextSource::Fragment => {
                            let Some((&byte, remaining)) = text.split_first() else {
                                return Err(CodecError::Contract(
                                    "pending text suffix is incomplete",
                                ));
                            };
                            *text = remaining;
                            byte
                        }
                    };
                    utf8.push(byte)?;
                    *offset += 1;
                    *escaped = 0;
                    *escape_len = match byte {
                        b'"' | b'\\' => {
                            escape[0] = b'\\';
                            escape[1] = byte;
                            2
                        }
                        0..=0x1f => {
                            escape[..4].copy_from_slice(b"\\u00");
                            escape[4] = b"0123456789abcdef"[(byte >> 4) as usize];
                            escape[5] = b"0123456789abcdef"[(byte & 15) as usize];
                            6
                        }
                        _ => {
                            escape[0] = byte;
                            1
                        }
                    };
                }
                Ok(written)
            }
        }
    }
}

/// A fixed number of syntax/data segments; never an event queue or
/// a serialized body buffer. Text segments retain only source/offset metadata.
struct Output {
    segments: [Option<Segment>; 12],
    head: usize,
    len: usize,
}
impl Output {
    fn new() -> Self {
        Self {
            segments: std::array::from_fn(|_| None),
            head: 0,
            len: 0,
        }
    }
    fn literal(&mut self, value: &'static [u8]) {
        self.push(Segment::Literal(value, 0));
    }
    fn push(&mut self, value: Segment) {
        assert!(
            self.len < self.segments.len(),
            "one event has bounded output segments"
        );
        self.segments[(self.head + self.len) % self.segments.len()] = Some(value);
        self.len += 1;
    }
    fn empty(&self) -> bool {
        self.len == 0
    }
    fn write(
        &mut self,
        text: &mut &[u8],
        model: &[u8],
        number: &[u8],
        stops: &Stops,
        output: &mut [u8],
        utf8: &mut Utf8,
    ) -> Result<usize, CodecError> {
        let mut written = 0;
        while !self.empty() && written < output.len() {
            let segment = self.segments[self.head]
                .as_mut()
                .expect("occupied output slot");
            written += segment.write(text, model, number, stops, &mut output[written..], utf8)?;
            if segment.done() {
                self.segments[self.head] = None;
                self.head = (self.head + 1) % self.segments.len();
                self.len -= 1;
            } else {
                break;
            }
        }
        Ok(written)
    }
}

#[derive(Clone, Copy)]
enum Phase {
    Outside,
    Message,
    Text,
    AfterText,
    StopList,
    StopText,
    Parts,
    AfterPart,
}

pub(crate) struct Encoder<'model> {
    format: Format,
    phase: Phase,
    output: Output,
    has_message: bool,
    messages_closed: bool,
    output_format: FormatWriter,
    has_system: bool,
    system: bool,
    system_open: bool,
    system_text: bool,
    system_closed: bool,
    mid_conversation_system: bool,
    previous_role: Option<Role>,
    needs_assistant: bool,
    preserve_reasoning: bool,
    reasoning: Option<crate::t2t::request::decoder::reasoning::ReasoningValue>,
    reasoning_remaining: usize,
    responses_store: Option<bool>,
    include_reasoning: Option<bool>,
    utf8: Utf8,
    finished: bool,
    failed: bool,
    ready: bool,
    configuration: Text2TextTranslatorConfiguration,
    has_configuration: bool,
    stream: bool,
    include_usage: Option<bool>,
    top_p: Option<f64>,
    stops: Stops,
    has_stops: bool,
    has_streaming: bool,
    model: &'model str,
    number: [u8; 10],
    number_start: usize,
    text_remaining: usize,
    parts: bool,
    has_part: bool,
    part_text: bool,
}

impl<'model> Encoder<'model> {
    pub fn new(
        format: Format,
        model: &'model str,
        configuration: Text2TextTranslatorConfiguration,
    ) -> Result<Self, CodecError> {
        if model.is_empty() {
            return Err(CodecError::InvalidInput("target model must not be empty"));
        }
        if model.len() > MODEL_BYTES {
            return Err(CodecError::LimitExceeded(
                "target model exceeds identifier allowance",
            ));
        }
        if configuration.max_output_tokens == 0 {
            return Err(CodecError::InvalidInput(
                "max_output_tokens must exceed zero",
            ));
        }
        let mut output = Output::new();
        output.literal(b"{\"model\":\"");
        output.push(Segment::text(TextSource::Model, model.len()));
        output.literal(b"\"");
        Ok(Self {
            format,
            phase: Phase::Outside,
            output,
            has_message: false,
            messages_closed: false,
            output_format: FormatWriter::new(format),
            has_system: false,
            system: false,
            system_open: false,
            system_text: false,
            system_closed: false,
            mid_conversation_system: false,
            previous_role: None,
            needs_assistant: false,
            preserve_reasoning: false,
            reasoning: None,
            reasoning_remaining: 0,
            responses_store: None,
            include_reasoning: None,
            utf8: Utf8::default(),
            finished: false,
            failed: false,
            ready: false,
            configuration,
            has_configuration: false,
            stream: false,
            include_usage: None,
            top_p: None,
            stops: Stops::default(),
            has_stops: false,
            has_streaming: false,
            model,
            number: [0; 10],
            number_start: 10,
            text_remaining: 0,
            parts: false,
            has_part: false,
            part_text: false,
        })
    }

    pub fn accept(&mut self, event: RequestEvent) -> Result<(), CodecError> {
        if self.failed {
            return Err(CodecError::Contract("encoder previously failed"));
        }
        let result = self.event(event);
        if result.is_err() {
            self.failed = true;
        }
        result
    }

    pub fn with_mid_conversation_system(mut self, supported: bool) -> Self {
        self.mid_conversation_system = supported;
        self
    }

    pub fn with_compatible_reasoning(mut self, compatible: bool) -> Self {
        self.preserve_reasoning = compatible;
        self
    }

    fn event(&mut self, event: RequestEvent) -> Result<(), CodecError> {
        if self.finished || !self.output.empty() || !self.ready {
            return Err(CodecError::Contract(
                "encoder accepts events only after NeedEvent",
            ));
        }
        self.ready = false;
        if let Some(validator) = &self.reasoning {
            match event {
                RequestEvent::ResponsesReasoningFragment(len) => {
                    self.reasoning_remaining = len;
                    self.text_remaining = len;
                    if self.preserve_reasoning && self.format == Format::Responses && len > 0 {
                        self.output.push(Segment::Raw(len));
                    }
                    return Ok(());
                }
                RequestEvent::ResponsesReasoningEnded if validator.done => {
                    self.reasoning = None;
                    return Ok(());
                }
                _ => return Err(CodecError::InvalidInput("invalid reasoning event ordering")),
            }
        }
        if matches!(
            event,
            RequestEvent::OutputFormat(OutputFormatEvent::Started)
        ) {
            if !matches!(self.phase, Phase::Outside) {
                return Err(CodecError::InvalidInput("output format inside message"));
            }
            let close = self.has_message && !self.messages_closed;
            if self.system_open {
                self.output.literal(if self.has_system {
                    b"]"
                } else {
                    b",\"system\":\"\""
                });
                self.system_open = false;
                self.system_closed = true;
            }
            self.output_format.start(close)?;
            return Ok(());
        }
        if self.output_format.processing() {
            return self.output_format.accept(event);
        }
        match (self.phase, event) {
            (Phase::Outside, RequestEvent::ResponsesStore(value)) => {
                if self.responses_store.replace(value).is_some() {
                    return Err(CodecError::InvalidInput("duplicate store setting"));
                }
            }
            (Phase::Outside, RequestEvent::ResponsesEncryptedReasoning(value)) => {
                if self.include_reasoning.replace(value).is_some() {
                    return Err(CodecError::InvalidInput("duplicate include setting"));
                }
            }
            (Phase::Outside, RequestEvent::ResponsesReasoningStarted) => {
                let validator = crate::t2t::request::decoder::reasoning::ReasoningValue::new()?;
                let prefix = crate::t2t::request::decoder::reasoning::REASONING_PREFIX;
                self.reasoning = Some(validator);
                if self.preserve_reasoning && self.format == Format::Responses {
                    if self.messages_closed {
                        return Err(CodecError::Unsupported("reasoning after closed input"));
                    }
                    self.output.literal(if self.has_message {
                        b","
                    } else {
                        b",\"input\":["
                    });
                    self.output.literal(prefix);
                    self.has_message = true;
                }
            }
            (Phase::Outside, RequestEvent::StopSequencesStarted) => {
                if self.has_stops {
                    return Err(CodecError::InvalidInput("duplicate stop sequences"));
                }
                self.has_stops = true;
                self.phase = Phase::StopList;
            }
            (Phase::StopList, RequestEvent::StopSequenceStarted) => {
                if self.format == Format::Responses {
                    return Err(CodecError::Unsupported(
                        "Responses has no stop-sequence parameter",
                    ));
                }
                self.stops.begin(if self.format == Format::Anthropic {
                    STOP_COUNT
                } else {
                    CHAT_STOP_COUNT
                })?;
                self.utf8 = Utf8::default();
                self.phase = Phase::StopText;
            }
            (Phase::StopText, RequestEvent::TextFragment(len)) => {
                if len > STOP_BYTES {
                    return Err(CodecError::LimitExceeded(
                        "stop fragment exceeds control capacity",
                    ));
                }
                self.text_remaining = len;
            }
            (Phase::StopText, RequestEvent::StopSequenceEnded) => {
                if !self.utf8.complete() {
                    return Err(CodecError::InvalidInput("incomplete UTF-8 stop sequence"));
                }
                self.stops.end()?;
                self.phase = Phase::StopList;
            }
            (Phase::StopList, RequestEvent::StopSequencesEnded) => self.phase = Phase::Outside,
            (Phase::Outside, RequestEvent::TopP(value)) => {
                if self.top_p.is_some() || !value.is_finite() || !(0.0..=1.0).contains(&value) {
                    return Err(CodecError::InvalidInput("invalid or duplicate top_p"));
                }
                self.top_p = Some(value);
            }
            (
                Phase::Outside,
                RequestEvent::Streaming {
                    stream,
                    include_usage,
                },
            ) => {
                if self.has_streaming {
                    return Err(CodecError::InvalidInput(
                        "duplicate streaming configuration",
                    ));
                }
                if !stream && include_usage.is_some() {
                    return Err(CodecError::InvalidInput(
                        "stream options require stream=true",
                    ));
                }
                self.stream = stream;
                self.include_usage = include_usage;
                self.has_streaming = true;
            }
            (_, RequestEvent::Structure(_)) => {
                return Err(CodecError::Unsupported(
                    "structured compatible Chat events require a compatible encoder",
                ));
            }
            (Phase::Outside, RequestEvent::Configuration(configuration)) => {
                if self.has_configuration || configuration.max_output_tokens == 0 {
                    return Err(CodecError::InvalidInput(
                        "invalid or duplicate request configuration",
                    ));
                }
                self.configuration = configuration;
                self.has_configuration = true;
            }
            (Phase::Outside, RequestEvent::MessageStarted { role }) => {
                self.parts = false;
                self.has_part = false;
                if self.messages_closed {
                    return Err(CodecError::Unsupported(
                        "output format cannot split the target conversation",
                    ));
                }
                let instruction = matches!(role, Role::System | Role::Developer);
                let role = if self.format == Format::Anthropic && instruction {
                    Role::System
                } else {
                    role
                };
                if self.format == Format::Anthropic {
                    if self.needs_assistant && role != Role::Assistant {
                        return Err(CodecError::Unsupported(
                            "system instruction must precede assistant or end history",
                        ));
                    }
                    if instruction && self.has_message {
                        if !self.mid_conversation_system || self.previous_role != Some(Role::User) {
                            return Err(CodecError::Unsupported(
                                "target does not support this system instruction placement",
                            ));
                        }
                        self.needs_assistant = true;
                    } else {
                        self.needs_assistant = false;
                    }
                }
                self.system = self.format == Format::Anthropic && instruction && !self.has_message;
                if self.system {
                    if self.system_closed {
                        return Err(CodecError::Unsupported(
                            "instruction group is already closed",
                        ));
                    }
                    if !self.system_open {
                        self.system_open = true;
                    }
                    self.system_text = false;
                    self.phase = Phase::Message;
                    return Ok(());
                }
                if self.system_open {
                    self.output.literal(if self.has_system {
                        b"]"
                    } else {
                        b",\"system\":\"\""
                    });
                    self.system_open = false;
                    self.system_closed = true;
                }
                if self.format == Format::Anthropic && !self.system_closed {
                    self.output.literal(b",\"system\":\"\"");
                    self.system_closed = true;
                }
                self.previous_role = Some(role);
                if !self.has_message {
                    self.output.literal(if self.format == Format::Responses {
                        b",\"input\":["
                    } else {
                        b",\"messages\":["
                    });
                }
                if self.has_message {
                    self.output.literal(b",");
                }
                self.output.literal(if self.format != Format::Responses {
                    b"{\"role\":\""
                } else {
                    b"{\"type\":\"message\",\"role\":\""
                });
                self.output.literal(match role {
                    Role::System => b"system",
                    Role::Developer => b"developer",
                    Role::User => b"user",
                    Role::Assistant => b"assistant",
                });
                self.output.literal(b"\",\"content\":");
                self.phase = Phase::Message;
            }
            (Phase::Message, RequestEvent::TextPartsStarted) => {
                self.parts = true;
                if !self.system {
                    self.output.literal(b"[");
                }
                self.phase = Phase::Parts;
            }
            (Phase::Parts | Phase::AfterPart, RequestEvent::TextStarted) => {
                self.part_text = false;
                self.utf8 = Utf8::default();
                if self.system {
                    self.system_text = false;
                } else {
                    if self.has_part {
                        self.output.literal(b",");
                    }
                    self.output.literal(match self.format {
                        Format::Responses => b"{\"type\":\"input_text\",\"text\":\"",
                        _ => b"{\"type\":\"text\",\"text\":\"",
                    });
                }
                self.has_part = true;
                self.phase = Phase::Text;
            }
            (Phase::AfterPart, RequestEvent::TextPartsEnded) => {
                if !self.system {
                    self.output.literal(b"]");
                }
                self.phase = Phase::AfterText;
            }
            (Phase::Message, RequestEvent::TextStarted) => {
                self.utf8 = Utf8::default();
                if !self.system {
                    self.output.literal(b"\"");
                }
                self.phase = Phase::Text;
            }
            (Phase::Text, RequestEvent::TextFragment(len)) => {
                self.part_text |= len > 0;
                self.text_remaining = len;
                if len > 0 {
                    if self.system && !self.system_text {
                        if self.has_system {
                            self.output.literal(b",");
                        } else {
                            self.output.literal(b",\"system\":[");
                        }
                        self.output.literal(b"{\"type\":\"text\",\"text\":\"");
                        self.system_text = true;
                    }
                    self.output.push(Segment::text(TextSource::Fragment, len));
                }
            }
            (Phase::Text, RequestEvent::TextEnded) => {
                if self.parts && self.format == Format::Anthropic && !self.system && !self.part_text
                {
                    return Err(CodecError::InvalidInput("empty Anthropic text block"));
                }
                if !self.utf8.complete() {
                    return Err(CodecError::InvalidInput("incomplete UTF-8 text"));
                }
                if !self.system || self.system_text {
                    self.output.literal(b"\"");
                }
                if self.parts {
                    if !self.system || self.system_text {
                        self.output.literal(b"}");
                        if self.system {
                            self.has_system = true;
                        }
                    }
                    self.phase = Phase::AfterPart;
                } else {
                    self.phase = Phase::AfterText;
                }
            }
            (Phase::AfterText, RequestEvent::MessageEnded) => {
                if self.system {
                    if self.system_text && !self.parts {
                        self.output.literal(b"}");
                        self.has_system = true;
                    }
                } else {
                    self.output.literal(b"}");
                    self.has_message = true;
                }
                self.phase = Phase::Outside;
            }
            (Phase::AfterText, RequestEvent::TextStarted) => {
                return Err(CodecError::Unsupported(
                    "multiple text parts are outside the string-content subset",
                ));
            }
            _ => {
                return Err(CodecError::InvalidInput(
                    "invalid text request event ordering",
                ));
            }
        }
        Ok(())
    }

    pub fn advance(
        &mut self,
        text: &mut &[u8],
        output: &mut [u8],
    ) -> Result<EncodeStep, CodecError> {
        if self.failed {
            return Err(CodecError::Contract("encoder previously failed"));
        }
        if output.is_empty() {
            self.failed = true;
            return Err(CodecError::Contract(
                "encoder output slice must not be empty",
            ));
        }
        if self.output_format.processing() && self.output.empty() {
            let result = self.output_format.advance(text, output);
            if result.is_err() {
                self.failed = true;
            }
            if matches!(&result, Ok(EncodeStep::NeedEvent)) {
                self.ready = true;
                self.messages_closed |= self.output_format.closed_messages;
            }
            return result;
        }
        if text.len() != self.text_remaining {
            self.failed = true;
            return Err(CodecError::Contract(
                "pending text length does not match event progress",
            ));
        }
        if self.reasoning_remaining > 0
            && (!self.preserve_reasoning || self.format != Format::Responses)
        {
            let validator = self
                .reasoning
                .as_mut()
                .ok_or(CodecError::Contract("missing reasoning validator"))?;
            let len = text.len().min(4096);
            if let Err(error) = text[..len]
                .iter()
                .try_for_each(|&byte| validator.push(byte))
            {
                self.failed = true;
                return Err(error);
            }
            *text = &text[len..];
            self.reasoning_remaining -= len;
            self.text_remaining -= len;
            if !text.is_empty() {
                output[0] = b' ';
                self.ready = false;
                return Ok(EncodeStep::Wrote(1));
            }
        }
        if matches!(self.phase, Phase::StopText) {
            let result = (|| {
                for &byte in *text {
                    self.utf8.push(byte)?;
                }
                self.stops.append(text)
            })();
            if let Err(error) = result {
                self.failed = true;
                return Err(error);
            }
            *text = &[];
            self.text_remaining = 0;
        }
        let len = output.len().min(4096);
        let before = text.len();
        let pending = *text;
        let written = match self.output.write(
            text,
            self.model.as_bytes(),
            &self.number[self.number_start..],
            &self.stops,
            &mut output[..len],
            &mut self.utf8,
        ) {
            Ok(written) => written,
            Err(error) => {
                self.failed = true;
                return Err(error);
            }
        };
        if self.reasoning_remaining > 0 {
            let consumed = before - text.len();
            let validator = self
                .reasoning
                .as_mut()
                .ok_or(CodecError::Contract("missing reasoning validator"))?;
            if let Err(error) = pending[..consumed]
                .iter()
                .try_for_each(|&byte| validator.push(byte))
            {
                self.failed = true;
                return Err(error);
            }
            self.reasoning_remaining -= consumed;
        }
        self.text_remaining -= before - text.len();
        if written > 0 {
            self.ready = false;
            return Ok(EncodeStep::Wrote(written));
        }
        self.ready = !self.finished;
        Ok(if self.finished {
            EncodeStep::Finished
        } else {
            EncodeStep::NeedEvent
        })
    }

    pub fn finish(&mut self) -> Result<(), CodecError> {
        if self.failed {
            return Err(CodecError::Contract("encoder previously failed"));
        }
        if self.finished || !self.output.empty() || !self.ready {
            self.failed = true;
            return Err(CodecError::Contract(
                "finish requires NeedEvent and a fresh finalization",
            ));
        }
        if !matches!(self.phase, Phase::Outside)
            || !self.has_message
            || self.output_format.processing()
            || self.reasoning.is_some()
        {
            self.failed = true;
            return Err(CodecError::InvalidInput(
                "request requires complete text messages",
            ));
        }
        if !self.messages_closed {
            self.output.literal(b"]");
        }
        if self.format == Format::Responses {
            if let Some(value) = self.responses_store {
                self.output.literal(if value {
                    b",\"store\":true"
                } else {
                    b",\"store\":false"
                });
            }
            if self.include_reasoning == Some(true) {
                self.output
                    .literal(b",\"include\":[\"reasoning.encrypted_content\"]");
            }
        }
        self.output.literal(match self.format {
            Format::Chat => b",\"max_completion_tokens\":",
            Format::Responses => b",\"max_output_tokens\":",
            Format::Anthropic => b",\"max_tokens\":",
        });
        let mut value = self.configuration.max_output_tokens;
        self.number_start = self.number.len();
        while value > 0 {
            self.number_start -= 1;
            self.number[self.number_start] = b'0' + (value % 10) as u8;
            value /= 10;
        }
        self.output.push(Segment::text(
            TextSource::Number,
            self.number.len() - self.number_start,
        ));
        if self.format != Format::Anthropic
            && let Some(value) = self.top_p
        {
            self.output.literal(b",\"top_p\":");
            self.output
                .push(Segment::Number(JsonFloat::try_from(value)?, 0));
        }
        if self.stops.count() > 0 {
            self.output.literal(if self.format == Format::Anthropic {
                b",\"stop_sequences\":"
            } else {
                b",\"stop\":"
            });
            self.output.push(Segment::Stops(StopWriter::default()));
        }
        self.output.literal(
            self.format
                .streaming_suffix(self.stream, self.include_usage),
        );
        self.finished = true;
        self.ready = false;
        Ok(())
    }
}
