use crate::operation::Operation;
use serde::{Deserialize, Serialize};

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

/// Filtering predicates. Pagination, ordering, authority, and connections are supplied separately.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Query {
    pub predicate: Predicate,
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

impl Query {
    pub fn into_predicate(self) -> Predicate {
        self.predicate
    }
}

/// Consumes a typed filter into database-independent predicates.
pub trait IntoQuery: Send {
    fn into_query(self: Box<Self>) -> Query;
}

impl IntoQuery for Query {
    fn into_query(self: Box<Self>) -> Query {
        *self
    }
}

impl<Q: IntoQuery + ?Sized> IntoQuery for Box<Q> {
    fn into_query(self: Box<Self>) -> Query {
        (*self).into_query()
    }
}
