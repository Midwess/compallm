use super::{QueryCompileError, column, operation, predicate};
use compallm_query::{
    Connection, Cursor, Edge, OrderBy, OrderByDirection, PageInfo, Pagination, Query,
};
use sea_orm::{
    ColumnTrait, ConnectionTrait, EntityTrait, IdenStatic, Iterable, ModelTrait,
    PrimaryKeyToColumn, QueryFilter, QueryOrder, QuerySelect, QueryTrait, Select,
    sea_query::{Condition, Expr, ExprTrait, NullOrdering, Order, Value},
};
use sha2::{Digest, Sha256};

pub struct PagePlan<Table: EntityTrait> {
    base: Select<Table>,
    order: Vec<(Table::Column, OrderByDirection)>,
    pagination: Pagination,
    size: u32,
    context: String,
    boundary: Option<Vec<Option<String>>>,
}

impl<Table: EntityTrait> PagePlan<Table> {
    pub(super) fn new(
        query: Query,
        select: Select<Table>,
        pagination: Pagination,
        ordering: Vec<OrderBy>,
        max_rows: u32,
        scope: &str,
    ) -> Result<Self, QueryCompileError> {
        let size = pagination.page_size(max_rows)?;
        let mut order = Vec::new();
        let mut fields = ordering.clone();
        for entry in &fields {
            let col = column::<Table>(&entry.field)?;
            if order
                .iter()
                .any(|(existing, _): &(Table::Column, _)| existing.as_str() == col.as_str())
            {
                return Err(QueryCompileError::DuplicateOrder);
            }
            order.push((col, entry.direction));
        }
        let keys: Vec<_> = Table::PrimaryKey::iter().collect();
        if keys.is_empty() {
            return Err(QueryCompileError::MissingPrimaryKey);
        }
        for key in keys {
            let col = key.into_column();
            if !order
                .iter()
                .any(|(existing, _)| existing.as_str() == col.as_str())
            {
                order.push((col, OrderByDirection::AscNullsLast));
                fields.push(OrderBy {
                    field: col.as_str().into(),
                    direction: OrderByDirection::AscNullsLast,
                });
            }
        }
        for (col, _) in &order {
            operation::cast(String::new(), col.def().get_column_type())?;
        }
        let table = Table::default();
        let binding = serde_json::to_vec(&(
            table.schema_name(),
            table.table_name(),
            scope,
            &query.predicate,
            fields,
        ))
        .map_err(|_| compallm_query::CursorError::Invalid)?;
        let context = format!("{:x}", Sha256::digest(binding));
        let boundary = pagination
            .after
            .as_ref()
            .or(pagination.before.as_ref())
            .map(|cursor| cursor.decode(&context))
            .transpose()?;
        if boundary
            .as_ref()
            .is_some_and(|values| values.len() != order.len())
        {
            return Err(compallm_query::CursorError::Invalid.into());
        }
        let mut base = select
            .filter(predicate::<Table>(query.predicate)?)
            .limit(None)
            .offset(None);
        QueryTrait::query(&mut base).clear_order_by();
        Ok(Self {
            base,
            order,
            pagination,
            size,
            context,
            boundary,
        })
    }

    fn backward(&self) -> bool {
        self.pagination.last.is_some() || self.pagination.before.is_some()
    }

    /// Returns the database-order query, including one overflow row.
    /// Use fetch to trim it, restore output order, and build connection metadata.
    pub fn select(&self) -> Result<Select<Table>, QueryCompileError> {
        let mut select = self.base.clone();
        if let Some(values) = &self.boundary {
            select = select.filter(self.seek(values, !self.backward())?);
        }
        for &(col, direction) in &self.order {
            let (asc, first) = direction_flags(direction);
            let reverse = self.backward();
            select = select.order_by_with_nulls(
                Expr::col((Table::default(), col)),
                if asc != reverse {
                    Order::Asc
                } else {
                    Order::Desc
                },
                if first != reverse {
                    NullOrdering::First
                } else {
                    NullOrdering::Last
                },
            );
        }
        Ok(select
            .limit(u64::from(self.size) + 1)
            .offset(self.pagination.offset.map(|v| v as u64)))
    }

    fn seek(&self, values: &[Option<String>], after: bool) -> Result<Condition, QueryCompileError> {
        let mut branches = Condition::any();
        let mut prefix = Condition::all();
        for (&(col, direction), value) in self.order.iter().zip(values) {
            let expr = Expr::col((Table::default(), col));
            let (asc, first) = direction_flags(direction);
            let nulls_after = first != after;
            let (comparison, equality) = match value {
                None => (
                    if nulls_after {
                        Condition::any().add(Expr::Constant(false.into()))
                    } else {
                        Condition::all().add(expr.clone().is_not_null())
                    },
                    expr.is_null(),
                ),
                Some(value) => {
                    let bound = operation::cast(value.clone(), col.def().get_column_type())?;
                    let cmp = if asc == after {
                        expr.clone().gt(bound.clone())
                    } else {
                        expr.clone().lt(bound.clone())
                    };
                    let mut cmp = Condition::any().add(cmp);
                    if nulls_after {
                        cmp = cmp.add(expr.clone().is_null());
                    }
                    (cmp, expr.eq(bound))
                }
            };
            branches = branches.add(prefix.clone().add(comparison));
            prefix = prefix.add(equality);
        }
        Ok(branches)
    }

    fn values(&self, node: &Table::Model) -> Result<Vec<Option<String>>, QueryCompileError> {
        self.order
            .iter()
            .map(|(col, _)| CursorValue::try_from(node.get(*col)).map(|value| value.0))
            .collect()
    }

    async fn exists<C: ConnectionTrait>(
        &self,
        db: &C,
        values: &[Option<String>],
        after: bool,
    ) -> Result<bool, QueryCompileError> {
        Ok(self
            .base
            .clone()
            .filter(self.seek(values, after)?)
            .limit(1)
            .one(db)
            .await?
            .is_some())
    }

    /// The base selection must return each entity once and retain every ordering column.
    /// Scope identifies verified authority and trusted predicates omitted from the client query.
    pub async fn fetch<C: ConnectionTrait>(
        self,
        db: &C,
    ) -> Result<Connection<Table::Model>, QueryCompileError> {
        let mut nodes = self.select()?.all(db).await?;
        let overflow = nodes.len() > self.size as usize;
        nodes.truncate(self.size as usize);
        if self.backward() {
            nodes.reverse();
        }
        let mut edges = Vec::with_capacity(nodes.len());
        for node in nodes {
            let cursor = Cursor::encode(self.values(&node)?, &self.context)?.0;
            edges.push(Edge { cursor, node });
        }
        let (has_next_page, has_previous_page) =
            if let (Some(first), Some(last)) = (edges.first(), edges.last()) {
                if self.backward() {
                    (
                        self.boundary.is_some()
                            && self.exists(db, &self.values(&last.node)?, true).await?,
                        overflow,
                    )
                } else {
                    (
                        overflow,
                        (self.boundary.is_some() || self.pagination.offset.unwrap_or_default() > 0)
                            && self.exists(db, &self.values(&first.node)?, false).await?,
                    )
                }
            } else if let Some(boundary) = &self.boundary {
                let side = self.exists(db, boundary, self.backward()).await?;
                let equal = self
                    .base
                    .clone()
                    .filter(self.equal(boundary)?)
                    .limit(1)
                    .one(db)
                    .await?
                    .is_some();
                let skipped = self.pagination.offset.unwrap_or_default() > 0
                    && self.exists(db, boundary, true).await?;
                if self.backward() {
                    (side || equal, overflow)
                } else {
                    (overflow, side || equal || skipped)
                }
            } else {
                let skipped = self.pagination.offset.unwrap_or_default() > 0
                    && self.base.clone().limit(1).one(db).await?.is_some();
                if self.backward() {
                    (false, overflow)
                } else {
                    (overflow, skipped)
                }
            };
        Ok(Connection {
            page_info: PageInfo {
                start_cursor: edges.first().map(|e| e.cursor.clone()),
                end_cursor: edges.last().map(|e| e.cursor.clone()),
                has_next_page,
                has_previous_page,
            },
            edges,
        })
    }

    fn equal(&self, values: &[Option<String>]) -> Result<Condition, QueryCompileError> {
        let mut condition = Condition::all();
        for ((col, _), value) in self.order.iter().zip(values) {
            let expr = Expr::col((Table::default(), *col));
            condition = condition.add(match value {
                None => expr.is_null(),
                Some(value) => {
                    expr.eq(operation::cast(value.clone(), col.def().get_column_type())?)
                }
            });
        }
        Ok(condition)
    }
}

fn direction_flags(direction: OrderByDirection) -> (bool, bool) {
    match direction {
        OrderByDirection::AscNullsFirst => (true, true),
        OrderByDirection::AscNullsLast => (true, false),
        OrderByDirection::DescNullsFirst => (false, true),
        OrderByDirection::DescNullsLast => (false, false),
    }
}

struct CursorValue(Option<String>);

impl TryFrom<Value> for CursorValue {
    type Error = QueryCompileError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        if value == value.as_null() {
            return Ok(Self(None));
        }
        let text = match value {
            Value::Bool(Some(v)) => v.to_string(),
            Value::TinyInt(Some(v)) => v.to_string(),
            Value::SmallInt(Some(v)) => v.to_string(),
            Value::Int(Some(v)) => v.to_string(),
            Value::BigInt(Some(v)) => v.to_string(),
            Value::Float(Some(v)) => v.to_string(),
            Value::Double(Some(v)) => v.to_string(),
            Value::String(Some(v)) => v,
            Value::Char(Some(v)) => v.to_string(),
            Value::Enum(sea_orm::sea_query::OptionEnum::Some(v)) => v.value.into_owned(),
            Value::Uuid(Some(v)) => v.to_string(),
            Value::Decimal(Some(v)) => v.to_string(),
            Value::ChronoDate(Some(v)) => v.to_string(),
            Value::ChronoTime(Some(v)) => v.to_string(),
            Value::ChronoDateTime(Some(v)) => v.to_string(),
            Value::ChronoDateTimeUtc(Some(v)) => v.to_rfc3339(),
            Value::ChronoDateTimeLocal(Some(v)) => v.to_rfc3339(),
            Value::ChronoDateTimeWithTimeZone(Some(v)) => v.to_rfc3339(),
            _ => return Err(QueryCompileError::UnsupportedScalar),
        };
        Ok(Self(Some(text)))
    }
}
