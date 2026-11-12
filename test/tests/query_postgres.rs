use compallm_postgres::{UserSchema, query::IntoSeaQuery};
use compallm_query::{
    IntoQuery, OrderBy, OrderByDirection, Pagination, Predicate, Query,
    filter::{BigIntFilter, StringFilter},
    operation::{Comparison, Operation, StringOperation},
};
use sea_orm::{
    ColumnTrait, EntityTrait, ModelTrait, QueryFilter, QueryOrder, QuerySelect, QueryTrait,
    sea_query::{Condition, Expr, ExprTrait, NullOrdering, Order},
};

type Users = <UserSchema as ModelTrait>::Entity;
type Column = <Users as EntityTrait>::Column;

#[derive(Default, IntoQuery)]
struct UserFilter {
    id: Option<BigIntFilter>,
    email: Option<StringFilter>,
    display_name: Option<StringFilter>,
    or: Option<Vec<UserFilter>>,
    not: Option<Box<UserFilter>>,
}

#[test]
fn derived_inputs_compile_to_bound_predicates_and_stable_ordering() {
    let input = UserFilter {
        email: Some(StringFilter {
            eq: Some("alice@example.com".into()),
            ..Default::default()
        }),
        ..Default::default()
    };
    let mut actual = Box::new(input)
        .into_sea_query::<Users>(Pagination::default(), vec![])
        .unwrap();
    let mut expected = Users::find()
        .filter(
            Condition::all().add(
                Condition::all()
                    .add(Expr::col((Users::default(), Column::Email)).eq("alice@example.com")),
            ),
        )
        .order_by_with_nulls(
            Expr::col((Users::default(), Column::Id)),
            Order::Asc,
            NullOrdering::Last,
        )
        .limit(30);
    assert_eq!(
        QueryTrait::query(&mut actual),
        QueryTrait::query(&mut expected)
    );
}

#[test]
fn ordering_and_offset_preserve_user_order_and_append_the_primary_key() {
    let query = Query {
        predicate: Predicate::Field {
            field: "id".into(),
            operation: Operation::Int(Comparison::Gte(10)),
        },
    };
    let ordering = vec![OrderBy {
        field: "display_name".into(),
        direction: OrderByDirection::DescNullsFirst,
    }];
    let pagination = Pagination {
        first: Some(20),
        offset: Some(40),
        ..Default::default()
    };
    let mut actual = Box::new(query)
        .into_sea_query::<Users>(pagination, ordering)
        .unwrap();
    let mut expected = Users::find()
        .filter(Condition::all().add(Expr::col((Users::default(), Column::Id)).gte(10)))
        .order_by_with_nulls(
            Expr::col((Users::default(), Column::DisplayName)),
            Order::Desc,
            NullOrdering::First,
        )
        .order_by_with_nulls(
            Expr::col((Users::default(), Column::Id)),
            Order::Asc,
            NullOrdering::Last,
        )
        .limit(20)
        .offset(40);
    assert_eq!(
        QueryTrait::query(&mut actual),
        QueryTrait::query(&mut expected)
    );
}

#[test]
fn explicit_empty_or_is_false_and_negation_is_preserved() {
    let mut actual = Box::new(Query {
        predicate: Predicate::Not(Box::new(Predicate::Or(vec![]))),
    })
    .into_sea_query::<Users>(Pagination::default(), vec![])
    .unwrap();
    let mut expected = Users::find()
        .filter(Condition::any().add(Expr::Constant(false.into())).not())
        .order_by_with_nulls(
            Expr::col((Users::default(), Column::Id)),
            Order::Asc,
            NullOrdering::Last,
        )
        .limit(30);
    assert_eq!(
        QueryTrait::query(&mut actual),
        QueryTrait::query(&mut expected)
    );
}

#[test]
fn trusted_base_selection_remains_outside_client_or_conditions() {
    let query = Query {
        predicate: Predicate::Or(vec![
            Predicate::Field {
                field: "email".into(),
                operation: Operation::String(StringOperation::Eq("a@example.com".into())),
            },
            Predicate::Field {
                field: "email".into(),
                operation: Operation::String(StringOperation::Eq("b@example.com".into())),
            },
        ]),
    };
    let mut actual = Box::new(query)
        .apply_to_sea_query(
            Users::find().filter(Column::Id.eq(42_i64)),
            Pagination::default(),
            vec![],
            30,
        )
        .unwrap();
    let mut expected = Users::find()
        .filter(Column::Id.eq(42_i64))
        .filter(
            Condition::any()
                .add(
                    Condition::all()
                        .add(Expr::col((Users::default(), Column::Email)).eq("a@example.com")),
                )
                .add(
                    Condition::all()
                        .add(Expr::col((Users::default(), Column::Email)).eq("b@example.com")),
                ),
        )
        .order_by_with_nulls(
            Expr::col((Users::default(), Column::Id)),
            Order::Asc,
            NullOrdering::Last,
        )
        .limit(30);
    assert_eq!(
        QueryTrait::query(&mut actual),
        QueryTrait::query(&mut expected)
    );
}

mod records {
    use sea_orm::entity::prelude::*;
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
    #[sea_orm(table_name = "query_records")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: i64,
        pub tags: Vec<String>,
    }
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}
    impl ActiveModelBehavior for ActiveModel {}
}

#[test]
fn array_membership_preserves_array_type_and_bound_values() {
    use compallm_query::operation::ListOperation;
    use sea_orm::sea_query::{Alias, ArrayType, BinOper, Value, extension::postgres::PgBinOper};
    let mut actual = Box::new(Query::new().filter(
        "tags",
        Operation::StringList(ListOperation::Contains(vec!["visible".into()])),
    ))
    .into_sea_query::<records::Entity>(Pagination::default(), vec![])
    .unwrap();
    let values = Value::Array(
        ArrayType::String,
        Some(Box::new(vec![Value::String(Some("visible".into()))])),
    );
    let predicate = Expr::col((records::Entity, records::Column::Tags)).binary(
        BinOper::PgOperator(PgBinOper::Contains),
        Expr::val(values).cast_as(Alias::new("varchar[]")),
    );
    let mut expected = records::Entity::find()
        .filter(Condition::all().add(Condition::all().add(predicate)))
        .order_by_with_nulls(
            Expr::col((records::Entity, records::Column::Id)),
            Order::Asc,
            NullOrdering::Last,
        )
        .limit(30);
    assert_eq!(
        QueryTrait::query(&mut actual),
        QueryTrait::query(&mut expected)
    );
}

#[test]
fn ilike_keeps_patterns_as_bound_values() {
    use sea_orm::sea_query::extension::postgres::PgBinOper;
    let pattern = "%O'Reilly_!%";
    let mut actual = Box::new(Query::new().filter(
        "email",
        Operation::String(StringOperation::ILike(pattern.into())),
    ))
    .into_sea_query::<Users>(Pagination::default(), vec![])
    .unwrap();
    let mut expected = Users::find()
        .filter(
            Condition::all().add(
                Condition::all().add(
                    Expr::col((Users::default(), Column::Email))
                        .binary(PgBinOper::ILike, Expr::val(pattern)),
                ),
            ),
        )
        .order_by_with_nulls(
            Expr::col((Users::default(), Column::Id)),
            Order::Asc,
            NullOrdering::Last,
        )
        .limit(30);
    assert_eq!(
        QueryTrait::query(&mut actual),
        QueryTrait::query(&mut expected)
    );
}

#[derive(IntoQuery)]
struct RecordFilter {
    tags: Option<compallm_query::filter::StringListFilter>,
}

#[async_trait::async_trait]
trait RecordRepository: Send + Sync {
    async fn list(
        &self,
        query: Box<dyn IntoQuery>,
        pagination: Pagination,
        ordering: Vec<OrderBy>,
    ) -> anyhow::Result<Vec<records::Model>>;
}

struct PgRecords(sea_orm::DatabaseConnection);

#[async_trait::async_trait]
impl RecordRepository for PgRecords {
    async fn list(
        &self,
        query: Box<dyn IntoQuery>,
        pagination: Pagination,
        ordering: Vec<OrderBy>,
    ) -> anyhow::Result<Vec<records::Model>> {
        Ok(query
            .into_sea_query::<records::Entity>(pagination, ordering)?
            .all(&self.0)
            .await?)
    }
}

#[tokio::test]
async fn repository_trait_object_accepts_a_derived_query_trait_object() {
    let db = sea_orm::MockDatabase::new(sea_orm::DbBackend::Postgres)
        .append_query_results([vec![records::Model {
            id: 42,
            tags: vec!["visible".into()],
        }]])
        .into_connection();
    let repository: Box<dyn RecordRepository> = Box::new(PgRecords(db));
    let input: Box<dyn IntoQuery> = Box::new(RecordFilter {
        tags: Some(compallm_query::filter::StringListFilter {
            contains: Some(vec!["visible".into()]),
            ..Default::default()
        }),
    });
    let records = repository
        .list(input, Pagination::default(), vec![])
        .await
        .unwrap();
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].id, 42);
    assert_eq!(records[0].tags, vec!["visible"]);
}
