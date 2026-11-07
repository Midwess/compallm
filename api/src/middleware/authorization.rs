use crate::{controllers::auth::delivery::TokenDeliverySettings, error::ApiError};
use axum::{
    extract::{FromRequestParts, Request},
    http::{
        Extensions, HeaderMap, HeaderValue, Method, StatusCode,
        header::{AUTHORIZATION, CACHE_CONTROL, COOKIE, SET_COOKIE, WWW_AUTHENTICATE},
        request::Parts,
    },
    middleware::Next,
    response::{IntoResponse, Response},
};
use compallm_core::auth::{
    AccessToken, AuthError, AuthService, EmailOtpAccessToken, RefreshToken, UserAccessToken,
    UserTokens,
};
use cookie::Cookie;

/// Require a concrete verified token from this request's extensions.
/// Missing or different token types return 401; Core token types stay independent of Axum.
#[derive(Clone)]
pub struct RequireToken<T>(pub T);

impl RequireToken<UserAccessToken> {
    pub fn require_organization(&self, organization_id: u64) -> Result<(), ApiError> {
        if self.0.session.organization_id == organization_id {
            Ok(())
        } else {
            Err(ApiError::Forbidden)
        }
    }

    pub fn require_roles(&self, roles: &[&str]) -> Result<(), ApiError> {
        if roles.contains(&self.0.organization_role.as_str()) {
            Ok(())
        } else {
            Err(ApiError::Forbidden)
        }
    }
}

impl<S, T> FromRequestParts<S> for RequireToken<T>
where
    S: Send + Sync,
    T: Clone + Send + Sync + 'static,
{
    type Rejection = ApiError;
    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<T>()
            .cloned()
            .map(Self)
            .ok_or(ApiError::Unauthenticated)
    }
}

pub struct Authorization;

enum Replacement {
    Cookies(UserTokens),
    Bearer { token: String, expires_at: u64 },
}

impl Authorization {
    fn bearer(headers: &HeaderMap) -> Result<Option<&str>, ApiError> {
        let mut values = headers.get_all(AUTHORIZATION).iter();
        let Some(value) = values.next() else {
            return Ok(None);
        };
        if values.next().is_some() {
            return Err(ApiError::Unauthenticated);
        }
        let value = value.to_str().map_err(|_| ApiError::Unauthenticated)?;
        let mut parts = value.split_ascii_whitespace();
        match (parts.next(), parts.next(), parts.next()) {
            (Some(scheme), Some(token), None) if scheme.eq_ignore_ascii_case("Bearer") => {
                Ok(Some(token))
            }
            _ => Err(ApiError::Unauthenticated),
        }
    }

    fn cookie(headers: &HeaderMap, name: &str) -> Result<Option<String>, ApiError> {
        let mut token = None;
        for header in headers.get_all(COOKIE) {
            let header = header.to_str().map_err(|_| ApiError::Unauthenticated)?;
            for cookie in Cookie::split_parse(header) {
                let cookie = cookie.map_err(|_| ApiError::Unauthenticated)?;
                if cookie.name() == name && token.replace(cookie.value().to_owned()).is_some() {
                    return Err(ApiError::Unauthenticated);
                }
            }
        }
        Ok(token)
    }

    async fn identity(
        auth: &AuthService,
        settings: &TokenDeliverySettings,
        headers: &HeaderMap,
        method: &Method,
    ) -> Result<(AccessToken, Option<Replacement>), ApiError> {
        if let Some(token) = Self::bearer(headers)? {
            let token = auth.authenticate_token(token)?;
            if let AccessToken::User(user) = &token
                && !auth.validate_access(user).await?
            {
                let encoded = auth.refresh_access(user).await?;
                let rebuilt = auth.authenticate_access(&encoded)?;
                let expires_at = rebuilt.validity.expires_at;
                return Ok((
                    AccessToken::User(rebuilt),
                    Some(Replacement::Bearer {
                        token: encoded,
                        expires_at,
                    }),
                ));
            }
            return Ok((token, None));
        }
        let safe = matches!(*method, Method::GET | Method::HEAD | Method::OPTIONS);
        let names = settings.cookie_names();
        if let Some(token) = Self::cookie(headers, names.0)? {
            if !safe {
                settings.validate_cookie_request(headers, false)?;
            }
            match auth.authenticate_token(&token) {
                Ok(AccessToken::User(user)) => {
                    if auth.validate_access(&user).await? {
                        return Ok((AccessToken::User(user), None));
                    }
                    let refresh =
                        Self::cookie(headers, names.1)?.ok_or(ApiError::Unauthenticated)?;
                    let refresh_claims =
                        RefreshToken::try_from(auth.authenticate_token(&refresh)?)?;
                    if !refresh_claims.session.has_same_identity(&user.session) {
                        return Err(AuthError::InvalidSession.into());
                    }
                    settings.validate_cookie_request(
                        headers,
                        matches!(*method, Method::GET | Method::HEAD),
                    )?;
                    let tokens = auth.refresh(&refresh).await?;
                    let rebuilt = auth.authenticate_access(&tokens.access_token)?;
                    return Ok((
                        AccessToken::User(rebuilt),
                        Some(Replacement::Cookies(tokens)),
                    ));
                }
                Ok(token @ AccessToken::EmailOtp(_)) => return Ok((token, None)),
                Ok(_) => return Err(AuthError::WrongTokenVariant.into()),
                Err(AuthError::ExpiredToken) => {}
                Err(error) => return Err(error.into()),
            }
        }
        let refresh = Self::cookie(headers, names.1)?.ok_or(ApiError::Unauthenticated)?;
        settings.validate_cookie_request(headers, matches!(*method, Method::GET | Method::HEAD))?;
        let tokens = auth.refresh(&refresh).await?;
        let user = auth.authenticate_access(&tokens.access_token)?;
        Ok((AccessToken::User(user), Some(Replacement::Cookies(tokens))))
    }

    fn insert_token(extensions: &mut Extensions, token: AccessToken) -> Result<(), ApiError> {
        match token {
            AccessToken::User(token) => {
                extensions.insert(token);
            }
            AccessToken::Refresh(token) => {
                extensions.insert(token);
            }
            AccessToken::EmailOtp(token) => {
                extensions.insert(token);
            }
            AccessToken::Encrypted(_) => return Err(ApiError::Unauthenticated),
        }
        Ok(())
    }

    fn insert_refresh(
        auth: &AuthService,
        extensions: &mut Extensions,
        user: &UserAccessToken,
        encoded: Option<&str>,
    ) {
        let Some(encoded) = encoded else {
            return;
        };
        let Ok(AccessToken::Refresh(refresh)) = auth.authenticate_token(encoded) else {
            return;
        };
        if !refresh.session.has_same_identity(&user.session) {
            return;
        }
        extensions.insert(refresh);
    }

    async fn run(
        auth: &AuthService,
        settings: &TokenDeliverySettings,
        mut request: Request,
        next: Next,
        cookie_mode: bool,
    ) -> Result<Response, ApiError> {
        // Only the selected credentials can populate token extensions.
        request.extensions_mut().remove::<UserAccessToken>();
        request.extensions_mut().remove::<RefreshToken>();
        request.extensions_mut().remove::<EmailOtpAccessToken>();
        let (token, renewed) =
            Self::identity(auth, settings, request.headers(), request.method()).await?;
        if cookie_mode && let AccessToken::User(user) = &token {
            let refresh = renewed
                .as_ref()
                .and_then(|replacement| match replacement {
                    Replacement::Cookies(tokens) => Some(tokens.refresh_token.clone()),
                    Replacement::Bearer { .. } => None,
                })
                .or_else(|| {
                    Self::cookie(request.headers(), settings.cookie_names().1)
                        .ok()
                        .flatten()
                });
            Self::insert_refresh(auth, request.extensions_mut(), user, refresh.as_deref());
        }
        Self::insert_token(request.extensions_mut(), token)?;
        let mut response = next.run(request).await;
        let tokens = match renewed {
            None => return Ok(response),
            Some(Replacement::Bearer { token, expires_at }) => {
                let mut credential = HeaderValue::from_str(&token)?;
                credential.set_sensitive(true);
                response
                    .headers_mut()
                    .insert("x-compallm-access-token", credential);
                response.headers_mut().insert(
                    "x-compallm-access-token-expires-at",
                    HeaderValue::from_str(&expires_at.to_string())?,
                );
                return Ok(response);
            }
            Some(Replacement::Cookies(tokens)) => tokens,
        };
        // Preserve credentials explicitly set by the handler (for example organization switching).
        let names = settings.cookie_names();
        let has_replacement = response.headers().get_all(SET_COOKIE).iter().any(|value| {
            let Ok(value) = value.to_str() else {
                return false;
            };
            let Ok(cookie) = Cookie::parse(value) else {
                return false;
            };
            cookie.name() == names.0 || cookie.name() == names.1
        });
        if !has_replacement {
            settings.set_cookies(response.headers_mut(), tokens)?;
        }
        Ok(response)
    }

    pub async fn authenticate(
        auth: &AuthService,
        settings: &TokenDeliverySettings,
        request: Request,
        next: Next,
    ) -> Response {
        // An explicit Authorization header selects Bearer mode even if malformed.
        let cookie_mode = !request.headers().contains_key(AUTHORIZATION);
        let mut response = Self::run(auth, settings, request, next, cookie_mode)
            .await
            .unwrap_or_else(|error| {
                let mut response = error.into_authentication_response();
                if cookie_mode
                    && matches!(
                        response.status(),
                        StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN
                    )
                    && let Err(error) = settings.clear_cookies(response.headers_mut())
                {
                    return error.into_response();
                }
                response
            });
        response
            .headers_mut()
            .insert(CACHE_CONTROL, HeaderValue::from_static("no-store"));
        if response.status() == StatusCode::UNAUTHORIZED {
            response
                .headers_mut()
                .insert(WWW_AUTHENTICATE, HeaderValue::from_static("Bearer"));
        }
        response
    }
}
