//! Authenticated CBOR Web Tokens. Only email OTP claims need extra encryption.

use super::AuthError;
use crate::compression::{DCompression, DictionaryType};
use crate::persistent::dictionary::DictionaryRepository;
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use compallm_utils::email::normalize_email;
use compallm_utils::time::unix_timestamp;
use coset::{
    CborSerializable, CoseMac0, CoseMac0Builder, Header, HeaderBuilder,
    cwt::{ClaimName, ClaimsSet, ClaimsSetBuilder, Timestamp},
    iana,
};
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::time::Duration;

const TOKEN_CLAIM: i64 = -65537;

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
    #[serde(rename = "c")]
    Compressed(CompressedToken),
    #[serde(rename = "e")]
    Encrypted(EncryptedToken),
    /// Internal payload: must be encrypted before signing a browser credential.
    #[serde(rename = "o")]
    EmailOtp(EmailOtpAccessToken),
    #[serde(rename = "u")]
    User(UserAccessToken),
    #[serde(rename = "r")]
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

    pub async fn encrypt(self, cipher: &EncryptedAccessToken) -> Result<Self, AuthError> {
        if matches!(self, Self::Encrypted(_) | Self::Compressed(_)) {
            return Err(AuthError::InvalidToken);
        }
        self.validate(unix_timestamp())?;
        Ok(Self::Encrypted(cipher.encrypt(&self).await?))
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
        let mut serialized = Vec::new();
        ciborium::ser::into_writer(&self, &mut serialized).map_err(|_| AuthError::InvalidToken)?;
        Ok(Self::Compressed(CompressedToken {
            dictionary_version: dictionary.version,
            data: dictionary.compress_bytes(&serialized).await?,
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
        let issued = unix_timestamp();
        let expires = issued
            .checked_add(lifetime.as_secs())
            .ok_or(AuthError::InvalidConfiguration)?;
        let mut token = Vec::new();
        ciborium::ser::into_writer(self, &mut token).map_err(|_| AuthError::InvalidToken)?;
        let claims = ClaimsSetBuilder::new()
            .issued_at(Timestamp::WholeSeconds(
                i64::try_from(issued).map_err(|_| AuthError::InvalidConfiguration)?,
            ))
            .expiration_time(Timestamp::WholeSeconds(
                i64::try_from(expires).map_err(|_| AuthError::InvalidConfiguration)?,
            ))
            .private_claim(TOKEN_CLAIM, ciborium::value::Value::Bytes(token))
            .build()
            .to_vec()
            .map_err(|_| AuthError::InvalidToken)?;
        let protected = HeaderBuilder::new()
            .algorithm(iana::Algorithm::HMAC_256_256)
            .build();
        let mac = CoseMac0Builder::new()
            .protected(protected)
            .payload(claims)
            .try_create_tag(b"", |message| -> Result<Vec<u8>, AuthError> {
                let mut mac = <Hmac<Sha256> as Mac>::new_from_slice(secret.as_bytes())
                    .map_err(|_| AuthError::InvalidConfiguration)?;
                mac.update(message);
                Ok(mac.finalize().into_bytes().to_vec())
            })?
            .build()
            .to_vec()
            .map_err(|_| AuthError::InvalidToken)?;
        let encoded = URL_SAFE_NO_PAD.encode(mac);
        Self::check_size(encoded.len())?;
        Ok(encoded)
    }

    /// Verify COSE authentication and CWT expiry before decompressing inner claims.
    pub async fn decode(
        encoded: &str,
        secret: &str,
        dictionaries: &dyn DictionaryRepository,
    ) -> Result<Self, AuthError> {
        if secret.len() < 32 {
            return Err(AuthError::InvalidConfiguration);
        }
        Self::check_size(encoded.len())?;
        let bytes = URL_SAFE_NO_PAD
            .decode(encoded)
            .map_err(|_| AuthError::InvalidToken)?;
        let cose = CoseMac0::from_slice(&bytes).map_err(|_| AuthError::InvalidToken)?;
        let expected_header = HeaderBuilder::new()
            .algorithm(iana::Algorithm::HMAC_256_256)
            .build();
        if cose.protected.header != expected_header || cose.unprotected != Header::default() {
            return Err(AuthError::InvalidToken);
        }
        cose.verify_payload_tag(
            b"",
            || AuthError::InvalidToken,
            |tag, message| {
                let mut mac = <Hmac<Sha256> as Mac>::new_from_slice(secret.as_bytes())
                    .map_err(|_| AuthError::InvalidConfiguration)?;
                mac.update(message);
                mac.verify_slice(tag).map_err(|_| AuthError::InvalidToken)
            },
        )?;
        let claims = ClaimsSet::from_slice(cose.payload.as_deref().ok_or(AuthError::InvalidToken)?)
            .map_err(|_| AuthError::InvalidToken)?;
        let (Some(Timestamp::WholeSeconds(issued)), Some(Timestamp::WholeSeconds(expires))) =
            (claims.issued_at, claims.expiration_time)
        else {
            return Err(AuthError::InvalidToken);
        };
        let now = i64::try_from(unix_timestamp()).map_err(|_| AuthError::InvalidToken)?;
        if issued < 0 || issued > now || expires <= issued {
            return Err(AuthError::InvalidToken);
        }
        if now >= expires {
            return Err(AuthError::ExpiredToken);
        }
        if claims.issuer.is_some()
            || claims.subject.is_some()
            || claims.audience.is_some()
            || claims.not_before.is_some()
            || claims.cwt_id.is_some()
            || claims.rest.len() != 1
        {
            return Err(AuthError::InvalidToken);
        }
        let (ClaimName::PrivateUse(TOKEN_CLAIM), ciborium::value::Value::Bytes(token)) =
            &claims.rest[0]
        else {
            return Err(AuthError::InvalidToken);
        };
        let mut input = token.as_slice();
        let token: Self =
            ciborium::de::from_reader(&mut input).map_err(|_| AuthError::InvalidToken)?;
        if !input.is_empty() {
            return Err(AuthError::InvalidToken);
        }
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
                    .decompress_bytes(&payload.data)
                    .await
                    .map_err(|_| AuthError::InvalidToken)?;
                let mut source = source.as_slice();
                let token =
                    ciborium::de::from_reader(&mut source).map_err(|_| AuthError::InvalidToken)?;
                if !source.is_empty() {
                    return Err(AuthError::InvalidToken);
                }
                token
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
            .encrypt(&EncryptedAccessToken::new(secret)?)
            .await?
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
