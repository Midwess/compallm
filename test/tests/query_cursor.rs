use compallm_postgres::query::IntoSeaQuery;
use compallm_query::{Cursor, OrderBy, OrderByDirection, Pagination, Query};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, Database, DatabaseConnection, DbBackend,
    EntityTrait, QueryFilter, Schema, Set,
};

mod records {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
    #[sea_orm(table_name = "cursor_records")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: i64,
        pub organization_id: i64,
        pub label: Option<String>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}
    impl ActiveModelBehavior for ActiveModel {}
}

fn plan(
    direction: OrderByDirection,
    pagination: Pagination,
    scope: &str,
) -> Result<
    compallm_postgres::query::PagePlan<records::Entity>,
    compallm_postgres::query::QueryCompileError,
> {
    Box::new(Query::new()).plan_page(
        scoped(),
        pagination,
        vec![OrderBy {
            field: "label".into(),
            direction,
        }],
        30,
        scope,
    )
}

async fn fixture() -> DatabaseConnection {
    let url = std::env::var("QUERY_PAGINATION_TEST_DATABASE_URL")
        .expect("set QUERY_PAGINATION_TEST_DATABASE_URL to an isolated PostgreSQL database");
    let db = Database::connect(url).await.unwrap();
    let schema = Schema::new(DbBackend::Postgres);
    db.execute(&schema.create_table_from_entity(records::Entity))
        .await
        .unwrap();
    for (id, organization_id, label) in [
        (9_007_199_254_740_993, 1, None),
        (9_007_199_254_740_994, 1, Some("same")),
        (9_007_199_254_740_995, 1, Some("same")),
        (9_007_199_254_740_996, 1, Some("zebra")),
        (9_007_199_254_740_997, 1, None),
        (9_007_199_254_740_998, 2, Some("other organization")),
    ] {
        records::ActiveModel {
            id: Set(id),
            organization_id: Set(organization_id),
            label: Set(label.map(str::to_owned)),
        }
        .insert(&db)
        .await
        .unwrap();
    }
    db
}

fn scoped() -> sea_orm::Select<records::Entity> {
    records::Entity::find().filter(records::Column::OrganizationId.eq(1))
}

mod composite {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
    #[sea_orm(table_name = "cursor_composite")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub tenant: i64,
        #[sea_orm(primary_key, auto_increment = false)]
        pub sequence: i64,
        pub label: Option<String>,
        pub rank: Option<i32>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}
    impl ActiveModelBehavior for ActiveModel {}
}

#[tokio::test]
#[ignore = "requires an isolated QUERY_PAGINATION_TEST_DATABASE_URL; creates cursor_records"]
async fn postgres_cursor_traversal_preserves_scope_null_order_and_unique_ties() {
    let db = fixture().await;
    let base = 9_007_199_254_740_993_i64;
    for (direction, positions) in [
        (OrderByDirection::AscNullsFirst, vec![0, 4, 1, 2, 3]),
        (OrderByDirection::AscNullsLast, vec![1, 2, 3, 0, 4]),
        (OrderByDirection::DescNullsFirst, vec![0, 4, 3, 1, 2]),
        (OrderByDirection::DescNullsLast, vec![3, 1, 2, 0, 4]),
    ] {
        let expected: Vec<_> = positions.into_iter().map(|i| base + i).collect();
        let mut seen = Vec::new();
        let mut after = None;
        let mut cursors = Vec::new();
        loop {
            let page = plan(
                direction,
                Pagination {
                    first: Some(2),
                    after,
                    ..Default::default()
                },
                "organization:1",
            )
            .unwrap()
            .fetch(&db)
            .await
            .unwrap();
            assert_eq!(page.page_info.has_previous_page, !seen.is_empty());
            seen.extend(page.edges.iter().map(|edge| edge.node.id));
            cursors.extend(page.edges.iter().map(|edge| edge.cursor.clone()));
            after = page.page_info.end_cursor.map(Cursor::from);
            if !page.page_info.has_next_page {
                break;
            }
        }
        assert_eq!(seen, expected);
        let mut backward = Vec::new();
        let mut before = None;
        loop {
            let page = plan(
                direction,
                Pagination {
                    last: Some(2),
                    before,
                    ..Default::default()
                },
                "organization:1",
            )
            .unwrap()
            .fetch(&db)
            .await
            .unwrap();
            assert_eq!(page.page_info.has_next_page, !backward.is_empty());
            let mut ids: Vec<_> = page.edges.iter().map(|edge| edge.node.id).collect();
            ids.extend(backward);
            backward = ids;
            before = page.page_info.start_cursor.map(Cursor::from);
            if !page.page_info.has_previous_page {
                break;
            }
        }
        assert_eq!(backward, expected);
        let end = cursors.last().unwrap();
        let empty = plan(
            direction,
            Pagination {
                first: Some(2),
                after: Some(end.as_str().into()),
                ..Default::default()
            },
            "organization:1",
        )
        .unwrap()
        .fetch(&db)
        .await
        .unwrap();
        assert!(empty.edges.is_empty());
        assert!(empty.page_info.has_previous_page);
        assert!(!empty.page_info.has_next_page);

        let zero = plan(
            direction,
            Pagination {
                first: Some(0),
                after: Some(cursors[1].as_str().into()),
                ..Default::default()
            },
            "organization:1",
        )
        .unwrap()
        .fetch(&db)
        .await
        .unwrap();
        assert!(zero.edges.is_empty());
        assert!(zero.page_info.has_previous_page && zero.page_info.has_next_page);

        let offset = plan(
            direction,
            Pagination {
                first: Some(1),
                after: Some(cursors[0].as_str().into()),
                offset: Some(2),
                ..Default::default()
            },
            "organization:1",
        )
        .unwrap()
        .fetch(&db)
        .await
        .unwrap();
        assert_eq!(offset.edges[0].node.id, expected[3]);
        assert!(offset.page_info.has_previous_page && offset.page_info.has_next_page);

        assert!(
            plan(
                direction,
                Pagination {
                    first: Some(2),
                    after: Some(cursors[0].as_str().into()),
                    ..Default::default()
                },
                "organization:2"
            )
            .is_err()
        );
    }

    db.execute(&Schema::new(DbBackend::Postgres).create_table_from_entity(composite::Entity))
        .await
        .unwrap();
    for (tenant, sequence, label, rank) in [
        (1, 1, Some("a"), Some(1)),
        (1, 2, Some("a"), Some(2)),
        (2, 1, Some("a"), Some(2)),
        (1, 3, Some("a"), None),
        (1, 4, None, Some(1)),
    ] {
        composite::ActiveModel {
            tenant: Set(tenant),
            sequence: Set(sequence),
            label: Set(label.map(str::to_owned)),
            rank: Set(rank),
        }
        .insert(&db)
        .await
        .unwrap();
    }
    let mut after = None;
    let mut seen = Vec::new();
    loop {
        let page = Box::new(Query::new())
            .plan_page(
                composite::Entity::find(),
                Pagination {
                    first: Some(1),
                    after,
                    ..Default::default()
                },
                vec![
                    OrderBy {
                        field: "label".into(),
                        direction: OrderByDirection::AscNullsLast,
                    },
                    OrderBy {
                        field: "rank".into(),
                        direction: OrderByDirection::DescNullsLast,
                    },
                ],
                30,
                "composite",
            )
            .unwrap()
            .fetch(&db)
            .await
            .unwrap();
        seen.extend(page.edges.iter().map(|e| (e.node.tenant, e.node.sequence)));
        after = page.page_info.end_cursor.map(Cursor::from);
        if !page.page_info.has_next_page {
            break;
        }
    }
    assert_eq!(seen, [(1, 2), (2, 1), (1, 1), (1, 3), (1, 4)]);
}

#[tokio::test]
async fn cursor_keeps_large_order_values_and_paging_direction_is_interchangeable() {
    use sea_orm::MockDatabase;
    let values = vec![
        None,
        Some("9007199254740993".into()),
        Some("0.12345678901234567890".into()),
    ];
    let cursor = Cursor::encode(values.clone(), "query-context").unwrap();
    assert_eq!(cursor.decode("query-context").unwrap(), values);

    let model = |id| records::Model {
        id,
        organization_id: 1,
        label: None,
    };
    let db = MockDatabase::new(DbBackend::Postgres)
        .append_query_results([vec![model(3), model(2), model(1)]])
        .into_connection();
    let page = plan(
        OrderByDirection::AscNullsLast,
        Pagination {
            last: Some(2),
            ..Default::default()
        },
        "organization:1",
    )
    .unwrap()
    .fetch(&db)
    .await
    .unwrap();
    assert_eq!(
        page.edges
            .iter()
            .map(|edge| edge.node.id)
            .collect::<Vec<_>>(),
        [2, 3]
    );
    assert!(page.page_info.has_previous_page);
    assert!(!page.page_info.has_next_page);
    let next = plan(
        OrderByDirection::AscNullsLast,
        Pagination {
            first: Some(2),
            after: page.page_info.end_cursor.map(Cursor::from),
            ..Default::default()
        },
        "organization:1",
    );
    assert!(next.is_ok());
}
