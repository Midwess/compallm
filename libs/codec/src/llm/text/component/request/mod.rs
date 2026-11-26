mod collection;
pub use collection::Collection;
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
pub enum TextRequestComponent<S = String> {
    /// Identifies the model requested by the caller.
    Model(S),
    /// One conversation message, including text, tool calls/results, or reasoning history.
    Message(Message<S>),
    /// Declares the functions the model may request the caller to execute.
    Tools(Collection<ToolDefinition<S>, S>),
    /// Controls whether the model may, must, or must not call a tool, or selects one function.
    ToolChoice(ToolChoice<S>),
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
    Stop(Stop<S>),
    /// Requests ordinary text, a JSON object, or JSON constrained by a supplied schema.
    ResponseFormat(ResponseFormat<S>),
    /// Configures provider routing requirements, such as support for requested parameters.
    Provider(Provider),
    /// Requests incremental response delivery instead of a single completed response.
    Stream(bool),
    /// Configures streamed response metadata, such as token usage; None represents explicit null.
    StreamOptions(Option<StreamOptions>),
}

impl<S> TextRequestComponent<S> {
    pub fn map_text<T>(self, mut convert: impl FnMut(S) -> T) -> TextRequestComponent<T> {
        match self {
            Self::Model(value) => TextRequestComponent::Model(convert(value)),
            Self::Message(value) => TextRequestComponent::Message(value.map_text(&mut convert)),
            Self::Tools(values) => TextRequestComponent::Tools(match values {
                Collection::Owned(values) => Collection::Owned(
                    values
                        .into_iter()
                        .map(|value| value.map_text(&mut convert))
                        .collect(),
                ),
                Collection::View(view) => Collection::View(view.map_text(&mut convert)),
            }),
            Self::ToolChoice(value) => {
                TextRequestComponent::ToolChoice(value.map_text(&mut convert))
            }
            Self::ParallelToolCalls(value) => TextRequestComponent::ParallelToolCalls(value),
            Self::MaxTokens(value) => TextRequestComponent::MaxTokens(value),
            Self::Reasoning(value) => TextRequestComponent::Reasoning(value),
            Self::TopP(value) => TextRequestComponent::TopP(value),
            Self::TopK(value) => TextRequestComponent::TopK(value),
            Self::Stop(value) => TextRequestComponent::Stop(value.map_text(&mut convert)),
            Self::ResponseFormat(value) => {
                TextRequestComponent::ResponseFormat(value.map_text(&mut convert))
            }
            Self::Provider(value) => TextRequestComponent::Provider(value),
            Self::Stream(value) => TextRequestComponent::Stream(value),
            Self::StreamOptions(value) => TextRequestComponent::StreamOptions(value),
        }
    }
}

impl<S: Into<String>> TextRequestComponent<S> {
    pub fn into_owned(self) -> TextRequestComponent {
        self.map_text(Into::into)
    }
}
