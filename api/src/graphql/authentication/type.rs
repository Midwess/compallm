use super::input::CredentialDelivery;
use crate::{graphql::error::Error, http::authentication::CookieDelivery};
use compallm_core::authentication::{
    EmailOtpAccessToken, IssuedEmailOtp, TokenValidity, UserTokens,
};

/// Exact Unix-second validity and client throttle values encoded as decimal strings.
pub struct TokenLifetime {
    pub issued_at: String,
    pub expires_at: String,
    pub expires_in_seconds: String,
    pub throttle_seconds: String,
}

impl From<(TokenValidity, u64)> for TokenLifetime {
    fn from((validity, throttle): (TokenValidity, u64)) -> Self {
        Self {
            issued_at: validity.issued_at.to_string(),
            expires_at: validity.expires_at.to_string(),
            expires_in_seconds: validity
                .expires_at
                .saturating_sub(compallm_utils::time::unix_timestamp())
                .to_string(),
            throttle_seconds: throttle.to_string(),
        }
    }
}

/// Email admission succeeded; cookie delivery never exposes the OTP credential.
pub struct EmailChallenge {
    pub lifetime: TokenLifetime,
    pub token: Option<String>,
}

/// Timing is always returned; credential values exist only in explicit TOKEN delivery.
pub struct SessionCredentials {
    pub access_lifetime: TokenLifetime,
    pub refresh_lifetime: TokenLifetime,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
}

impl TryFrom<(IssuedEmailOtp, CredentialDelivery, &CookieDelivery)> for EmailChallenge {
    type Error = Error;
    fn try_from(
        (issued, mode, cookies): (IssuedEmailOtp, CredentialDelivery, &CookieDelivery),
    ) -> Result<Self, Error> {
        let remaining = issued
            .validity
            .expires_at
            .saturating_sub(compallm_utils::time::unix_timestamp());
        let lifetime = (issued.validity, EmailOtpAccessToken::THROTTLE_SECONDS).into();
        let token = match mode {
            CredentialDelivery::Token => Some(issued.token),
            CredentialDelivery::Cookie => {
                cookies.set_otp(issued.token, remaining)?;
                None
            }
        };
        Ok(Self { lifetime, token })
    }
}

impl TryFrom<(UserTokens, CredentialDelivery, &CookieDelivery)> for SessionCredentials {
    type Error = Error;
    fn try_from(
        (tokens, mode, cookies): (UserTokens, CredentialDelivery, &CookieDelivery),
    ) -> Result<Self, Error> {
        let access_lifetime = (tokens.access_validity.clone(), 0).into();
        let refresh_lifetime = (tokens.refresh_validity.clone(), 0).into();
        let (access_token, refresh_token) = match mode {
            CredentialDelivery::Token => (Some(tokens.access_token), Some(tokens.refresh_token)),
            CredentialDelivery::Cookie => {
                cookies.set_user(tokens)?;
                (None, None)
            }
        };
        Ok(Self {
            access_lifetime,
            refresh_lifetime,
            access_token,
            refresh_token,
        })
    }
}

/// Exact Unix-second validity and client throttle values encoded as decimal strings.
#[juniper::graphql_object]
impl TokenLifetime {
    fn issued_at(&self) -> &str {
        &self.issued_at
    }
    fn expires_at(&self) -> &str {
        &self.expires_at
    }
    fn expires_in_seconds(&self) -> &str {
        &self.expires_in_seconds
    }
    fn throttle_seconds(&self) -> &str {
        &self.throttle_seconds
    }
}

/// Email admission succeeded; cookie delivery never exposes the OTP credential.
#[juniper::graphql_object]
impl EmailChallenge {
    fn lifetime(&self) -> &TokenLifetime {
        &self.lifetime
    }
    fn token(&self) -> Option<&str> {
        self.token.as_deref()
    }
}

/// Timing is always returned; credential values exist only in explicit TOKEN delivery.
#[juniper::graphql_object]
impl SessionCredentials {
    fn access_lifetime(&self) -> &TokenLifetime {
        &self.access_lifetime
    }
    fn refresh_lifetime(&self) -> &TokenLifetime {
        &self.refresh_lifetime
    }
    fn access_token(&self) -> Option<&str> {
        self.access_token.as_deref()
    }
    fn refresh_token(&self) -> Option<&str> {
        self.refresh_token.as_deref()
    }
}
