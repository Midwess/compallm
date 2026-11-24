mod message;
mod provider;
mod reasoning;
mod response_format;
mod stop;
mod stream_options;
mod tool_choice;
mod tools;

pub use message::*;
pub use provider::Provider;
pub use reasoning::{Reasoning, ReasoningEffort};
pub use response_format::{JsonSchema, ResponseFormat};
pub use stop::Stop;
pub use stream_options::StreamOptions;
pub use tool_choice::ToolChoice;
pub use tools::{FunctionDefinition, ToolDefinition, ToolType};

/// Completed request data, independent of byte chunks and parser state.
#[derive(Clone, Debug, PartialEq)]
pub enum TextRequestComponent {
    /// Identifies the model requested by the caller.
    Model(String),
    /// One conversation message, including text, tool calls/results, or reasoning history.
    Message(Message),
    /// Declares the functions the model may request the caller to execute.
    Tools(Vec<ToolDefinition>),
    /// Controls whether the model may, must, or must not call a tool, or selects one function.
    ToolChoice(ToolChoice),
    /// Controls whether the model may request multiple tool calls in one turn.
    ParallelToolCalls(bool),
    /// Limits generated tokens, including reasoning tokens where the provider counts them.
    MaxTokens(u32),
    /// Controls reasoning generation and visibility independently of reasoning history.
    Reasoning(Reasoning),
    /// Nucleus sampling: considers tokens within this cumulative probability mass.
    TopP(f64),
    /// Limits sampling to the most likely K tokens; zero disables this filter.
    TopK(u32),
    /// Supplies text sequences at which generation should stop, or disables explicit stops.
    Stop(Stop),
    /// Requests ordinary text, a JSON object, or JSON constrained by a supplied schema.
    ResponseFormat(ResponseFormat),
    /// Configures provider routing requirements, such as support for requested parameters.
    Provider(Provider),
    /// Requests incremental response delivery instead of a single completed response.
    Stream(bool),
    /// Configures streamed response metadata, such as token usage; None represents explicit null.
    StreamOptions(Option<StreamOptions>),
}
