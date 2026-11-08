use serde::{Deserialize, Serialize};

/// Dictionary version and compressed serialized token, protected by the JWT signature.
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CompressedToken {
    pub dictionary_version: u32,
    pub data: String,
}
