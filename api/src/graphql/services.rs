use compallm_core::auth::AuthService;

/// Shared Core services supplied by application DI when the schema is built.
///
/// Caller identity belongs in request data, never in this container. Add concrete
/// services here as implemented operations need them; do not construct them in resolvers.
pub struct Services {
    pub auth: &'static AuthService,
}
