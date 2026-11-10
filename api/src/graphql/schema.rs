use juniper::{
    Arguments, DefaultScalarValue, EmptySubscription, ExecutionResult, Executor, GraphQLType,
    GraphQLValue, GraphQLValueAsync, Registry, RootNode, graphql_object,
};
use std::sync::Arc;

use crate::middleware::authentication::RequestIdentity;

use super::{
    GraphqlContext,
    authentication::{self, Mutation, Session},
};

pub type ApplicationSchema = Arc<RootNode<Query, Mutation, EmptySubscription<GraphqlContext>>>;

/// Flat public query root delegating to domain-owned operations.
pub struct Query;

struct QueryFields;

#[graphql_object(name = "Query", context = GraphqlContext, scalar = DefaultScalarValue)]
impl QueryFields {
    /// Read the selected login's verified identity and organization-name snapshot.
    ///
    /// Requires active, current user access. No credentials, policy source, or
    /// freshness internals are exposed; no user/organization hydration is performed.
    fn session(
        #[graphql(context)] identity: &RequestIdentity,
    ) -> Result<Session, super::error::Error> {
        Ok(authentication::query::session(identity.user()?))
    }
}

// Juniper discovers types through root fields. Register reusable contracts in
// that same schema without inventing executable operations to make them reachable.
impl GraphQLType for Query {
    fn name(info: &()) -> Option<juniper::ArcStr> {
        QueryFields::name(info)
    }

    fn meta(info: &(), registry: &mut Registry) -> juniper::meta::MetaType {
        use super::{
            authentication,
            provider::{catalog, input},
        };
        registry.get_type::<authentication::EmailInput>(&());
        registry.get_type::<authentication::EmailCodeInput>(&());
        registry.get_type::<input::ProvidersCreateInput>(&());
        registry.get_type::<input::ProviderReplaceInput>(&());
        registry.get_type::<input::ProviderPatchInput>(&());
        registry.get_type::<catalog::ProviderTemplate>(&());
        QueryFields::meta(info, registry)
    }
}

impl GraphQLValue for Query {
    type Context = GraphqlContext;
    type TypeInfo = ();

    fn type_name(&self, info: &()) -> Option<juniper::ArcStr> {
        QueryFields.type_name(info)
    }

    fn resolve_field(
        &self,
        info: &(),
        field_name: &str,
        arguments: &Arguments,
        executor: &Executor<Self::Context>,
    ) -> ExecutionResult {
        QueryFields.resolve_field(info, field_name, arguments, executor)
    }
}

impl GraphQLValueAsync for Query {
    fn resolve_field_async<'a>(
        &'a self,
        info: &'a (),
        field_name: &'a str,
        arguments: &'a Arguments,
        executor: &'a Executor<Self::Context>,
    ) -> juniper::BoxFuture<'a, ExecutionResult> {
        QueryFields.resolve_field_async(info, field_name, arguments, executor)
    }
}

/// Construct the shared schema without storing any caller or service state.
pub fn build_schema(max_depth: usize, max_fields: usize, max_tokens: usize) -> ApplicationSchema {
    Arc::new(
        RootNode::new(Query, Mutation, EmptySubscription::new())
            .limit_depth(max_depth)
            .limit_fields(max_fields)
            .limit_tokens(max_tokens),
    )
}
