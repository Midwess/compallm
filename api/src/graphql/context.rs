use compallm_core::authentication::AuthService;
use juniper::FromContext;

use crate::{
    di::Di,
    http::authentication::{AuthenticationContext, CookieDelivery, UserContext},
    http::authorization::AuthorizationContext,
};

/// Request-local identity and a reference to the application's existing DI.
///
/// Resolvers receive projected dependencies, never this container or DI itself.
pub struct GraphqlContext {
    di: &'static Di,
    user: UserContext,
    authentication: AuthenticationContext,
    delivery: CookieDelivery,
    authorization: AuthorizationContext,
}

impl juniper::Context for GraphqlContext {}

impl GraphqlContext {
    pub fn new(
        di: &'static Di,
        user: UserContext,
        authentication: AuthenticationContext,
        delivery: CookieDelivery,
        authorization: AuthorizationContext,
    ) -> Self {
        Self {
            di,
            user,
            authentication,
            delivery,
            authorization,
        }
    }
}

impl FromContext<GraphqlContext> for AuthorizationContext {
    fn from(context: &GraphqlContext) -> &Self {
        &context.authorization
    }
}

impl FromContext<GraphqlContext> for AuthService {
    fn from(context: &GraphqlContext) -> &Self {
        <Self as FromContext<Di>>::from(context.di)
    }
}

impl FromContext<GraphqlContext> for UserContext {
    fn from(context: &GraphqlContext) -> &Self {
        &context.user
    }
}

impl FromContext<GraphqlContext> for AuthenticationContext {
    fn from(context: &GraphqlContext) -> &Self {
        &context.authentication
    }
}

impl FromContext<GraphqlContext> for CookieDelivery {
    fn from(context: &GraphqlContext) -> &Self {
        &context.delivery
    }
}
