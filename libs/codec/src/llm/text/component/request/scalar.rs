use super::{Content, Message, Role, Stop, TextRequestComponent};

#[derive(Clone, Copy)]
pub(crate) enum ScalarText {
    Model,
    Stop,
    UserMessage,
    SystemMessage,
}

impl<'a> From<(ScalarText, &'a str)> for TextRequestComponent<&'a str> {
    fn from((kind, text): (ScalarText, &'a str)) -> Self {
        match kind {
            ScalarText::Model => Self::Model(text),
            ScalarText::Stop => Self::Stop(Stop::Text(text)),
            ScalarText::UserMessage | ScalarText::SystemMessage => Self::Message(Message {
                role: match kind {
                    ScalarText::SystemMessage => Role::System,
                    _ => Role::User,
                },
                content: Some(Content::Text(text)),
                tool_calls: None,
                tool_call_id: None,
                reasoning: None,
                reasoning_content: None,
                reasoning_details: None,
            }),
        }
    }
}
