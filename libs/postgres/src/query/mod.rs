mod operation;
mod page;
pub use page::PagePlan;

use compallm_query::{IntoQuery, OrderBy, OrderByDirection, Pagination, Predicate, Query};
use sea_orm::{
    ColumnTrait, EntityTrait, IdenStatic, Iterable, PrimaryKeyToColumn, QueryFilter, QueryOrder,
    QuerySelect, Select,
    sea_query::{Condition, Expr, NullOrdering, Order},
};

#[derive(Debug, thiserror::Error)]
pub enum QueryCompileError {
    #[error(transparent)]
    Cursor(#[from] compallm_query::CursorError),
    #[error(transparent)]
    Database(#[from] sea_orm::DbErr),
    #[error("pagination requires a primary key and unique base-entity rows")]
    MissingPrimaryKey,
    #[error("duplicate ordering field")]
    DuplicateOrder,
    #[error("query field does not match a column of this table")]
    UnknownField,
    #[error(transparent)]
    Pagination(#[from] compallm_query::PaginationError),
    #[error("cursor and backward pagination require a pagination planner")]
    UnsupportedPagination,
    #[error("operation does not match its bound database column")]
    TypeMismatch,
    #[error("database scalar type is unsupported by the query translator")]
    UnsupportedScalar,
}

pub trait IntoSeaQuery: IntoQuery {
    /// Builds a connection page from a unique, unpaginated entity selection.
    /// Scope must identify verified authority and all trusted collection constraints.
    fn plan_page<Table: EntityTrait>(
        self: Box<Self>,
        select: Select<Table>,
        pagination: Pagination,
        ordering: Vec<OrderBy>,
        max_rows: u32,
        scope: &str,
    ) -> Result<PagePlan<Table>, QueryCompileError> {
        PagePlan::new(
            self.into_query(),
            select,
            pagination,
            ordering,
            max_rows,
            scope,
        )
    }

    fn into_sea_query<Table: EntityTrait>(
        self: Box<Self>,
        pagination: Pagination,
        ordering: Vec<OrderBy>,
    ) -> Result<Select<Table>, QueryCompileError> {
        self.apply_to_sea_query(
            Table::find(),
            pagination,
            ordering,
            Pagination::DEFAULT_MAX_ROWS,
        )
    }

    fn apply_to_sea_query<Table: EntityTrait>(
        self: Box<Self>,
        select: Select<Table>,
        pagination: Pagination,
        ordering: Vec<OrderBy>,
        max_rows: u32,
    ) -> Result<Select<Table>, QueryCompileError> {
        compile(self.into_query(), select, pagination, ordering, max_rows)
    }
}

impl<Q: IntoQuery + ?Sized> IntoSeaQuery for Q {}

fn compile<Table: EntityTrait>(
    query: Query,
    select: Select<Table>,
    pagination: Pagination,
    ordering: Vec<OrderBy>,
    max_rows: u32,
) -> Result<Select<Table>, QueryCompileError> {
    let page_size = pagination.page_size(max_rows)?;
    if pagination.last.is_some() || pagination.before.is_some() || pagination.after.is_some() {
        return Err(QueryCompileError::UnsupportedPagination);
    }
    let mut select = select.filter(predicate::<Table>(query.predicate)?);
    let mut ordered = Vec::new();
    for entry in ordering {
        let column = column::<Table>(&entry.field)?;
        let (order, nulls) = match entry.direction {
            OrderByDirection::AscNullsFirst => (Order::Asc, NullOrdering::First),
            OrderByDirection::AscNullsLast => (Order::Asc, NullOrdering::Last),
            OrderByDirection::DescNullsFirst => (Order::Desc, NullOrdering::First),
            OrderByDirection::DescNullsLast => (Order::Desc, NullOrdering::Last),
        };
        select = select.order_by_with_nulls(Expr::col((Table::default(), column)), order, nulls);
        ordered.push(column);
    }
    for key in <Table as EntityTrait>::PrimaryKey::iter() {
        let column = key.into_column();
        if !ordered
            .iter()
            .any(|existing| existing.as_str() == column.as_str())
        {
            select = select.order_by_with_nulls(
                Expr::col((Table::default(), column)),
                Order::Asc,
                NullOrdering::Last,
            );
        }
    }
    select = select.limit(u64::from(page_size));
    if let Some(offset) = pagination.offset {
        select = select.offset(offset as u64);
    }
    Ok(select)
}

fn predicate<Table: EntityTrait>(value: Predicate) -> Result<Condition, QueryCompileError> {
    match value {
        Predicate::And(children) => {
            let mut condition = Condition::all();
            if children.is_empty() {
                return Ok(condition.add(Expr::Constant(true.into())));
            }
            for child in children {
                condition = condition.add(predicate::<Table>(child)?);
            }
            Ok(condition)
        }
        Predicate::Or(children) => {
            let mut condition = Condition::any();
            if children.is_empty() {
                return Ok(condition.add(Expr::Constant(false.into())));
            }
            for child in children {
                condition = condition.add(predicate::<Table>(child)?);
            }
            Ok(condition)
        }
        Predicate::Not(child) => Ok(predicate::<Table>(*child)?.not()),
        Predicate::Field { field, operation } => {
            let column = column::<Table>(&field)?;
            operation::translate(
                Expr::col((Table::default(), column)),
                column.def().get_column_type(),
                operation,
            )
        }
    }
}

fn column<Table: EntityTrait>(name: &str) -> Result<Table::Column, QueryCompileError> {
    Table::Column::iter()
        .find(|column| column.as_str() == name)
        .ok_or(QueryCompileError::UnknownField)
}
