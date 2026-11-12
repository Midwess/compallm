//! Incremental message and structured request vocabulary.

use crate::t2t::Text2TextTranslatorConfiguration;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutputFormatKind {
    Text,
    JsonObject,
    JsonSchema,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutputFormatEvent {
    Started,
    Mode(OutputFormatKind),
    NameStarted,
    NameEnded,
    DescriptionStarted,
    DescriptionEnded,
    Strict(Option<bool>),
    SchemaStarted,
    SchemaEnded,
    Ended,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Role {
    System,
    Developer,
    User,
    Assistant,
}

/// Fragments are decoded payload bytes, not JSON-escaped text or entire messages.
/// Structured values also use fragments for schema keys and JSON numbers.
/// Codecs must bound fragment sizes and handle UTF-8 split across fragments.
#[derive(Clone, Debug, PartialEq)]
pub enum RequestEvent<T = usize> {
    ResponsesStore(bool),
    ResponsesEncryptedReasoning(bool),
    /// Opens a Responses item with the fixed {"type":"reasoning" prefix.
    ResponsesReasoningStarted,
    /// Raw JSON suffix bytes through the item's closing brace, in text scratch.
    ResponsesReasoningFragment(T),
    ResponsesReasoningEnded,
    OutputFormat(OutputFormatEvent),
    SchemaFragment(T),
    /// Typed structured values. String/key/number payloads use TextFragment.
    Structure(Structure),
    /// Explicit source settings, emitted at most once and outside messages.
    Configuration(Text2TextTranslatorConfiguration),
    TopP(f64),
    StopSequencesStarted,
    StopSequenceStarted,
    StopSequenceEnded,
    StopSequencesEnded,
    /// Resolved stream/options settings, independent of the token budget.
    Streaming {
        stream: bool,
        include_usage: Option<bool>,
    },
    MessageStarted {
        role: Role,
    },
    /// A message content array; each text block retains its own text boundaries.
    TextPartsStarted,
    TextPartsEnded,
    TextStarted,
    /// Production decoders return a length into caller-owned decoded scratch.
    TextFragment(T),
    TextEnded,
    MessageEnded,
}

impl<T> RequestEvent<T> {
    pub fn map_text<U>(self, map: impl FnOnce(T) -> U) -> RequestEvent<U> {
        match self {
            Self::ResponsesStore(value) => RequestEvent::ResponsesStore(value),
            Self::ResponsesEncryptedReasoning(value) => {
                RequestEvent::ResponsesEncryptedReasoning(value)
            }
            Self::ResponsesReasoningStarted => RequestEvent::ResponsesReasoningStarted,
            Self::ResponsesReasoningFragment(bytes) => {
                RequestEvent::ResponsesReasoningFragment(map(bytes))
            }
            Self::ResponsesReasoningEnded => RequestEvent::ResponsesReasoningEnded,
            Self::OutputFormat(event) => RequestEvent::OutputFormat(event),
            Self::SchemaFragment(bytes) => RequestEvent::SchemaFragment(map(bytes)),
            Self::Structure(value) => RequestEvent::Structure(value),
            Self::Configuration(value) => RequestEvent::Configuration(value),
            Self::TopP(value) => RequestEvent::TopP(value),
            Self::StopSequencesStarted => RequestEvent::StopSequencesStarted,
            Self::StopSequenceStarted => RequestEvent::StopSequenceStarted,
            Self::StopSequenceEnded => RequestEvent::StopSequenceEnded,
            Self::StopSequencesEnded => RequestEvent::StopSequencesEnded,
            Self::Streaming {
                stream,
                include_usage,
            } => RequestEvent::Streaming {
                stream,
                include_usage,
            },
            Self::MessageStarted { role } => RequestEvent::MessageStarted { role },
            Self::TextPartsStarted => RequestEvent::TextPartsStarted,
            Self::TextPartsEnded => RequestEvent::TextPartsEnded,
            Self::TextStarted => RequestEvent::TextStarted,
            Self::TextFragment(text) => RequestEvent::TextFragment(map(text)),
            Self::TextEnded => RequestEvent::TextEnded,
            Self::MessageEnded => RequestEvent::MessageEnded,
        }
    }
}

/// Container/scalar boundaries for compatible Chat tools and reasoning history.
/// These events are transient; replay retains original request bytes only.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Structure {
    ObjectStarted,
    ObjectEnded,
    ArrayStarted,
    ArrayEnded,
    Field(RequestField),
    KeyStarted,
    KeyEnded,
    StringStarted,
    StringEnded,
    NumberStarted,
    NumberEnded,
    Boolean(bool),
    Null,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RequestField {
    Model,
    Messages,
    Stream,
    MaxCompletionTokens,
    MaxTokens,
    Tools,
    ToolChoice,
    ParallelToolCalls,
    Role,
    Content,
    ToolCalls,
    ToolCallId,
    Reasoning,
    ReasoningContent,
    ReasoningDetails,
    Id,
    Type,
    Function,
    Name,
    Arguments,
    Description,
    Parameters,
    Strict,
    Text,
    Summary,
    Data,
    Signature,
    Format,
    Index,
    StreamOptions,
    IncludeUsage,
    TopP,
    TopK,
    Stop,
    ResponseFormat,
    JsonSchema,
    Schema,
    Provider,
    RequireParameters,
}
