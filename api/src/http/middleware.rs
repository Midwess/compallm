//! Axum request setup and response finalization.
use super::{
    authentication::{self, Authentication, AuthenticationContext},
    cookie::CookieDelivery,
    error::no_store,
};
use axum::{
    Extension,
    extract::{Request, State},
    middleware::Next,
    response::{IntoResponse, Response},
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
    authentication: Authentication,
    strict: bool,
}

impl AuthenticationMiddleware {
    pub fn optional(authentication: Authentication) -> Self {
        Self {
            authentication,
            strict: false,
        }
    }
    pub fn required(authentication: Authentication) -> Self {
        Self {
            authentication,
            strict: true,
        }
    }

    pub async fn handle(
        State(middleware): State<Self>,
        mut request: Request,
        next: Next,
    ) -> Response {
        let authentication = middleware.authentication;
        let strict = middleware.strict;
        let headers = request.headers().clone();
        authentication
            .db
            .scope(async {
                let delivery = CookieDelivery {
                    settings: authentication.settings.clone(),
                    request: headers.clone(),
                    response: Arc::new(Mutex::new(Default::default())),
                };
                let context = AuthenticationContext::new(authentication, headers);
                let user = context.user_context();
                if strict {
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
