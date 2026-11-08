//! Hero-signed JWT payloads. Only email OTP claims need extra encryption.
//! The enum validates shared rules and delegates to the independent token types.

use super::AuthError;
use crate::compression::{DCompression, DictionaryType};
use crate::persistent::dictionary::DictionaryRepository;
use compallm_utils::email::normalize_email;
use compallm_utils::time::unix_timestamp;
use hero::token::jwt::{create_jwt_token, decode_jwt_token};
use serde::{Deserialize, Serialize};
use std::time::Duration;

mod compress_token;
mod email_otp;
mod encrypted;
mod refresh;
mod user;
pub mod validity;

pub use compress_token::CompressedToken;
pub use email_otp::EmailOtpAccessToken;
pub use encrypted::{EncryptedAccessToken, EncryptedToken};
pub use refresh::RefreshToken;
pub use user::UserAccessToken;
pub use validity::TokenValidity;

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccessToken {
    Compressed(CompressedToken),
    Encrypted(EncryptedToken),
    /// Internal payload: must be encrypted before signing a browser credential.
    EmailOtp(EmailOtpAccessToken),
    User(UserAccessToken),
    Refresh(RefreshToken),
}

impl AccessToken {
    /// Wire budget for both bearer and cookie credentials; allow cookie attributes.
    pub const MAX_ENCODED_BYTES: usize = 3_800;

    pub fn check_size(size: usize) -> Result<(), AuthError> {
        if size > Self::MAX_ENCODED_BYTES {
            return Err(AuthError::TokenTooLarge {
                size,
                limit: Self::MAX_ENCODED_BYTES,
            });
        }
        Ok(())
    }

    pub fn validate(&self, now: u64) -> Result<(), AuthError> {
        match self {
            Self::Compressed(token) => {
                if token.dictionary_version == 0 || token.data.is_empty() {
                    Err(AuthError::InvalidToken)
                } else {
                    Ok(())
                }
            }
            Self::EmailOtp(token) => token.validate(now),
            Self::User(token) => token.validate(now),
            Self::Refresh(token) => token.validate(now),
            Self::Encrypted(token) => token.validate(),
        }
    }

    pub fn encrypt(self, cipher: &EncryptedAccessToken) -> Result<Self, AuthError> {
        if matches!(self, Self::Encrypted(_) | Self::Compressed(_)) {
            return Err(AuthError::InvalidToken);
        }
        self.validate(unix_timestamp())?;
        Ok(Self::Encrypted(cipher.encrypt(&self)?))
    }

    pub async fn compress(self, dictionary: &DCompression) -> Result<Self, AuthError> {
        if dictionary.kind != DictionaryType::AccessToken || dictionary.version == 0 {
            return Err(AuthError::InvalidConfiguration);
        }
        if matches!(self, Self::Compressed(_)) {
            return Err(AuthError::InvalidToken);
        }
        if matches!(self, Self::EmailOtp(_)) {
            return Err(AuthError::WrongTokenVariant);
        }
        self.validate(unix_timestamp())?;
        Ok(Self::Compressed(CompressedToken {
            dictionary_version: dictionary.version,
            data: dictionary.compress(&serde_json::to_string(&self)?).await?,
        }))
    }

    pub fn encode(&self, secret: &str, lifetime: Duration) -> Result<String, AuthError> {
        if secret.len() < 32 || lifetime.as_secs() == 0 {
            return Err(AuthError::InvalidConfiguration);
        }
        if !matches!(self, Self::Compressed(_)) {
            return Err(AuthError::WrongTokenVariant);
        }
        self.validate(unix_timestamp())?;
        let encoded =
            create_jwt_token(self, secret, lifetime).map_err(|_| AuthError::InvalidToken)?;
        Self::check_size(encoded.len())?;
        Ok(encoded)
    }

    /// Verify the signed envelope before decrypting or validating inner claims.
    pub async fn decode(
        encoded: &str,
        secret: &str,
        dictionaries: &dyn DictionaryRepository,
    ) -> Result<Self, AuthError> {
        if secret.len() < 32 {
            return Err(AuthError::InvalidConfiguration);
        }
        Self::check_size(encoded.len())?;
        let token: Self = decode_jwt_token(encoded, secret).map_err(|error| match error {
            hero::token::jwt::JwtErrors::CodingError(error)
                if matches!(
                    error.kind(),
                    jsonwebtoken::errors::ErrorKind::ExpiredSignature
                ) =>
            {
                AuthError::ExpiredToken
            }
            _ => AuthError::InvalidToken,
        })?;
        token.validate(unix_timestamp())?;
        let token = match token {
            Self::Compressed(payload) => {
                let version = payload.dictionary_version;
                let dictionary = dictionaries
                    .get(DictionaryType::AccessToken, version)
                    .await
                    .map_err(|_| AuthError::DictionaryUnavailable)?
                    .ok_or(AuthError::DictionaryUnavailable)?;
                if dictionary.kind != DictionaryType::AccessToken || dictionary.version != version {
                    return Err(AuthError::DictionaryUnavailable);
                }
                let source = dictionary
                    .decompress(&payload.data)
                    .await
                    .map_err(|_| AuthError::InvalidToken)?;
                serde_json::from_str(&source).map_err(|_| AuthError::InvalidToken)?
            }
            token => token,
        };
        token.validate(unix_timestamp())?;
        match token {
            Self::Encrypted(payload) => {
                let token: Self = EncryptedAccessToken::new(secret)?.decrypt(&payload)?;
                if !matches!(token, Self::EmailOtp(_)) {
                    return Err(AuthError::WrongTokenVariant);
                }
                token.validate(unix_timestamp())?;
                Ok(token)
            }
            Self::EmailOtp(_) | Self::Compressed(_) => Err(AuthError::WrongTokenVariant),
            token => Ok(token),
        }
    }
}

impl TryFrom<AccessToken> for EmailOtpAccessToken {
    type Error = AuthError;
    fn try_from(token: AccessToken) -> Result<Self, Self::Error> {
        match token {
            AccessToken::EmailOtp(token) => Ok(token),
            _ => Err(AuthError::WrongTokenVariant),
        }
    }
}

impl TryFrom<AccessToken> for UserAccessToken {
    type Error = AuthError;
    fn try_from(token: AccessToken) -> Result<Self, Self::Error> {
        match token {
            AccessToken::User(token) => Ok(token),
            _ => Err(AuthError::WrongTokenVariant),
        }
    }
}

impl TryFrom<AccessToken> for RefreshToken {
    type Error = AuthError;
    fn try_from(token: AccessToken) -> Result<Self, Self::Error> {
        match token {
            AccessToken::Refresh(token) => Ok(token),
            _ => Err(AuthError::WrongTokenVariant),
        }
    }
}

/// Server-only delivery result. Send the code through email, never to the browser.
pub struct EmailOtpChallenge {
    pub access_token: String,
    pub code: String,
    pub validity: TokenValidity,
}

pub struct IssuedEmailOtp {
    pub token: String,
    pub validity: TokenValidity,
}

impl EmailOtpChallenge {
    pub async fn new(
        email: &str,
        secret: &str,
        dictionary: &DCompression,
    ) -> Result<Self, AuthError> {
        let email = normalize_email(email);
        let claims = EmailOtpAccessToken::new(email)?;
        let code = claims.code.clone();
        let validity = claims.validity.clone();
        let access_token = AccessToken::EmailOtp(claims)
            .encrypt(&EncryptedAccessToken::new(secret)?)?
            .compress(dictionary)
            .await?
            .encode(
                secret,
                Duration::from_secs(EmailOtpAccessToken::TTL_SECONDS),
            )?;
        Ok(Self {
            access_token,
            code,
            validity,
        })
    }
}

pub struct UserTokens {
    pub access_token: String,
    pub refresh_token: String,
    pub access_validity: TokenValidity,
    pub refresh_validity: TokenValidity,
}
