mod operation;

use compallm_query::{IntoQuery, OrderByDirection, Pagination, Predicate, Query, QueryTarget};
use sea_orm::{
    ColumnTrait, EntityTrait, IdenStatic, Iterable, PrimaryKeyToColumn, QueryFilter, QueryOrder,
    QuerySelect, Select,
    sea_query::{Condition, Expr, NullOrdering, Order},
};

#[derive(Debug, thiserror::Error)]
pub enum QueryCompileError {
    #[error(transparent)]
    Pagination(#[from] compallm_query::PaginationError),
    #[error("cursor and backward pagination require a pagination planner")]
    UnsupportedPagination,
    #[error("operation does not match its bound database column")]
    TypeMismatch,
    #[error("database scalar type is unsupported by the query translator")]
    UnsupportedScalar,
}

#[derive(Debug, thiserror::Error)]
pub enum IntoSeaQueryError<Input> {
    #[error("query input conversion failed")]
    Input(#[source] Input),
    #[error(transparent)]
    Compile(#[from] QueryCompileError),
}

/// Trusted bindings between resource fields and SeaORM metadata.
pub trait PostgresQueryTarget: QueryTarget {
    type Table: EntityTrait;
    const MAX_ROWS: u32 = Pagination::DEFAULT_MAX_ROWS;

    fn column(field: &Self::Field) -> <Self::Table as EntityTrait>::Column;

    fn select() -> Select<Self::Table> {
        Self::Table::find()
    }

    fn expression(field: &Self::Field) -> Expr {
        Expr::col((Self::Table::default(), Self::column(field)))
    }
}

pub trait IntoSeaQuery: IntoQuery
where
    Self::Target: PostgresQueryTarget,
{
    fn into_sea_query(
        self,
    ) -> Result<Select<<Self::Target as PostgresQueryTarget>::Table>, IntoSeaQueryError<Self::Error>>
    where
        Self: Sized,
    {
        let query = self.into_query().map_err(IntoSeaQueryError::Input)?;
        compile(query).map_err(IntoSeaQueryError::Compile)
    }
}

impl<Q> IntoSeaQuery for Q
where
    Q: IntoQuery,
    Q::Target: PostgresQueryTarget,
{
}

fn compile<Target: PostgresQueryTarget>(
    query: Query<Target>,
) -> Result<Select<Target::Table>, QueryCompileError> {
    let page_size = query.pagination.page_size(Target::MAX_ROWS)?;
    if query.pagination.last.is_some()
        || query.pagination.before.is_some()
        || query.pagination.after.is_some()
    {
        return Err(QueryCompileError::UnsupportedPagination);
    }
    let mut select = Target::select().filter(predicate::<Target>(query.predicate)?);
    let mut ordered = Vec::new();
    for entry in query.order_by {
        let column = Target::column(&entry.field);
        let (order, nulls) = match entry.direction {
            OrderByDirection::AscNullsFirst => (Order::Asc, NullOrdering::First),
            OrderByDirection::AscNullsLast => (Order::Asc, NullOrdering::Last),
            OrderByDirection::DescNullsFirst => (Order::Desc, NullOrdering::First),
            OrderByDirection::DescNullsLast => (Order::Desc, NullOrdering::Last),
        };
        select = select.order_by_with_nulls(Target::expression(&entry.field), order, nulls);
        ordered.push(column);
    }
    for key in <Target::Table as EntityTrait>::PrimaryKey::iter() {
        let column = key.into_column();
        if !ordered
            .iter()
            .any(|existing| existing.as_str() == column.as_str())
        {
            select = select.order_by_with_nulls(
                Expr::col((Target::Table::default(), column)),
                Order::Asc,
                NullOrdering::Last,
            );
        }
    }
    select = select.limit(u64::from(page_size));
    if let Some(offset) = query.pagination.offset {
        select = select.offset(offset as u64);
    }
    Ok(select)
}

fn predicate<Target: PostgresQueryTarget>(
    value: Predicate<Target::Field>,
) -> Result<Condition, QueryCompileError> {
    match value {
        Predicate::And(children) => {
            let mut condition = Condition::all();
            if children.is_empty() {
                return Ok(condition.add(Expr::Constant(true.into())));
            }
            for child in children {
                condition = condition.add(predicate::<Target>(child)?);
            }
            Ok(condition)
        }
        Predicate::Or(children) => {
            let mut condition = Condition::any();
            if children.is_empty() {
                return Ok(condition.add(Expr::Constant(false.into())));
            }
            for child in children {
                condition = condition.add(predicate::<Target>(child)?);
            }
            Ok(condition)
        }
        Predicate::Not(child) => Ok(predicate::<Target>(*child)?.not()),
        Predicate::Field { field, operation } => {
            let column = Target::column(&field).def();
            operation::translate(
                Target::expression(&field),
                column.get_column_type(),
                operation,
            )
        }
    }
}
