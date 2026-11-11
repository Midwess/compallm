use juniper::{DefaultScalarValue, graphql_object};
use super::GraphqlContext;

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
