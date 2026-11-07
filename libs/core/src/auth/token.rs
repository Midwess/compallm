//! Hero-signed JWT payloads. Only email OTP claims need extra encryption.
//! The enum validates shared rules and delegates to the independent token types.

use super::AuthError;
use compallm_utils::email::normalize_email;
use compallm_utils::time::unix_timestamp;
use hero::token::jwt::create_jwt_token;
use serde::{Deserialize, Serialize};
use std::time::Duration;

mod email_otp;
mod encrypted;
mod refresh;
mod user;
pub mod validity;

pub use email_otp::EmailOtpAccessToken;
pub use encrypted::{EncryptedAccessToken, EncryptedToken};
pub use refresh::RefreshToken;
pub use user::UserAccessToken;
pub use validity::TokenValidity;

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccessToken {
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
            Self::EmailOtp(token) => token.validate(now),
            Self::User(token) => token.validate(now),
            Self::Refresh(token) => token.validate(now),
            Self::Encrypted(token) => token.validate(),
        }
    }

    pub fn encrypt(self, cipher: &EncryptedAccessToken) -> Result<Self, AuthError> {
        if matches!(self, Self::Encrypted(_)) {
            return Err(AuthError::InvalidToken);
        }
        self.validate(unix_timestamp())?;
        Ok(Self::Encrypted(cipher.encrypt(&self)?))
    }

    pub fn encode(&self, secret: &str, lifetime: Duration) -> Result<String, AuthError> {
        if secret.len() < 32 || lifetime.as_secs() == 0 {
            return Err(AuthError::InvalidConfiguration);
        }
        if matches!(self, Self::EmailOtp(_)) {
            return Err(AuthError::WrongTokenVariant);
        }
        self.validate(unix_timestamp())?;
        let encoded = if let Self::User(user) = self {
            // Preserve Hero's envelope while capping its expiry at the access authority deadline.
            #[derive(Serialize)]
            struct Claims<'a> {
                exp: u64,
                iat: u64,
                data: &'a AccessToken,
            }
            let iat = unix_timestamp();
            let exp = iat
                .saturating_add(lifetime.as_secs())
                .min(user.validity.expires_at);
            if exp <= iat {
                return Err(AuthError::ExpiredToken);
            }
            jsonwebtoken::encode(
                &jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256),
                &Claims {
                    exp,
                    iat,
                    data: self,
                },
                &jsonwebtoken::EncodingKey::from_secret(secret.as_bytes()),
            )
            .map_err(|_| AuthError::InvalidToken)?
        } else {
            create_jwt_token(self, secret, lifetime).map_err(|_| AuthError::InvalidToken)?
        };
        Self::check_size(encoded.len())?;
        Ok(encoded)
    }

    /// Verify the signed envelope before decrypting or validating inner claims.
    pub fn decode(encoded: &str, secret: &str) -> Result<Self, AuthError> {
        if secret.len() < 32 {
            return Err(AuthError::InvalidConfiguration);
        }
        Self::check_size(encoded.len())?;
        #[derive(Deserialize)]
        struct Claims {
            exp: u64,
            #[serde(rename = "iat")]
            _issued_at: u64,
            data: AccessToken,
        }
        let verified = jsonwebtoken::decode::<Claims>(
            encoded,
            &jsonwebtoken::DecodingKey::from_secret(secret.as_bytes()),
            &jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256),
        )
        .map_err(|error| {
            if matches!(
                error.kind(),
                jsonwebtoken::errors::ErrorKind::ExpiredSignature
            ) {
                AuthError::ExpiredToken
            } else {
                AuthError::InvalidToken
            }
        })?;
        let expires_at = verified.claims.exp;
        let cap_expiry = |mut token: Self| {
            if let Self::User(user) = &mut token {
                user.validity.expires_at = user.validity.expires_at.min(expires_at);
            }
            token
        };
        let token = cap_expiry(verified.claims.data);
        token.validate(unix_timestamp())?;
        match token {
            Self::Encrypted(payload) => {
                let token: Self = EncryptedAccessToken::new(secret)?.decrypt(&payload)?;
                if matches!(token, Self::Encrypted(_)) {
                    return Err(AuthError::InvalidToken);
                }
                let token = cap_expiry(token);
                token.validate(unix_timestamp())?;
                Ok(token)
            }
            Self::EmailOtp(_) => Err(AuthError::WrongTokenVariant),
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
    pub fn new(email: &str, secret: &str) -> Result<Self, AuthError> {
        let email = normalize_email(email);
        let claims = EmailOtpAccessToken::new(email)?;
        let code = claims.code.clone();
        let validity = claims.validity.clone();
        let access_token = AccessToken::EmailOtp(claims)
            .encrypt(&EncryptedAccessToken::new(secret)?)?
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

impl UserTokens {
    /// Call after the workflow authorizes the user.
    pub fn new(
        session: &super::Session,
        organization_role: impl Into<String>,
        policy_snapshot: crate::authorization::PolicySnapshot,
        secret: &str,
        user_lifetime: Duration,
        refresh_lifetime: Duration,
    ) -> Result<Self, AuthError> {
        let refresh = RefreshToken::from_session(session, refresh_lifetime)?;
        Self::from_refresh(
            &refresh,
            organization_role,
            policy_snapshot,
            secret,
            user_lifetime,
        )
    }

    pub fn from_refresh(
        refresh: &RefreshToken,
        organization_role: impl Into<String>,
        policy_snapshot: crate::authorization::PolicySnapshot,
        secret: &str,
        user_lifetime: Duration,
    ) -> Result<Self, AuthError> {
        refresh.validate(unix_timestamp())?;
        let refresh_lifetime =
            Duration::from_secs(refresh.validity.expires_at - refresh.validity.issued_at);
        let user = UserAccessToken::new(
            refresh.session.clone(),
            organization_role,
            policy_snapshot,
            user_lifetime,
        )?;
        let access_validity = user.validity.clone();
        Ok(Self {
            access_validity,
            refresh_validity: refresh.validity.clone(),
            access_token: AccessToken::User(user).encode(secret, user_lifetime)?,
            refresh_token: AccessToken::Refresh(refresh.clone())
                .encode(secret, refresh_lifetime)?,
        })
    }
}
