use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use sha2::{Digest, Sha256};
use std::sync::Arc;

/// A shared dictionary category; its contents are not cryptographic secrets.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DictionaryType {
    /// Shared LZ4 vocabulary for serialized authentication tokens, including Cedar policy.
    AccessToken,
}

/// One immutable version, shared by concurrent operations.
pub struct DCompression {
    pub kind: DictionaryType,
    pub version: u32,
    prepared: Arc<PreparedDictionary>,
}

struct PreparedDictionary {
    fingerprint: [u8; 32],
    bytes: Vec<u8>,
}

impl DCompression {
    pub const MAX_BYTES: usize = 256 * 1024;
    const DICTIONARY_WINDOW_BYTES: usize = 64 * 1024;

    pub fn new(kind: DictionaryType, version: u32, content: &str) -> anyhow::Result<Self> {
        Self::from_items(kind, version, vec![content.to_owned()])
    }

    /// Assemble ordered, human-readable items into LZ4's external dictionary window.
    pub fn from_items(
        kind: DictionaryType,
        version: u32,
        items: Vec<String>,
    ) -> anyhow::Result<Self> {
        let mut bytes = Vec::new();
        for item in items {
            let item = item.as_bytes();
            if item.len() >= Self::DICTIONARY_WINDOW_BYTES {
                bytes.clear();
                bytes.extend_from_slice(&item[item.len() - Self::DICTIONARY_WINDOW_BYTES..]);
            } else {
                let excess = bytes
                    .len()
                    .saturating_add(item.len())
                    .saturating_sub(Self::DICTIONARY_WINDOW_BYTES);
                bytes.drain(..excess);
                bytes.extend_from_slice(item);
            }
        }
        Ok(Self {
            kind,
            version,
            prepared: Arc::new(PreparedDictionary {
                fingerprint: Sha256::digest(&bytes).into(),
                bytes,
            }),
        })
    }

    /// Compress UTF-8 and encode the dictionary fingerprint and LZ4 block.
    pub async fn compress(&self, value: &str) -> anyhow::Result<String> {
        let bytes = self.compress_bytes(value.as_bytes()).await?;
        Ok(tokio::task::spawn_blocking(move || URL_SAFE_NO_PAD.encode(bytes)).await?)
    }

    /// Compress binary input using the versioned external dictionary.
    pub async fn compress_bytes(&self, value: &[u8]) -> anyhow::Result<Vec<u8>> {
        anyhow::ensure!(
            value.len() <= Self::MAX_BYTES,
            "compression input exceeds limit"
        );
        let value = value.to_vec();
        let prepared = Arc::clone(&self.prepared);
        tokio::task::spawn_blocking(move || {
            let compressed =
                lz4_flex::block::compress_prepend_size_with_dict(&value, &prepared.bytes);
            let mut output = Vec::with_capacity(prepared.fingerprint.len() + compressed.len());
            output.extend_from_slice(&prepared.fingerprint);
            output.extend_from_slice(&compressed);
            Ok(output)
        })
        .await?
    }

    /// Decode with the exact dictionary and a bounded allocation.
    pub async fn decompress(&self, value: &str) -> anyhow::Result<String> {
        let max_encoded =
            (lz4_flex::block::get_maximum_output_size(Self::MAX_BYTES) + 36).div_ceil(3) * 4;
        anyhow::ensure!(value.len() <= max_encoded, "compressed input exceeds limit");
        let value = value.to_owned();
        let bytes = tokio::task::spawn_blocking(move || URL_SAFE_NO_PAD.decode(value)).await??;
        Ok(String::from_utf8(self.decompress_bytes(&bytes).await?)?)
    }

    /// Decompress a binary block with a bounded allocation.
    pub async fn decompress_bytes(&self, value: &[u8]) -> anyhow::Result<Vec<u8>> {
        anyhow::ensure!(
            value.len() <= lz4_flex::block::get_maximum_output_size(Self::MAX_BYTES) + 36,
            "compressed input exceeds limit"
        );
        let bytes = value.to_vec();
        let prepared = Arc::clone(&self.prepared);
        tokio::task::spawn_blocking(move || {
            anyhow::ensure!(
                bytes.len() >= 36 && bytes[..32] == prepared.fingerprint,
                "dictionary mismatch"
            );
            let size = u32::from_le_bytes(bytes[32..36].try_into()?) as usize;
            anyhow::ensure!(
                size <= Self::MAX_BYTES,
                "decompression output exceeds limit"
            );
            let mut output = vec![0; size];
            let written = lz4_flex::block::decompress_into_with_dict(
                &bytes[36..],
                &mut output,
                &prepared.bytes,
            )?;
            anyhow::ensure!(written == size, "decompressed size mismatch");
            Ok(output)
        })
        .await?
    }
}
