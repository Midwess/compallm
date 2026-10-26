//! Hero-signed JWT payloads. Only email OTP claims need extra encryption.
//! The enum validates shared rules and delegates to the independent token types.

use super::AuthError;
use compallm_utils::email::normalize_email;
use compallm_utils::time::unix_timestamp;
use hero::token::jwt::{create_jwt_token, decode_jwt_token};
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
        create_jwt_token(self, secret, lifetime).map_err(|_| AuthError::InvalidToken)
    }

    /// Verify the signed envelope before decrypting or validating inner claims.
    pub fn decode(encoded: &str, secret: &str) -> Result<Self, AuthError> {
        if secret.len() < 32 {
            return Err(AuthError::InvalidConfiguration);
        }
        if encoded.len() > 16 * 1024 {
            return Err(AuthError::InvalidToken);
        }
        let token: Self = decode_jwt_token(encoded, secret).map_err(|_| AuthError::InvalidToken)?;
        token.validate(unix_timestamp())?;
        match token {
            Self::Encrypted(payload) => {
                let token: Self = EncryptedAccessToken::new(secret)?.decrypt(&payload)?;
                if matches!(token, Self::Encrypted(_)) {
                    return Err(AuthError::InvalidToken);
                }
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
}

impl EmailOtpChallenge {
    pub fn new(email: &str, secret: &str) -> Result<Self, AuthError> {
        let email = normalize_email(email);
        let claims = EmailOtpAccessToken::new(email)?;
        let code = claims.code.clone();
        let access_token = AccessToken::EmailOtp(claims)
            .encrypt(&EncryptedAccessToken::new(secret)?)?
            .encode(
                secret,
                Duration::from_secs(EmailOtpAccessToken::TTL_SECONDS),
            )?;
        Ok(Self { access_token, code })
    }
}

pub struct UserTokens {
    pub access_token: String,
    pub refresh_token: String,
}

impl UserTokens {
    /// Call after the workflow authorizes the user.
    pub fn new(
        session: &super::Session,
        organization_role: crate::organization::OrganizationRole,
        secret: &str,
        user_lifetime: Duration,
        refresh_lifetime: Duration,
    ) -> Result<Self, AuthError> {
        let refresh = RefreshToken::from_session(session, refresh_lifetime)?;
        Self::from_refresh(&refresh, organization_role, secret, user_lifetime)
    }

    pub fn from_refresh(
        refresh: &RefreshToken,
        organization_role: crate::organization::OrganizationRole,
        secret: &str,
        user_lifetime: Duration,
    ) -> Result<Self, AuthError> {
        refresh.validate(unix_timestamp())?;
        let refresh_lifetime =
            Duration::from_secs(refresh.validity.expires_at - refresh.validity.issued_at);
        Ok(Self {
            access_token: AccessToken::User(UserAccessToken::new(
                refresh.session.clone(),
                organization_role,
                user_lifetime,
            )?)
            .encode(secret, user_lifetime)?,
            refresh_token: AccessToken::Refresh(refresh.clone())
                .encode(secret, refresh_lifetime)?,
        })
    }
}
