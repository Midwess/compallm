mod message;
pub(super) use message::MessageFields;
mod provider;
mod response_format;
mod stop;
mod stream_options;
mod tool_choice;
mod tools;

use crate::decoder::DecoderError;
use crate::llm::text::component::{
    Content, Message, Provider, ResponseFormat, Stop, StreamOptions, TextRequestComponent,
    ToolChoice, ToolDefinition,
};
use message::ContentFields;
use serde::Deserialize;
use serde_json::value::RawValue;

#[derive(Debug)]
pub(super) struct Decoded<T>(pub(super) T);

impl<'de> Deserialize<'de> for Decoded<StreamOptions> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        stream_options::StreamFields::deserialize(deserializer).map(Self)
    }
}

impl<'de, T> Deserialize<'de> for Decoded<Vec<T>>
where
    Decoded<T>: Deserialize<'de>,
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Vec::<Decoded<T>>::deserialize(deserializer)
            .map(|values| Self(values.into_iter().map(|Decoded(value)| value).collect()))
    }
}

impl<'de, T> Deserialize<'de> for Decoded<Option<T>>
where
    Decoded<T>: Deserialize<'de>,
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Option::<Decoded<T>>::deserialize(deserializer)
            .map(|value| Self(value.map(|Decoded(value)| value)))
    }
}

trait ComponentDeserializeHelper {
    fn decode<'de, D, T>(deserializer: D) -> Result<T, D::Error>
    where
        D: serde::Deserializer<'de>,
        Decoded<T>: Deserialize<'de>,
    {
        Decoded::<T>::deserialize(deserializer).map(|Decoded(value)| value)
    }

    fn schema_text<E: serde::de::Error>(value: Box<RawValue>) -> Result<String, E> {
        match value.get().as_bytes().first() {
            Some(b'{') => Ok(Box::<str>::from(value).into_string()),
            Some(b'"') => serde_json::from_str(value.get()).map_err(E::custom),
            _ => Err(E::custom("expected schema object or string")),
        }
    }

    fn schema<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<String, D::Error> {
        Self::schema_text(Box::<RawValue>::deserialize(deserializer)?)
    }

    fn optional_schema<'de, D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<String>, D::Error> {
        Option::<Box<RawValue>>::deserialize(deserializer)?
            .map(Self::schema_text)
            .transpose()
    }

    fn invalid() -> DecoderError {
        DecoderError::InvalidInput("invalid request component")
    }

    fn deserialize_message<'de, D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Message, D::Error> {
        use serde::de::Error;
        MessageFields::deserialize(deserializer)?
            .try_into()
            .map_err(D::Error::custom)
    }

    fn nonempty<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<String, D::Error> {
        use serde::de::Error;
        let text = String::deserialize(deserializer)?;
        if text.is_empty() {
            return Err(D::Error::custom("expected nonempty string"));
        }
        Ok(text)
    }

    fn present<'de, D: serde::Deserializer<'de>, T: Deserialize<'de>>(
        deserializer: D,
    ) -> Result<Option<T>, D::Error> {
        T::deserialize(deserializer).map(Some)
    }

    fn content<'de, D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<Content>, D::Error> {
        ContentFields::deserialize(deserializer).map(|content| Some(content.into()))
    }

    fn present_signature<'de, D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<Option<String>>, D::Error> {
        Option::<String>::deserialize(deserializer).map(Some)
    }
}

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

impl ComponentDeserializeHelper for CompatibleOpenAIRequestComponent {}

impl CompatibleOpenAIRequestComponent {
    pub fn parse(json: &[u8]) -> Result<Self, DecoderError> {
        let component: Self = serde_json::from_slice(json).map_err(|_| Self::invalid())?;
        component.validate()
    }

    pub(super) fn validate(self) -> Result<Self, DecoderError> {
        match &self {
            Self::Model(model) if model.is_empty() => return Err(Self::invalid()),
            Self::MaxCompletionTokens(0) => return Err(Self::invalid()),
            Self::TopP(p) if !p.is_finite() || !(0.0..=1.0).contains(p) => {
                return Err(Self::invalid());
            }
            _ => {}
        }
        Ok(self)
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
