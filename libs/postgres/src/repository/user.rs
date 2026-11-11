use anyhow::Context;
use compallm_core::{
    persistent::DbContext,
    repository::{
        Queryable,
        user::{
            UserFilter, UserListOptions, UserRepository as CoreUserRepository, UserRepositoryError,
        },
    },
    user::{User, UserStatus},
};
use modql::filter::{FilterGroups, ListOptions, OpValString};
use sea_orm::{
    ActiveModelTrait, DbErr, EntityTrait, ModelTrait, QueryFilter, QueryTrait, RuntimeErr, Set,
    prelude::ChronoUtc,
    sea_query::{Condition, Expr, ExprTrait, Func, IntoColumnRef},
};
use validify::Validify;

use crate::{UserSchema, postgres_connection::PostgresConnection, schema::UserStatusSchema};

type Users = <UserSchema as ModelTrait>::Entity;
type UserColumn = <Users as EntityTrait>::Column;
type UserActiveModel = <Users as EntityTrait>::ActiveModel;

/// PostgreSQL user persistence. Every operation uses the caller's scoped context.
#[derive(Default)]
pub struct UserRepository;

impl UserRepository {
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

#[async_trait::async_trait]
impl Queryable<UserFilter> for UserRepository {
    type Entity = User;
    type Error = UserRepositoryError;

    async fn list(
        &self,
        mut query: UserFilter,
        options: ListOptions,
    ) -> Result<Vec<User>, UserRepositoryError> {
        query.validify()?;
        let email = query.email.take();
        let filters: FilterGroups = query.into();
        let mut condition = Condition::all().add(
            filters
                .into_sea_condition()
                .map_err(UserRepositoryError::database)?,
        );
        if let Some(email) = email {
            for op in email.0 {
                let column = Func::lower(Expr::col((Users::default(), UserColumn::Email)));
                let predicate = match op {
                    OpValString::Eq(value) => Condition::all().add(column.eq(value)),
                    OpValString::In(values) => Condition::all().add(column.is_in(values)),
                    op => op
                        .into_sea_cond_expr(
                            &(Users::default(), UserColumn::Email).into_column_ref(),
                            &Default::default(),
                        )
                        .map_err(UserRepositoryError::database)?,
                };
                condition = condition.add(predicate);
            }
        }
        let mut options = UserListOptions::from(options);
        options.validify()?;
        let options = ListOptions::from(options).append_order_by("id");
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        let mut query = Users::find().filter(condition);
        options.apply_to_sea_query(QueryTrait::query(&mut query));
        query
            .all(db)
            .await
            .map_err(UserRepositoryError::database)?
            .into_iter()
            .map(User::try_from)
            .collect()
    }
}

#[async_trait::async_trait]
impl CoreUserRepository for UserRepository {
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
