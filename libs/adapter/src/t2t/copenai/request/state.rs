use crate::decoder::RequestDecoderError;

/// Protocol positions and byte-level parsing progress for compatible Chat JSON.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CompatibleOpenAIDecoderState {
    Request,
    Model,
    Stream,
    StreamOptions,
    StreamOptionsIncludeUsage,
    MaxCompletionTokens,
    TopP,
    TopK,
    Stop,
    StopSequence,
    ParallelToolCalls,

    Messages,
    Message,
    MessageRole,
    MessageContent,
    ContentPart,
    ContentPartType,
    ContentPartText,
    /// messages[].tool_call_id; a tool result is an ordinary tool-role message.
    MessageToolCallId,

    Tools,
    Tool,
    ToolType,
    ToolCalls,
    ToolCall,
    ToolCallId,
    ToolCallType,
    /// The nested function object in a tool, tool call, or named tool choice.
    Function,
    FunctionName,
    FunctionDescription,
    FunctionParameters,
    FunctionStrict,
    /// String-valued JSON arguments, not a nested JSON object on the wire.
    FunctionArguments,
    ToolChoice,
    ToolChoiceType,

    /// Assistant history fields, not root-level reasoning generation controls.
    MessageReasoning,
    ReasoningDetails,
    ReasoningDetail,
    ReasoningDetailType,
    ReasoningDetailId,
    ReasoningDetailFormat,
    ReasoningDetailIndex,
    ReasoningDetailSummary,
    ReasoningDetailText,
    ReasoningDetailData,
    ReasoningDetailSignature,

    ResponseFormat,
    ResponseFormatType,
    ResponseFormatJsonSchema,
    ResponseFormatJsonSchemaName,
    ResponseFormatJsonSchemaDescription,
    ResponseFormatJsonSchemaStrict,
    ResponseFormatJsonSchemaSchema,
    Provider,
    ProviderRequireParameters,

    Object {
        position: Box<Self>,
        parent: Box<Self>,
        phase: ObjectPhase,
    },
    Array {
        position: Box<Self>,
        parent: Box<Self>,
        phase: ArrayPhase,
    },
    Key {
        object: Box<Self>,
        text: String,
        progress: StringProgress,
    },
    Colon {
        object: Box<Self>,
        field: Box<Self>,
    },
    Value {
        position: Box<Self>,
        parent: Box<Self>,
    },
    String {
        position: Box<Self>,
        parent: Box<Self>,
        progress: StringProgress,
    },
    Number {
        position: Box<Self>,
        parent: Box<Self>,
        progress: NumberProgress,
    },
    Literal {
        position: Box<Self>,
        parent: Box<Self>,
        expected: &'static [u8],
        offset: usize,
    },
    Finished,
}

impl CompatibleOpenAIDecoderState {
    pub fn new() -> Self {
        Self::Request
    }

    pub fn next(self, byte: u8) -> Result<Self, RequestDecoderError> {
        use CompatibleOpenAIDecoderState::*;

        if whitespace(byte)
            && !matches!(
                &self,
                Key { .. } | String { .. } | Number { .. } | Literal { .. }
            )
        {
            return Ok(self);
        }

        match self {
            Request if byte == b'{' => Ok(Object {
                position: Box::new(Request),
                parent: Box::new(Finished),
                phase: ObjectPhase::KeyOrEnd,
            }),
            Object {
                position,
                parent,
                phase,
            } => match (phase, byte) {
                (ObjectPhase::KeyOrEnd | ObjectPhase::CommaOrEnd, b'}') => Ok(*parent),
                (ObjectPhase::KeyOrEnd | ObjectPhase::Key, b'"') => Ok(Key {
                    object: Box::new(Object {
                        position,
                        parent,
                        phase,
                    }),
                    text: std::string::String::new(),
                    progress: StringProgress::default(),
                }),
                (ObjectPhase::CommaOrEnd, b',') => Ok(Object {
                    position,
                    parent,
                    phase: ObjectPhase::Key,
                }),
                _ => Err(invalid()),
            },
            Key {
                object,
                mut text,
                mut progress,
            } => {
                match progress.push(byte)? {
                    StringStep::Character(character) if !object.position().opaque() => {
                        if text.len() + character.len_utf8() > 64 {
                            return Err(RequestDecoderError::LimitExceeded("field name"));
                        }
                        text.push(character);
                    }
                    StringStep::End => {
                        let field = object
                            .position()
                            .clone()
                            .field(&text)
                            .ok_or(RequestDecoderError::Unsupported("request field"))?;
                        return Ok(Colon {
                            object,
                            field: Box::new(field),
                        });
                    }
                    _ => {}
                }
                Ok(Key {
                    object,
                    text,
                    progress,
                })
            }
            Colon { object, field } if byte == b':' => {
                let Object {
                    position, parent, ..
                } = *object
                else {
                    return Err(RequestDecoderError::InvalidState);
                };
                Ok(Value {
                    position: field,
                    parent: Box::new(Object {
                        position,
                        parent,
                        phase: ObjectPhase::CommaOrEnd,
                    }),
                })
            }
            Value { position, parent } => Self::begin_value(position, parent, byte),
            Array {
                position,
                parent,
                phase,
            } => match (phase, byte) {
                (ArrayPhase::ValueOrEnd | ArrayPhase::CommaOrEnd, b']') => Ok(*parent),
                (ArrayPhase::CommaOrEnd, b',') => Ok(Array {
                    position,
                    parent,
                    phase: ArrayPhase::Value,
                }),
                (ArrayPhase::ValueOrEnd | ArrayPhase::Value, _) => {
                    let item = position.as_ref().clone().item().ok_or_else(invalid)?;
                    Self::begin_value(
                        Box::new(item),
                        Box::new(Array {
                            position,
                            parent,
                            phase: ArrayPhase::CommaOrEnd,
                        }),
                        byte,
                    )
                }
                _ => Err(invalid()),
            },
            String {
                position,
                parent,
                mut progress,
            } => {
                if progress.push(byte)? == StringStep::End {
                    Ok(*parent)
                } else {
                    Ok(String {
                        position,
                        parent,
                        progress,
                    })
                }
            }
            Number {
                position,
                parent,
                progress,
            } => {
                if delimiter(byte) {
                    if !progress.complete() {
                        return Err(invalid());
                    }
                    parent.next(byte)
                } else {
                    Ok(Number {
                        position,
                        parent,
                        progress: progress.next(byte)?,
                    })
                }
            }
            Literal {
                position,
                parent,
                expected,
                offset,
            } => {
                if offset == expected.len() {
                    if !delimiter(byte) {
                        return Err(invalid());
                    }
                    parent.next(byte)
                } else if expected[offset] == byte {
                    Ok(Literal {
                        position,
                        parent,
                        expected,
                        offset: offset + 1,
                    })
                } else {
                    Err(invalid())
                }
            }
            _ => Err(invalid()),
        }
    }

    pub fn position(&self) -> &Self {
        match self {
            Self::Object { position, .. }
            | Self::Array { position, .. }
            | Self::Value { position, .. }
            | Self::String { position, .. }
            | Self::Number { position, .. }
            | Self::Literal { position, .. } => position,
            Self::Key { object, .. } => object.position(),
            Self::Colon { field, .. } => field,
            position => position,
        }
    }

    pub fn finish(&self) -> Result<(), RequestDecoderError> {
        if matches!(self, Self::Finished) {
            Ok(())
        } else {
            Err(RequestDecoderError::IncompleteInput)
        }
    }

    fn begin_value(
        position: Box<Self>,
        parent: Box<Self>,
        byte: u8,
    ) -> Result<Self, RequestDecoderError> {
        if !position.accepts(byte) {
            return Err(invalid());
        }
        if matches!(byte, b'{' | b'[') && parent.depth() >= 64 {
            return Err(RequestDecoderError::LimitExceeded("JSON nesting"));
        }
        Ok(match byte {
            b'{' => Self::Object {
                position,
                parent,
                phase: ObjectPhase::KeyOrEnd,
            },
            b'[' => Self::Array {
                position,
                parent,
                phase: ArrayPhase::ValueOrEnd,
            },
            b'"' => Self::String {
                position,
                parent,
                progress: StringProgress::default(),
            },
            b'-' => Self::Number {
                position,
                parent,
                progress: NumberProgress::Minus,
            },
            b'0' => Self::Number {
                position,
                parent,
                progress: NumberProgress::Zero,
            },
            b'1'..=b'9' => Self::Number {
                position,
                parent,
                progress: NumberProgress::Integer,
            },
            b't' => Self::Literal {
                position,
                parent,
                expected: b"true",
                offset: 1,
            },
            b'f' => Self::Literal {
                position,
                parent,
                expected: b"false",
                offset: 1,
            },
            b'n' => Self::Literal {
                position,
                parent,
                expected: b"null",
                offset: 1,
            },
            _ => return Err(invalid()),
        })
    }

    fn depth(&self) -> usize {
        let mut depth = 0;
        let mut state = self;
        loop {
            match state {
                Self::Object { parent, .. } | Self::Array { parent, .. } => {
                    depth += 1;
                    state = parent;
                }
                _ => return depth,
            }
        }
    }

    fn opaque(&self) -> bool {
        matches!(
            self,
            Self::FunctionParameters | Self::ResponseFormatJsonSchemaSchema
        )
    }

    fn accepts(&self, byte: u8) -> bool {
        use CompatibleOpenAIDecoderState::*;
        if self.opaque() {
            return true;
        }
        match self {
            Messages | Tools | ToolCalls | ReasoningDetails => byte == b'[',
            Message
            | Tool
            | ToolCall
            | ContentPart
            | Function
            | ReasoningDetail
            | ResponseFormat
            | ResponseFormatJsonSchema
            | Provider => byte == b'{',
            StreamOptions => matches!(byte, b'{' | b'n'),
            ToolChoice => matches!(byte, b'{' | b'"'),
            MessageContent => matches!(byte, b'"' | b'[' | b'n'),
            Stop => matches!(byte, b'"' | b'[' | b'n'),
            MessageReasoning | ReasoningDetailId | ReasoningDetailSignature => {
                matches!(byte, b'"' | b'n')
            }
            Stream | StreamOptionsIncludeUsage | ParallelToolCalls | ProviderRequireParameters => {
                matches!(byte, b't' | b'f')
            }
            FunctionStrict | ResponseFormatJsonSchemaStrict => matches!(byte, b't' | b'f' | b'n'),
            TopP => matches!(byte, b'-' | b'0'..=b'9' | b'n'),
            TopK | MaxCompletionTokens | ReasoningDetailIndex => byte.is_ascii_digit(),
            _ => byte == b'"',
        }
    }

    pub fn field(self, key: &str) -> Option<Self> {
        use CompatibleOpenAIDecoderState::*;

        Some(match (self, key) {
            (Request, "model") => Model,
            (Request, "messages") => Messages,
            (Request, "tools") => Tools,
            (Request, "tool_choice") => ToolChoice,
            (Request, "parallel_tool_calls") => ParallelToolCalls,
            (Request, "max_completion_tokens") => MaxCompletionTokens,
            (Request, "top_p") => TopP,
            (Request, "top_k") => TopK,
            (Request, "stop") => Stop,
            (Request, "stream") => Stream,
            (Request, "stream_options") => StreamOptions,
            (Request, "response_format") => ResponseFormat,
            (Request, "provider") => Provider,
            (StreamOptions, "include_usage") => StreamOptionsIncludeUsage,
            (Message, "role") => MessageRole,
            (Message, "content") => MessageContent,
            (Message, "tool_calls") => ToolCalls,
            (Message, "tool_call_id") => MessageToolCallId,
            (Message, "reasoning" | "reasoning_content") => MessageReasoning,
            (Message, "reasoning_details") => ReasoningDetails,
            (ContentPart, "type") => ContentPartType,
            (ContentPart, "text") => ContentPartText,
            (Tool, "type") => ToolType,
            (Tool | ToolCall | ToolChoice, "function") => Function,
            (ToolCall, "id") => ToolCallId,
            (ToolCall, "type") => ToolCallType,
            (ToolChoice, "type") => ToolChoiceType,
            (Function, "name") => FunctionName,
            (Function, "description") => FunctionDescription,
            (Function, "parameters") => FunctionParameters,
            (Function, "strict") => FunctionStrict,
            (Function, "arguments") => FunctionArguments,
            (ReasoningDetail, "type") => ReasoningDetailType,
            (ReasoningDetail, "id") => ReasoningDetailId,
            (ReasoningDetail, "format") => ReasoningDetailFormat,
            (ReasoningDetail, "index") => ReasoningDetailIndex,
            (ReasoningDetail, "summary") => ReasoningDetailSummary,
            (ReasoningDetail, "text") => ReasoningDetailText,
            (ReasoningDetail, "data") => ReasoningDetailData,
            (ReasoningDetail, "signature") => ReasoningDetailSignature,
            (ResponseFormat, "type") => ResponseFormatType,
            (ResponseFormat, "json_schema") => ResponseFormatJsonSchema,
            (ResponseFormatJsonSchema, "name") => ResponseFormatJsonSchemaName,
            (ResponseFormatJsonSchema, "description") => ResponseFormatJsonSchemaDescription,
            (ResponseFormatJsonSchema, "strict") => ResponseFormatJsonSchemaStrict,
            (ResponseFormatJsonSchema, "schema") => ResponseFormatJsonSchemaSchema,
            (Provider, "require_parameters") => ProviderRequireParameters,
            (position @ (FunctionParameters | ResponseFormatJsonSchemaSchema), _) => position,
            _ => return None,
        })
    }

    pub fn item(self) -> Option<Self> {
        use CompatibleOpenAIDecoderState::*;

        Some(match self {
            Messages => Message,
            Tools => Tool,
            ToolCalls => ToolCall,
            MessageContent => ContentPart,
            ReasoningDetails => ReasoningDetail,
            Stop => StopSequence,
            FunctionParameters | ResponseFormatJsonSchemaSchema => self,
            _ => return None,
        })
    }
}

impl Default for CompatibleOpenAIDecoderState {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ObjectPhase {
    KeyOrEnd,
    Key,
    CommaOrEnd,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArrayPhase {
    ValueOrEnd,
    Value,
    CommaOrEnd,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NumberProgress {
    Minus,
    Zero,
    Integer,
    Point,
    Fraction,
    Exponent,
    ExponentSign,
    ExponentDigits,
}

impl NumberProgress {
    fn next(self, byte: u8) -> Result<Self, RequestDecoderError> {
        use NumberProgress::*;
        Ok(match (self, byte) {
            (Minus, b'0') => Zero,
            (Minus, b'1'..=b'9') => Integer,
            (Integer, b'0'..=b'9') => Integer,
            (Zero | Integer, b'.') => Point,
            (Point | Fraction, b'0'..=b'9') => Fraction,
            (Zero | Integer | Fraction, b'e' | b'E') => Exponent,
            (Exponent, b'+' | b'-') => ExponentSign,
            (Exponent | ExponentSign | ExponentDigits, b'0'..=b'9') => ExponentDigits,
            _ => return Err(invalid()),
        })
    }

    fn complete(self) -> bool {
        matches!(
            self,
            Self::Zero | Self::Integer | Self::Fraction | Self::ExponentDigits
        )
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct StringProgress {
    escape: EscapeProgress,
    utf8: [u8; 4],
    length: usize,
    expected: usize,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum EscapeProgress {
    #[default]
    None,
    Escape,
    Unicode {
        value: u16,
        digits: u8,
        high: Option<u16>,
    },
    LowSlash(u16),
    LowU(u16),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum StringStep {
    Pending,
    Character(char),
    End,
}

impl StringProgress {
    fn push(&mut self, byte: u8) -> Result<StringStep, RequestDecoderError> {
        use EscapeProgress::*;
        if self.expected > 0 {
            if !(0x80..=0xbf).contains(&byte) {
                return Err(invalid());
            }
            self.utf8[self.length] = byte;
            self.length += 1;
            if self.length < self.expected {
                return Ok(StringStep::Pending);
            }
            let text = std::str::from_utf8(&self.utf8[..self.length]).map_err(|_| invalid())?;
            let character = text.chars().next().ok_or_else(invalid)?;
            self.expected = 0;
            self.length = 0;
            return Ok(StringStep::Character(character));
        }
        match self.escape {
            Escape => {
                self.escape = None;
                let character = match byte {
                    b'"' => '"',
                    b'\\' => '\\',
                    b'/' => '/',
                    b'b' => '\u{8}',
                    b'f' => '\u{c}',
                    b'n' => '\n',
                    b'r' => '\r',
                    b't' => '\t',
                    b'u' => {
                        self.escape = Unicode {
                            value: 0,
                            digits: 0,
                            high: Option::None,
                        };
                        return Ok(StringStep::Pending);
                    }
                    _ => return Err(invalid()),
                };
                Ok(StringStep::Character(character))
            }
            Unicode {
                value,
                digits,
                high,
            } => {
                let digit = (byte as char).to_digit(16).ok_or_else(invalid)? as u16;
                let value = (value << 4) | digit;
                if digits < 3 {
                    self.escape = Unicode {
                        value,
                        digits: digits + 1,
                        high,
                    };
                    return Ok(StringStep::Pending);
                }
                if let Some(high) = high {
                    if !(0xdc00..=0xdfff).contains(&value) {
                        return Err(invalid());
                    }
                    self.escape = None;
                    let scalar =
                        0x10000 + ((u32::from(high) - 0xd800) << 10) + (u32::from(value) - 0xdc00);
                    return Ok(StringStep::Character(
                        char::from_u32(scalar).ok_or_else(invalid)?,
                    ));
                }
                if (0xd800..=0xdbff).contains(&value) {
                    self.escape = LowSlash(value);
                    return Ok(StringStep::Pending);
                }
                self.escape = None;
                Ok(StringStep::Character(
                    char::from_u32(u32::from(value)).ok_or_else(invalid)?,
                ))
            }
            LowSlash(high) if byte == b'\\' => {
                self.escape = LowU(high);
                Ok(StringStep::Pending)
            }
            LowU(high) if byte == b'u' => {
                self.escape = Unicode {
                    value: 0,
                    digits: 0,
                    high: Some(high),
                };
                Ok(StringStep::Pending)
            }
            LowSlash(_) | LowU(_) => Err(invalid()),
            None => match byte {
                b'"' => Ok(StringStep::End),
                b'\\' => {
                    self.escape = Escape;
                    Ok(StringStep::Pending)
                }
                0..=0x1f => Err(invalid()),
                0x20..=0x7f => Ok(StringStep::Character(byte as char)),
                0xc2..=0xf4 => {
                    self.expected = match byte {
                        0xc2..=0xdf => 2,
                        0xe0..=0xef => 3,
                        _ => 4,
                    };
                    self.utf8[0] = byte;
                    self.length = 1;
                    Ok(StringStep::Pending)
                }
                _ => Err(invalid()),
            },
        }
    }
}

fn whitespace(byte: u8) -> bool {
    matches!(byte, b' ' | b'\t' | b'\r' | b'\n')
}

fn delimiter(byte: u8) -> bool {
    whitespace(byte) || matches!(byte, b',' | b'}' | b']')
}

fn invalid() -> RequestDecoderError {
    RequestDecoderError::InvalidInput("invalid request state transition")
}
