use crate::persistent::ConnectionError;
use crate::user::User;
use async_trait::async_trait;
use modql::filter::{
    FilterNodes, ListOptions, OpValString, OpValsInt64, OpValsString, OrderBy, OrderBys,
};

use super::Queryable;
use validify::{ValidationError, Validify};

/// Indexed user filters. ID operators and email equality/membership are supported.
/// Validify trims and ASCII-lowercases email equality and membership values.
#[derive(Clone, Debug, Default, FilterNodes, Validify)]
#[modql(rel = "users")]
pub struct UserFilter {
    pub id: Option<OpValsInt64>,
    #[modify(custom(normalize_email_filter))]
    #[validate(custom(function = validate_email_filter))]
    pub email: Option<OpValsString>,
}

fn validate_email_filter(filter: &OpValsString) -> Result<(), ValidationError> {
    filter
        .0
        .iter()
        .all(|op| matches!(op, OpValString::Eq(_) | OpValString::In(_)))
        .then_some(())
        .ok_or_else(|| ValidationError::new_field("email_operator"))
}

fn normalize_email_filter(filter: &mut OpValsString) {
    for op in &mut filter.0 {
        match op {
            OpValString::Eq(value) => {
                *value = compallm_utils::email::normalize_email(value);
            }
            OpValString::In(values) => {
                for value in values {
                    *value = compallm_utils::email::normalize_email(value);
                }
            }
            _ => {}
        }
    }
}

#[derive(Clone, Debug, Validify)]
pub struct UserListOptions {
    #[validate(range(min = 0., max = 1000.))]
    pub limit: Option<i64>,
    #[validate(range(min = 0.))]
    pub offset: Option<i64>,
    #[validate(custom(function = validate_ordering))]
    pub order_bys: Option<OrderBys>,
}

fn validate_ordering(order_bys: &OrderBys) -> Result<(), ValidationError> {
    order_bys
        .into_iter()
        .all(|order| {
            let (OrderBy::Asc(column) | OrderBy::Desc(column)) = order;
            column == "id"
        })
        .then_some(())
        .ok_or_else(|| ValidationError::new_field("ordering"))
}

impl From<ListOptions> for UserListOptions {
    fn from(options: ListOptions) -> Self {
        Self {
            limit: Some(options.limit.unwrap_or(100)),
            offset: options.offset,
            order_bys: options.order_bys,
        }
    }
}

impl From<UserListOptions> for ListOptions {
    fn from(options: UserListOptions) -> Self {
        Self {
            limit: options.limit,
            offset: options.offset,
            order_bys: options.order_bys,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum UserRepositoryError {
    #[error(transparent)]
    Validation(#[from] validify::ValidationErrors),
    #[error("email is already registered")]
    EmailAlreadyExists,
    #[error("database operation failed")]
    Database(#[from] anyhow::Error),
}

impl UserRepositoryError {
    pub fn database(source: impl Into<anyhow::Error>) -> Self {
        Self::Database(source.into())
    }
}

impl From<ConnectionError> for UserRepositoryError {
    fn from(error: ConnectionError) -> Self {
        Self::database(error)
    }
}

/// Use the active `DbContext`. Never open or commit a separate transaction
/// in these operations.
#[async_trait]
pub trait UserRepository:
    Queryable<UserFilter, Entity = User, Error = UserRepositoryError>
{
    /// Enforce email uniqueness atomically; translate conflicts into
    /// EmailAlreadyExists. Set verification and creation timestamps on insert;
    /// the signup workflow verifies email ownership before calling this method.
    /// Persist a prepared domain User without normalizing or validating its fields again.
    async fn create(&self, user: User) -> Result<User, UserRepositoryError>;
}
