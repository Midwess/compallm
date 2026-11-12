use crate::error::CodecError;
use crate::t2t::request::decoder::stop::{CHAT_STOP_COUNT, STOP_BYTES};
use crate::t2t::request::{
    MODEL_BYTES,
    event::{RequestField as F, Structure as S},
    json::Number,
};

pub(crate) fn invalid() -> CodecError {
    CodecError::InvalidInput("invalid compatible Chat request")
}
pub(crate) fn unsupported() -> CodecError {
    CodecError::Unsupported("field or shape outside compatible Chat subset")
}

impl From<F> for &'static str {
    fn from(field: F) -> Self {
        match field {
            F::Model => "model",
            F::Messages => "messages",
            F::Stream => "stream",
            F::MaxTokens => "max_tokens",
            F::MaxCompletionTokens => "max_completion_tokens",
            F::Tools => "tools",
            F::ToolChoice => "tool_choice",
            F::ParallelToolCalls => "parallel_tool_calls",
            F::Role => "role",
            F::Content => "content",
            F::ToolCalls => "tool_calls",
            F::ToolCallId => "tool_call_id",
            F::Reasoning => "reasoning",
            F::ReasoningContent => "reasoning_content",
            F::ReasoningDetails => "reasoning_details",
            F::Id => "id",
            F::Type => "type",
            F::Function => "function",
            F::Name => "name",
            F::Arguments => "arguments",
            F::Description => "description",
            F::Parameters => "parameters",
            F::Strict => "strict",
            F::Text => "text",
            F::Summary => "summary",
            F::Data => "data",
            F::Signature => "signature",
            F::Format => "format",
            F::Index => "index",
            F::StreamOptions => "stream_options",
            F::IncludeUsage => "include_usage",
            F::TopP => "top_p",
            F::TopK => "top_k",
            F::Stop => "stop",
            F::ResponseFormat => "response_format",
            F::JsonSchema => "json_schema",
            F::Schema => "schema",
            F::Provider => "provider",
            F::RequireParameters => "require_parameters",
        }
    }
}
impl TryFrom<&[u8]> for F {
    type Error = CodecError;
    fn try_from(key: &[u8]) -> Result<Self, Self::Error> {
        const FIELDS: &[F] = &[
            F::Model,
            F::Messages,
            F::Stream,
            F::MaxCompletionTokens,
            F::MaxTokens,
            F::Tools,
            F::ToolChoice,
            F::ParallelToolCalls,
            F::Role,
            F::Content,
            F::ToolCalls,
            F::ToolCallId,
            F::Reasoning,
            F::ReasoningContent,
            F::ReasoningDetails,
            F::Id,
            F::Type,
            F::Function,
            F::Name,
            F::Arguments,
            F::Description,
            F::Parameters,
            F::Strict,
            F::Text,
            F::Summary,
            F::Data,
            F::Signature,
            F::Format,
            F::Index,
            F::StreamOptions,
            F::IncludeUsage,
            F::TopP,
            F::TopK,
            F::Stop,
            F::ResponseFormat,
            F::JsonSchema,
            F::Schema,
            F::Provider,
            F::RequireParameters,
        ];
        FIELDS
            .iter()
            .copied()
            .find(|field| <&str>::from(*field).as_bytes() == key)
            .ok_or_else(unsupported)
    }
}

fn bit(field: F) -> u64 {
    1 << field as u8
}
fn fields(list: &[F]) -> u64 {
    list.iter().fold(0, |mask, field| mask | bit(*field))
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Kind {
    Root,
    Message,
    Messages,
    Tool,
    Tools,
    Call,
    Calls,
    CallFunction,
    Definition,
    Choice,
    ChoiceFunction,
    Detail,
    Details,
    Content,
    TextPart,
    JsonObject,
    JsonArray,
    StreamOptions,
    Stops,
    ResponseFormat,
    JsonSchema,
    Provider,
}
impl Kind {
    fn array(self) -> bool {
        matches!(
            self,
            Self::Messages
                | Self::Tools
                | Self::Calls
                | Self::Details
                | Self::Content
                | Self::JsonArray
                | Self::Stops
        )
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
enum Phase {
    Key(bool),
    Value(bool),
    After,
}
#[derive(Clone, Copy)]
struct Frame {
    kind: Kind,
    phase: Phase,
    field: Option<F>,
    seen: u64,
    nonempty: bool,
    role: u8,
    detail: u8,
    null_content: bool,
    payload: bool,
}
impl Frame {
    const EMPTY: Self = Self {
        kind: Kind::Root,
        phase: Phase::Key(true),
        field: None,
        seen: 0,
        nonempty: false,
        role: 0,
        detail: 0,
        null_content: false,
        payload: false,
    };
    fn new(kind: Kind) -> Self {
        Self {
            kind,
            phase: if kind.array() {
                Phase::Value(true)
            } else {
                Phase::Key(true)
            },
            ..Self::EMPTY
        }
    }
}
#[derive(Clone, Copy)]
enum Rule {
    Any,
    Model,
    Nonempty,
    Role,
    Function,
    TextPart,
    Choice,
    Detail,
    Tokens,
    Index,
    Probability,
    Stop,
    OutputType,
    FormatName,
}
#[derive(Clone, Copy)]
enum Expect {
    Object(Kind),
    Array(Kind),
    String(Rule),
    NullableString,
    Content,
    Choice,
    Bool,
    NullableBool,
    StreamOptions,
    Probability,
    Stop,
    OutputFormat,
    Number(Rule),
    Any,
}
#[derive(Clone, Copy, PartialEq, Eq)]
enum ScalarKind {
    String,
    Number,
    Key,
}

struct Scalar {
    kind: ScalarKind,
    rule: Rule,
    bytes: [u8; 256],
    len: usize,
    number: Number,
    integer: u32,
}

pub(crate) struct Machine {
    frames: [Frame; 64],
    depth: usize,
    scalar: Option<Scalar>,
    started: bool,
    pub done: bool,
    implicit: bool,
    pub tokens: Option<u32>,
    pub stream: bool,
    pub include_usage: Option<bool>,
    pub top_p: Option<f64>,
    stop_count: usize,
    stop_bytes: usize,
    options_object: bool,
}
impl Machine {
    pub fn new(implicit: bool) -> Self {
        Self {
            frames: [Frame::EMPTY; 64],
            depth: 0,
            scalar: None,
            started: false,
            done: false,
            implicit,
            tokens: None,
            stream: false,
            include_usage: None,
            top_p: None,
            stop_count: 0,
            stop_bytes: 0,
            options_object: false,
        }
    }
    pub fn kind(&self) -> Option<Kind> {
        self.depth.checked_sub(1).map(|i| self.frames[i].kind)
    }
    pub fn key_expected(&self) -> bool {
        self.depth > 0 && matches!(self.frames[self.depth - 1].phase, Phase::Key(_))
    }
    pub fn after(&self) -> bool {
        self.depth > 0 && self.frames[self.depth - 1].phase == Phase::After
    }
    pub fn field(&self) -> Option<F> {
        self.depth.checked_sub(1).and_then(|i| self.frames[i].field)
    }
    pub fn comma(&mut self) -> Result<(), CodecError> {
        if self.scalar.is_some() || !self.after() {
            return Err(invalid());
        }
        let f = &mut self.frames[self.depth - 1];
        f.phase = if f.kind.array() {
            Phase::Value(false)
        } else {
            Phase::Key(false)
        };
        Ok(())
    }
    fn expected(&self) -> Result<Expect, CodecError> {
        if self.depth == 0 {
            return if !self.started {
                Ok(Expect::Object(Kind::Root))
            } else {
                Err(invalid())
            };
        }
        let f = self.frames[self.depth - 1];
        if !matches!(f.phase, Phase::Value(_)) {
            return Err(invalid());
        }
        Ok(match f.kind {
            Kind::Messages => Expect::Object(Kind::Message),
            Kind::Tools => Expect::Object(Kind::Tool),
            Kind::Calls => Expect::Object(Kind::Call),
            Kind::Details => Expect::Object(Kind::Detail),
            Kind::Content => Expect::Object(Kind::TextPart),
            Kind::Stops => Expect::String(Rule::Stop),
            Kind::JsonObject | Kind::JsonArray => Expect::Any,
            Kind::Root => match f.field {
                Some(F::Model) => Expect::String(Rule::Model),
                Some(F::Messages) => Expect::Array(Kind::Messages),
                Some(F::Stream) => Expect::Bool,
                Some(F::StreamOptions) => Expect::StreamOptions,
                Some(F::TopP) => Expect::Probability,
                Some(F::TopK) => Expect::Number(Rule::Index),
                Some(F::Stop) => Expect::Stop,
                Some(F::ResponseFormat) => Expect::OutputFormat,
                Some(F::Provider) => Expect::Object(Kind::Provider),
                Some(F::MaxTokens | F::MaxCompletionTokens) => Expect::Number(Rule::Tokens),
                Some(F::Tools) => Expect::Array(Kind::Tools),
                Some(F::ToolChoice) => Expect::Choice,
                Some(F::ParallelToolCalls) => Expect::Bool,
                _ => return Err(unsupported()),
            },
            Kind::Message => match f.field {
                Some(F::Role) => Expect::String(Rule::Role),
                Some(F::Content) => Expect::Content,
                Some(F::ToolCalls) => Expect::Array(Kind::Calls),
                Some(F::ToolCallId) => Expect::String(Rule::Nonempty),
                Some(F::Reasoning | F::ReasoningContent) => Expect::NullableString,
                Some(F::ReasoningDetails) => Expect::Array(Kind::Details),
                _ => return Err(unsupported()),
            },
            Kind::ResponseFormat => match f.field {
                Some(F::Type) => Expect::String(Rule::OutputType),
                Some(F::JsonSchema) => Expect::Object(Kind::JsonSchema),
                _ => return Err(unsupported()),
            },
            Kind::Provider => match f.field {
                Some(F::RequireParameters) => Expect::Bool,
                _ => return Err(unsupported()),
            },
            Kind::JsonSchema => match f.field {
                Some(F::Name) => Expect::String(Rule::FormatName),
                Some(F::Description) => Expect::String(Rule::Any),
                Some(F::Strict) => Expect::NullableBool,
                Some(F::Schema) => Expect::Object(Kind::JsonObject),
                _ => return Err(unsupported()),
            },
            Kind::StreamOptions => match f.field {
                Some(F::IncludeUsage) => Expect::Bool,
                _ => return Err(unsupported()),
            },
            Kind::Call | Kind::Tool | Kind::Choice => match f.field {
                Some(F::Type) => Expect::String(Rule::Function),
                Some(F::Id) if f.kind == Kind::Call => Expect::String(Rule::Nonempty),
                Some(F::Function) => Expect::Object(match f.kind {
                    Kind::Call => Kind::CallFunction,
                    Kind::Tool => Kind::Definition,
                    _ => Kind::ChoiceFunction,
                }),
                _ => return Err(unsupported()),
            },
            Kind::CallFunction | Kind::Definition | Kind::ChoiceFunction => match f.field {
                Some(F::Name) => Expect::String(Rule::Nonempty),
                Some(F::Arguments) if f.kind == Kind::CallFunction => Expect::String(Rule::Any),
                Some(F::Description) if f.kind == Kind::Definition => Expect::String(Rule::Any),
                Some(F::Parameters) if f.kind == Kind::Definition => {
                    Expect::Object(Kind::JsonObject)
                }
                Some(F::Strict) if f.kind == Kind::Definition => Expect::NullableBool,
                _ => return Err(unsupported()),
            },
            Kind::Detail => match f.field {
                Some(F::Type) => Expect::String(Rule::Detail),
                Some(F::Id | F::Signature) => Expect::NullableString,
                Some(F::Index) => Expect::Number(Rule::Index),
                Some(F::Text | F::Summary | F::Data | F::Format) => Expect::String(Rule::Any),
                _ => return Err(unsupported()),
            },
            Kind::TextPart => match f.field {
                Some(F::Type) => Expect::String(Rule::TextPart),
                Some(F::Text) => Expect::String(Rule::Any),
                _ => return Err(unsupported()),
            },
        })
    }
    fn value(&mut self) -> Result<Expect, CodecError> {
        if self.scalar.is_some() || self.done {
            return Err(invalid());
        }
        if self.implicit && self.after() && self.kind().is_some_and(Kind::array) {
            self.comma()?;
        }
        let expected = self.expected()?;
        if self.depth > 0 {
            let f = &mut self.frames[self.depth - 1];
            f.phase = Phase::After;
            f.nonempty = true;
        } else {
            self.started = true;
        }
        Ok(expected)
    }
    fn field_event(&mut self, field: F) -> Result<(), CodecError> {
        if self.scalar.is_some() || self.depth == 0 {
            return Err(invalid());
        }
        if self.implicit && self.after() {
            self.comma()?;
        }
        let f = &mut self.frames[self.depth - 1];
        if !matches!(f.phase, Phase::Key(_)) || matches!(f.kind, Kind::JsonObject | Kind::JsonArray)
        {
            return Err(invalid());
        }
        let mask = bit(field);
        let aliases = match field {
            F::MaxTokens | F::MaxCompletionTokens => {
                fields(&[F::MaxTokens, F::MaxCompletionTokens])
            }
            F::Reasoning | F::ReasoningContent => fields(&[F::Reasoning, F::ReasoningContent]),
            _ => mask,
        };
        if f.seen & aliases != 0 {
            return Err(invalid());
        }
        f.seen |= mask;
        f.field = Some(field);
        f.phase = Phase::Value(false);
        self.expected()?;
        Ok(())
    }
    pub fn event(&mut self, event: S) -> Result<(), CodecError> {
        match event {
            S::Field(field) => self.field_event(field),
            S::ObjectStarted | S::ArrayStarted => {
                let expected = self.value()?;
                let kind = match (event, expected) {
                    (S::ObjectStarted, Expect::Object(kind))
                    | (S::ArrayStarted, Expect::Array(kind)) => kind,
                    (S::ObjectStarted, Expect::Choice) => Kind::Choice,
                    (S::ObjectStarted, Expect::StreamOptions) => {
                        self.options_object = true;
                        Kind::StreamOptions
                    }
                    (S::ArrayStarted, Expect::Content) => Kind::Content,
                    (S::ArrayStarted, Expect::Stop) => Kind::Stops,
                    (S::ObjectStarted, Expect::OutputFormat) => Kind::ResponseFormat,
                    (S::ObjectStarted, Expect::Any) => Kind::JsonObject,
                    (S::ArrayStarted, Expect::Any) => Kind::JsonArray,
                    _ => return Err(unsupported()),
                };
                if self.depth == self.frames.len() {
                    return Err(CodecError::LimitExceeded(
                        "JSON nesting exceeds 64 containers",
                    ));
                }
                self.frames[self.depth] = Frame::new(kind);
                self.depth += 1;
                Ok(())
            }
            S::ObjectEnded | S::ArrayEnded => {
                if self.scalar.is_some() || self.depth == 0 {
                    return Err(invalid());
                }
                let f = self.frames[self.depth - 1];
                if f.kind.array() != (event == S::ArrayEnded)
                    || !matches!(
                        f.phase,
                        Phase::After | Phase::Key(true) | Phase::Value(true)
                    )
                {
                    return Err(invalid());
                }
                Self::validate(f)?;
                self.depth -= 1;
                if self.depth == 0 {
                    if self.options_object && !self.stream {
                        return Err(invalid());
                    }
                    self.done = true;
                } else if matches!(f.kind, Kind::Calls | Kind::Details) && f.nonempty {
                    self.frames[self.depth - 1].payload = true;
                }
                Ok(())
            }
            S::StringStarted | S::NumberStarted => {
                let expected = self.value()?;
                let rule = match (event, expected) {
                    (S::StringStarted, Expect::String(rule))
                    | (S::NumberStarted, Expect::Number(rule)) => rule,
                    (S::StringStarted, Expect::Choice) => Rule::Choice,
                    (S::NumberStarted, Expect::Probability) => Rule::Probability,
                    (S::StringStarted, Expect::Stop) => Rule::Stop,
                    (S::StringStarted, Expect::NullableString | Expect::Content | Expect::Any)
                    | (S::NumberStarted, Expect::Any) => Rule::Any,
                    _ => return Err(unsupported()),
                };
                self.scalar = Some(Scalar {
                    kind: if event == S::StringStarted {
                        ScalarKind::String
                    } else {
                        ScalarKind::Number
                    },
                    rule,
                    bytes: [0; 256],
                    len: 0,
                    number: Number::Start,
                    integer: 0,
                });
                Ok(())
            }
            S::KeyStarted => {
                if self.implicit && self.after() {
                    self.comma()?;
                }
                if self.scalar.is_some()
                    || self.kind() != Some(Kind::JsonObject)
                    || !self.key_expected()
                {
                    return Err(invalid());
                }
                self.scalar = Some(Scalar {
                    kind: ScalarKind::Key,
                    rule: Rule::Any,
                    bytes: [0; 256],
                    len: 0,
                    number: Number::Start,
                    integer: 0,
                });
                Ok(())
            }
            S::KeyEnded | S::StringEnded | S::NumberEnded => self.end_scalar(event),
            S::Boolean(value) => {
                let e = self.value()?;
                if matches!(e, Expect::Bool | Expect::NullableBool | Expect::Any) {
                    if self.kind() == Some(Kind::Root) && self.field() == Some(F::Stream) {
                        self.stream = value;
                    }
                    if self.kind() == Some(Kind::StreamOptions) {
                        self.include_usage = Some(value);
                    }
                    Ok(())
                } else {
                    Err(unsupported())
                }
            }
            S::Null => {
                let e = self.value()?;
                if !matches!(
                    e,
                    Expect::NullableString
                        | Expect::NullableBool
                        | Expect::Content
                        | Expect::StreamOptions
                        | Expect::Probability
                        | Expect::Stop
                        | Expect::OutputFormat
                        | Expect::Any
                ) {
                    return Err(unsupported());
                }
                if self.kind() == Some(Kind::Message) && self.field() == Some(F::Content) {
                    self.frames[self.depth - 1].null_content = true;
                }
                Ok(())
            }
        }
    }
    pub fn bytes(&mut self, bytes: &[u8]) -> Result<(), CodecError> {
        let s = self.scalar.as_mut().ok_or_else(invalid)?;
        for &byte in bytes {
            if matches!(s.rule, Rule::FormatName)
                && (s.len == 64 || !matches!(byte,b'a'..=b'z'|b'A'..=b'Z'|b'0'..=b'9'|b'_'|b'-'))
            {
                return Err(invalid());
            }
            if matches!(s.rule, Rule::Stop) {
                if self.stop_bytes == STOP_BYTES {
                    return Err(CodecError::LimitExceeded("stop text exceeds 4096 bytes"));
                }
                self.stop_bytes += 1;
            }
            if s.kind == ScalarKind::Number {
                s.number.push(byte)?;
                if matches!(s.rule, Rule::Tokens | Rule::Index) {
                    if !byte.is_ascii_digit() {
                        return Err(invalid());
                    }
                    s.integer = s
                        .integer
                        .checked_mul(10)
                        .and_then(|n| n.checked_add(u32::from(byte - b'0')))
                        .ok_or_else(invalid)?;
                }
            }
            if !matches!(
                s.rule,
                Rule::Any | Rule::Tokens | Rule::Index | Rule::Model | Rule::Stop
            ) {
                if s.len == s.bytes.len() {
                    return Err(CodecError::LimitExceeded("control value exceeds 256 bytes"));
                }
                s.bytes[s.len] = byte;
            }
            s.len = s.len.saturating_add(1);
            if matches!(s.rule, Rule::Model) && s.len > MODEL_BYTES {
                return Err(CodecError::LimitExceeded(
                    "model exceeds identifier allowance",
                ));
            }
        }
        Ok(())
    }
    fn end_scalar(&mut self, event: S) -> Result<(), CodecError> {
        let s = self.scalar.take().ok_or_else(invalid)?;
        if !matches!(
            (s.kind, event),
            (ScalarKind::String, S::StringEnded)
                | (ScalarKind::Key, S::KeyEnded)
                | (ScalarKind::Number, S::NumberEnded)
        ) {
            return Err(invalid());
        }
        if s.kind == ScalarKind::Key {
            self.frames[self.depth - 1].phase = Phase::Value(false);
            return Ok(());
        }
        if s.kind == ScalarKind::Number && !s.number.complete() {
            return Err(invalid());
        }
        let text = &s.bytes[..s.len.min(s.bytes.len())];
        match s.rule {
            Rule::Model | Rule::Nonempty if s.len == 0 => return Err(invalid()),
            Rule::FormatName if s.len == 0 => return Err(invalid()),
            Rule::OutputType => {
                self.frames[self.depth - 1].detail = match text {
                    b"text" => 1,
                    b"json_object" => 2,
                    b"json_schema" => 3,
                    _ => return Err(unsupported()),
                }
            }
            Rule::Function if text != b"function" => return Err(unsupported()),
            Rule::TextPart if text != b"text" => return Err(unsupported()),
            Rule::Choice if !matches!(text, b"auto" | b"none" | b"required") => {
                return Err(unsupported());
            }
            Rule::Role => {
                self.frames[self.depth - 1].role = match text {
                    b"system" => 1,
                    b"developer" => 2,
                    b"user" => 3,
                    b"assistant" => 4,
                    b"tool" => 5,
                    _ => return Err(unsupported()),
                }
            }
            Rule::Detail => {
                self.frames[self.depth - 1].detail = match text {
                    b"reasoning.summary" => 1,
                    b"reasoning.text" => 2,
                    b"reasoning.encrypted" => 3,
                    _ => return Err(unsupported()),
                }
            }
            Rule::Tokens => {
                if s.integer == 0 {
                    return Err(invalid());
                }
                self.tokens = Some(s.integer);
            }
            Rule::Probability => self.top_p = Some(s.number.probability(text)?),
            Rule::Stop => {
                if s.len == 0 {
                    return Err(invalid());
                }
                if self.stop_count == CHAT_STOP_COUNT {
                    return Err(CodecError::LimitExceeded("too many stop sequences"));
                }
                self.stop_count += 1;
            }
            _ => {}
        }
        if self.kind() == Some(Kind::Message)
            && matches!(self.field(), Some(F::Reasoning | F::ReasoningContent))
        {
            self.frames[self.depth - 1].payload = true;
        }
        Ok(())
    }
    fn validate(f: Frame) -> Result<(), CodecError> {
        let required = match f.kind {
            Kind::Root => fields(&[F::Model, F::Messages]),
            Kind::ResponseFormat => {
                if f.detail == 3 {
                    fields(&[F::Type, F::JsonSchema])
                } else {
                    if f.seen != bit(F::Type) {
                        return Err(invalid());
                    }
                    bit(F::Type)
                }
            }
            Kind::JsonSchema => fields(&[F::Name, F::Schema]),
            Kind::Message => bit(F::Role),
            Kind::Call => fields(&[F::Id, F::Type, F::Function]),
            Kind::Tool | Kind::Choice => fields(&[F::Type, F::Function]),
            Kind::CallFunction => fields(&[F::Name, F::Arguments]),
            Kind::Definition | Kind::ChoiceFunction => bit(F::Name),
            Kind::TextPart => fields(&[F::Type, F::Text]),
            Kind::Detail => {
                bit(F::Type)
                    | match f.detail {
                        1 => bit(F::Summary),
                        2 => bit(F::Text),
                        3 => bit(F::Data),
                        _ => return Err(invalid()),
                    }
            }
            _ => 0,
        };
        if f.seen & required != required {
            return Err(invalid());
        }
        if matches!(f.kind, Kind::Messages | Kind::Calls) && !f.nonempty {
            return Err(invalid());
        }
        if f.kind == Kind::Message {
            let has_content = f.seen & bit(F::Content) != 0 && !f.null_content;
            let assistant_fields = fields(&[
                F::ToolCalls,
                F::Reasoning,
                F::ReasoningContent,
                F::ReasoningDetails,
            ]);
            if f.role != 4 && f.seen & assistant_fields != 0 {
                return Err(invalid());
            }
            if f.role != 5 && f.seen & bit(F::ToolCallId) != 0 {
                return Err(invalid());
            }
            if f.role == 5 && (!has_content || f.seen & bit(F::ToolCallId) == 0) {
                return Err(invalid());
            }
            if !has_content && (f.role != 4 || !f.payload) {
                return Err(invalid());
            }
        }
        if f.kind == Kind::Detail {
            let allowed = match f.detail {
                1 => fields(&[F::Summary]),
                2 => fields(&[F::Text, F::Signature]),
                3 => fields(&[F::Data]),
                _ => return Err(invalid()),
            } | fields(&[F::Type, F::Id, F::Format, F::Index]);
            if f.seen & !allowed != 0 {
                return Err(invalid());
            }
        }
        Ok(())
    }
}
