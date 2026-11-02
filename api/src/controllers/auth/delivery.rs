use super::SignupEmailResponse;
use crate::error::ApiError;
use axum::{
    Json,
    http::{
        HeaderMap, HeaderValue, StatusCode, Uri,
        header::{CACHE_CONTROL, HOST, ORIGIN, SET_COOKIE},
    },
    response::{IntoResponse, Response},
};
use compallm_core::auth::{EmailOtpAccessToken, IssuedEmailOtp, TokenValidity, UserTokens};
use cookie::{Cookie, SameSite, time::Duration};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// How each signup/signin step delivers access credentials. Never stored in JWT claims.
#[derive(Clone, Copy, Default, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum TokenDelivery {
    /// Set HttpOnly cookies; omit token values from JSON. Requires an allowed Origin.
    #[default]
    Cookie,
    /// Return token values in JSON without setting cookies.
    Token,
}

/// Controller-only cookie policy supplied by DI.
pub struct TokenDeliverySettings {
    pub secure: bool,
    pub public_origin: String,
}

/// Public timing metadata; contains no token claims or credentials.
#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TokenLifetime {
    /// Token issuance time, Unix seconds.
    pub issued_at: u64,
    /// Token expiry time, Unix seconds.
    pub expires_at: u64,
    /// Remaining whole seconds when the response is prepared. Zero means expired.
    pub expires_in_seconds: u64,
    /// Minimum client wait before requesting another token, in seconds.
    /// OTP responses return 30; access and refresh tokens return 0. Independent of expiry.
    #[schema(default = 0)]
    pub throttle_seconds: u64,
}

impl From<TokenValidity> for TokenLifetime {
    fn from(validity: TokenValidity) -> Self {
        Self::from((validity, 0))
    }
}

impl From<(TokenValidity, u64)> for TokenLifetime {
    fn from((validity, throttle_seconds): (TokenValidity, u64)) -> Self {
        Self {
            throttle_seconds,
            issued_at: validity.issued_at,
            expires_at: validity.expires_at,
            expires_in_seconds: validity
                .expires_at
                .saturating_sub(compallm_utils::time::unix_timestamp()),
        }
    }
}

/// Both delivery modes return validity metadata; only token delivery includes credentials.
#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AuthTokensResponse {
    /// Access-token validity, present for both delivery modes.
    pub access_lifetime: TokenLifetime,
    /// Refresh-token validity, present for both delivery modes.
    pub refresh_lifetime: TokenLifetime,
    /// Access JWT, present only when delivery is token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    /// Refresh JWT, present only when delivery is token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
}

impl TokenDeliverySettings {
    pub fn cookie_names(&self) -> (&'static str, &'static str) {
        if self.secure {
            ("__Host-access", "__Host-refresh")
        } else {
            ("compallm-access", "compallm-refresh")
        }
    }

    fn append_cookie(
        &self,
        headers: &mut HeaderMap,
        name: &str,
        value: String,
        max_age: u64,
    ) -> Result<(), ApiError> {
        let cookie = Cookie::build((name, value))
            .http_only(true)
            .secure(self.secure)
            .same_site(SameSite::Lax)
            .path("/")
            .max_age(Duration::seconds(max_age as i64))
            .build();
        headers.append(SET_COOKIE, HeaderValue::from_str(&cookie.to_string())?);
        Ok(())
    }

    pub fn set_otp_cookie(
        &self,
        headers: &mut HeaderMap,
        token: String,
        max_age: u64,
    ) -> Result<(), ApiError> {
        let names = self.cookie_names();
        self.append_cookie(headers, names.0, token, max_age)?;
        // An OTP access token has no session refresh credential.
        self.append_cookie(headers, names.1, String::new(), 0)
    }

    pub fn set_cookies(&self, headers: &mut HeaderMap, tokens: UserTokens) -> Result<(), ApiError> {
        let names = self.cookie_names();
        self.append_cookie(
            headers,
            names.0,
            tokens.access_token,
            tokens
                .access_validity
                .expires_at
                .saturating_sub(compallm_utils::time::unix_timestamp()),
        )?;
        self.append_cookie(
            headers,
            names.1,
            tokens.refresh_token,
            tokens
                .refresh_validity
                .expires_at
                .saturating_sub(compallm_utils::time::unix_timestamp()),
        )
    }

    pub fn clear_cookies(&self, headers: &mut HeaderMap) -> Result<(), ApiError> {
        let names = self.cookie_names();
        self.append_cookie(headers, names.0, String::new(), 0)?;
        self.append_cookie(headers, names.1, String::new(), 0)
    }

    pub fn validate_cookie_request(&self, headers: &HeaderMap, safe: bool) -> Result<(), ApiError> {
        if !headers.contains_key(ORIGIN)
            && safe
            && headers
                .get("sec-fetch-site")
                .is_some_and(|value| value == "same-origin")
        {
            return Ok(());
        }
        TokenDelivery::Cookie.validate_origin(headers, self)
    }
}

impl TokenDelivery {
    pub fn validate_origin(
        self,
        headers: &HeaderMap,
        settings: &TokenDeliverySettings,
    ) -> Result<(), ApiError> {
        if matches!(self, Self::Token) {
            return Ok(());
        }
        let origin = headers
            .get(ORIGIN)
            .and_then(|value| value.to_str().ok())
            .ok_or(ApiError::UntrustedOrigin)?;
        if !settings.public_origin.is_empty() {
            return if origin == settings.public_origin {
                Ok(())
            } else {
                Err(ApiError::UntrustedOrigin)
            };
        }
        // Local HTTP development follows Hero's selected port, never forwarded headers.
        let uri = origin
            .parse::<Uri>()
            .map_err(|_| ApiError::UntrustedOrigin)?;
        let local = matches!(uri.host(), Some("localhost" | "127.0.0.1" | "[::1]"));
        let host = headers.get(HOST).and_then(|value| value.to_str().ok());
        if !settings.secure
            && local
            && uri.scheme_str() == Some("http")
            && uri.authority().map(|value| value.as_str()) == host
            && uri.path() == "/"
            && uri.query().is_none()
        {
            Ok(())
        } else {
            Err(ApiError::UntrustedOrigin)
        }
    }

    pub fn otp_response(
        self,
        issued: IssuedEmailOtp,
        settings: &TokenDeliverySettings,
    ) -> Result<Response, ApiError> {
        let lifetime =
            TokenLifetime::from((issued.validity, EmailOtpAccessToken::THROTTLE_SECONDS));
        let token = issued.token;
        let mut headers = HeaderMap::new();
        headers.insert(CACHE_CONTROL, HeaderValue::from_static("no-store"));
        let token = match self {
            Self::Token => Some(token),
            Self::Cookie => {
                settings.set_otp_cookie(&mut headers, token, lifetime.expires_in_seconds)?;
                None
            }
        };
        Ok((headers, Json(SignupEmailResponse { token, lifetime })).into_response())
    }

    pub fn response(
        self,
        tokens: UserTokens,
        status: StatusCode,
        settings: &TokenDeliverySettings,
    ) -> Result<Response, ApiError> {
        let mut headers = HeaderMap::new();
        headers.insert(CACHE_CONTROL, HeaderValue::from_static("no-store"));
        let access_lifetime = TokenLifetime::from(tokens.access_validity.clone());
        let refresh_lifetime = TokenLifetime::from(tokens.refresh_validity.clone());
        let body = match self {
            Self::Token => AuthTokensResponse {
                access_lifetime,
                refresh_lifetime,
                access_token: Some(tokens.access_token),
                refresh_token: Some(tokens.refresh_token),
            },
            Self::Cookie => {
                settings.set_cookies(&mut headers, tokens)?;
                AuthTokensResponse {
                    access_lifetime,
                    refresh_lifetime,
                    access_token: None,
                    refresh_token: None,
                }
            }
        };
        Ok((status, headers, Json(body)).into_response())
    }
}
