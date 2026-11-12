use crate::persistent::ConnectionError;
use crate::user::User;
use async_trait::async_trait;
use modql::filter::{ListOptions, OrderBy, OrderBys};

use validify::{ValidationError, Validify};

#[derive(Clone, Debug, Default, Validify)]
pub struct UserQuery {
    pub id: Option<i64>,
    pub ids: Option<Vec<i64>>,
    #[modify(trim, custom(str::make_ascii_lowercase))]
    pub email: Option<String>,
    #[modify(custom(normalize_emails))]
    pub emails: Option<Vec<String>>,
}

fn normalize_emails(values: &mut [String]) {
    for value in values {
        *value = compallm_utils::email::normalize_email(value);
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
pub trait UserRepository: Send + Sync {
    async fn list(
        &self,
        query: UserQuery,
        options: ListOptions,
    ) -> Result<Vec<User>, UserRepositoryError>;

    async fn one(
        &self,
        query: UserQuery,
        options: ListOptions,
    ) -> Result<Option<User>, UserRepositoryError> {
        let users = self.list(query, options.with_limit(1)).await?;
        Ok(users.into_iter().next())
    }

    /// Enforce email uniqueness atomically; translate conflicts into
    /// EmailAlreadyExists. Set verification and creation timestamps on insert;
    /// the signup workflow verifies email ownership before calling this method.
    /// Persist a prepared domain User without normalizing or validating its fields again.
    async fn create(&self, user: User) -> Result<User, UserRepositoryError>;
}
