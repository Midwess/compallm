use super::ToolType;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Message<S = String> {
    pub role: Role,
    pub content: Option<Content<S>>,
    pub tool_calls: Option<Vec<ToolCall<S>>>,
    pub tool_call_id: Option<S>,
    pub reasoning: Option<S>,
    pub reasoning_content: Option<S>,
    pub reasoning_details: Option<Vec<ReasoningDetail<S>>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Content<S = String> {
    Null,
    Text(S),
    Object(ObjectMessage<S>),
    Array(Vec<ObjectMessage<S>>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ObjectMessage<S = String> {
    Text { text: S },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ToolCall<S = String> {
    pub id: S,
    pub kind: ToolType,
    pub function: FunctionCall<S>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FunctionCall<S = String> {
    pub name: S,
    pub arguments: S,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReasoningDetail<S = String> {
    pub content: ReasoningDetailContent<S>,
    pub id: Option<S>,
    pub format: Option<S>,
    pub index: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ReasoningDetailContent<S = String> {
    Summary { summary: S },
    Text { text: S, signature: Option<S> },
    Encrypted { data: S },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Role {
    System,
    Developer,
    User,
    Assistant,
    Tool,
}

impl<S> Message<S> {
    pub fn map_text<T>(self, mut convert: impl FnMut(S) -> T) -> Message<T> {
        Message {
            role: self.role,
            content: self.content.map(|value| value.map_text(&mut convert)),
            tool_calls: self.tool_calls.map(|calls| {
                calls
                    .into_iter()
                    .map(|call| ToolCall {
                        id: convert(call.id),
                        kind: call.kind,
                        function: FunctionCall {
                            name: convert(call.function.name),
                            arguments: convert(call.function.arguments),
                        },
                    })
                    .collect()
            }),
            tool_call_id: self.tool_call_id.map(&mut convert),
            reasoning: self.reasoning.map(&mut convert),
            reasoning_content: self.reasoning_content.map(&mut convert),
            reasoning_details: self.reasoning_details.map(|details| {
                details
                    .into_iter()
                    .map(|detail| {
                        let content = match detail.content {
                            ReasoningDetailContent::Summary { summary } => {
                                ReasoningDetailContent::Summary {
                                    summary: convert(summary),
                                }
                            }
                            ReasoningDetailContent::Text { text, signature } => {
                                ReasoningDetailContent::Text {
                                    text: convert(text),
                                    signature: signature.map(&mut convert),
                                }
                            }
                            ReasoningDetailContent::Encrypted { data } => {
                                ReasoningDetailContent::Encrypted {
                                    data: convert(data),
                                }
                            }
                        };
                        ReasoningDetail {
                            content,
                            id: detail.id.map(&mut convert),
                            format: detail.format.map(&mut convert),
                            index: detail.index,
                        }
                    })
                    .collect()
            }),
        }
    }
}

impl<S> Content<S> {
    pub fn map_text<T>(self, mut convert: impl FnMut(S) -> T) -> Content<T> {
        match self {
            Self::Null => Content::Null,
            Self::Text(text) => Content::Text(convert(text)),
            Self::Object(ObjectMessage::Text { text }) => Content::Object(ObjectMessage::Text {
                text: convert(text),
            }),
            Self::Array(parts) => Content::Array(
                parts
                    .into_iter()
                    .map(|ObjectMessage::Text { text }| ObjectMessage::Text {
                        text: convert(text),
                    })
                    .collect(),
            ),
        }
    }
}
