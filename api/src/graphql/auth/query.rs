use super::Session;
use crate::graphql::GraphqlContext;

/// Project the already-verified login without hydration or another freshness lookup.
pub(crate) fn session(context: &GraphqlContext) -> Session {
    context.access().access().session.clone().into()
}
