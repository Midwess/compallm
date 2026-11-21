use super::{CompatibleOpenAIRequestComponent, message::CompatibleOpenAIContent, stream_options};
use crate::decoder::{DecoderError, DecoderStatus};
use crate::llm::text::component::*;
use serde::Deserialize;
use serde_json::value::RawValue;

#[derive(Debug)]
pub(in crate::llm::text::copenai::request) struct Decoded<T>(
    pub(in crate::llm::text::copenai::request) T,
);

impl<T> From<DecoderStatus<Decoded<T>>> for DecoderStatus<TextRequestComponent>
where
    Decoded<T>: Into<TextRequestComponent>,
{
    fn from(status: DecoderStatus<Decoded<T>>) -> Self {
        match status {
            DecoderStatus::Component(value) => Self::Component(value.into()),
            DecoderStatus::NeedInput => Self::NeedInput,
            DecoderStatus::Finished => Self::Finished,
        }
    }
}

impl<'de> Deserialize<'de> for Decoded<StreamOptions> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        stream_options::CompatibleOpenAIStreamOptions::deserialize(deserializer).map(Self)
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

pub(super) trait ComponentDeserializeHelper {
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
        Self::decode::<_, Message>(deserializer)
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
        CompatibleOpenAIContent::deserialize(deserializer).map(|content| Some(content.into()))
    }
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
