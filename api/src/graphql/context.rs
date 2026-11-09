use std::sync::Arc;

use super::Services;
use crate::middleware::authentication::VerifiedAccess;

/// Request-local verified caller and shared Core dependencies.
///
/// The schema never retains caller identity. Only the HTTP adapter combines
/// middleware-verified access with dependencies supplied by application DI.
pub struct GraphqlContext {
    services: Arc<Services>,
    access: VerifiedAccess,
}

impl juniper::Context for GraphqlContext {}

impl GraphqlContext {
    pub fn new(services: Arc<Services>, access: VerifiedAccess) -> Self {
        Self { services, access }
    }

    pub fn services(&self) -> &Services {
        &self.services
    }

    pub fn access(&self) -> &VerifiedAccess {
        &self.access
    }
}
