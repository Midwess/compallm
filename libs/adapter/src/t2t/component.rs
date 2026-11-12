/// Neutral text request values, independent of decoder boundaries and fragments.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextRequestComponent {
    /// May follow content or tool calls when the source puts role last.
    Role(Role),
    ContentNull,
    ToolType(ToolType),
    ToolCallType(ToolType),
    FunctionStrict(Option<bool>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReasoningKind {
    Text,
    Summary,
    Encrypted,
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
