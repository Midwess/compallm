use super::query::IntoQuery;
use anyhow::Context;
use compallm_core::repository::ListOptions;
use compallm_core::{
    persistent::DbContext,
    repository::user::{UserQuery, UserRepository as CoreUserRepository, UserRepositoryError},
    user::{User, UserStatus},
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, ModelTrait, QueryFilter, QueryOrder,
    QueryTrait, RuntimeErr, Select, Set,
    prelude::ChronoUtc,
    sea_query::{Condition, Expr, ExprTrait, Func},
};

use crate::{UserSchema, postgres_connection::PostgresConnection, schema::UserStatusSchema};

type Users = <UserSchema as ModelTrait>::Entity;
type UserActiveModel = <Users as EntityTrait>::ActiveModel;

/// PostgreSQL user persistence. Every operation uses the caller's scoped context.
#[derive(Default)]
pub struct UserRepository;

impl UserRepository {
    pub async fn list<Q>(
        &self,
        query: Q,
        mut options: ListOptions,
    ) -> Result<Vec<User>, UserRepositoryError>
    where
        Q: IntoQuery<Table = Users> + Send,
    {
        let limit = *options.limit.get_or_insert(100);
        let invalid_ordering = options.order_bys.as_ref().is_some_and(|orders| {
            orders.into_iter().any(|order| {
                let (compallm_core::repository::OrderBy::Asc(column)
                | compallm_core::repository::OrderBy::Desc(column)) = order;
                column != "id"
            })
        });
        if !(0..=1000).contains(&limit)
            || options.offset.is_some_and(|offset| offset < 0)
            || invalid_ordering
        {
            return Err(UserRepositoryError::database(anyhow::anyhow!(
                "unsupported user list options"
            )));
        }
        let mut query = query.into_query();
        options.apply_to_sea_query(QueryTrait::query(&mut query));
        query = query.order_by_asc(<Users as EntityTrait>::Column::Id);
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        query
            .all(db)
            .await
            .map_err(UserRepositoryError::database)?
            .into_iter()
            .map(User::try_from)
            .collect()
    }

    pub async fn one<Q>(
        &self,
        query: Q,
        options: ListOptions,
    ) -> Result<Option<User>, UserRepositoryError>
    where
        Q: IntoQuery<Table = Users> + Send,
    {
        let users = self.list(query, options.with_limit(1)).await?;
        Ok(users.into_iter().next())
    }

    fn database_id(id: u64) -> Result<i64, UserRepositoryError> {
        crate::to_bigint(id)
            .context("user ID exceeds PostgreSQL BIGINT")
            .map_err(Into::into)
    }

    fn insert_error(error: DbErr) -> UserRepositoryError {
        if let DbErr::Exec(RuntimeErr::SqlxError(driver))
        | DbErr::Query(RuntimeErr::SqlxError(driver)) = &error
            && let sea_orm::sqlx::Error::Database(database) = driver.as_ref()
            && database.is_unique_violation()
            && database.constraint() == Some("users_email_unique")
        {
            return UserRepositoryError::EmailAlreadyExists;
        }
        UserRepositoryError::database(error)
    }
}

impl IntoQuery for UserQuery {
    type Table = Users;

    fn into_query(self) -> Select<Users> {
        type Column = <Users as EntityTrait>::Column;
        let email_column = || Func::lower(Expr::col((Users::default(), Column::Email)));
        Users::find().filter(
            Condition::all()
                .add_option(self.id.map(|id| Column::Id.eq(id)))
                .add_option(self.ids.map(|ids| Column::Id.is_in(ids)))
                .add_option(
                    self.email.map(|email| {
                        email_column().eq(compallm_utils::email::normalize_email(&email))
                    }),
                )
                .add_option(self.emails.map(|emails| {
                    email_column().is_in(
                        emails
                            .into_iter()
                            .map(|email| compallm_utils::email::normalize_email(&email)),
                    )
                })),
        )
    }
}

#[async_trait::async_trait]
impl CoreUserRepository for UserRepository {
    async fn list(
        &self,
        query: UserQuery,
        options: ListOptions,
    ) -> Result<Vec<User>, UserRepositoryError> {
        UserRepository::list(self, query, options).await
    }

    async fn create(&self, user: User) -> Result<User, UserRepositoryError> {
        let id = Self::database_id(user.id)?;
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        let model = UserActiveModel {
            id: Set(id),
            email: Set(user.email),
            display_name: Set(user.display_name),
            status: Set(match user.status {
                UserStatus::Active => UserStatusSchema::Active,
                UserStatus::Disabled => UserStatusSchema::Disabled,
            }),
            email_verified_at: Set(ChronoUtc::now().fixed_offset()),
            ..Default::default()
        };
        User::try_from(model.insert(db).await.map_err(Self::insert_error)?)
    }
}
