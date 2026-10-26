use crate::error::ApiError;
use axum::{
    Json,
    http::{
        HeaderMap, HeaderValue, StatusCode, Uri,
        header::{CACHE_CONTROL, HOST, ORIGIN, SET_COOKIE},
    },
    response::{IntoResponse, Response},
};
use compallm_core::auth::UserTokens;
use cookie::{Cookie, SameSite, time::Duration};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// How signup/signin confirmation delivers credentials. Never stored in JWT claims.
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
    pub access_max_age_seconds: u64,
    pub refresh_max_age_seconds: u64,
}

/// Cookie delivery returns an empty object. Token delivery returns both credentials.
#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AuthTokensResponse {
    /// Access JWT, present only when delivery is token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    /// Refresh JWT, present only when delivery is token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
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

    pub fn response(
        self,
        tokens: UserTokens,
        status: StatusCode,
        settings: &TokenDeliverySettings,
    ) -> Result<Response, ApiError> {
        let mut headers = HeaderMap::new();
        headers.insert(CACHE_CONTROL, HeaderValue::from_static("no-store"));
        let body = match self {
            Self::Token => AuthTokensResponse {
                access_token: Some(tokens.access_token),
                refresh_token: Some(tokens.refresh_token),
            },
            Self::Cookie => {
                let names = if settings.secure {
                    ("__Host-access", "__Host-refresh")
                } else {
                    ("compallm-access", "compallm-refresh")
                };
                for (name, token, max_age) in [
                    (
                        names.0,
                        tokens.access_token,
                        settings.access_max_age_seconds,
                    ),
                    (
                        names.1,
                        tokens.refresh_token,
                        settings.refresh_max_age_seconds,
                    ),
                ] {
                    let cookie = Cookie::build((name, token))
                        .http_only(true)
                        .secure(settings.secure)
                        .same_site(SameSite::Lax)
                        .path("/")
                        .max_age(Duration::seconds(max_age as i64))
                        .build();
                    headers.append(SET_COOKIE, HeaderValue::from_str(&cookie.to_string())?);
                }
                AuthTokensResponse {
                    access_token: None,
                    refresh_token: None,
                }
            }
        };
        Ok((status, headers, Json(body)).into_response())
    }
}
