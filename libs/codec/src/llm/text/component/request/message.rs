use super::ToolType;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Message {
    pub role: Role,
    pub content: Option<Content>,
    pub tool_calls: Option<Vec<ToolCall>>,
    pub tool_call_id: Option<String>,
    pub reasoning: Option<String>,
    pub reasoning_content: Option<String>,
    pub reasoning_details: Option<Vec<ReasoningDetail>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Content {
    Null,
    Text(String),
    Object(ObjectMessage),
    Array(Vec<ObjectMessage>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ObjectMessage {
    Text { text: String },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ToolCall {
    pub id: String,
    pub kind: ToolType,
    pub function: FunctionCall,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReasoningDetail {
    pub content: ReasoningDetailContent,
    pub id: Option<String>,
    pub format: Option<String>,
    pub index: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ReasoningDetailContent {
    Summary {
        summary: String,
    },
    Text {
        text: String,
        signature: Option<String>,
    },
    Encrypted {
        data: String,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Role {
    System,
    Developer,
    User,
    Assistant,
    Tool,
}
