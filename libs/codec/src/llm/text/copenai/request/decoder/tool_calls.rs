use crate::decoder::DecoderError;
use crate::json::JsonToken;
use crate::llm::text::component::{FunctionCall, ToolCall, ToolType};

#[derive(Debug, Default)]
pub(super) struct ToolCallsState {
    pub(super) active: bool,
    calls: Vec<ToolCall>,
    current: Option<CallDraft>,
}

#[derive(Debug, Default)]
struct CallDraft {
    id: Option<String>,
    has_type: bool,
    has_function: bool,
    in_function: bool,
    name: Option<String>,
    arguments: Option<String>,
}

impl ToolCallsState {
    pub(super) fn proceed(
        &mut self,
        token: JsonToken<'_>,
    ) -> Result<Option<Vec<ToolCall>>, DecoderError> {
        if !self.active {
            if !matches!(token, JsonToken::ValueArray(Some("tool_calls"))) {
                return Err(DecoderError::InvalidInput("tool calls must be an array"));
            }
            self.active = true;
            return Ok(None);
        }
        if let Some(call) = &mut self.current {
            if call.in_function {
                match token {
                    JsonToken::ValueString(Some("name"), value) if !value.is_empty() => {
                        call.name = Some(value.to_owned())
                    }
                    JsonToken::ValueString(Some("arguments"), value) => {
                        call.arguments = Some(value.to_owned())
                    }
                    JsonToken::ObjectEnd => {
                        if call.name.is_none() || call.arguments.is_none() {
                            return Err(DecoderError::InvalidInput("incomplete function call"));
                        }
                        call.in_function = false;
                    }
                    _ => return Err(DecoderError::InvalidInput("invalid function call")),
                }
            } else {
                match token {
                    JsonToken::ValueString(Some("id"), value) if !value.is_empty() => {
                        call.id = Some(value.to_owned())
                    }
                    JsonToken::ValueString(Some("type"), "function") => call.has_type = true,
                    JsonToken::ValueObject(Some("function")) => {
                        call.has_function = true;
                        call.in_function = true;
                    }
                    JsonToken::ObjectEnd => {
                        let call = self.current.take().ok_or(DecoderError::InvalidState)?;
                        self.calls.push(call.try_into()?);
                    }
                    _ => return Err(DecoderError::InvalidInput("invalid tool call")),
                }
            }
            return Ok(None);
        }
        match token {
            JsonToken::ValueObject(None) => {
                self.current = Some(CallDraft::default());
                Ok(None)
            }
            JsonToken::ArrayEnd => {
                self.active = false;
                Ok(Some(std::mem::take(&mut self.calls)))
            }
            _ => Err(DecoderError::InvalidInput("tool call must be an object")),
        }
    }
}

impl TryFrom<CallDraft> for ToolCall {
    type Error = DecoderError;
    fn try_from(call: CallDraft) -> Result<Self, Self::Error> {
        if !call.has_type || !call.has_function {
            return Err(DecoderError::InvalidInput("incomplete tool call"));
        }
        Ok(Self {
            id: call
                .id
                .ok_or(DecoderError::InvalidInput("missing tool call id"))?,
            kind: ToolType::Function,
            function: FunctionCall {
                name: call
                    .name
                    .ok_or(DecoderError::InvalidInput("missing function name"))?,
                arguments: call
                    .arguments
                    .ok_or(DecoderError::InvalidInput("missing function arguments"))?,
            },
        })
    }
}
