//! Axum request setup and response finalization.
use super::{
    authentication::{self, AuthenticationContext, UserContext},
    cookie::{AuthenticationSettings, CookieDelivery},
    error::no_store,
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
        mut request: Request,
        next: Next,
    ) -> Response {
        request
            .extensions_mut()
            .insert(super::authorization::context(user));
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
        middleware
            .db
            .scope(async {
                let delivery = CookieDelivery {
                    settings: middleware.settings.clone(),
                    request: headers.clone(),
                    response: Arc::new(Mutex::new(Default::default())),
                };
                let context =
                    AuthenticationContext::new(middleware.auth, middleware.settings, headers);
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
    }
}
