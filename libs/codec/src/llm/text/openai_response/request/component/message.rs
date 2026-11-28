use crate::llm::text::component::*;
use compallm_serde_rules::borrowed;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum Kind {
    Message,
    FunctionCall,
    FunctionCallOutput,
}
#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum MessageRole {
    System,
    Developer,
    User,
    Assistant,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(in crate::llm::text::openai_response::request) struct MessageInput<'a> {
    #[serde(rename = "type")]
    kind: Option<Kind>,
    role: Option<MessageRole>,
    #[serde(default, borrow, deserialize_with = "borrowed::present_text")]
    content: Option<&'a str>,
    #[serde(default, borrow, deserialize_with = "borrowed::present_text")]
    call_id: Option<&'a str>,
    #[serde(default, borrow, deserialize_with = "borrowed::present_text")]
    name: Option<&'a str>,
    #[serde(default, borrow, deserialize_with = "borrowed::present_text")]
    arguments: Option<&'a str>,
    #[serde(default, borrow, deserialize_with = "borrowed::present_text")]
    output: Option<&'a str>,
}
impl<'a> TryFrom<MessageInput<'a>> for Message<&'a str> {
    type Error = &'static str;
    fn try_from(v: MessageInput<'a>) -> Result<Self, Self::Error> {
        if v.call_id.is_some_and(str::is_empty) || v.name.is_some_and(str::is_empty) {
            return Err("empty call identity");
        }
        let mut message = Message {
            role: Role::User,
            content: None,
            tool_calls: None,
            tool_call_id: None,
            reasoning: None,
            reasoning_content: None,
            reasoning_details: None,
        };
        match v.kind.unwrap_or(Kind::Message) {
            Kind::Message
                if v.call_id.is_none()
                    && v.name.is_none()
                    && v.arguments.is_none()
                    && v.output.is_none() =>
            {
                message.role = match v.role.ok_or("missing role")? {
                    MessageRole::System => Role::System,
                    MessageRole::Developer => Role::Developer,
                    MessageRole::User => Role::User,
                    MessageRole::Assistant => Role::Assistant,
                };
                message.content = Some(Content::Text(v.content.ok_or("missing content")?));
            }
            Kind::FunctionCall if v.role.is_none() && v.content.is_none() && v.output.is_none() => {
                message.role = Role::Assistant;
                message.tool_calls = Some(Collection::Single(ToolCall {
                    id: v.call_id.ok_or("missing call id")?,
                    kind: ToolType::Function,
                    function: FunctionCall {
                        name: v.name.ok_or("missing function name")?,
                        arguments: crate::json::JsonArgument::Text(
                            v.arguments.ok_or("missing arguments")?,
                        ),
                    },
                }));
            }
            Kind::FunctionCallOutput
                if v.role.is_none()
                    && v.content.is_none()
                    && v.name.is_none()
                    && v.arguments.is_none() =>
            {
                message.role = Role::Tool;
                message.tool_call_id = Some(v.call_id.ok_or("missing call id")?);
                message.content = Some(Content::Text(v.output.ok_or("missing output")?));
            }
            _ => return Err("invalid input item"),
        }
        Ok(message)
    }
}

#[derive(Serialize)]
#[serde(untagged)]
pub(in crate::llm::text::openai_response::request) enum MessageOutput<'a> {
    Message {
        role: &'static str,
        content: &'a str,
    },
    Call {
        #[serde(rename = "type")]
        kind: &'static str,
        call_id: &'a str,
        name: &'a str,
        arguments: &'a str,
    },
    RawCall(super::serialize::RawCall<'a>),
    Result {
        #[serde(rename = "type")]
        kind: &'static str,
        call_id: &'a str,
        output: &'a str,
    },
}
impl<'a, S: AsRef<str>> TryFrom<(&'a Message<S>, usize)> for MessageOutput<'a> {
    type Error = &'static str;
    fn try_from((value, index): (&'a Message<S>, usize)) -> Result<Self, Self::Error> {
        if value.reasoning.is_some()
            || value.reasoning_content.is_some()
            || value.reasoning_details.is_some()
        {
            return Err("unsupported reasoning history");
        }
        if let Some(calls) = &value.tool_calls {
            if value.role != Role::Assistant || value.tool_call_id.is_some() {
                return Err("invalid tool message");
            }
            let text = matches!(value.content, Some(Content::Text(_)));
            if text && index == 0 {
                let Some(Content::Text(content)) = &value.content else {
                    unreachable!()
                };
                return Ok(Self::Message {
                    role: "assistant",
                    content: content.as_ref(),
                });
            }
            if value
                .content
                .as_ref()
                .is_some_and(|v| !matches!(v, Content::Text(_) | Content::Null))
            {
                return Err("unsupported content");
            }
            let index = index - usize::from(text);
            let call = match calls {
                Collection::Single(call) if index == 0 => call,
                Collection::Owned(calls) => calls.get(index).ok_or("missing call")?,
                Collection::View(view) => {
                    return Ok(Self::RawCall(super::serialize::RawCall(
                        view.borrowed()
                            .items()
                            .and_then(|mut items| items.nth(index))
                            .ok_or("missing call")?,
                    )));
                }
                _ => return Err("missing call"),
            };
            if call.id.as_ref().is_empty() || call.function.name.as_ref().is_empty() {
                return Err("invalid call");
            }
            return Ok(Self::Call {
                kind: "function_call",
                call_id: call.id.as_ref(),
                name: call.function.name.as_ref(),
                arguments: call.function.arguments.as_ref(),
            });
        }
        let Some(Content::Text(content)) = &value.content else {
            return Err("expected text");
        };
        if value.role == Role::Tool {
            return Ok(Self::Result {
                kind: "function_call_output",
                call_id: value
                    .tool_call_id
                    .as_ref()
                    .filter(|id| !id.as_ref().is_empty())
                    .ok_or("missing call id")?
                    .as_ref(),
                output: content.as_ref(),
            });
        }
        if value.tool_call_id.is_some() {
            return Err("unexpected call id");
        }
        Ok(Self::Message {
            role: match value.role {
                Role::System => "system",
                Role::Developer => "developer",
                Role::User => "user",
                Role::Assistant => "assistant",
                _ => return Err("invalid role"),
            },
            content: content.as_ref(),
        })
    }
}
pub(in crate::llm::text::openai_response::request) fn item_count<S: AsRef<str>>(
    value: &Message<S>,
) -> usize {
    match &value.tool_calls {
        None => 1,
        Some(calls) => {
            usize::from(matches!(value.content, Some(Content::Text(_))))
                + match calls {
                    Collection::Single(_) => 1,
                    Collection::Owned(v) => v.len(),
                    Collection::View(v) => v.borrowed().items().map_or(0, Iterator::count),
                }
        }
    }
}
