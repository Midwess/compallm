use crate::decoder::DecoderError;
use crate::json::JsonToken;
use crate::t2t::component::{
    Content, FunctionCall, Message, ReasoningDetail, ReasoningDetailContent, Role,
    TextRequestComponent, ToolCall, ToolType,
};

use super::CompatibleOpenAIRequestDecoder;

#[derive(Debug, Default)]
struct ReasoningState {
    phase: CallPhase,
    fields: u8,
    key: u8,
    kind: Option<ReasoningKind>,
    summary: Option<String>,
    text: Option<String>,
    signature: Option<String>,
    data: Option<String>,
    id: Option<String>,
    format: Option<String>,
    index: Option<u32>,
    details: Vec<ReasoningDetail>,
}

#[derive(Debug)]
enum ReasoningKind {
    Summary,
    Text,
    Encrypted,
}

impl CompatibleOpenAIRequestDecoder {
    fn reasoning_details(
        state: &mut ReasoningState,
        token: JsonToken<'_>,
    ) -> Result<Option<Vec<ReasoningDetail>>, DecoderError> {
        use CallPhase as P;
        use JsonToken as T;
        let invalid = || DecoderError::InvalidInput("invalid reasoning detail");
        match (state.phase, token) {
            (P::ItemOrEnd | P::ItemCommaOrEnd, T::ArrayEnd) => {
                return Ok(Some(std::mem::take(&mut state.details)));
            }
            (P::ItemOrEnd | P::Item, T::ObjectStart) => {
                state.fields = 0;
                state.phase = P::KeyOrEnd;
            }
            (P::KeyOrEnd | P::Key, T::String(key)) => {
                state.key = match key {
                    "type" => 1,
                    "summary" => 2,
                    "text" => 4,
                    "signature" => 8,
                    "data" => 16,
                    "id" => 32,
                    "format" => 64,
                    "index" => 128,
                    _ => return Err(DecoderError::Unsupported("reasoning detail field")),
                };
                if state.fields & state.key != 0 {
                    return Err(invalid());
                }
                state.fields |= state.key;
                state.phase = P::Colon;
            }
            (P::Colon, T::Colon) => state.phase = P::Value,
            (P::Value, token) => {
                match (state.key, token) {
                    (1, T::String(kind)) => {
                        state.kind = Some(match kind {
                            "summary" | "reasoning.summary" => ReasoningKind::Summary,
                            "text" | "reasoning.text" => ReasoningKind::Text,
                            "encrypted" | "reasoning.encrypted" => ReasoningKind::Encrypted,
                            _ => return Err(DecoderError::Unsupported("reasoning detail type")),
                        })
                    }
                    (128, T::Number(text)) if text.bytes().all(|byte| byte.is_ascii_digit()) => {
                        state.index = Some(text.parse().map_err(|_| invalid())?);
                    }
                    (8 | 32 | 64 | 128, T::Null) => {}
                    (key, T::String(text)) => {
                        let target = match key {
                            2 => &mut state.summary,
                            4 => &mut state.text,
                            8 => &mut state.signature,
                            16 => &mut state.data,
                            32 => &mut state.id,
                            64 => &mut state.format,
                            _ => return Err(invalid()),
                        };
                        *target = Some(text.to_owned());
                    }
                    _ => return Err(invalid()),
                }
                state.phase = P::CommaOrEnd;
            }
            (P::CommaOrEnd, T::Comma) => state.phase = P::Key,
            (P::KeyOrEnd | P::CommaOrEnd, T::ObjectEnd) => {
                let content = match state.kind.take().ok_or_else(invalid)? {
                    ReasoningKind::Summary if state.fields & 30 == 2 => {
                        ReasoningDetailContent::Summary {
                            summary: state.summary.take().ok_or_else(invalid)?,
                        }
                    }
                    ReasoningKind::Text if state.fields & 22 == 4 => ReasoningDetailContent::Text {
                        text: state.text.take().ok_or_else(invalid)?,
                        signature: state.signature.take(),
                    },
                    ReasoningKind::Encrypted if state.fields & 30 == 16 => {
                        ReasoningDetailContent::Encrypted {
                            data: state.data.take().ok_or_else(invalid)?,
                        }
                    }
                    _ => return Err(invalid()),
                };
                state.details.push(ReasoningDetail {
                    content,
                    id: state.id.take(),
                    format: state.format.take(),
                    index: state.index.take(),
                });
                state.phase = P::ItemCommaOrEnd;
            }
            (P::ItemCommaOrEnd, T::Comma) => state.phase = P::Item,
            _ => return Err(invalid()),
        }
        Ok(None)
    }
}

#[derive(Debug, Default)]
struct ToolCallState {
    phase: CallPhase,
    function: bool,
    key: u8,
    fields: u8,
    function_fields: u8,
    id: Option<String>,
    name: Option<String>,
    arguments: Option<String>,
    calls: Vec<ToolCall>,
}

#[derive(Clone, Copy, Debug, Default)]
enum CallPhase {
    #[default]
    ItemOrEnd,
    Item,
    KeyOrEnd,
    Key,
    Colon,
    Value,
    CommaOrEnd,
    ItemCommaOrEnd,
}

impl CompatibleOpenAIRequestDecoder {
    fn tool_result(token: JsonToken<'_>) -> Result<String, DecoderError> {
        match token {
            JsonToken::String(id) if !id.is_empty() => Ok(id.to_owned()),
            _ => Err(DecoderError::InvalidInput(
                "tool result requires a nonempty call ID",
            )),
        }
    }

    fn tool_call(
        state: &mut ToolCallState,
        token: JsonToken<'_>,
    ) -> Result<Option<Vec<ToolCall>>, DecoderError> {
        use CallPhase as P;
        use JsonToken as T;
        let invalid = || DecoderError::InvalidInput("invalid function tool call");
        match (state.phase, token) {
            (P::ItemOrEnd | P::ItemCommaOrEnd, T::ArrayEnd) => {
                return Ok(Some(std::mem::take(&mut state.calls)));
            }
            (P::ItemOrEnd | P::Item, T::ObjectStart) => {
                state.fields = 0;
                state.function_fields = 0;
                state.phase = P::KeyOrEnd;
            }
            (P::KeyOrEnd | P::Key, T::String(key)) => {
                state.key = if state.function {
                    match key {
                        "name" => 1,
                        "arguments" => 2,
                        _ => return Err(DecoderError::Unsupported("tool function field")),
                    }
                } else {
                    match key {
                        "id" => 1,
                        "type" => 2,
                        "function" => 4,
                        _ => return Err(DecoderError::Unsupported("tool call field")),
                    }
                };
                let fields = if state.function {
                    &mut state.function_fields
                } else {
                    &mut state.fields
                };
                if *fields & state.key != 0 {
                    return Err(invalid());
                }
                *fields |= state.key;
                state.phase = P::Colon;
            }
            (P::Colon, T::Colon) => state.phase = P::Value,
            (P::Value, T::ObjectStart) if !state.function && state.key == 4 => {
                state.function = true;
                state.phase = P::KeyOrEnd;
            }
            (P::Value, T::String(text)) => {
                match (state.function, state.key) {
                    (false, 1) if !text.is_empty() => state.id = Some(text.to_owned()),
                    (false, 2) if text == "function" => {}
                    (true, 1) if !text.is_empty() => state.name = Some(text.to_owned()),
                    (true, 2) => state.arguments = Some(text.to_owned()),
                    _ => return Err(invalid()),
                }
                state.phase = P::CommaOrEnd;
            }
            (P::CommaOrEnd, T::Comma) => state.phase = P::Key,
            (P::KeyOrEnd | P::CommaOrEnd, T::ObjectEnd) => {
                if state.function {
                    if state.function_fields != 3 {
                        return Err(invalid());
                    }
                    state.function = false;
                    state.phase = P::CommaOrEnd;
                } else {
                    if state.fields != 7 || state.function_fields != 3 {
                        return Err(invalid());
                    }
                    state.calls.push(ToolCall {
                        id: state.id.take().ok_or_else(invalid)?,
                        kind: ToolType::Function,
                        function: FunctionCall {
                            name: state.name.take().ok_or_else(invalid)?,
                            arguments: state.arguments.take().ok_or_else(invalid)?,
                        },
                    });
                    state.phase = P::ItemCommaOrEnd;
                }
            }
            (P::ItemCommaOrEnd, T::Comma) => state.phase = P::Item,
            _ => return Err(invalid()),
        }
        Ok(None)
    }
}

#[derive(Debug, Default)]
pub(super) struct MessageState {
    phase: Phase,
    field: Field,
    fields: u8,
    role: Option<Role>,
    content: Option<Content>,
    tool_calls: Option<Vec<ToolCall>>,
    tool_call_id: Option<String>,
    call: ToolCallState,
    reasoning: Option<String>,
    reasoning_content: Option<String>,
    reasoning_details: Option<Vec<ReasoningDetail>>,
    detail: ReasoningState,
}

#[derive(Clone, Copy, Debug, Default)]
enum Phase {
    #[default]
    KeyOrEnd,
    Key,
    Colon,
    Value,
    CommaOrEnd,
    ToolCalls,
    ReasoningDetails,
}

#[derive(Clone, Copy, Debug, Default)]
enum Field {
    #[default]
    Role,
    Content,
    ToolCalls,
    ToolCallId,
    Reasoning,
    ReasoningContent,
    ReasoningDetails,
}

impl CompatibleOpenAIRequestDecoder {
    pub(super) fn message(
        state: &mut MessageState,
        token: JsonToken<'_>,
    ) -> Result<Option<TextRequestComponent>, DecoderError> {
        use JsonToken as T;
        let invalid = || DecoderError::InvalidInput("invalid message");
        match (state.phase, token) {
            (Phase::KeyOrEnd | Phase::Key, T::String(key)) => {
                let (field, bit) = match key {
                    "role" => (Field::Role, 1),
                    "content" => (Field::Content, 2),
                    "tool_calls" => (Field::ToolCalls, 4),
                    "tool_call_id" => (Field::ToolCallId, 8),
                    "reasoning" => (Field::Reasoning, 16),
                    "reasoning_content" => (Field::ReasoningContent, 32),
                    "reasoning_details" => (Field::ReasoningDetails, 64),
                    _ => return Err(DecoderError::Unsupported("message field")),
                };
                if state.fields & bit != 0 {
                    return Err(invalid());
                }
                state.fields |= bit;
                state.field = field;
                state.phase = Phase::Colon;
            }
            (Phase::Colon, T::Colon) => state.phase = Phase::Value,
            (Phase::Value, token) => {
                match (state.field, token) {
                    (Field::Role, T::String(role)) => {
                        state.role = Some(match role {
                            "system" => Role::System,
                            "developer" => Role::Developer,
                            "user" => Role::User,
                            "assistant" => Role::Assistant,
                            "tool" => Role::Tool,
                            _ => return Err(invalid()),
                        })
                    }
                    (Field::Content, T::String(text)) => {
                        state.content = Some(Content::Text(text.to_owned()))
                    }
                    (Field::Content, T::Null) => state.content = Some(Content::Null),
                    (Field::Reasoning, T::String(text)) => state.reasoning = Some(text.to_owned()),
                    (Field::ReasoningContent, T::String(text)) => {
                        state.reasoning_content = Some(text.to_owned())
                    }
                    (Field::Reasoning | Field::ReasoningContent, T::Null) => {}
                    (Field::ReasoningDetails, T::ArrayStart) => {
                        state.detail = ReasoningState::default();
                        state.phase = Phase::ReasoningDetails;
                        return Ok(None);
                    }
                    (Field::ToolCallId, token) => {
                        state.tool_call_id = Some(Self::tool_result(token)?)
                    }
                    (Field::ToolCalls, T::ArrayStart) => {
                        state.call = ToolCallState::default();
                        state.phase = Phase::ToolCalls;
                        return Ok(None);
                    }
                    _ => return Err(invalid()),
                }
                state.phase = Phase::CommaOrEnd;
            }
            (Phase::CommaOrEnd, T::Comma) => state.phase = Phase::Key,
            (Phase::ToolCalls, token) => {
                if let Some(calls) = Self::tool_call(&mut state.call, token)? {
                    state.tool_calls = Some(calls);
                    state.phase = Phase::CommaOrEnd;
                }
            }
            (Phase::ReasoningDetails, token) => {
                if let Some(details) = Self::reasoning_details(&mut state.detail, token)? {
                    state.reasoning_details = Some(details);
                    state.phase = Phase::CommaOrEnd;
                }
            }
            (Phase::KeyOrEnd | Phase::CommaOrEnd, T::ObjectEnd) => {
                let role = state.role.take().ok_or_else(invalid)?;
                let text = matches!(state.content, Some(Content::Text(_)));
                let calls = state
                    .tool_calls
                    .as_ref()
                    .is_some_and(|calls| !calls.is_empty());
                let history = calls
                    || state.reasoning.is_some()
                    || state.reasoning_content.is_some()
                    || state
                        .reasoning_details
                        .as_ref()
                        .is_some_and(|details| !details.is_empty());
                if (role != Role::Assistant && state.fields & 116 != 0)
                    || (role == Role::Tool && state.tool_call_id.is_none())
                    || (role != Role::Tool && state.tool_call_id.is_some())
                    || (!text && !(role == Role::Assistant && history))
                {
                    return Err(invalid());
                }
                return Ok(Some(TextRequestComponent::Message(Message {
                    role,
                    content: state.content.take(),
                    tool_calls: state.tool_calls.take(),
                    tool_call_id: state.tool_call_id.take(),
                    reasoning: state.reasoning.take(),
                    reasoning_content: state.reasoning_content.take(),
                    reasoning_details: state.reasoning_details.take(),
                })));
            }
            _ => return Err(invalid()),
        }
        Ok(None)
    }
}
