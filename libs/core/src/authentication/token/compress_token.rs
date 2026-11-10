use serde::{Deserialize, Serialize};

/// Dictionary version and compressed serialized token, protected by the CWT MAC.
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CompressedToken {
    #[serde(rename = "v")]
    pub dictionary_version: u32,
    #[serde(rename = "d")]
    #[serde(with = "serde_bytes")]
    pub data: Vec<u8>,
}
