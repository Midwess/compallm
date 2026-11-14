//! Axum request setup and response finalization.
use super::{
    authentication::{self, AuthenticationContext, UserContext},
    cookie::{AuthenticationSettings, CookieDelivery},
    error::{RequestError, no_store},
};
use axum::{
    Extension,
    extract::{Request, State},
    middleware::Next,
    response::{IntoResponse, Response},
};
use compallm_core::{
    authentication::{AuthError, AuthService},
    persistent::DbContext,
};
use std::sync::{Arc, Mutex};

pub struct AuthorizationMiddleware;

impl AuthorizationMiddleware {
    pub async fn handle(
        Extension(user): Extension<super::authentication::UserContext>,
        Extension(cookies): Extension<CookieDelivery>,
        mut request: Request,
        next: Next,
    ) -> Response {
        let identity = user.clone();
        let authorization = super::authorization::context(user, cookies);
        let loader = authorization.clone();
        let Ok(context) = DbContext::current() else {
            return no_store(RequestError::Internal.into_response());
        };
        if context
            .restrict(async move { loader.load_permission().await })
            .is_err()
        {
            return no_store(RequestError::Internal.into_response());
        }
        let loader: compallm_core::persistent::connection::IdentityLoader =
            Arc::new(async_once_cell::Lazy::new(Box::pin(async move {
                identity
                    .require()
                    .await
                    .map(|verified| verified.access().session.clone())
                    .map_err(|error| {
                        Arc::new(super::authorization::AuthorizationError::Authentication(
                            error,
                        )) as Arc<dyn std::error::Error + Send + Sync>
                    })
            })));
        if context.postgres().identify(loader).is_err() {
            return no_store(RequestError::Internal.into_response());
        }
        request.extensions_mut().insert(authorization);
        next.run(request).await
    }
}

#[derive(Clone)]
pub struct AuthenticationMiddleware {
    db: &'static DbContext,
    auth: &'static AuthService,
    settings: AuthenticationSettings,
    strict: bool,
}

impl AuthenticationMiddleware {
    pub fn optional(
        db: &'static DbContext,
        auth: &'static AuthService,
        settings: AuthenticationSettings,
    ) -> Result<Self, AuthError> {
        settings.validate()?;
        Ok(Self {
            db,
            auth,
            settings,
            strict: false,
        })
    }
    pub fn required(
        db: &'static DbContext,
        auth: &'static AuthService,
        settings: AuthenticationSettings,
    ) -> Result<Self, AuthError> {
        let mut middleware = Self::optional(db, auth, settings)?;
        middleware.strict = true;
        Ok(middleware)
    }

    pub async fn handle(
        State(middleware): State<Self>,
        mut request: Request,
        next: Next,
    ) -> Response {
        let headers = request.headers().clone();
        let Ok(context_db) = middleware.db.fresh() else {
            return no_store(RequestError::Internal.into_response());
        };
        let Ok(trusted_db) = middleware.db.fresh() else {
            return no_store(RequestError::Internal.into_response());
        };
        context_db
            .request_scope(async {
                let delivery = CookieDelivery {
                    settings: middleware.settings.clone(),
                    request: headers.clone(),
                    response: Arc::new(Mutex::new(Default::default())),
                };
                let context = AuthenticationContext::new(
                    middleware.auth,
                    trusted_db,
                    middleware.settings,
                    headers,
                );
                let user = UserContext(context.clone());
                if middleware.strict {
                    match user.require().await {
                        Ok(access) => {
                            request.extensions_mut().insert(access.clone());
                        }
                        Err(error) => {
                            let mut response = error.into_response();
                            if let Err(error) =
                                authentication::finalize(&context, &delivery, &mut response)
                            {
                                return error.into_response();
                            }
                            return no_store(response);
                        }
                    }
                }
                request.extensions_mut().insert(user);
                request.extensions_mut().insert(context.clone());
                request.extensions_mut().insert(delivery.clone());
                let mut response = next.run(request).await;
                if let Err(error) = authentication::finalize(&context, &delivery, &mut response) {
                    return error.into_response();
                }
                no_store(response)
            })
            .await
            .unwrap_or_else(|_| no_store(RequestError::Internal.into_response()))
    }
}
