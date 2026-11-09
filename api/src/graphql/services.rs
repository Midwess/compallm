use compallm_core::auth::AuthService;

/// Shared Core services supplied by application DI for typed request contexts.
///
/// Caller identity belongs in GraphqlContext, never in this shared container. Add concrete
/// services here as implemented operations need them; do not construct them in resolvers.
pub struct Services {
    pub auth: &'static AuthService,
}
