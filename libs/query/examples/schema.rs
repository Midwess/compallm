use compallm_query::{
    Cursor, IntoQuery, OrderByDirection, PageInfo, Pagination,
    filter::{IDFilter, IntFilter, StringFilter, StringListFilter},
};
use juniper::{EmptyMutation, EmptySubscription, GraphQLInputObject, GraphQLObject, RootNode};

#[derive(Clone, Debug, Default, GraphQLInputObject, IntoQuery)]
struct BlogFilter {
    node_id: Option<IDFilter>,
    id: Option<IntFilter>,
    name: Option<StringFilter>,
    tags: Option<StringListFilter>,
    and: Option<Vec<BlogFilter>>,
    or: Option<Vec<BlogFilter>>,
    not: Option<Box<BlogFilter>>,
}

#[derive(Clone, Debug, Default, GraphQLInputObject)]
struct BlogOrderBy {
    id: Option<OrderByDirection>,
    name: Option<OrderByDirection>,
}

impl TryFrom<BlogOrderBy> for compallm_query::OrderBy {
    type Error = compallm_query::order::OrderByError;

    fn try_from(value: BlogOrderBy) -> Result<Self, Self::Error> {
        match (value.id, value.name) {
            (Some(direction), None) => Ok(Self {
                field: "id".into(),
                direction,
            }),
            (None, Some(direction)) => Ok(Self {
                field: "name".into(),
                direction,
            }),
            _ => Err(Self::Error::ExactlyOneField),
        }
    }
}

#[juniper::graphql_interface(for = Blog)]
trait Node {
    fn node_id(&self) -> &juniper::ID;
}

struct Blog {
    node_id: juniper::ID,
    id: i32,
    name: String,
}

#[juniper::graphql_object(impl = NodeValue)]
impl Blog {
    fn node_id(&self) -> &juniper::ID {
        Node::node_id(self)
    }
    fn id(&self) -> i32 {
        self.id
    }
    fn name(&self) -> &str {
        &self.name
    }
}

impl Node for Blog {
    fn node_id(&self) -> &juniper::ID {
        &self.node_id
    }
}

#[derive(GraphQLObject)]
struct BlogEdge {
    cursor: String,
    node: Blog,
}

#[derive(GraphQLObject)]
struct BlogConnection {
    edges: Vec<BlogEdge>,
    page_info: PageInfo,
}

struct Query;

#[juniper::graphql_object]
impl Query {
    fn node(node_id: juniper::ID) -> Option<NodeValue> {
        let _ = node_id;
        None
    }

    fn blog_by_pk(id: i32) -> Option<Blog> {
        let _ = id;
        None
    }

    #[allow(clippy::too_many_arguments)]
    fn blog_collection(
        first: Option<i32>,
        last: Option<i32>,
        before: Option<Cursor>,
        after: Option<Cursor>,
        offset: Option<i32>,
        filter: Option<BlogFilter>,
        order_by: Option<Vec<BlogOrderBy>>,
    ) -> juniper::FieldResult<BlogConnection> {
        let pagination = Pagination {
            first,
            last,
            before,
            after,
            offset,
        };
        pagination
            .validate()
            .map_err(|error| juniper::FieldError::new(error.to_string(), juniper::Value::null()))?;
        let _query = Box::new(filter.unwrap_or_default()).into_query();
        let _ordering: Vec<compallm_query::OrderBy> = order_by
            .unwrap_or_default()
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<_, _>>()
            .map_err(|error: compallm_query::order::OrderByError| {
                juniper::FieldError::new(error.to_string(), juniper::Value::null())
            })?;
        let _options = (pagination, _ordering);
        Ok(BlogConnection {
            edges: vec![],
            page_info: PageInfo::default(),
        })
    }
}

fn main() {
    let schema = RootNode::new(
        Query,
        EmptyMutation::<()>::new(),
        EmptySubscription::<()>::new(),
    );
    println!("{}", schema.as_sdl());
}
