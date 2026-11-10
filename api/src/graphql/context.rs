use compallm_core::authentication::AuthService;
use juniper::FromContext;

use crate::{
    di::Di,
    middleware::authentication::{CookieDelivery, RequestIdentity},
};

/// Request-local identity and a reference to the application's existing DI.
///
/// Resolvers receive projected dependencies, never this container or DI itself.
pub struct GraphqlContext {
    di: &'static Di,
    identity: RequestIdentity,
    delivery: CookieDelivery,
}

impl juniper::Context for GraphqlContext {}

impl GraphqlContext {
    pub fn new(di: &'static Di, identity: RequestIdentity, delivery: CookieDelivery) -> Self {
        Self {
            di,
            identity,
            delivery,
        }
    }
}

impl FromContext<GraphqlContext> for AuthService {
    fn from(context: &GraphqlContext) -> &Self {
        <Self as FromContext<Di>>::from(context.di)
    }
}

impl FromContext<GraphqlContext> for RequestIdentity {
    fn from(context: &GraphqlContext) -> &Self {
        &context.identity
    }
}

impl FromContext<GraphqlContext> for CookieDelivery {
    fn from(context: &GraphqlContext) -> &Self {
        &context.delivery
    }
}
