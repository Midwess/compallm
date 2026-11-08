use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use sha2::{Digest, Sha256};
use zstd::{
    bulk::{Compressor, Decompressor},
    dict::{DecoderDictionary, EncoderDictionary},
};

/// A shared dictionary category; its contents are not cryptographic secrets.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DictionaryType {
    /// Shared Zstd vocabulary for the complete serialized user access token, including Cedar policy.
    AccessToken,
}

/// One immutable version, prepared once for reuse by concurrent operations.
pub struct Dictionary {
    pub kind: DictionaryType,
    pub version: u32,
    fingerprint: [u8; 32],
    encoder: EncoderDictionary<'static>,
    decoder: DecoderDictionary<'static>,
}

impl Dictionary {
    pub const MAX_BYTES: usize = 256 * 1024;

    /// Prepare exactly the ordered, concatenated UTF-8 dictionary content.
    pub fn new(kind: DictionaryType, version: u32, content: &str) -> anyhow::Result<Self> {
        let bytes = content.as_bytes();
        Ok(Self {
            kind,
            version,
            fingerprint: Sha256::digest(bytes).into(),
            encoder: EncoderDictionary::try_copy(bytes, 3)?,
            decoder: DecoderDictionary::try_copy(bytes)?,
        })
    }

    /// Compress UTF-8 and encode the fingerprint and frame as a URL-safe string.
    pub fn compress(&self, value: &str) -> anyhow::Result<String> {
        anyhow::ensure!(
            value.len() <= Self::MAX_BYTES,
            "compression input exceeds limit"
        );
        let mut compressor = Compressor::with_prepared_dictionary(&self.encoder)?;
        let compressed = compressor.compress(value.as_bytes())?;
        let mut output = Vec::with_capacity(self.fingerprint.len() + compressed.len());
        output.extend_from_slice(&self.fingerprint);
        output.extend_from_slice(&compressed);
        Ok(URL_SAFE_NO_PAD.encode(output))
    }

    /// Decode only with the exact dictionary, limiting allocations and output size.
    pub fn decompress(&self, value: &str) -> anyhow::Result<String> {
        // Zstd's maximum encoded output for our bounded input plus fingerprint.
        let max_encoded = (zstd::zstd_safe::compress_bound(Self::MAX_BYTES) + 32).div_ceil(3) * 4;
        anyhow::ensure!(value.len() <= max_encoded, "compressed input exceeds limit");
        let bytes = URL_SAFE_NO_PAD.decode(value)?;
        anyhow::ensure!(
            bytes.len() >= 32 && bytes[..32] == self.fingerprint,
            "dictionary mismatch"
        );
        let mut decompressor = Decompressor::with_prepared_dictionary(&self.decoder)?;
        Ok(String::from_utf8(
            decompressor.decompress(&bytes[32..], Self::MAX_BYTES)?,
        )?)
    }
}
