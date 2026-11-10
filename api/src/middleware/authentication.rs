//! Shared HTTP session authentication, independent of endpoint payloads.
use axum::{
    Json,
    extract::{Request, State},
    http::{
        HeaderMap, HeaderValue, StatusCode, Uri,
        header::{
            AUTHORIZATION, CACHE_CONTROL, COOKIE, HOST, ORIGIN, SET_COOKIE, WWW_AUTHENTICATE,
        },
    },
    middleware::Next,
    response::{IntoResponse, Response},
};
use compallm_core::{
    authentication::{
        AccessToken, AuthError, AuthService, EmailOtpAccessToken, RefreshToken, UserAccessToken,
        UserTokens,
    },
    persistent::DbContext,
};
use cookie::{Cookie, SameSite};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering},
};

/// Verified request facts and verification failures, independent of endpoint policy.
/// Only this HTTP boundary constructs the credential slots.
#[derive(Clone)]
pub struct RequestIdentity {
    user: Result<VerifiedAccess, RequestError>,
    otp: Result<EmailOtpAccessToken, RequestError>,
    refresh: Result<VerifiedRefresh, RequestError>,
    user_requested: Arc<AtomicBool>,
}

#[derive(Clone)]
struct VerifiedRefresh {
    encoded: String,
    bearer: bool,
}

impl RequestIdentity {
    fn missing(error: RequestError) -> Self {
        Self {
            user: Err(error),
            otp: Err(error),
            refresh: Err(error),
            user_requested: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn user(&self) -> Result<&VerifiedAccess, RequestError> {
        self.user_requested.store(true, Ordering::Relaxed);
        self.user.as_ref().map_err(|error| *error)
    }

    pub fn otp(&self) -> Result<&EmailOtpAccessToken, RequestError> {
        self.otp.as_ref().map_err(|error| *error)
    }

    /// The resolver requires explicit Bearer refresh authority, never cookie fallback.
    pub fn refresh(&self) -> Result<&str, RequestError> {
        let refresh = self.refresh.as_ref().map_err(|error| *error)?;
        if refresh.bearer {
            Ok(&refresh.encoded)
        } else {
            Err(RequestError::Unauthenticated)
        }
    }
}

/// Request-local cookie delivery. It never stores identity or service dependencies.
#[derive(Clone)]
pub struct CookieDelivery {
    settings: AuthenticationSettings,
    request: HeaderMap,
    response: Arc<Mutex<HeaderMap>>,
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

/// Verified request identity. Only this module can construct it.
#[derive(Clone)]
pub struct VerifiedAccess(UserAccessToken);

impl VerifiedAccess {
    pub fn access(&self) -> &UserAccessToken {
        &self.0
    }
}

#[derive(Clone)]
pub struct AuthenticationSettings {
    pub cookie_secure: bool,
    pub public_origin: String,
}

impl AuthenticationSettings {
    fn validate(&self) -> Result<(), AuthError> {
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

    fn names(&self) -> (&'static str, &'static str) {
        if self.cookie_secure {
            ("__Host-access", "__Host-refresh")
        } else {
            ("compallm-access", "compallm-refresh")
        }
    }

    fn validate_origin(&self, headers: &HeaderMap) -> Result<(), RequestError> {
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

    fn replacement_headers(&self, tokens: UserTokens) -> Result<HeaderMap, RequestError> {
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

    fn clear_headers(&self) -> Result<HeaderMap, RequestError> {
        let mut headers = HeaderMap::new();
        let names = self.names();
        self.append_cookie(&mut headers, names.0, String::new(), 0)?;
        self.append_cookie(&mut headers, names.1, String::new(), 0)?;
        Ok(headers)
    }
}

/// Reusable dependencies for protected HTTP routes; no per-request identity is retained.
#[derive(Clone)]
pub struct Authentication {
    db: &'static DbContext,
    auth: &'static AuthService,
    settings: AuthenticationSettings,
}

impl Authentication {
    pub fn new(
        db: &'static DbContext,
        auth: &'static AuthService,
        settings: AuthenticationSettings,
    ) -> Result<Self, AuthError> {
        settings.validate()?;
        Ok(Self { db, auth, settings })
    }
    async fn renew(
        &self,
        headers: &HeaderMap,
        expected: Option<&UserAccessToken>,
    ) -> Result<(UserAccessToken, HeaderMap), RequestError> {
        let encoded =
            cookie(headers, self.settings.names().1)?.ok_or(RequestError::Unauthenticated)?;
        let refresh = RefreshToken::try_from(self.auth.authenticate_token(&encoded).await?)?;
        if expected.is_some_and(|user| !refresh.session.has_same_identity(&user.session)) {
            return Err(RequestError::Unauthenticated);
        }
        let tokens = self.auth.refresh(&encoded).await?;
        // Core loaded coherent current grants/version; decoding adds no freshness query.
        let user = self.auth.authenticate_access(&tokens.access_token).await?;
        let headers = self.settings.replacement_headers(tokens)?;
        Ok((user, headers))
    }

    async fn current_user(
        &self,
        headers: &HeaderMap,
        user: UserAccessToken,
        bearer_mode: bool,
    ) -> Result<(UserAccessToken, HeaderMap), RequestError> {
        match self.auth.validate_access(&user).await {
            Ok(true) => Ok((user, HeaderMap::new())),
            Ok(false) | Err(AuthError::ExpiredToken) if !bearer_mode => {
                self.renew(headers, Some(&user)).await
            }
            Ok(false) | Err(AuthError::ExpiredToken) => Err(RequestError::Expired),
            Err(error) => Err(error.into()),
        }
    }

    /// Verify supplied HTTP credentials without knowing which endpoint will use them.
    async fn collect(&self, headers: &HeaderMap) -> (RequestIdentity, HeaderMap) {
        let mut identity = RequestIdentity::missing(RequestError::Unauthenticated);
        // Collect refresh-cookie facts independently of the access-cookie kind.
        // Explicit Authorization suppresses all cookie fallback.
        if !headers.contains_key(AUTHORIZATION) {
            identity.refresh = match cookie(headers, self.settings.names().1) {
                Ok(Some(encoded)) => match self.auth.authenticate_token(&encoded).await {
                    Ok(AccessToken::Refresh(_)) => Ok(VerifiedRefresh {
                        encoded,
                        bearer: false,
                    }),
                    Ok(_) => Err(RequestError::Unauthenticated),
                    Err(error) => Err(error.into()),
                },
                Ok(None) => Err(RequestError::Unauthenticated),
                Err(error) => Err(error),
            };
        }
        let verified = async {
            let explicit = bearer(headers)?;
            let bearer_mode = explicit.is_some();
            let encoded = if let Some(encoded) = explicit {
                encoded.to_owned()
            } else {
                let access = cookie(headers, self.settings.names().0)?;
                if access.is_none() && cookie(headers, self.settings.names().1)?.is_none() {
                    return Err(RequestError::Unauthenticated);
                }
                self.settings.validate_origin(headers)?;
                let Some(encoded) = access else {
                    return self.renew(headers, None).await;
                };
                encoded
            };
            match self.auth.authenticate_token(&encoded).await {
                Ok(AccessToken::User(user)) => self.current_user(headers, user, bearer_mode).await,
                Ok(AccessToken::EmailOtp(otp)) => {
                    identity.otp = Ok(otp);
                    Err(RequestError::Unauthenticated)
                }
                Ok(AccessToken::Refresh(_)) if bearer_mode => {
                    identity.refresh = Ok(VerifiedRefresh {
                        encoded,
                        bearer: true,
                    });
                    Err(RequestError::Unauthenticated)
                }
                Ok(_) => Err(RequestError::Unauthenticated),
                Err(AuthError::ExpiredToken) => {
                    identity.otp = Err(RequestError::ExpiredChallenge);
                    if bearer_mode {
                        identity.refresh = Err(RequestError::Expired);
                        Err(RequestError::Expired)
                    } else {
                        self.renew(headers, None).await
                    }
                }
                Err(error) => {
                    let error = RequestError::from(error);
                    identity.otp = Err(error);
                    if bearer_mode {
                        identity.refresh = Err(error);
                    }
                    Err(error)
                }
            }
        }
        .await;
        match verified {
            Ok((user, replacements)) => {
                identity.user = Ok(VerifiedAccess(user));
                (identity, replacements)
            }
            Err(error) => {
                identity.user = Err(error);
                // Cookie authority requires Origin even when the output delivery is TOKEN.
                if matches!(error, RequestError::Origin) {
                    identity.otp = Err(error);
                }
                (identity, HeaderMap::new())
            }
        }
    }

    /// Populate request facts; strict HTTP routes may require user access before delivery.
    async fn run(&self, mut request: Request, next: Next, require_user: bool) -> Response {
        let headers = request.headers().clone();
        self.db
            .scope(async {
                let delivery = CookieDelivery {
                    settings: self.settings.clone(),
                    request: headers.clone(),
                    response: Arc::new(Mutex::new(HeaderMap::new())),
                };
                let (identity, replacements) = self.collect(&headers).await;
                if require_user {
                    match identity.user() {
                        Ok(access) => {
                            request.extensions_mut().insert(access.clone());
                        }
                        Err(error) => {
                            let mut response = error.into_response();
                            if !headers.contains_key(AUTHORIZATION)
                                && matches!(
                                    error,
                                    RequestError::Unauthenticated | RequestError::Expired
                                )
                            {
                                match self.settings.clear_headers() {
                                    Ok(headers) => response.headers_mut().extend(headers),
                                    Err(error) => return error.into_response(),
                                }
                            }
                            return response;
                        }
                    }
                }
                let user_requested = identity.user_requested.clone();
                let user_error = identity.user.as_ref().err().copied();
                request.extensions_mut().insert(identity);
                request.extensions_mut().insert(delivery.clone());
                let mut response = next.run(request).await;
                let staged = match delivery.response.lock() {
                    Ok(headers) => headers.clone(),
                    Err(_) => return RequestError::Internal.into_response(),
                };
                if user_requested.load(Ordering::Relaxed) {
                    if !headers.contains_key(AUTHORIZATION)
                        && user_error.is_some_and(|error| {
                            matches!(error, RequestError::Unauthenticated | RequestError::Expired)
                        })
                        && staged.is_empty()
                    {
                        match self.settings.clear_headers() {
                            Ok(headers) => response.headers_mut().extend(headers),
                            Err(error) => return error.into_response(),
                        }
                    }
                    // Explicit handler cookies override automatic user renewal.
                    for value in replacements.get_all(SET_COOKIE) {
                        let replacement = value
                            .to_str()
                            .ok()
                            .and_then(|value| Cookie::parse(value).ok());
                        let overridden = replacement.is_some_and(|replacement| {
                            staged
                                .get_all(SET_COOKIE)
                                .iter()
                                .chain(response.headers().get_all(SET_COOKIE).iter())
                                .any(|value| {
                                    value
                                        .to_str()
                                        .ok()
                                        .and_then(|value| Cookie::parse(value).ok())
                                        .is_some_and(|cookie| cookie.name() == replacement.name())
                                })
                        });
                        if !overridden {
                            response.headers_mut().append(SET_COOKIE, value.clone());
                        }
                    }
                }
                response.headers_mut().extend(staged);
                no_store(response)
            })
            .await
    }
}

/// Attach with Axum's `from_fn_with_state` to authenticate each protected request.
pub async fn authenticate(
    State(authentication): State<Authentication>,
    request: Request,
    next: Next,
) -> Response {
    authentication.run(request, next, true).await
}

/// Optional authentication: endpoints/resolvers choose which verified slot they require.
pub async fn populate(
    State(authentication): State<Authentication>,
    request: Request,
    next: Next,
) -> Response {
    authentication.run(request, next, false).await
}

fn bearer(headers: &HeaderMap) -> Result<Option<&str>, RequestError> {
    let mut values = headers.get_all(AUTHORIZATION).iter();
    let Some(value) = values.next() else {
        return Ok(None);
    };
    if values.next().is_some() {
        return Err(RequestError::Unauthenticated);
    }
    let value = value.to_str().map_err(|_| RequestError::Unauthenticated)?;
    let mut parts = value.split_ascii_whitespace();
    match (parts.next(), parts.next(), parts.next()) {
        (Some(scheme), Some(token), None) if scheme.eq_ignore_ascii_case("Bearer") => {
            Ok(Some(token))
        }
        _ => Err(RequestError::Unauthenticated),
    }
}

fn cookie(headers: &HeaderMap, name: &str) -> Result<Option<String>, RequestError> {
    let mut value = None;
    for header in headers.get_all(COOKIE) {
        let header = header.to_str().map_err(|_| RequestError::Unauthenticated)?;
        for cookie in Cookie::split_parse(header) {
            let cookie = cookie.map_err(|_| RequestError::Unauthenticated)?;
            if cookie.name() == name && value.replace(cookie.value().to_owned()).is_some() {
                return Err(RequestError::Unauthenticated);
            }
        }
    }
    Ok(value)
}

#[derive(Clone, Copy)]
pub enum RequestError {
    Unauthenticated,
    Expired,
    ExpiredChallenge,
    Origin,
    Unavailable,
    CredentialTooLarge,
    Internal,
}

impl From<AuthError> for RequestError {
    fn from(error: AuthError) -> Self {
        match error {
            AuthError::ExpiredToken => Self::Expired,
            AuthError::TokenTooLarge { .. } => Self::CredentialTooLarge,
            AuthError::SessionPersistence(_)
            | AuthError::OrganizationRepository(_)
            | AuthError::Connection(_)
            | AuthError::DictionaryUnavailable
            | AuthError::Persistence(_) => Self::Unavailable,
            AuthError::InvalidConfiguration
            | AuthError::InvalidClock(_)
            | AuthError::IdGeneration(_)
            | AuthError::Unexpected(_)
            | AuthError::Organization(_)
            | AuthError::Email(_) => Self::Internal,
            _ => Self::Unauthenticated,
        }
    }
}

impl IntoResponse for RequestError {
    fn into_response(self) -> Response {
        let expired = matches!(self, Self::Expired);
        let (status, code, message) = match self {
            Self::Unauthenticated => (
                StatusCode::UNAUTHORIZED,
                "UNAUTHENTICATED",
                "Authentication required",
            ),
            Self::Expired => (
                StatusCode::UNAUTHORIZED,
                "UNAUTHENTICATED",
                "Access token expired; refresh explicitly",
            ),
            Self::ExpiredChallenge => (
                StatusCode::UNAUTHORIZED,
                "EXPIRED_TOKEN",
                "Email challenge expired; request a new code",
            ),
            Self::Origin => (
                StatusCode::FORBIDDEN,
                "FORBIDDEN",
                "Untrusted browser origin",
            ),
            Self::Unavailable => (
                StatusCode::SERVICE_UNAVAILABLE,
                "SERVICE_UNAVAILABLE",
                "Authorization state is temporarily unavailable",
            ),
            Self::CredentialTooLarge => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "CREDENTIAL_TOO_LARGE",
                "Authentication credential exceeds the supported size limit",
            ),
            Self::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "INTERNAL_SERVER_ERROR",
                "Request failed",
            ),
        };
        let mut response = error_response(status, code, message);
        if status == StatusCode::UNAUTHORIZED {
            response.headers_mut().insert(WWW_AUTHENTICATE, HeaderValue::from_static(if expired {
                "Bearer error=\"invalid_token\", error_description=\"Access token expired; refresh explicitly\""
            } else { "Bearer" }));
        }
        response
    }
}

fn error_response(status: StatusCode, code: &'static str, message: &'static str) -> Response {
    no_store(
        (
            status,
            Json(serde_json::json!({
                "errors": [{ "message": message, "extensions": { "code": code } }]
            })),
        )
            .into_response(),
    )
}

fn no_store(mut response: Response) -> Response {
    response
        .headers_mut()
        .insert(CACHE_CONTROL, HeaderValue::from_static("no-store"));
    response
}
