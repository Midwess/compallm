use std::fmt;

use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use chacha20poly1305::{
    ChaCha20Poly1305, KeyInit, Nonce,
    aead::{Aead, Payload},
};
use compallm_utils::input;
use hkdf::Hkdf;
use rand::RngCore;
use sha2::Sha256;

use super::ProviderApiKeyError;

const CONTEXT: &[u8] = b"compallm:provider:credentials:v1";
const NONCE_LENGTH: usize = 12;
const TAG_LENGTH: usize = 16;

/// Reusable provider-specific cipher. The caller supplies configuration explicitly.
pub struct ProviderCredentialCipher(ChaCha20Poly1305);

impl ProviderCredentialCipher {
    pub fn new(secret: &str) -> Result<Self, ProviderApiKeyError> {
        if secret.len() < 32 {
            return Err(ProviderApiKeyError::InvalidConfiguration);
        }
        let mut key = [0u8; 32];
        Hkdf::<Sha256>::new(Some(CONTEXT), secret.as_bytes())
            .expand(b"credential-encryption", &mut key)
            .map_err(|_| ProviderApiKeyError::InvalidConfiguration)?;
        Ok(Self(ChaCha20Poly1305::new((&key).into())))
    }
}

/// Always encrypted, including on repository reads and clones.
///
/// Debug is redacted and serialization is deliberately not implemented.
/// `read` decrypts a temporary value without changing or caching this ciphertext.
#[derive(Clone, PartialEq, Eq)]
pub struct ProviderApiKey(Vec<u8>);

impl ProviderApiKey {
    /// Encrypt a submitted API key immediately; plaintext is never stored here.
    pub fn new(
        value: &str,
        cipher: &ProviderCredentialCipher,
    ) -> Result<Self, ProviderApiKeyError> {
        input::api_key(value).map_err(|_| ProviderApiKeyError::InvalidKey)?;
        let mut nonce = [0u8; NONCE_LENGTH];
        rand::rng().fill_bytes(&mut nonce);
        let ciphertext = cipher
            .0
            .encrypt(
                Nonce::from_slice(&nonce),
                Payload {
                    msg: value.as_bytes(),
                    aad: CONTEXT,
                },
            )
            .map_err(|_| ProviderApiKeyError::EncryptionFailed)?;
        let mut bytes = nonce.to_vec();
        bytes.extend(ciphertext);
        Ok(Self(bytes))
    }

    pub fn read(&self, cipher: &ProviderCredentialCipher) -> Result<String, ProviderApiKeyError> {
        let plaintext = cipher
            .0
            .decrypt(
                Nonce::from_slice(&self.0[..NONCE_LENGTH]),
                Payload {
                    msg: &self.0[NONCE_LENGTH..],
                    aad: CONTEXT,
                },
            )
            .map_err(|_| ProviderApiKeyError::DecryptionFailed)?;
        let value =
            String::from_utf8(plaintext).map_err(|_| ProviderApiKeyError::DecryptionFailed)?;
        input::api_key(&value).map_err(|_| ProviderApiKeyError::DecryptionFailed)?;
        Ok(value)
    }
}

impl TryFrom<String> for ProviderApiKey {
    type Error = ProviderApiKeyError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let encoded = value
            .strip_prefix("v1.")
            .ok_or(ProviderApiKeyError::InvalidCiphertext)?;
        let bytes = URL_SAFE_NO_PAD
            .decode(encoded)
            .map_err(|_| ProviderApiKeyError::InvalidCiphertext)?;
        if bytes.len() <= NONCE_LENGTH + TAG_LENGTH {
            return Err(ProviderApiKeyError::InvalidCiphertext);
        }
        Ok(Self(bytes))
    }
}

impl From<ProviderApiKey> for String {
    fn from(value: ProviderApiKey) -> Self {
        format!("v1.{}", URL_SAFE_NO_PAD.encode(value.0))
    }
}

impl fmt::Debug for ProviderApiKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("ProviderApiKey([REDACTED])")
    }
}
