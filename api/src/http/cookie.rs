use super::authentication::RequestError;
use axum::http::{
    HeaderMap, HeaderValue, Uri,
    header::{AUTHORIZATION, HOST, ORIGIN, SET_COOKIE},
};
use compallm_core::authentication::{AuthError, UserTokens};
use cookie::{Cookie, SameSite};
use std::sync::{Arc, Mutex};

/// Request-local cookie delivery. It never stores identity or service dependencies.
#[derive(Clone)]
pub struct CookieDelivery {
    pub(super) settings: AuthenticationSettings,
    pub(super) request: HeaderMap,
    pub(super) response: Arc<Mutex<HeaderMap>>,
}

impl CookieDelivery {
    pub fn validate_origin(&self) -> Result<(), RequestError> {
        self.settings.validate_origin(&self.request)
    }

    pub fn set_otp(&self, token: String, seconds: u64) -> Result<(), RequestError> {
        let mut headers = HeaderMap::new();
        let names = self.settings.names();
        self.settings
            .append_cookie(&mut headers, names.0, token, seconds)?;
        self.settings
            .append_cookie(&mut headers, names.1, String::new(), 0)?;
        self.replace(headers)
    }

    pub fn set_user(&self, tokens: UserTokens) -> Result<(), RequestError> {
        self.replace(self.settings.replacement_headers(tokens)?)
    }

    pub fn clear(&self) -> Result<(), RequestError> {
        if self.request.contains_key(AUTHORIZATION) {
            return Ok(());
        }
        self.replace(self.settings.clear_headers()?)
    }

    fn replace(&self, headers: HeaderMap) -> Result<(), RequestError> {
        *self.response.lock().map_err(|_| RequestError::Internal)? = headers;
        Ok(())
    }
}

#[derive(Clone)]
pub struct AuthenticationSettings {
    pub cookie_secure: bool,
    pub public_origin: String,
}

impl AuthenticationSettings {
    pub(super) fn validate(&self) -> Result<(), AuthError> {
        if self.public_origin.is_empty() {
            return if self.cookie_secure {
                Err(AuthError::InvalidConfiguration)
            } else {
                Ok(())
            };
        }
        let uri: Uri = self
            .public_origin
            .parse()
            .map_err(|_| AuthError::InvalidConfiguration)?;
        let scheme = uri.scheme_str();
        if uri.host().is_none()
            || uri
                .authority()
                .is_some_and(|value| value.as_str().contains('@'))
            || uri.path() != "/"
            || uri.query().is_some()
            || !matches!(scheme, Some("http" | "https"))
            || (self.cookie_secure && scheme != Some("https"))
            || uri.authority().is_some_and(|authority| {
                format!("{}://{}", scheme.unwrap_or_default(), authority) != self.public_origin
            })
        {
            return Err(AuthError::InvalidConfiguration);
        }
        Ok(())
    }

    pub(super) fn names(&self) -> (&'static str, &'static str) {
        if self.cookie_secure {
            ("__Host-access", "__Host-refresh")
        } else {
            ("compallm-access", "compallm-refresh")
        }
    }

    pub(super) fn validate_origin(&self, headers: &HeaderMap) -> Result<(), RequestError> {
        let mut origins = headers.get_all(ORIGIN).iter();
        let origin = origins
            .next()
            .and_then(|value| value.to_str().ok())
            .ok_or(RequestError::Origin)?;
        if origins.next().is_some() {
            return Err(RequestError::Origin);
        }
        if !self.public_origin.is_empty() {
            return if origin == self.public_origin {
                Ok(())
            } else {
                Err(RequestError::Origin)
            };
        }
        let uri: Uri = origin.parse().map_err(|_| RequestError::Origin)?;
        let host = headers.get(HOST).and_then(|value| value.to_str().ok());
        if !self.cookie_secure
            && matches!(uri.host(), Some("localhost" | "127.0.0.1" | "[::1]"))
            && uri.scheme_str() == Some("http")
            && uri.authority().map(|value| value.as_str()) == host
            && uri.path() == "/"
            && uri.query().is_none()
        {
            Ok(())
        } else {
            Err(RequestError::Origin)
        }
    }

    fn append_cookie(
        &self,
        headers: &mut HeaderMap,
        name: &'static str,
        value: String,
        seconds: u64,
    ) -> Result<(), RequestError> {
        let seconds = i64::try_from(seconds).map_err(|_| RequestError::Internal)?;
        let value = Cookie::build((name, value))
            .http_only(true)
            .secure(self.cookie_secure)
            .same_site(SameSite::Lax)
            .path("/")
            .max_age(cookie::time::Duration::seconds(seconds))
            .build()
            .to_string();
        if value.len() > 4_096 {
            return Err(RequestError::CredentialTooLarge);
        }
        headers.append(
            SET_COOKIE,
            HeaderValue::from_str(&value).map_err(|_| RequestError::Internal)?,
        );
        Ok(())
    }

    pub(super) fn replacement_headers(
        &self,
        tokens: UserTokens,
    ) -> Result<HeaderMap, RequestError> {
        let mut headers = HeaderMap::new();
        let names = self.names();
        let now = compallm_utils::time::unix_timestamp();
        self.append_cookie(
            &mut headers,
            names.0,
            tokens.access_token,
            tokens.access_validity.expires_at.saturating_sub(now),
        )?;
        self.append_cookie(
            &mut headers,
            names.1,
            tokens.refresh_token,
            tokens.refresh_validity.expires_at.saturating_sub(now),
        )?;
        Ok(headers)
    }

    pub(super) fn clear_headers(&self) -> Result<HeaderMap, RequestError> {
        let mut headers = HeaderMap::new();
        let names = self.names();
        self.append_cookie(&mut headers, names.0, String::new(), 0)?;
        self.append_cookie(&mut headers, names.1, String::new(), 0)?;
        Ok(headers)
    }
}
