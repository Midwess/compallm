use super::component::{ReasoningKind, TextRequestComponent};

/// Compatible OpenAI decoder boundaries, fragments, and neutral component values.
/// States retain source field order without collecting complete messages.
/// Boundaries do not imply a valid complete request; finish validates actual EOF.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CompatibleOpenAIDecoderState<'a> {
    Component(TextRequestComponent),

    RequestStarted,
    ModelStarted,
    ModelEnded,

    MessagesStarted,
    MessageStarted,
    TextStarted,
    TextEnded,

    ToolsStarted,
    ToolStarted,
    ToolEnded,
    ToolsEnded,

    ToolCallsStarted,
    ToolCallStarted,
    /// The id field inside a tool_calls entry.
    ToolCallIdStarted,
    ToolCallIdEnded,
    ToolCallEnded,
    ToolCallsEnded,

    ToolResultStarted,
    /// Identifies the call answered by this result, independent of source protocol.
    ToolResultCallIdStarted,
    ToolResultCallIdEnded,
    ToolResultEnded,

    /// The function object inside a tool definition or a tool call.
    FunctionStarted,
    FunctionNameStarted,
    FunctionNameEnded,
    FunctionDescriptionStarted,
    FunctionDescriptionEnded,
    FunctionParametersStarted,
    FunctionParametersEnded,
    /// Use JsonStringFragment for source string content or JsonFragment for a
    /// source JSON value. Do not mix representations within one argument value.
    /// The encoder owns conversion to the target protocol's representation.
    FunctionArgumentsStarted,
    FunctionArgumentsEnded,
    FunctionEnded,

    /// Reasoning stays distinct from ordinary message text; preserve its kind.
    ReasoningStarted {
        kind: ReasoningKind,
    },
    ReasoningSignatureStarted,
    ReasoningSignatureEnded,
    ReasoningEnded,

    /// Raw JSON string content without outer quotes. Escapes remain encoded;
    /// fragments may split escape sequences or UTF-8 scalars. Used for model,
    /// text, identifiers, function fields, reasoning, and signatures.
    JsonStringFragment(&'a [u8]),
    /// Raw JSON value bytes, including delimiters, for parameters or arguments.
    /// Fragments may split tokens; they are not independently valid JSON values.
    JsonFragment(&'a [u8]),

    MessageEnded,
    MessagesEnded,
    RequestEnded,
}
