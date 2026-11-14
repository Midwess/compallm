use compallm_core::{authentication::AuthService, authorization::rbac::RbacService};
use juniper::FromContext;
use std::sync::Arc;

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
    authentication: Arc<AuthenticationContext>,
    delivery: CookieDelivery,
    authorization: AuthorizationContext,
}

impl juniper::Context for GraphqlContext {}

impl FromContext<GraphqlContext> for Option<Arc<compallm_storage::S3Storage>> {
    fn from(context: &GraphqlContext) -> &Self {
        <Self as FromContext<Di>>::from(context.di)
    }
}

impl GraphqlContext {
    pub fn new(
        di: &'static Di,
        user: UserContext,
        authentication: Arc<AuthenticationContext>,
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

impl FromContext<GraphqlContext> for RbacService {
    fn from(context: &GraphqlContext) -> &Self {
        <Self as FromContext<Di>>::from(context.di)
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

impl FromContext<GraphqlContext>
    for Box<dyn compallm_core::persistent::organization::OrganizationRepository>
{
    fn from(context: &GraphqlContext) -> &Self {
        <Self as FromContext<Di>>::from(context.di)
    }
}

impl FromContext<GraphqlContext> for compallm_core::preference::UserPreferenceService {
    fn from(context: &GraphqlContext) -> &Self {
        <Self as FromContext<Di>>::from(context.di)
    }
}

impl FromContext<GraphqlContext>
    for Box<dyn compallm_core::persistent::preference::user::UserPreferenceRepository>
{
    fn from(context: &GraphqlContext) -> &Self {
        <Self as FromContext<Di>>::from(context.di)
    }
}

impl FromContext<GraphqlContext> for compallm_core::persistent::DbContext {
    fn from(context: &GraphqlContext) -> &Self {
        <Self as FromContext<Di>>::from(context.di)
    }
}

impl FromContext<GraphqlContext> for compallm_core::user::ProfileService {
    fn from(context: &GraphqlContext) -> &Self {
        <Self as FromContext<Di>>::from(context.di)
    }
}
impl FromContext<GraphqlContext> for Box<dyn compallm_core::persistent::user::UserRepository> {
    fn from(context: &GraphqlContext) -> &Self {
        <Self as FromContext<Di>>::from(context.di)
    }
}

impl FromContext<GraphqlContext> for compallm_core::organization::profile_service::ProfileService {
    fn from(context: &GraphqlContext) -> &Self {
        <Self as FromContext<Di>>::from(context.di)
    }
}

impl FromContext<GraphqlContext>
    for Box<dyn compallm_core::persistent::provider::ProviderRepository>
{
    fn from(context: &GraphqlContext) -> &Self {
        <Self as FromContext<Di>>::from(context.di)
    }
}

impl FromContext<GraphqlContext> for Box<dyn compallm_core::persistent::model::ModelRepository> {
    fn from(context: &GraphqlContext) -> &Self {
        <Self as FromContext<Di>>::from(context.di)
    }
}
