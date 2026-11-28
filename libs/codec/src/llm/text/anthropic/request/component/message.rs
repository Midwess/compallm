use crate::llm::text::component::{Content, Message, Role};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum MessageRole {
    User,
    Assistant,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(in crate::llm::text::anthropic::request) struct MessageInput<'a> {
    role: MessageRole,
    #[serde(borrow)]
    content: &'a str,
}

impl<'a> From<MessageInput<'a>> for Message<&'a str> {
    fn from(value: MessageInput<'a>) -> Self {
        Self {
            role: match value.role {
                MessageRole::User => Role::User,
                MessageRole::Assistant => Role::Assistant,
            },
            content: Some(Content::Text(value.content)),
            tool_calls: None,
            tool_call_id: None,
            reasoning: None,
            reasoning_content: None,
            reasoning_details: None,
        }
    }
}

#[derive(Serialize)]
pub(in crate::llm::text::anthropic::request) struct MessageOutput<'a> {
    pub role: &'static str,
    pub content: &'a str,
}

impl<'a, S: AsRef<str>> TryFrom<&'a Message<S>> for MessageOutput<'a> {
    type Error = &'static str;

    fn try_from(value: &'a Message<S>) -> Result<Self, Self::Error> {
        if value.tool_calls.is_some()
            || value.tool_call_id.is_some()
            || value.reasoning.is_some()
            || value.reasoning_content.is_some()
            || value.reasoning_details.is_some()
        {
            return Err("unsupported message metadata");
        }
        let Some(Content::Text(content)) = &value.content else {
            return Err("expected text content");
        };
        Ok(Self {
            role: match value.role {
                Role::System => "system",
                Role::User => "user",
                Role::Assistant => "assistant",
                _ => return Err("unsupported message role"),
            },
            content: content.as_ref(),
        })
    }
}
