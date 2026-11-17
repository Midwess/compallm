//! Request-scoped, lazily verified authentication.
use compallm_core::persistent::DbContext;
use std::{future::Future, sync::Arc};

use axum::http::{HeaderMap, header::AUTHORIZATION};
use compallm_core::authentication::{
    AccessToken, AuthError, AuthService, EmailOtpAccessToken, RefreshToken, UserAccessToken,
};
use tokio::sync::OnceCell;

pub use super::cookie::{AuthenticationSettings, CookieDelivery};
pub use super::error::RequestError;
use super::header;

#[derive(Clone)]
pub struct VerifiedAccess(UserAccessToken);

impl VerifiedAccess {
    pub fn access(&self) -> &UserAccessToken {
        &self.0
    }
}

struct SelectedToken {
    token: AccessToken,
    encoded: String,
    source: CredentialSource,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum CredentialSource {
    Bearer,
    Cookie,
}

struct VerifiedUser {
    access: VerifiedAccess,
    replacements: HeaderMap,
}

pub struct AuthenticationContext {
    auth: &'static AuthService,
    trusted: DbContext,
    settings: AuthenticationSettings,
    headers: HeaderMap,
    metadata: compallm_core::authentication::SessionMetadata,
    geolocation: Arc<crate::geolocation::LocationClient>,
    selected: OnceCell<Result<SelectedToken, RequestError>>,
    passkey_otp: OnceCell<Result<EmailOtpAccessToken, RequestError>>,
    user: OnceCell<Result<VerifiedUser, RequestError>>,
}

#[derive(Clone)]
pub struct UserContext(pub(super) Arc<AuthenticationContext>);

impl AuthenticationContext {
    pub(super) fn new(
        auth: &'static AuthService,
        trusted: DbContext,
        settings: AuthenticationSettings,
        headers: HeaderMap,
        metadata: compallm_core::authentication::SessionMetadata,
        geolocation: Arc<crate::geolocation::LocationClient>,
    ) -> Arc<Self> {
        Arc::new(Self {
            auth,
            trusted,
            settings,
            headers,
            metadata,
            geolocation,
            selected: OnceCell::new(),
            passkey_otp: OnceCell::new(),
            user: OnceCell::new(),
        })
    }

    pub async fn session_metadata(&self) -> compallm_core::authentication::SessionMetadata {
        let mut metadata = self.metadata.clone();
        metadata.location = match metadata.ip_address {
            Some(ip) => self.geolocation.location(ip).await,
            None => None,
        };
        metadata
    }

    pub async fn execute<T, E: From<compallm_core::persistent::ConnectionError>>(
        &self,
        operation: impl Future<Output = Result<T, E>>,
    ) -> Result<T, E> {
        self.trusted.scope(operation).await
    }

    async fn selected(&self) -> Result<&SelectedToken, RequestError> {
        self.selected
            .get_or_init(|| async {
                let headers = &self.headers;
                let bearer = header::bearer(headers)?;
                let (encoded, source) = if let Some(bearer) = bearer {
                    (bearer.to_owned(), CredentialSource::Bearer)
                } else {
                    let encoded = header::cookie(headers, self.settings.names().0)?;
                    if encoded.is_some()
                        || header::cookie(headers, self.settings.names().1)?.is_some()
                    {
                        self.settings.validate_origin(headers)?;
                    }
                    (
                        encoded.ok_or(RequestError::Unauthenticated)?,
                        CredentialSource::Cookie,
                    )
                };
                let token = self
                    .execute(self.auth.authenticate_token(&encoded))
                    .await
                    .map_err(RequestError::from)?;
                Ok(SelectedToken {
                    token,
                    encoded,
                    source,
                })
            })
            .await
            .as_ref()
            .map_err(|error| *error)
    }

    pub fn passkey_change(&self) -> Result<String, AuthError> {
        let encoded = if self.headers.contains_key(AUTHORIZATION) {
            self.headers
                .get("x-passkey-change")
                .and_then(|h| h.to_str().ok())
                .map(str::to_owned)
        } else {
            self.settings
                .validate_origin(&self.headers)
                .map_err(|_| AuthError::InvalidToken)?;
            header::cookie(&self.headers, self.settings.passkey_change_name())
                .map_err(|_| AuthError::InvalidToken)?
        };
        encoded.ok_or(AuthError::ReauthenticationRequired)
    }
    pub async fn passkey_otp(&self) -> Result<&EmailOtpAccessToken, RequestError> {
        self.passkey_otp
            .get_or_init(|| async {
                let encoded = if self.headers.contains_key(AUTHORIZATION) {
                    self.headers
                        .get("x-passkey-otp")
                        .and_then(|h| h.to_str().ok())
                        .map(str::to_owned)
                } else {
                    self.settings.validate_origin(&self.headers)?;
                    header::cookie(&self.headers, self.settings.passkey_otp_name())?
                }
                .ok_or(RequestError::Unauthenticated)?;
                let claims = EmailOtpAccessToken::try_from(
                    self.execute(self.auth.authenticate_token(&encoded)).await?,
                )?;
                if claims.purpose
                    != compallm_core::authentication::AuthenticationPurpose::PasskeyChange
                {
                    return Err(RequestError::Unauthenticated);
                }
                Ok(claims)
            })
            .await
            .as_ref()
            .map_err(|e| *e)
    }
    pub async fn invitation(
        &self,
    ) -> Result<Option<compallm_core::authentication::token::InvitationToken>, RequestError> {
        let Some(encoded) = header::cookie(&self.headers, self.settings.invitation_name())? else {
            return Ok(None);
        };
        self.settings.validate_origin(&self.headers)?;
        let claims = compallm_core::authentication::token::InvitationToken::try_from(
            self.execute(self.auth.authenticate_token(&encoded)).await?,
        )?;
        Ok(Some(claims))
    }

    pub async fn otp(&self) -> Result<&EmailOtpAccessToken, RequestError> {
        match self.selected().await {
            Ok(SelectedToken {
                token: AccessToken::EmailOtp(otp),
                ..
            }) => Ok(otp),
            Err(RequestError::Expired) => Err(RequestError::ExpiredChallenge),
            Err(error) => Err(error),
            _ => Err(RequestError::Unauthenticated),
        }
    }

    pub async fn refresh(&self) -> Result<&str, RequestError> {
        let SelectedToken {
            token: AccessToken::Refresh(_),
            encoded,
            source: CredentialSource::Bearer,
        } = self.selected().await?
        else {
            return Err(RequestError::Unauthenticated);
        };
        Ok(encoded)
    }

    async fn renew(
        &self,
        expected: Option<&UserAccessToken>,
    ) -> Result<VerifiedUser, RequestError> {
        let settings = &self.settings;
        let encoded = header::cookie(&self.headers, settings.names().1)?
            .ok_or(RequestError::Unauthenticated)?;
        let refresh =
            RefreshToken::try_from(self.execute(self.auth.authenticate_token(&encoded)).await?)?;
        if expected.is_some_and(|user| !refresh.session.has_same_identity(&user.session)) {
            return Err(RequestError::Unauthenticated);
        }
        let tokens = self.execute(self.auth.refresh(&encoded)).await?;
        let access = self
            .execute(self.auth.authenticate_access(&tokens.access_token))
            .await?;
        Ok(VerifiedUser {
            access: VerifiedAccess(access),
            replacements: settings.replacement_headers(tokens)?,
        })
    }

    async fn load_user(&self) -> Result<VerifiedUser, RequestError> {
        let selected = match self.selected().await {
            Ok(selected) => selected,
            Err(error) => return self.handle_selection_error(error).await,
        };
        let AccessToken::User(user) = &selected.token else {
            return Err(RequestError::Unauthenticated);
        };
        self.verify_user(user, selected.source).await
    }

    async fn verify_user(
        &self,
        user: &UserAccessToken,
        source: CredentialSource,
    ) -> Result<VerifiedUser, RequestError> {
        match self.execute(self.auth.validate_access(user)).await {
            Ok(true) => Ok(VerifiedUser {
                access: VerifiedAccess(user.clone()),
                replacements: HeaderMap::new(),
            }),
            Ok(false) | Err(AuthError::ExpiredToken) if source == CredentialSource::Cookie => {
                self.renew(Some(user)).await
            }
            Ok(false) | Err(AuthError::ExpiredToken) => Err(RequestError::Expired),
            Err(error) => Err(error.into()),
        }
    }

    async fn handle_selection_error(
        &self,
        error: RequestError,
    ) -> Result<VerifiedUser, RequestError> {
        if self.headers.contains_key(AUTHORIZATION) {
            return Err(error);
        }
        match error {
            RequestError::Expired => self.renew(None).await,
            RequestError::Unauthenticated => self.renew_missing_access().await,
            _ => Err(error),
        }
    }

    async fn renew_missing_access(&self) -> Result<VerifiedUser, RequestError> {
        let headers = &self.headers;
        let settings = &self.settings;
        let (access, refresh) = settings.names();
        if header::cookie(headers, access)?.is_some() || header::cookie(headers, refresh)?.is_none()
        {
            return Err(RequestError::Unauthenticated);
        }
        settings.validate_origin(headers)?;
        self.renew(None).await
    }
}

impl UserContext {
    pub(super) fn signout_invalid_authority(&self, cookies: &CookieDelivery) {
        if self.0.headers.contains_key(AUTHORIZATION) {
            return;
        }
        let _ = cookies.clear();
    }

    pub async fn require(&self) -> Result<&VerifiedAccess, RequestError> {
        self.0
            .user
            .get_or_init(|| self.0.load_user())
            .await
            .as_ref()
            .map(|user| &user.access)
            .map_err(|error| *error)
    }
}

pub(super) fn finalize(
    context: &AuthenticationContext,
    delivery: &CookieDelivery,
    response: &mut axum::response::Response,
) -> Result<(), RequestError> {
    use axum::http::header::SET_COOKIE;
    let staged = delivery
        .response
        .lock()
        .map_err(|_| RequestError::Internal)?
        .clone();
    if let Some(user) = context.user.get() {
        match user {
            Ok(user) => {
                for value in user.replacements.get_all(SET_COOKIE) {
                    if !cookie_is_overridden(value, &staged, response.headers()) {
                        response.headers_mut().append(SET_COOKIE, value.clone());
                    }
                }
            }
            Err(RequestError::Unauthenticated | RequestError::Expired)
                if !context.headers.contains_key(AUTHORIZATION) && staged.is_empty() =>
            {
                for value in context.settings.clear_headers()?.get_all(SET_COOKIE) {
                    if !cookie_is_overridden(value, &staged, response.headers()) {
                        response.headers_mut().append(SET_COOKIE, value.clone());
                    }
                }
            }
            _ => {}
        }
    }
    for value in staged.get_all(SET_COOKIE) {
        response.headers_mut().append(SET_COOKIE, value.clone());
    }
    Ok(())
}

fn cookie_is_overridden(
    replacement: &axum::http::HeaderValue,
    staged: &HeaderMap,
    response: &HeaderMap,
) -> bool {
    use axum::http::header::SET_COOKIE;
    use cookie::Cookie;
    let Some(replacement) = replacement
        .to_str()
        .ok()
        .and_then(|value| Cookie::parse(value).ok())
    else {
        return false;
    };
    staged
        .get_all(SET_COOKIE)
        .iter()
        .chain(response.get_all(SET_COOKIE).iter())
        .any(|value| {
            value
                .to_str()
                .ok()
                .and_then(|value| Cookie::parse(value).ok())
                .is_some_and(|cookie| cookie.name() == replacement.name())
        })
}
