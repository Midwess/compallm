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
    auth::{
        AuthError, AuthService, EmailOtpAccessToken, RefreshToken, UserAccessToken, UserTokens,
    },
    persistent::DbContext,
};
use cookie::{Cookie, SameSite};
use std::sync::{Arc, Mutex};

#[derive(Clone, Copy)]
pub enum CredentialPolicy {
    Public,
    EmailOtp,
    User,
    Refresh,
}

/// Credential kinds stay separate; only the HTTP boundary constructs verified values.
#[derive(Clone)]
pub struct RequestIdentity(Identity);

#[derive(Clone)]
enum Identity {
    Anonymous,
    EmailOtp(EmailOtpAccessToken),
    User(VerifiedAccess),
    Refresh(String),
}

impl RequestIdentity {
    pub fn user(&self) -> Result<&VerifiedAccess, AuthError> {
        match &self.0 {
            Identity::User(access) => Ok(access),
            _ => Err(AuthError::WrongTokenVariant),
        }
    }

    pub fn otp(&self) -> Result<&EmailOtpAccessToken, AuthError> {
        match &self.0 {
            Identity::EmailOtp(access) => Ok(access),
            _ => Err(AuthError::WrongTokenVariant),
        }
    }

    pub fn refresh(&self) -> Result<&str, AuthError> {
        match &self.0 {
            Identity::Refresh(encoded) => Ok(encoded),
            _ => Err(AuthError::WrongTokenVariant),
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

    async fn authenticate(
        &self,
        headers: &HeaderMap,
    ) -> Result<(UserAccessToken, HeaderMap), RequestError> {
        if let Some(encoded) = bearer(headers)? {
            let user = self.auth.authenticate_access(encoded).await?;
            if !self.auth.validate_access(&user).await? {
                return Err(RequestError::Expired);
            }
            return Ok((user, HeaderMap::new()));
        }
        if cookie(headers, self.settings.names().0)?.is_none()
            && cookie(headers, self.settings.names().1)?.is_none()
        {
            return Err(RequestError::Unauthenticated);
        }
        self.settings.validate_origin(headers)?;
        let Some(encoded) = cookie(headers, self.settings.names().0)? else {
            return self.renew(headers, None).await;
        };
        match self.auth.authenticate_access(&encoded).await {
            Ok(user) => match self.auth.validate_access(&user).await {
                Ok(true) => Ok((user, HeaderMap::new())),
                Ok(false) | Err(AuthError::ExpiredToken) => self.renew(headers, Some(&user)).await,
                Err(error) => Err(error.into()),
            },
            Err(AuthError::ExpiredToken) => self.renew(headers, None).await,
            Err(error) => Err(error.into()),
        }
    }

    /// Select a credential policy before execution, reusing the same HTTP boundary.
    pub async fn run_policy(
        &self,
        mut request: Request,
        next: Next,
        policy: CredentialPolicy,
    ) -> Response {
        let headers = request.headers().clone();
        self.db
            .scope(async {
                let delivery = CookieDelivery {
                    settings: self.settings.clone(),
                    request: headers.clone(),
                    response: Arc::new(Mutex::new(HeaderMap::new())),
                };
                let verified: Result<_, RequestError> = async {
                    match policy {
                        CredentialPolicy::Public => {
                            Ok((RequestIdentity(Identity::Anonymous), HeaderMap::new()))
                        }
                        CredentialPolicy::User => {
                            self.authenticate(&headers).await.map(|(access, headers)| {
                                (
                                    RequestIdentity(Identity::User(VerifiedAccess(access))),
                                    headers,
                                )
                            })
                        }
                        CredentialPolicy::EmailOtp => {
                            let encoded = if let Some(encoded) = bearer(&headers)? {
                                encoded.to_owned()
                            } else {
                                self.settings.validate_origin(&headers)?;
                                cookie(&headers, self.settings.names().0)?
                                    .ok_or(RequestError::Unauthenticated)?
                            };
                            let otp = EmailOtpAccessToken::try_from(
                                self.auth
                                    .authenticate_token(&encoded)
                                    .await
                                    .map_err(|error| {
                                        if matches!(error, AuthError::ExpiredToken) {
                                            RequestError::ExpiredChallenge
                                        } else {
                                            error.into()
                                        }
                                    })?,
                            )?;
                            Ok((RequestIdentity(Identity::EmailOtp(otp)), HeaderMap::new()))
                        }
                        CredentialPolicy::Refresh => {
                            let encoded = bearer(&headers)?.ok_or(RequestError::Unauthenticated)?;
                            RefreshToken::try_from(self.auth.authenticate_token(encoded).await?)?;
                            Ok((
                                RequestIdentity(Identity::Refresh(encoded.to_owned())),
                                HeaderMap::new(),
                            ))
                        }
                    }
                }
                .await;
                let (identity, replacements) = match verified {
                    Ok(value) => value,
                    Err(error) => {
                        let clear = matches!(policy, CredentialPolicy::User)
                            && !headers.contains_key(AUTHORIZATION)
                            && matches!(
                                error,
                                RequestError::Unauthenticated | RequestError::Expired
                            );
                        let mut response = error.into_response();
                        if clear && let Ok(headers) = self.settings.clear_headers() {
                            response.headers_mut().extend(headers);
                        }
                        return response;
                    }
                };
                if let RequestIdentity(Identity::User(access)) = &identity {
                    request.extensions_mut().insert(access.clone());
                }
                request.extensions_mut().insert(identity);
                request.extensions_mut().insert(delivery.clone());
                let mut response = next.run(request).await;
                let staged = match delivery.response.lock() {
                    Ok(headers) => headers.clone(),
                    Err(_) => return RequestError::Internal.into_response(),
                };
                response.headers_mut().extend(staged);
                // Handler cookies, including revocation, override automatic renewal.
                for value in replacements.get_all(SET_COOKIE) {
                    let replacement = value
                        .to_str()
                        .ok()
                        .and_then(|value| Cookie::parse(value).ok());
                    let overridden = replacement.is_some_and(|replacement| {
                        response.headers().get_all(SET_COOKIE).iter().any(|value| {
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
    authentication
        .run_policy(request, next, CredentialPolicy::User)
        .await
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
