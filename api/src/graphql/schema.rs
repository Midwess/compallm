use async_graphql::{EmptyMutation, EmptySubscription, MergedObject, Schema};

use super::{Services, auth::AuthQuery};

pub type ApplicationSchema = Schema<Query, EmptyMutation, EmptySubscription>;

/// Flat public query root composed from domain-owned operations.
#[derive(Default, MergedObject)]
pub struct Query(AuthQuery);

/// Construct the shared schema from DI-provided services and execution limits.
pub fn build_schema(
    services: Services,
    max_depth: usize,
    max_complexity: usize,
) -> ApplicationSchema {
    Schema::build(Query::default(), EmptyMutation, EmptySubscription)
        .data(services)
        .limit_depth(max_depth)
        .limit_complexity(max_complexity)
        .finish()
}
