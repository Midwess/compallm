use compallm_api::graphql::{
    base_type::{Collection, Connection, Edge, PageInfo, Payload},
    organization::{group::Group, role::Role},
};
use juniper::{DefaultScalarValue, EmptyMutation, EmptySubscription, RootNode, Variables};
use serde_json::json;

struct RoleName(String);

impl From<RoleName> for Role {
    fn from(value: RoleName) -> Self {
        Self {
            id: juniper::ID::new("42:1"),
            name: value.0,
            description: "Administrator".into(),
            color: "#c06040".into(),
        }
    }
}

struct Query;

#[juniper::graphql_object(scalar = DefaultScalarValue)]
impl Query {
    fn role() -> Payload<Role> {
        Role::from(RoleName("Admin".into())).into()
    }

    fn acknowledged() -> Payload<bool> {
        true.into()
    }

    fn roles() -> Connection<Role> {
        let page = compallm_query::Connection {
            edges: vec![compallm_query::Edge {
                cursor: "opaque-position".into(),
                node: RoleName("Admin".into()),
            }],
            page_info: PageInfo {
                start_cursor: Some("opaque-position".into()),
                end_cursor: Some("opaque-position".into()),
                has_next_page: true,
                has_previous_page: false,
            },
        };
        Connection::<Role>::from(page).map_nodes(|mut role| {
            role.name.make_ascii_uppercase();
            role
        })
    }

    fn groups() -> Collection<Group> {
        Collection::complete(
            vec![Group {
                id: juniper::ID::new("42:2"),
                name: "Engineering".into(),
                description: String::new(),
            }],
            "organization:42:groups",
            |group| vec![Some(group.id.to_string())],
        )
        .unwrap()
    }
}

#[test]
fn collections_keep_resource_names_and_pagination_metadata_in_one_schema() {
    let schema = RootNode::new(
        Query,
        EmptyMutation::<()>::new(),
        EmptySubscription::<()>::new(),
    );
    let (value, errors) = juniper::execute_sync(
        "{ role { __typename node { name } } acknowledged { __typename node } roles { __typename edges { __typename cursor node { id name } } pageInfo { startCursor endCursor hasNextPage hasPreviousPage } } groups { __typename edges { __typename node { id name } } } }",
        None, &schema, &Variables::new(), &(),
    ).unwrap();
    assert!(errors.is_empty());
    assert_eq!(
        serde_json::to_value(value).unwrap(),
        json!({
            "role": { "__typename": "RolePayload", "node": { "name": "Admin" } },
            "acknowledged": { "__typename": "BooleanPayload", "node": true },
            "roles": {
                "__typename": "RoleConnection",
                "edges": [{ "__typename": "RoleEdge", "cursor": "opaque-position", "node": { "id": "42:1", "name": "ADMIN" } }],
                "pageInfo": { "startCursor": "opaque-position", "endCursor": "opaque-position", "hasNextPage": true, "hasPreviousPage": false }
            },
            "groups": { "__typename": "GroupConnection", "edges": [{ "__typename": "GroupEdge", "node": { "id": "42:2", "name": "Engineering" } }] }
        })
    );
    let complete = Query::groups();
    assert_eq!(
        complete.page_info.start_cursor.as_ref(),
        Some(&complete.edges[0].cursor)
    );
    assert_eq!(
        complete.page_info.end_cursor,
        complete.page_info.start_cursor
    );
    assert!(!complete.page_info.has_next_page && !complete.page_info.has_previous_page);
    assert_eq!(
        compallm_query::Cursor(complete.edges[0].cursor.clone())
            .decode("organization:42:groups")
            .unwrap(),
        vec![Some("42:2".into())],
    );
}

struct Access {
    allowed: bool,
}
impl juniper::Context for Access {}

struct ProtectedRecord;

#[juniper::graphql_object(context = Access, scalar = DefaultScalarValue)]
impl ProtectedRecord {
    async fn name(context: &Access) -> juniper::FieldResult<String> {
        if !context.allowed {
            return Err("Denied".into());
        }
        Ok("Protected".into())
    }
}

struct ProtectedQuery;

#[juniper::graphql_object(context = Access, scalar = DefaultScalarValue)]
impl ProtectedQuery {
    async fn record() -> Option<Payload<ProtectedRecord>> {
        Some(ProtectedRecord.into())
    }

    async fn records() -> Option<Collection<ProtectedRecord>> {
        Some(Collection {
            edges: vec![Edge {
                cursor: "protected-position".into(),
                node: ProtectedRecord,
            }],
            page_info: PageInfo::default(),
        })
    }
}

#[tokio::test]
async fn connection_nodes_retain_async_context_and_propagate_field_denial() {
    let schema = RootNode::new(
        ProtectedQuery,
        EmptyMutation::<Access>::new(),
        EmptySubscription::<Access>::new(),
    );
    let document = "{ record { node { name } } records { edges { cursor node { name } } } }";
    let (value, errors) = juniper::execute(
        document,
        None,
        &schema,
        &Variables::new(),
        &Access { allowed: true },
    )
    .await
    .unwrap();
    assert!(errors.is_empty());
    assert_eq!(
        serde_json::to_value(value).unwrap(),
        json!({ "record": { "node": { "name": "Protected" } }, "records": { "edges": [{ "cursor": "protected-position", "node": { "name": "Protected" } }] } })
    );
    let (value, errors) = juniper::execute(
        document,
        None,
        &schema,
        &Variables::new(),
        &Access { allowed: false },
    )
    .await
    .unwrap();
    assert_eq!(errors.len(), 2);
    assert_eq!(
        serde_json::to_value(value).unwrap(),
        json!({ "record": null, "records": null })
    );
}
