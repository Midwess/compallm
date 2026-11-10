//! Request-scoped, lazily verified authentication.
use std::sync::Arc;

use axum::http::{HeaderMap, header::AUTHORIZATION};
use compallm_core::{
    authentication::{
        AccessToken, AuthError, AuthService, EmailOtpAccessToken, RefreshToken, UserAccessToken,
    },
    persistent::DbContext,
};
use tokio::sync::OnceCell;

pub use super::cookie::{AuthenticationSettings, CookieDelivery};
pub use super::error::RequestError;
use super::header;

#[derive(Clone)]
pub struct Authentication {
    pub(super) db: &'static DbContext,
    auth: &'static AuthService,
    pub(super) settings: AuthenticationSettings,
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
}

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
    bearer: bool,
}

pub(super) struct VerifiedUser {
    access: VerifiedAccess,
    replacements: HeaderMap,
}

struct State {
    authentication: Authentication,
    headers: HeaderMap,
    selected: OnceCell<Result<SelectedToken, RequestError>>,
    user: OnceCell<Result<VerifiedUser, RequestError>>,
}

#[derive(Clone)]
pub struct AuthenticationContext(Arc<State>);

#[derive(Clone)]
pub struct UserContext(AuthenticationContext);

impl AuthenticationContext {
    pub(super) fn new(authentication: Authentication, headers: HeaderMap) -> Self {
        Self(Arc::new(State {
            authentication,
            headers,
            selected: OnceCell::new(),
            user: OnceCell::new(),
        }))
    }

    pub(super) fn user_context(&self) -> UserContext {
        UserContext(self.clone())
    }

    async fn selected(&self) -> Result<&SelectedToken, RequestError> {
        self.0
            .selected
            .get_or_init(|| async {
                let headers = &self.0.headers;
                let bearer = header::bearer(headers)?;
                let is_bearer = bearer.is_some();
                let encoded = if let Some(bearer) = bearer {
                    bearer.to_owned()
                } else {
                    let encoded =
                        header::cookie(headers, self.0.authentication.settings.names().0)?;
                    if encoded.is_some()
                        || header::cookie(headers, self.0.authentication.settings.names().1)?
                            .is_some()
                    {
                        self.0.authentication.settings.validate_origin(headers)?;
                    }
                    encoded.ok_or(RequestError::Unauthenticated)?
                };
                let token = self
                    .0
                    .authentication
                    .auth
                    .authenticate_token(&encoded)
                    .await
                    .map_err(RequestError::from)?;
                Ok(SelectedToken {
                    token,
                    encoded,
                    bearer: is_bearer,
                })
            })
            .await
            .as_ref()
            .map_err(|error| *error)
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
        match self.selected().await {
            Ok(SelectedToken {
                token: AccessToken::Refresh(_),
                encoded,
                bearer: true,
            }) => Ok(encoded),
            Err(error) => Err(error),
            _ => Err(RequestError::Unauthenticated),
        }
    }

    async fn renew(
        &self,
        expected: Option<&UserAccessToken>,
    ) -> Result<VerifiedUser, RequestError> {
        let settings = &self.0.authentication.settings;
        let encoded = header::cookie(&self.0.headers, settings.names().1)?
            .ok_or(RequestError::Unauthenticated)?;
        let refresh = RefreshToken::try_from(
            self.0
                .authentication
                .auth
                .authenticate_token(&encoded)
                .await?,
        )?;
        if expected.is_some_and(|user| !refresh.session.has_same_identity(&user.session)) {
            return Err(RequestError::Unauthenticated);
        }
        let tokens = self.0.authentication.auth.refresh(&encoded).await?;
        let access = self
            .0
            .authentication
            .auth
            .authenticate_access(&tokens.access_token)
            .await?;
        Ok(VerifiedUser {
            access: VerifiedAccess(access),
            replacements: settings.replacement_headers(tokens)?,
        })
    }

    async fn load_user(&self) -> Result<VerifiedUser, RequestError> {
        let selected = self.selected().await;
        match selected {
            Ok(SelectedToken {
                token: AccessToken::User(user),
                bearer,
                ..
            }) => match self.0.authentication.auth.validate_access(user).await {
                Ok(true) => Ok(VerifiedUser {
                    access: VerifiedAccess(user.clone()),
                    replacements: HeaderMap::new(),
                }),
                Ok(false) | Err(AuthError::ExpiredToken) if !bearer => self.renew(Some(user)).await,
                Ok(false) | Err(AuthError::ExpiredToken) => Err(RequestError::Expired),
                Err(error) => Err(error.into()),
            },
            Err(RequestError::Unauthenticated)
                if !self.0.headers.contains_key(AUTHORIZATION)
                    && header::cookie(
                        &self.0.headers,
                        self.0.authentication.settings.names().0,
                    )?
                    .is_none() =>
            {
                if header::cookie(&self.0.headers, self.0.authentication.settings.names().1)?
                    .is_none()
                {
                    return Err(RequestError::Unauthenticated);
                }
                self.0
                    .authentication
                    .settings
                    .validate_origin(&self.0.headers)?;
                self.renew(None).await
            }
            Err(RequestError::Expired) if !self.0.headers.contains_key(AUTHORIZATION) => {
                self.renew(None).await
            }
            Err(error) => Err(error),
            _ => Err(RequestError::Unauthenticated),
        }
    }

    pub(super) fn user_result(&self) -> Option<&Result<VerifiedUser, RequestError>> {
        self.0.user.get()
    }
}

impl UserContext {
    pub async fn require(&self) -> Result<&VerifiedAccess, RequestError> {
        self.0
            .0
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
    use cookie::Cookie;
    let staged = delivery
        .response
        .lock()
        .map_err(|_| RequestError::Internal)?
        .clone();
    if let Some(user) = context.user_result() {
        match user {
            Ok(user) => {
                for value in user.replacements.get_all(SET_COOKIE) {
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
            Err(RequestError::Unauthenticated | RequestError::Expired)
                if !context.0.headers.contains_key(AUTHORIZATION) && staged.is_empty() =>
            {
                response
                    .headers_mut()
                    .extend(context.0.authentication.settings.clear_headers()?)
            }
            _ => {}
        }
    }
    response.headers_mut().extend(staged);
    Ok(())
}
