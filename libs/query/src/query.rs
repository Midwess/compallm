use crate::{OrderBy, Pagination, operation::Operation};
use std::convert::Infallible;

/// Identifies a resource and its allowlisted fields without database metadata.
pub trait QueryTarget {
    type Field;
}

#[derive(Clone, Debug, PartialEq)]
pub enum Predicate<Field> {
    And(Vec<Self>),
    Or(Vec<Self>),
    Not(Box<Self>),
    Field { field: Field, operation: Operation },
}

impl<Field> Default for Predicate<Field> {
    fn default() -> Self {
        Self::And(Vec::new())
    }
}

/// A read description. Authority and database connections are supplied separately.
pub struct Query<Target: QueryTarget> {
    pub predicate: Predicate<Target::Field>,
    pub order_by: Vec<OrderBy<Target::Field>>,
    pub pagination: Pagination,
}

impl<Target: QueryTarget> Default for Query<Target> {
    fn default() -> Self {
        Self {
            predicate: Predicate::default(),
            order_by: Vec::new(),
            pagination: Pagination::default(),
        }
    }
}

impl<Target: QueryTarget> Query<Target> {
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds an AND predicate without changing existing Boolean group boundaries.
    pub fn filter(mut self, field: Target::Field, operation: Operation) -> Self {
        let predicate = Predicate::Field { field, operation };
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

/// Converts typed inputs into a resource-bound read description, never into SQL.
pub trait IntoQuery {
    type Target: QueryTarget;
    type Error;

    fn into_query(self) -> Result<Query<Self::Target>, Self::Error>;
}

impl<Target: QueryTarget> IntoQuery for Query<Target> {
    type Target = Target;
    type Error = Infallible;

    fn into_query(self) -> Result<Self, Self::Error> {
        Ok(self)
    }
}
