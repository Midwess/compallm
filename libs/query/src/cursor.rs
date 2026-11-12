use crate::Cursor;
use base64::{Engine, engine::general_purpose::STANDARD};
use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error)]
pub enum CursorError {
    #[error("cursor exceeds the supported size")]
    TooLarge,
    #[error("cursor is malformed or has an unsupported version")]
    Invalid,
    #[error("cursor belongs to a different query or scope")]
    ContextMismatch,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Payload {
    version: u8,
    context: String,
    values: Vec<Option<String>>,
}

impl Cursor {
    pub const MAX_ENCODED_BYTES: usize = 65_536;

    /// Encodes lossless ordering values. Base64 provides neither secrecy nor authority.
    pub fn encode(values: Vec<Option<String>>, context: &str) -> Result<Self, CursorError> {
        let payload = Payload {
            version: 1,
            context: context.into(),
            values,
        };
        let bytes = serde_json::to_vec(&payload).map_err(|_| CursorError::Invalid)?;
        let encoded = STANDARD.encode(bytes);
        if encoded.len() > Self::MAX_ENCODED_BYTES {
            return Err(CursorError::TooLarge);
        }
        Ok(Self(encoded))
    }

    pub fn decode(&self, context: &str) -> Result<Vec<Option<String>>, CursorError> {
        if self.0.len() > Self::MAX_ENCODED_BYTES {
            return Err(CursorError::TooLarge);
        }
        let bytes = STANDARD.decode(&self.0).map_err(|_| CursorError::Invalid)?;
        let payload: Payload = serde_json::from_slice(&bytes).map_err(|_| CursorError::Invalid)?;
        if payload.version != 1 {
            return Err(CursorError::Invalid);
        }
        if payload.context != context {
            return Err(CursorError::ContextMismatch);
        }
        Ok(payload.values)
    }
}
