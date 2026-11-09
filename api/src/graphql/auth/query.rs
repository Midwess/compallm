use super::Session;
use crate::middleware::authentication::VerifiedAccess;

/// Project the already-verified login without hydration or another freshness lookup.
pub(crate) fn session(access: &VerifiedAccess) -> Session {
    access.access().session.clone().into()
}
