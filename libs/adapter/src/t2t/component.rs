/// Completed request data, independent of byte chunks and parser state.
#[derive(Clone, Debug, PartialEq)]
pub enum TextRequestComponent {
    Model(String),
    Message(Message),
    Tools(Vec<ToolDefinition>),
    ToolChoice(ToolChoice),
    ParallelToolCalls(bool),
    MaxCompletionTokens(u32),
    TopP(f64),
    TopK(u32),
    Stop(Stop),
    ResponseFormat(ResponseFormat),
    Provider(Provider),
    Stream(bool),
    StreamOptions(Option<StreamOptions>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Provider {
    RequireParameter(bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Stop {
    Null,
    Text(String),
    Array(Vec<String>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ResponseFormat {
    Text,
    JsonObject,
    JsonSchema(JsonSchema),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JsonSchema {
    pub name: String,
    pub description: Option<String>,
    pub strict: Option<bool>,
    pub schema: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ToolChoice {
    Auto,
    None,
    Required,
    Function(String),
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StreamOptions {
    pub include_usage: Option<bool>,
}

/// An owned message emitted only after its fields have been decoded and validated.
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
pub struct ToolDefinition {
    pub kind: ToolType,
    pub function: FunctionDefinition,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FunctionDefinition {
    pub name: String,
    pub description: Option<String>,
    pub parameters: Option<String>,
    pub strict: Option<bool>,
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
pub enum ToolType {
    Function,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Role {
    System,
    Developer,
    User,
    Assistant,
    Tool,
}
