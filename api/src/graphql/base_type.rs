use juniper::{
    ArcStr, Arguments, BoxFuture, DefaultScalarValue, ExecutionResult, Executor, FieldError,
    GraphQLType, GraphQLValue, GraphQLValueAsync, Registry,
    macros::reflect::{BaseSubTypes, BaseType, WrappedType},
    marker::IsOutputType,
    meta::MetaType,
};

pub use compallm_query::PageInfo;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Payload<T> {
    pub node: T,
}

impl<T> From<T> for Payload<T> {
    fn from(node: T) -> Self {
        Self { node }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Edge<T> {
    pub cursor: String,
    pub node: T,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Connection<T> {
    pub edges: Vec<Edge<T>>,
    pub page_info: PageInfo,
}

/// A collection has the same connection shape whether paging arguments are supplied or omitted.
pub type Collection<T> = Connection<T>;

impl<T> Connection<T> {
    /// Wraps a complete authorized result. Callers supply stable ordering values and scope.
    pub fn complete(
        nodes: Vec<T>,
        scope: &str,
        position: impl Fn(&T) -> Vec<Option<String>>,
    ) -> Result<Self, compallm_query::CursorError> {
        let edges = nodes
            .into_iter()
            .map(|node| {
                Ok(Edge {
                    cursor: compallm_query::Cursor::encode(position(&node), scope)?.0,
                    node,
                })
            })
            .collect::<Result<Vec<_>, compallm_query::CursorError>>()?;
        Ok(Self {
            page_info: PageInfo {
                start_cursor: edges.first().map(|edge| edge.cursor.clone()),
                end_cursor: edges.last().map(|edge| edge.cursor.clone()),
                ..Default::default()
            },
            edges,
        })
    }

    pub fn map_nodes<U>(self, mut map: impl FnMut(T) -> U) -> Connection<U> {
        Connection {
            edges: self
                .edges
                .into_iter()
                .map(|edge| Edge {
                    cursor: edge.cursor,
                    node: map(edge.node),
                })
                .collect(),
            page_info: self.page_info,
        }
    }
}

impl<T, U: From<T>> From<compallm_query::Edge<T>> for Edge<U> {
    fn from(edge: compallm_query::Edge<T>) -> Self {
        Self {
            cursor: edge.cursor,
            node: edge.node.into(),
        }
    }
}

impl<T, U: From<T>> From<compallm_query::Connection<T>> for Connection<U> {
    fn from(connection: compallm_query::Connection<T>) -> Self {
        Self {
            edges: connection.edges.into_iter().map(Into::into).collect(),
            page_info: connection.page_info,
        }
    }
}

// Juniper's reflection names are compile-time strings. Keep their storage with
// each generic instantiation so schema and resolver names agree without a cache.
struct TypeName {
    bytes: [u8; 256],
    len: usize,
}

impl TypeName {
    const fn new(node: &str, suffix: &str) -> Self {
        let len = node.len() + suffix.len();
        assert!(len <= 256, "GraphQL connection type name exceeds 256 bytes");
        let mut name = Self {
            bytes: [0; 256],
            len,
        };
        let mut i = 0;
        while i < len {
            name.bytes[i] = if i < node.len() {
                node.as_bytes()[i]
            } else {
                suffix.as_bytes()[i - node.len()]
            };
            i += 1;
        }
        name
    }

    const fn as_str(&self) -> &str {
        match std::str::from_utf8(self.bytes.split_at(self.len).0) {
            Ok(name) => name,
            Err(_) => panic!("GraphQL type name must be UTF-8"),
        }
    }
}

macro_rules! connection_type {
    ($wrapper:ident, $suffix:literal) => {
        impl<T: BaseType<DefaultScalarValue>> $wrapper<T> {
            const TYPE_NAME: TypeName = TypeName::new(T::NAME, $suffix);
        }

        impl<T: BaseType<DefaultScalarValue>> BaseType<DefaultScalarValue> for $wrapper<T> {
            const NAME: &'static str = Self::TYPE_NAME.as_str();
        }

        impl<T: BaseType<DefaultScalarValue>> BaseSubTypes<DefaultScalarValue> for $wrapper<T> {
            const NAMES: &'static [&'static str] = &[Self::NAME];
        }

        impl<T> WrappedType<DefaultScalarValue> for $wrapper<T> {
            const VALUE: u128 = 1;
        }

        impl<
            T: IsOutputType<DefaultScalarValue>
                + GraphQLType<TypeInfo = ()>
                + BaseType<DefaultScalarValue>,
        > IsOutputType<DefaultScalarValue> for $wrapper<T>
        {
        }
    };
}

connection_type!(Payload, "Payload");
connection_type!(Edge, "Edge");
connection_type!(Connection, "Connection");

impl<T> GraphQLType for Edge<T>
where
    T: GraphQLType<TypeInfo = ()> + BaseType<DefaultScalarValue>,
{
    fn name(_: &()) -> Option<ArcStr> {
        Some(Self::NAME.into())
    }

    fn meta(_: &(), registry: &mut Registry) -> MetaType {
        let fields = [
            registry.field::<String>("cursor", &()),
            registry.field::<T>("node", &()),
        ];
        registry.build_object_type::<Self>(&(), &fields).into_meta()
    }
}

impl<T> GraphQLValue for Edge<T>
where
    T: GraphQLType<TypeInfo = ()> + BaseType<DefaultScalarValue>,
{
    type Context = T::Context;
    type TypeInfo = ();

    fn type_name(&self, _: &()) -> Option<ArcStr> {
        Self::name(&())
    }

    fn concrete_type_name(&self, _: &Self::Context, _: &()) -> String {
        Self::NAME.into()
    }

    fn resolve_field(
        &self,
        _: &(),
        field: &str,
        _: &Arguments,
        executor: &Executor<Self::Context>,
    ) -> ExecutionResult {
        match field {
            "cursor" => executor.resolve_with_ctx(&(), &self.cursor),
            "node" => executor.resolve(&(), &self.node),
            _ => Err(FieldError::from("Unknown edge field")),
        }
    }
}

impl<T> GraphQLValueAsync for Edge<T>
where
    T: GraphQLType<TypeInfo = ()> + GraphQLValueAsync + BaseType<DefaultScalarValue>,
    T::Context: Sync,
{
    fn resolve_field_async<'a>(
        &'a self,
        _: &'a (),
        field: &'a str,
        _: &'a Arguments,
        executor: &'a Executor<Self::Context>,
    ) -> BoxFuture<'a, ExecutionResult> {
        Box::pin(async move {
            match field {
                "cursor" => executor.resolve_with_ctx_async(&(), &self.cursor).await,
                "node" => executor.resolve_async(&(), &self.node).await,
                _ => Err(FieldError::from("Unknown edge field")),
            }
        })
    }
}

impl<T> GraphQLType for Connection<T>
where
    T: GraphQLType<TypeInfo = ()> + BaseType<DefaultScalarValue>,
{
    fn name(_: &()) -> Option<ArcStr> {
        Some(Self::NAME.into())
    }

    fn meta(_: &(), registry: &mut Registry) -> MetaType {
        let fields = [
            registry.field::<Vec<Edge<T>>>("edges", &()),
            registry.field::<PageInfo>("pageInfo", &()),
        ];
        registry.build_object_type::<Self>(&(), &fields).into_meta()
    }
}

impl<T> GraphQLValue for Connection<T>
where
    T: GraphQLType<TypeInfo = ()> + BaseType<DefaultScalarValue>,
{
    type Context = T::Context;
    type TypeInfo = ();

    fn type_name(&self, _: &()) -> Option<ArcStr> {
        Self::name(&())
    }

    fn concrete_type_name(&self, _: &Self::Context, _: &()) -> String {
        Self::NAME.into()
    }

    fn resolve_field(
        &self,
        _: &(),
        field: &str,
        _: &Arguments,
        executor: &Executor<Self::Context>,
    ) -> ExecutionResult {
        match field {
            "edges" => executor.resolve(&(), &self.edges),
            "pageInfo" => executor.resolve_with_ctx(&(), &self.page_info),
            _ => Err(FieldError::from("Unknown connection field")),
        }
    }
}

impl<T> GraphQLValueAsync for Connection<T>
where
    T: GraphQLType<TypeInfo = ()> + GraphQLValueAsync + BaseType<DefaultScalarValue>,
    T::Context: Sync,
{
    fn resolve_field_async<'a>(
        &'a self,
        _: &'a (),
        field: &'a str,
        _: &'a Arguments,
        executor: &'a Executor<Self::Context>,
    ) -> BoxFuture<'a, ExecutionResult> {
        Box::pin(async move {
            match field {
                "edges" => executor.resolve_async(&(), &self.edges).await,
                "pageInfo" => executor.resolve_with_ctx_async(&(), &self.page_info).await,
                _ => Err(FieldError::from("Unknown connection field")),
            }
        })
    }
}

impl<T> GraphQLType for Payload<T>
where
    T: GraphQLType<TypeInfo = ()> + BaseType<DefaultScalarValue>,
{
    fn name(_: &()) -> Option<ArcStr> {
        Some(Self::NAME.into())
    }

    fn meta(_: &(), registry: &mut Registry) -> MetaType {
        let fields = [registry.field::<T>("node", &())];
        registry.build_object_type::<Self>(&(), &fields).into_meta()
    }
}

impl<T> GraphQLValue for Payload<T>
where
    T: GraphQLType<TypeInfo = ()> + BaseType<DefaultScalarValue>,
{
    type Context = T::Context;
    type TypeInfo = ();

    fn type_name(&self, _: &()) -> Option<ArcStr> {
        Self::name(&())
    }

    fn concrete_type_name(&self, _: &Self::Context, _: &()) -> String {
        Self::NAME.into()
    }

    fn resolve_field(
        &self,
        _: &(),
        field: &str,
        _: &Arguments,
        executor: &Executor<Self::Context>,
    ) -> ExecutionResult {
        match field {
            "node" => executor.resolve(&(), &self.node),
            _ => Err(FieldError::from("Unknown payload field")),
        }
    }
}

impl<T> GraphQLValueAsync for Payload<T>
where
    T: GraphQLType<TypeInfo = ()> + GraphQLValueAsync + BaseType<DefaultScalarValue>,
    T::Context: Sync,
{
    fn resolve_field_async<'a>(
        &'a self,
        _: &'a (),
        field: &'a str,
        _: &'a Arguments,
        executor: &'a Executor<Self::Context>,
    ) -> BoxFuture<'a, ExecutionResult> {
        Box::pin(async move {
            match field {
                "node" => executor.resolve_async(&(), &self.node).await,
                _ => Err(FieldError::from("Unknown payload field")),
            }
        })
    }
}
