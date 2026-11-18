use super::super::component::{CompatibleOpenAIRequestComponent, Decoded, MessageFields};
use super::content::ContentState;
use super::reasoning::ReasoningState;
use super::tool_calls::ToolCallsState;
use crate::decoder::DecoderError;
use crate::json::JsonToken;
use crate::llm::text::component::{Content, Message, Role};

#[derive(Debug, Default)]
pub(super) struct MessageState {
    pub(super) active: bool,
    has_message: bool,
    current: Option<MessageDraft>,
}

impl MessageState {
    pub(super) fn proceed(
        &mut self,
        token: JsonToken<'_>,
    ) -> Result<Option<CompatibleOpenAIRequestComponent>, DecoderError> {
        if !self.active {
            if !matches!(token, JsonToken::ValueArray(None)) {
                return Err(DecoderError::InvalidInput("messages must be an array"));
            }
            self.active = true;
            self.has_message = false;
            return Ok(None);
        }
        if let Some(draft) = &mut self.current {
            if matches!(token, JsonToken::ObjectEnd) && !draft.nested() {
                let draft = self.current.take().ok_or(DecoderError::InvalidState)?;
                let component = CompatibleOpenAIRequestComponent::try_from(draft)?;
                self.has_message = true;
                return Ok(Some(component));
            }
            draft.proceed(token)?;
            return Ok(None);
        }
        match token {
            JsonToken::ValueObject(None) => {
                self.current = Some(MessageDraft::default());
                Ok(None)
            }
            JsonToken::ArrayEnd => {
                if !self.has_message {
                    return Err(DecoderError::InvalidInput("messages must not be empty"));
                }
                self.active = false;
                Ok(None)
            }
            _ => Err(DecoderError::InvalidInput("message must be an object")),
        }
    }
}

#[derive(Debug)]
struct MessageDraft {
    fields: MessageFields,
    has_role: bool,
    has_content: bool,
    has_tool_call_id: bool,
    has_reasoning: bool,
    has_reasoning_content: bool,
    content: ContentState,
    tool_calls: ToolCallsState,
    reasoning_details: ReasoningState,
}

impl Default for MessageDraft {
    fn default() -> Self {
        Self {
            fields: MessageFields {
                role: Role::User,
                content: None,
                tool_calls: None,
                tool_call_id: None,
                reasoning: None,
                reasoning_content: None,
                reasoning_details: None,
            },
            has_role: false,
            has_content: false,
            has_tool_call_id: false,
            has_reasoning: false,
            has_reasoning_content: false,
            content: ContentState::default(),
            tool_calls: ToolCallsState::default(),
            reasoning_details: ReasoningState::default(),
        }
    }
}

impl MessageDraft {
    fn nested(&self) -> bool {
        self.content.active || self.tool_calls.active || self.reasoning_details.active
    }

    fn proceed(&mut self, token: JsonToken<'_>) -> Result<(), DecoderError> {
        if self.tool_calls.active {
            if let Some(calls) = self.tool_calls.proceed(token)? {
                self.fields.tool_calls = Some(calls.into_iter().map(Decoded).collect());
            }
            return Ok(());
        }
        if self.reasoning_details.active {
            if let Some(details) = self.reasoning_details.proceed(token)? {
                self.fields.reasoning_details = Some(details.into_iter().map(Decoded).collect());
            }
            return Ok(());
        }
        if self.content.active {
            if let Some(content) = self
                .content
                .proceed(token)
                .map_err(|_| DecoderError::InvalidInput("invalid message content"))?
            {
                self.fields.content = Some(content);
            }
            return Ok(());
        }
        match token {
            token @ JsonToken::ValueArray(Some("tool_calls")) => {
                self.tool_calls.proceed(token)?;
            }
            token @ JsonToken::ValueArray(Some("reasoning_details")) => {
                self.reasoning_details.proceed(token)?;
            }
            JsonToken::ValueNull(Some("tool_calls" | "reasoning_details")) => {}
            token @ (JsonToken::ValueObject(Some("content"))
            | JsonToken::ValueArray(Some("content"))) => {
                if self.has_content {
                    return Err(DecoderError::InvalidInput("duplicate message content"));
                }
                self.has_content = true;
                self.content
                    .proceed(token)
                    .map_err(|_| DecoderError::InvalidInput("invalid message content"))?;
            }
            JsonToken::ValueString(Some("role"), role) => {
                if self.has_role {
                    return Err(DecoderError::InvalidInput("duplicate message field"));
                }
                self.has_role = true;
                self.fields.role = match role {
                    "system" => Role::System,
                    "developer" => Role::Developer,
                    "user" => Role::User,
                    "assistant" => Role::Assistant,
                    "tool" => Role::Tool,
                    _ => return Err(DecoderError::InvalidInput("invalid message role")),
                };
            }
            JsonToken::ValueString(Some("content"), value) => {
                if self.has_content {
                    return Err(DecoderError::InvalidInput("duplicate message field"));
                }
                self.has_content = true;
                self.fields.content = Some(Content::Text(value.to_owned()))
            }
            JsonToken::ValueNull(Some("content")) => {
                if self.has_content {
                    return Err(DecoderError::InvalidInput("duplicate message field"));
                }
                self.has_content = true;
                self.fields.content = Some(Content::Null);
            }
            JsonToken::ValueString(Some("tool_call_id"), value) => {
                if self.has_tool_call_id {
                    return Err(DecoderError::InvalidInput("duplicate message field"));
                }
                self.has_tool_call_id = true;
                self.fields.tool_call_id = Some(value.to_owned())
            }
            JsonToken::ValueNull(Some("tool_call_id")) => {
                if self.has_tool_call_id {
                    return Err(DecoderError::InvalidInput("duplicate message field"));
                }
                self.has_tool_call_id = true;
                self.fields.tool_call_id = None;
            }
            JsonToken::ValueString(Some("reasoning"), value) => {
                if self.has_reasoning {
                    return Err(DecoderError::InvalidInput("duplicate message field"));
                }
                self.has_reasoning = true;
                self.fields.reasoning = Some(value.to_owned())
            }
            JsonToken::ValueNull(Some("reasoning")) => {
                if self.has_reasoning {
                    return Err(DecoderError::InvalidInput("duplicate message field"));
                }
                self.has_reasoning = true;
                self.fields.reasoning = None;
            }
            JsonToken::ValueString(Some("reasoning_content"), value) => {
                if self.has_reasoning_content {
                    return Err(DecoderError::InvalidInput("duplicate message field"));
                }
                self.has_reasoning_content = true;
                self.fields.reasoning_content = Some(value.to_owned())
            }
            JsonToken::ValueNull(Some("reasoning_content")) => {
                if self.has_reasoning_content {
                    return Err(DecoderError::InvalidInput("duplicate message field"));
                }
                self.has_reasoning_content = true;
                self.fields.reasoning_content = None;
            }
            JsonToken::ValueObject(_) | JsonToken::ValueArray(_) => {
                return Err(DecoderError::Unsupported("nested message field"));
            }
            _ => return Err(DecoderError::InvalidInput("invalid message field value")),
        }
        Ok(())
    }
}

impl TryFrom<MessageDraft> for CompatibleOpenAIRequestComponent {
    type Error = DecoderError;

    fn try_from(draft: MessageDraft) -> Result<Self, Self::Error> {
        if !draft.has_role {
            return Err(DecoderError::InvalidInput("missing message role"));
        }
        Message::try_from(draft.fields)
            .map(Self::Message)
            .map_err(DecoderError::InvalidInput)
    }
}
