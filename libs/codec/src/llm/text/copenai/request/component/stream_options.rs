use crate::llm::text::component::StreamOptions;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_with::{DeserializeAs, SerializeAs};

#[derive(Serialize, Deserialize)]
#[serde(remote = "StreamOptions", deny_unknown_fields)]
pub(in crate::llm::text::copenai::request) struct StreamOptionsWire {
    #[serde(skip_serializing_if = "Option::is_none")]
    include_usage: Option<bool>,
}

impl SerializeAs<StreamOptions> for StreamOptionsWire {
    fn serialize_as<S: Serializer>(
        value: &StreamOptions,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        Self::serialize(value, serializer)
    }
}

impl<'de> DeserializeAs<'de, StreamOptions> for StreamOptionsWire {
    fn deserialize_as<D: Deserializer<'de>>(deserializer: D) -> Result<StreamOptions, D::Error> {
        Self::deserialize(deserializer)
    }
}
