mod deserialize;
mod message;
mod provider;
mod response_format;
mod serialize;
mod stop;
mod stream_options;
mod tool_choice;
mod tools;

use crate::llm::text::component::{
    Message, Provider, ResponseFormat, Stop, StreamOptions, TextRequestComponent, ToolChoice,
    ToolDefinition,
};
use deserialize::ComponentDeserializeHelper;
pub(super) use deserialize::Decoded;
pub(super) use serialize::Encoded;

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompatibleOpenAIRequestComponent {
    Model(String),
    Message(
        #[serde(deserialize_with = "CompatibleOpenAIRequestComponent::deserialize_message")]
        Message,
    ),
    Tools(
        #[serde(
            deserialize_with = "CompatibleOpenAIRequestComponent::decode::<_, Vec<ToolDefinition>>"
        )]
        Vec<ToolDefinition>,
    ),
    ToolChoice(
        #[serde(deserialize_with = "CompatibleOpenAIRequestComponent::decode::<_, ToolChoice>")]
        ToolChoice,
    ),
    ParallelToolCalls(bool),
    MaxCompletionTokens(u32),
    TopP(f64),
    TopK(u32),
    Stop(#[serde(deserialize_with = "CompatibleOpenAIRequestComponent::decode::<_, Stop>")] Stop),
    ResponseFormat(
        #[serde(
            deserialize_with = "CompatibleOpenAIRequestComponent::decode::<_, ResponseFormat>"
        )]
        ResponseFormat,
    ),
    Provider(
        #[serde(deserialize_with = "CompatibleOpenAIRequestComponent::decode::<_, Provider>")]
        Provider,
    ),
    Stream(bool),
    StreamOptions(
        #[serde(
            deserialize_with = "CompatibleOpenAIRequestComponent::decode::<_, Option<StreamOptions>>"
        )]
        Option<StreamOptions>,
    ),
}

impl From<Decoded<Message>> for TextRequestComponent {
    fn from(Decoded(value): Decoded<Message>) -> Self {
        Self::Message(value)
    }
}

impl From<Decoded<Provider>> for TextRequestComponent {
    fn from(Decoded(value): Decoded<Provider>) -> Self {
        Self::Provider(value)
    }
}

impl From<Decoded<ResponseFormat>> for TextRequestComponent {
    fn from(Decoded(value): Decoded<ResponseFormat>) -> Self {
        Self::ResponseFormat(value)
    }
}

impl From<Decoded<StreamOptions>> for TextRequestComponent {
    fn from(Decoded(value): Decoded<StreamOptions>) -> Self {
        Self::StreamOptions(Some(value))
    }
}

impl From<Decoded<ToolChoice>> for TextRequestComponent {
    fn from(Decoded(value): Decoded<ToolChoice>) -> Self {
        Self::ToolChoice(value)
    }
}

impl From<Decoded<Vec<ToolDefinition>>> for TextRequestComponent {
    fn from(Decoded(value): Decoded<Vec<ToolDefinition>>) -> Self {
        Self::Tools(value)
    }
}

impl From<Decoded<Stop>> for TextRequestComponent {
    fn from(Decoded(value): Decoded<Stop>) -> Self {
        Self::Stop(value)
    }
}

impl From<CompatibleOpenAIRequestComponent> for TextRequestComponent {
    fn from(component: CompatibleOpenAIRequestComponent) -> Self {
        match component {
            CompatibleOpenAIRequestComponent::Model(value) => Self::Model(value),
            CompatibleOpenAIRequestComponent::Message(value) => Self::Message(value),
            CompatibleOpenAIRequestComponent::Tools(value) => Self::Tools(value),
            CompatibleOpenAIRequestComponent::ToolChoice(value) => Self::ToolChoice(value),
            CompatibleOpenAIRequestComponent::ParallelToolCalls(value) => {
                Self::ParallelToolCalls(value)
            }
            CompatibleOpenAIRequestComponent::MaxCompletionTokens(value) => {
                Self::MaxCompletionTokens(value)
            }
            CompatibleOpenAIRequestComponent::TopP(value) => Self::TopP(value),
            CompatibleOpenAIRequestComponent::TopK(value) => Self::TopK(value),
            CompatibleOpenAIRequestComponent::Stop(value) => Self::Stop(value),
            CompatibleOpenAIRequestComponent::ResponseFormat(value) => Self::ResponseFormat(value),
            CompatibleOpenAIRequestComponent::Provider(value) => Self::Provider(value),
            CompatibleOpenAIRequestComponent::Stream(value) => Self::Stream(value),
            CompatibleOpenAIRequestComponent::StreamOptions(value) => Self::StreamOptions(value),
        }
    }
}

impl From<TextRequestComponent> for CompatibleOpenAIRequestComponent {
    fn from(component: TextRequestComponent) -> Self {
        match component {
            TextRequestComponent::Model(value) => Self::Model(value),
            TextRequestComponent::Message(value) => Self::Message(value),
            TextRequestComponent::Tools(value) => Self::Tools(value),
            TextRequestComponent::ToolChoice(value) => Self::ToolChoice(value),
            TextRequestComponent::ParallelToolCalls(value) => Self::ParallelToolCalls(value),
            TextRequestComponent::MaxCompletionTokens(value) => Self::MaxCompletionTokens(value),
            TextRequestComponent::TopP(value) => Self::TopP(value),
            TextRequestComponent::TopK(value) => Self::TopK(value),
            TextRequestComponent::Stop(value) => Self::Stop(value),
            TextRequestComponent::ResponseFormat(value) => Self::ResponseFormat(value),
            TextRequestComponent::Provider(value) => Self::Provider(value),
            TextRequestComponent::Stream(value) => Self::Stream(value),
            TextRequestComponent::StreamOptions(value) => Self::StreamOptions(value),
        }
    }
}
