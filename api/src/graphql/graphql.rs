use super::GraphqlContext;
use juniper::{
    Arguments, DefaultScalarValue, EmptySubscription, ExecutionResult, Executor, GraphQLType,
    GraphQLValue, GraphQLValueAsync, Registry, RootNode, graphql_object,
};
use schemars::JsonSchema;
use std::sync::Arc;

pub struct Query;

#[graphql_object(name = "Query", context = GraphqlContext, scalar = DefaultScalarValue)]
impl Query {
    fn organization() -> super::organization::query::Query {
        super::organization::query::Query
    }

    fn authentication() -> super::authentication::query::Query {
        super::authentication::query::Query
    }
}

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

pub type ApplicationSchema =
    Arc<RootNode<RegisteredQuery, Mutation, EmptySubscription<GraphqlContext>>>;

/// Construct the shared schema without storing any caller or service state.
pub fn build_schema(max_depth: usize, max_fields: usize, max_tokens: usize) -> ApplicationSchema {
    Arc::new(
        RootNode::new(RegisteredQuery, Mutation, EmptySubscription::new())
            .limit_depth(max_depth)
            .limit_fields(max_fields)
            .limit_tokens(max_tokens),
    )
}

pub struct RegisteredQuery;

// Juniper discovers types through root fields. Register reusable contracts in
// that same schema without inventing executable operations to make them reachable.
impl GraphQLType for RegisteredQuery {
    fn name(info: &()) -> Option<juniper::ArcStr> {
        Query::name(info)
    }

    fn meta(info: &(), registry: &mut Registry) -> juniper::meta::MetaType {
        use super::{
            authentication,
            provider::{catalog, input},
        };
        registry.get_type::<authentication::input::EmailInput>(&());
        registry.get_type::<authentication::input::EmailCodeInput>(&());
        registry.get_type::<input::ProvidersCreateInput>(&());
        registry.get_type::<input::ProviderReplaceInput>(&());
        registry.get_type::<input::ProviderPatchInput>(&());
        registry.get_type::<catalog::ProviderTemplate>(&());
        Query::meta(info, registry)
    }
}

impl GraphQLValue for RegisteredQuery {
    type Context = GraphqlContext;
    type TypeInfo = ();

    fn type_name(&self, info: &()) -> Option<juniper::ArcStr> {
        Query.type_name(info)
    }

    fn resolve_field(
        &self,
        info: &(),
        field_name: &str,
        arguments: &Arguments,
        executor: &Executor<Self::Context>,
    ) -> ExecutionResult {
        Query.resolve_field(info, field_name, arguments, executor)
    }
}

impl GraphQLValueAsync for RegisteredQuery {
    fn resolve_field_async<'a>(
        &'a self,
        info: &'a (),
        field_name: &'a str,
        arguments: &'a Arguments,
        executor: &'a Executor<Self::Context>,
    ) -> juniper::BoxFuture<'a, ExecutionResult> {
        Query.resolve_field_async(info, field_name, arguments, executor)
    }
}

#[derive(JsonSchema)]
#[allow(dead_code)]
struct Inputs {
    role_create: super::organization::role::input::RoleCreateInput,
    group_create: super::organization::group::input::GroupCreateInput,
    email: super::authentication::input::EmailInput,
    code: super::authentication::input::EmailCodeInput,
    provider_create: super::provider::input::ProvidersCreateInput,
    provider_replace: super::provider::input::ProviderReplaceInput,
    provider_patch: super::provider::input::ProviderPatchInput,
}

pub(crate) fn validation_schema() -> serde_json::Value {
    serde_json::to_value(schemars::schema_for!(Inputs))
        .expect("serializable GraphQL validation schema")
}
