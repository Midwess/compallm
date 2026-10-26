use crate::{controllers::auth::delivery::TokenDeliverySettings, error::ApiError};
use axum::{
    extract::{FromRequestParts, Request},
    http::{
        HeaderMap, HeaderValue, Method, StatusCode,
        header::{AUTHORIZATION, CACHE_CONTROL, COOKIE, SET_COOKIE, WWW_AUTHENTICATE},
        request::Parts,
    },
    middleware::Next,
    response::{IntoResponse, Response},
};
use compallm_core::{
    auth::{AuthError, AuthService, UserAccessToken, UserTokens},
    organization::OrganizationRole,
};
use cookie::Cookie;

/// Identity scoped to this request, never shared as application state.
#[derive(Clone)]
pub struct CurrentUser(pub UserAccessToken);

impl CurrentUser {
    pub fn require_organization(&self, organization_id: u64) -> Result<(), ApiError> {
        if self.0.session.organization_id == organization_id {
            Ok(())
        } else {
            Err(ApiError::Forbidden)
        }
    }

    pub fn require_roles(&self, roles: &[OrganizationRole]) -> Result<(), ApiError> {
        if roles.contains(&self.0.organization_role) {
            Ok(())
        } else {
            Err(ApiError::Forbidden)
        }
    }
}

impl<S: Send + Sync> FromRequestParts<S> for CurrentUser {
    type Rejection = ApiError;
    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<Self>()
            .cloned()
            .ok_or(ApiError::Unauthenticated)
    }
}

pub struct Authorization;

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
    ) -> Result<(CurrentUser, Option<UserTokens>), ApiError> {
        if let Some(token) = Self::bearer(headers)? {
            return Ok((CurrentUser(auth.authenticate_access(token)?), None));
        }
        let safe = matches!(*method, Method::GET | Method::HEAD | Method::OPTIONS);
        let names = settings.cookie_names();
        if let Some(token) = Self::cookie(headers, names.0)? {
            if !safe {
                settings.validate_cookie_request(headers, false)?;
            }
            match auth.authenticate_access(&token) {
                Ok(user) => return Ok((CurrentUser(user), None)),
                Err(AuthError::ExpiredToken) => {}
                Err(error) => return Err(error.into()),
            }
        }
        let refresh = Self::cookie(headers, names.1)?.ok_or(ApiError::Unauthenticated)?;
        settings.validate_cookie_request(headers, matches!(*method, Method::GET | Method::HEAD))?;
        let tokens = auth.refresh(&refresh).await?;
        let user = auth.authenticate_access(&tokens.access_token)?;
        Ok((CurrentUser(user), Some(tokens)))
    }

    pub async fn authenticate(
        auth: &AuthService,
        settings: &TokenDeliverySettings,
        mut request: Request,
        next: Next,
    ) -> Response {
        // An explicit Authorization header selects Bearer mode even if malformed.
        let cookie_mode = !request.headers().contains_key(AUTHORIZATION);
        let mut response = match Self::identity(auth, settings, request.headers(), request.method())
            .await
        {
            Ok((user, renewed)) => {
                request.extensions_mut().insert(user);
                let mut response = next.run(request).await;
                if let Some(tokens) = renewed {
                    // A handler issuing its own replacement credentials takes precedence.
                    let names = settings.cookie_names();
                    let has_replacement =
                        response.headers().get_all(SET_COOKIE).iter().any(|value| {
                            value
                                .to_str()
                                .ok()
                                .and_then(|value| Cookie::parse(value).ok())
                                .is_some_and(|cookie| {
                                    cookie.name() == names.0 || cookie.name() == names.1
                                })
                        });
                    if !has_replacement
                        && let Err(error) = settings.set_cookies(response.headers_mut(), tokens)
                    {
                        return error.into_response();
                    }
                }
                response
            }
            Err(error) => {
                let invalid_credentials = matches!(
                    &error,
                    ApiError::Unauthenticated
                        | ApiError::Auth(
                            AuthError::InvalidSession
                                | AuthError::InvalidToken
                                | AuthError::ExpiredToken
                                | AuthError::WrongTokenVariant
                                | AuthError::UserNotFound
                                | AuthError::UserDisabled
                                | AuthError::OrganizationAccessDenied
                                | AuthError::TokenEncoding(_)
                                | AuthError::TokenPayload(_)
                                | AuthError::TokenCrypto(_)
                        )
                );
                let mut response = if matches!(&error, ApiError::Auth(AuthError::UserNotFound)) {
                    ApiError::Unauthenticated.into_response()
                } else {
                    error.into_response()
                };
                if cookie_mode
                    && invalid_credentials
                    && let Err(error) = settings.clear_cookies(response.headers_mut())
                {
                    return error.into_response();
                }
                response
            }
        };
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
