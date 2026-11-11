use juniper::{EmptySubscription, RootNode, graphql_object};
use std::sync::Arc;
use super::{GraphqlContext, registration::RegisteredQuery};

pub struct Mutation;

#[graphql_object(context = GraphqlContext)]
impl Mutation {
    fn authentication() -> super::authentication::mutation::Mutation {
        super::authentication::mutation::Mutation
    }

    fn organization() -> super::organization::mutation::Mutation {
        super::organization::mutation::Mutation
    }
}

pub type ApplicationSchema = Arc<RootNode<RegisteredQuery, Mutation, EmptySubscription<GraphqlContext>>>;

/// Construct the shared schema without storing any caller or service state.
pub fn build_schema(max_depth: usize, max_fields: usize, max_tokens: usize) -> ApplicationSchema {
    Arc::new(
        RootNode::new(RegisteredQuery, Mutation, EmptySubscription::new())
            .limit_depth(max_depth)
            .limit_fields(max_fields)
            .limit_tokens(max_tokens),
    )
}
