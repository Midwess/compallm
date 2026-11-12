use compallm_postgres::{
    UserSchema,
    query::{IntoSeaQuery, PostgresQueryTarget},
};
use compallm_query::{
    IntoQuery, OrderBy, OrderByDirection, Pagination, Predicate, Query, QueryTarget,
    filter::{BigIntFilter, StringFilter},
    operation::{Comparison, Operation, StringOperation},
};
use sea_orm::{
    ColumnTrait, EntityTrait, ModelTrait, QueryFilter, QueryOrder, QuerySelect, QueryTrait,
    sea_query::{Condition, Expr, ExprTrait, NullOrdering, Order},
};

type Users = <UserSchema as ModelTrait>::Entity;
type Column = <Users as EntityTrait>::Column;

struct UserTarget;
#[derive(Clone, Copy)]
enum UserField {
    Id,
    Email,
    DisplayName,
}
impl QueryTarget for UserTarget {
    type Field = UserField;
}
impl PostgresQueryTarget for UserTarget {
    type Table = Users;
    fn column(field: &UserField) -> Column {
        match field {
            UserField::Id => Column::Id,
            UserField::Email => Column::Email,
            UserField::DisplayName => Column::DisplayName,
        }
    }
}

#[derive(Default, IntoQuery)]
#[query(target = UserTarget, fields = UserField)]
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
    let mut actual = input.into_sea_query().unwrap();
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
    let query = Query::<UserTarget> {
        predicate: Predicate::Field {
            field: UserField::Id,
            operation: Operation::Int(Comparison::Gte(10)),
        },
        order_by: vec![OrderBy {
            field: UserField::DisplayName,
            direction: OrderByDirection::DescNullsFirst,
        }],
        pagination: Pagination {
            first: Some(20),
            offset: Some(40),
            ..Default::default()
        },
    };
    let mut actual = query.into_sea_query().unwrap();
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
    let mut actual = Query::<UserTarget> {
        predicate: Predicate::Not(Box::new(Predicate::Or(vec![]))),
        ..Default::default()
    }
    .into_sea_query()
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

struct ScopedUserTarget;
impl QueryTarget for ScopedUserTarget {
    type Field = UserField;
}
impl PostgresQueryTarget for ScopedUserTarget {
    type Table = Users;
    fn column(field: &UserField) -> Column {
        UserTarget::column(field)
    }
    fn select() -> sea_orm::Select<Users> {
        Users::find().filter(Column::Id.eq(42_i64))
    }
}

#[test]
fn trusted_base_selection_remains_outside_client_or_conditions() {
    let query = Query::<ScopedUserTarget> {
        predicate: Predicate::Or(vec![
            Predicate::Field {
                field: UserField::Email,
                operation: Operation::String(StringOperation::Eq("a@example.com".into())),
            },
            Predicate::Field {
                field: UserField::Email,
                operation: Operation::String(StringOperation::Eq("b@example.com".into())),
            },
        ]),
        ..Default::default()
    };
    let mut actual = query.into_sea_query().unwrap();
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

struct RecordTarget;
enum RecordField {
    Tags,
}
impl QueryTarget for RecordTarget {
    type Field = RecordField;
}
impl PostgresQueryTarget for RecordTarget {
    type Table = records::Entity;
    fn column(_: &RecordField) -> records::Column {
        records::Column::Tags
    }
}

#[test]
fn array_membership_preserves_array_type_and_bound_values() {
    use compallm_query::operation::ListOperation;
    use sea_orm::sea_query::{Alias, ArrayType, BinOper, Value, extension::postgres::PgBinOper};
    let mut actual = Query::<RecordTarget>::new()
        .filter(
            RecordField::Tags,
            Operation::StringList(ListOperation::Contains(vec!["visible".into()])),
        )
        .into_sea_query()
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
    let mut actual = Query::<UserTarget>::new()
        .filter(
            UserField::Email,
            Operation::String(StringOperation::ILike(pattern.into())),
        )
        .into_sea_query()
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
