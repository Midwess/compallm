use super::input::{AuthenticationPurpose, CredentialDelivery};
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
    pub purpose: AuthenticationPurpose,
    pub access_lifetime: Option<TokenLifetime>,
    pub refresh_lifetime: Option<TokenLifetime>,
    pub passkey_change_lifetime: Option<TokenLifetime>,
    pub passkey_change_token: Option<String>,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
}

impl TryFrom<(IssuedEmailOtp, CredentialDelivery, &CookieDelivery)> for EmailChallenge {
    type Error = Error;
    fn try_from(
        tuple: (IssuedEmailOtp, CredentialDelivery, &CookieDelivery),
    ) -> Result<Self, Error> {
        Self::try_from((tuple.0, tuple.1, tuple.2, AuthenticationPurpose::Login))
    }
}
impl
    TryFrom<(
        IssuedEmailOtp,
        CredentialDelivery,
        &CookieDelivery,
        AuthenticationPurpose,
    )> for EmailChallenge
{
    type Error = Error;
    fn try_from(
        (issued, mode, cookies, purpose): (
            IssuedEmailOtp,
            CredentialDelivery,
            &CookieDelivery,
            AuthenticationPurpose,
        ),
    ) -> Result<Self, Error> {
        let remaining = issued
            .validity
            .expires_at
            .saturating_sub(compallm_utils::time::unix_timestamp());
        let lifetime = (issued.validity, EmailOtpAccessToken::THROTTLE_SECONDS).into();
        let token = match mode {
            CredentialDelivery::Token => Some(issued.token),
            CredentialDelivery::Cookie => {
                if purpose == AuthenticationPurpose::PasskeyChange {
                    cookies.set_passkey_otp(issued.token, remaining)?;
                } else {
                    cookies.set_otp(issued.token, remaining)?;
                }
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
        let access_lifetime = Some((tokens.access_validity.clone(), 0).into());
        let refresh_lifetime = Some((tokens.refresh_validity.clone(), 0).into());
        let passkey_change_lifetime = tokens
            .passkey_change
            .as_ref()
            .map(|g| (g.claims.validity.clone(), 0).into());
        let passkey_change_token = if mode == CredentialDelivery::Token {
            tokens.passkey_change.as_ref().map(|g| g.token.clone())
        } else {
            None
        };
        let (access_token, refresh_token) = match mode {
            CredentialDelivery::Token => (Some(tokens.access_token), Some(tokens.refresh_token)),
            CredentialDelivery::Cookie => {
                cookies.set_user(tokens)?;
                (None, None)
            }
        };
        Ok(Self {
            purpose: AuthenticationPurpose::Login,
            passkey_change_lifetime,
            passkey_change_token,
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
    fn purpose(&self) -> AuthenticationPurpose {
        self.purpose
    }
    fn access_lifetime(&self) -> Option<&TokenLifetime> {
        self.access_lifetime.as_ref()
    }
    fn refresh_lifetime(&self) -> Option<&TokenLifetime> {
        self.refresh_lifetime.as_ref()
    }
    fn passkey_change_lifetime(&self) -> Option<&TokenLifetime> {
        self.passkey_change_lifetime.as_ref()
    }
    fn passkey_change_token(&self) -> Option<&str> {
        self.passkey_change_token.as_deref()
    }
    fn access_token(&self) -> Option<&str> {
        self.access_token.as_deref()
    }
    fn refresh_token(&self) -> Option<&str> {
        self.refresh_token.as_deref()
    }
}

impl
    TryFrom<(
        compallm_core::authentication::IssuedPasskeyChange,
        CredentialDelivery,
        &CookieDelivery,
    )> for SessionCredentials
{
    type Error = Error;
    fn try_from(
        (issued, mode, cookies): (
            compallm_core::authentication::IssuedPasskeyChange,
            CredentialDelivery,
            &CookieDelivery,
        ),
    ) -> Result<Self, Error> {
        let lifetime = Some((issued.claims.validity.clone(), 0).into());
        let token = if mode == CredentialDelivery::Token {
            Some(issued.token)
        } else {
            cookies.set_passkey_change(issued)?;
            None
        };
        Ok(Self {
            purpose: AuthenticationPurpose::PasskeyChange,
            access_lifetime: None,
            refresh_lifetime: None,
            access_token: None,
            refresh_token: None,
            passkey_change_lifetime: lifetime,
            passkey_change_token: token,
        })
    }
}
