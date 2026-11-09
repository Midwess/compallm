use compallm_core::auth::AuthService;
use juniper::FromContext;

use crate::{di::Di, middleware::authentication::VerifiedAccess};

/// Request-local identity and a reference to the application's existing DI.
///
/// Resolvers receive projected dependencies, never this container or DI itself.
pub struct GraphqlContext {
    di: &'static Di,
    access: VerifiedAccess,
}

impl juniper::Context for GraphqlContext {}

impl GraphqlContext {
    pub fn new(di: &'static Di, access: VerifiedAccess) -> Self {
        Self { di, access }
    }
}

impl FromContext<GraphqlContext> for AuthService {
    fn from(context: &GraphqlContext) -> &Self {
        <Self as FromContext<Di>>::from(context.di)
    }
}

impl FromContext<GraphqlContext> for VerifiedAccess {
    fn from(context: &GraphqlContext) -> &Self {
        &context.access
    }
}
