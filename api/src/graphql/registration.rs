use super::{GraphqlContext, query::Query};
use juniper::{
    Arguments, ExecutionResult, Executor, GraphQLType, GraphQLValue, GraphQLValueAsync, Registry,
};

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
