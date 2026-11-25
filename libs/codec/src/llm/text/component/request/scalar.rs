use super::{Content, Message, Role, Stop, TextRequestComponent};
use std::borrow::Cow;

#[derive(Clone, Copy)]
pub(crate) enum ScalarText {
    Model,
    Stop,
    UserMessage,
    SystemMessage,
}

impl<'a> From<(ScalarText, &'a str)> for TextRequestComponent<Cow<'a, str>> {
    fn from((kind, text): (ScalarText, &'a str)) -> Self {
        match kind {
            ScalarText::Model => Self::Model(Cow::Borrowed(text)),
            ScalarText::Stop => Self::Stop(Stop::Text(Cow::Borrowed(text))),
            ScalarText::UserMessage | ScalarText::SystemMessage => Self::Message(Message {
                role: match kind {
                    ScalarText::SystemMessage => Role::System,
                    _ => Role::User,
                },
                content: Some(Content::Text(Cow::Borrowed(text))),
                tool_calls: None,
                tool_call_id: None,
                reasoning: None,
                reasoning_content: None,
                reasoning_details: None,
            }),
        }
    }
}
