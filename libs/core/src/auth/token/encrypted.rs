use anyhow::Context;
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use chacha20poly1305::{
    ChaCha20Poly1305, KeyInit, Nonce,
    aead::{Aead, Payload},
};
use hkdf::Hkdf;
use rand::RngCore;
use sha2::Sha256;

use super::{AccessToken, EncryptedToken};
use crate::auth::AuthError;
use compallm_utils::time::unix_timestamp;

const AAD: &[u8] = b"compallm:auth:encrypted-access-token:v1";

/// Concrete encryption helper configured from the same secret used by Hero JWT.
/// HKDF separates the cipher key from the raw HS256 signing key. No env reads.
pub struct EncryptedAccessToken {
    cipher: ChaCha20Poly1305,
}

impl EncryptedAccessToken {
    pub fn new(jwt_secret: &str) -> Result<Self, AuthError> {
        if jwt_secret.len() < 32 {
            return Err(AuthError::InvalidConfiguration);
        }
        let mut key = [0u8; 32];
        Hkdf::<Sha256>::new(Some(AAD), jwt_secret.as_bytes())
            .expand(b"payload-encryption", &mut key)
            .context("failed to derive the access-token encryption key")?;
        Ok(Self {
            cipher: ChaCha20Poly1305::new((&key).into()),
        })
    }

    pub fn encrypt(&self, token: AccessToken) -> Result<AccessToken, AuthError> {
        let now = unix_timestamp();
        token.validate_plain(now)?;
        let plaintext = serde_json::to_vec(&token)
            .context("failed to serialize access-token claims for encryption")?;
        if plaintext.len() > 4096 {
            return Err(AuthError::InvalidToken);
        }
        let mut nonce = [0u8; 12];
        rand::rng().fill_bytes(&mut nonce);
        let ciphertext = self.cipher.encrypt(
            Nonce::from_slice(&nonce),
            Payload {
                msg: &plaintext,
                aad: AAD,
            },
        )?;
        let mut bytes = nonce.to_vec();
        bytes.extend(ciphertext);
        Ok(AccessToken::Encrypted(EncryptedToken(format!(
            "v1.{}",
            URL_SAFE_NO_PAD.encode(bytes)
        ))))
    }

    pub fn decrypt(&self, token: EncryptedToken) -> Result<AccessToken, AuthError> {
        if token.0.len() > 8192 {
            return Err(AuthError::InvalidToken);
        }
        let encoded = token.0.strip_prefix("v1.").ok_or(AuthError::InvalidToken)?;
        let bytes = URL_SAFE_NO_PAD.decode(encoded)?;
        if bytes.len() < 28 {
            return Err(AuthError::InvalidToken);
        }
        let plaintext = self.cipher.decrypt(
            Nonce::from_slice(&bytes[..12]),
            Payload {
                msg: &bytes[12..],
                aad: AAD,
            },
        )?;
        let token: AccessToken = serde_json::from_slice(&plaintext)?;
        token.validate_plain(unix_timestamp())?;
        Ok(token)
    }
}
