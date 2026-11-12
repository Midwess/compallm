use crate::{
    OrderBy, Pagination, operation::Operation, order::OrderByError, pagination::PaginationError,
};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Predicate {
    And(Vec<Self>),
    Or(Vec<Self>),
    Not(Box<Self>),
    Field { field: String, operation: Operation },
}

impl Default for Predicate {
    fn default() -> Self {
        Self::And(Vec::new())
    }
}

/// A read description. Authority and database connections are supplied separately.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Query {
    pub predicate: Predicate,
    pub order_by: Vec<OrderBy>,
    pub pagination: Pagination,
}

impl Query {
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds an AND predicate without changing existing Boolean group boundaries.
    pub fn filter(mut self, field: impl Into<String>, operation: Operation) -> Self {
        let predicate = Predicate::Field {
            field: field.into(),
            operation,
        };
        self.predicate = match self.predicate {
            Predicate::And(mut predicates) => {
                predicates.push(predicate);
                Predicate::And(predicates)
            }
            existing => Predicate::And(vec![existing, predicate]),
        };
        self
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum QueryError {
    #[error("nested filters cannot carry ordering or pagination")]
    NestedQueryOptions,
    #[error(transparent)]
    OrderBy(#[from] OrderByError),
    #[error(transparent)]
    Pagination(#[from] PaginationError),
}

impl From<Infallible> for QueryError {
    fn from(value: Infallible) -> Self {
        match value {}
    }
}

impl Query {
    pub fn into_predicate(self) -> Result<Predicate, QueryError> {
        if !self.order_by.is_empty() || self.pagination != Pagination::default() {
            return Err(QueryError::NestedQueryOptions);
        }
        Ok(self.predicate)
    }
}

/// Consumes a typed request into a database-independent read description.
pub trait IntoQuery: Send {
    fn into_query(self: Box<Self>) -> Result<Query, QueryError>;
}

impl IntoQuery for Query {
    fn into_query(self: Box<Self>) -> Result<Query, QueryError> {
        Ok(*self)
    }
}

impl<Q: IntoQuery + ?Sized> IntoQuery for Box<Q> {
    fn into_query(self: Box<Self>) -> Result<Query, QueryError> {
        (*self).into_query()
    }
}
