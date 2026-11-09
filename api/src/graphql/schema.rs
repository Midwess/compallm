use juniper::{EmptyMutation, EmptySubscription, RootNode, graphql_object};
use std::sync::Arc;

use super::{
    GraphqlContext,
    auth::{self, Session},
};

pub type ApplicationSchema =
    Arc<RootNode<Query, EmptyMutation<GraphqlContext>, EmptySubscription<GraphqlContext>>>;

/// Flat public query root delegating to domain-owned operations.
pub struct Query;

#[graphql_object(context = GraphqlContext)]
impl Query {
    /// Read the selected login's verified identity and organization-name snapshot.
    ///
    /// Requires active, current user access. No credentials, policy source, or
    /// freshness internals are exposed; no user/organization hydration is performed.
    fn session(context: &GraphqlContext) -> Session {
        auth::query::session(context)
    }
}

/// Construct the shared schema without storing any caller or service state.
pub fn build_schema(max_depth: usize, max_fields: usize, max_tokens: usize) -> ApplicationSchema {
    Arc::new(
        RootNode::new(Query, EmptyMutation::new(), EmptySubscription::new())
            .limit_depth(max_depth)
            .limit_fields(max_fields)
            .limit_tokens(max_tokens),
    )
}
