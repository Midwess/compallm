use anyhow::Context;
use compallm_core::{
    persistent::{
        DbContext,
        user::{UserRepository as CoreUserRepository, UserRepositoryError},
    },
    user::{User, UserStatus},
};
use sea_orm::{
    DbErr, EntityTrait, ModelTrait, QueryFilter, RuntimeErr, Set,
    prelude::ChronoUtc,
    sea_query::{Expr, ExprTrait, Func, OnConflict},
};

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
        {
            match database.constraint() {
                Some("users_email_unique") => return UserRepositoryError::EmailAlreadyExists,
                Some("users_username_unique") => return UserRepositoryError::UsernameAlreadyExists,
                _ => {}
            }
        }
        UserRepositoryError::database(error)
    }
}

#[async_trait::async_trait]
impl CoreUserRepository for UserRepository {
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, UserRepositoryError> {
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        Users::find()
            .filter(Func::lower(Expr::col(UserColumn::Email)).eq(email))
            .one(db)
            .await
            .map_err(UserRepositoryError::database)?
            .map(User::try_from)
            .transpose()
    }

    async fn find_by_id(&self, user_id: u64) -> Result<Option<User>, UserRepositoryError> {
        let id = Self::database_id(user_id)?;
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        Users::find_by_id(id)
            .one(db)
            .await
            .map_err(UserRepositoryError::database)?
            .map(User::try_from)
            .transpose()
    }

    async fn create(&self, user: User) -> Result<User, UserRepositoryError> {
        let id = Self::database_id(user.id)?;
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        let model = UserActiveModel {
            id: Set(id),
            email: Set(user.email),
            display_name: Set(user.display_name),
            username: Set(user.username),
            avatar_path: Set(user.avatar_path),
            status: Set(match user.status {
                UserStatus::Active => UserStatusSchema::Active,
                UserStatus::Disabled => UserStatusSchema::Disabled,
            }),
            email_verified_at: Set(ChronoUtc::now().fixed_offset()),
            ..Default::default()
        };
        let inserted = Users::insert(model)
            .on_conflict(
                OnConflict::column(UserColumn::Username)
                    .do_nothing()
                    .to_owned(),
            )
            .exec_with_returning(db)
            .await
            .map_err(|error| match error {
                DbErr::RecordNotInserted => UserRepositoryError::UsernameAlreadyExists,
                other => Self::insert_error(other),
            })?;
        User::try_from(inserted)
    }
    async fn update_profile(
        &self,
        user_id: u64,
        display_name: Option<String>,
        username: Option<String>,
        avatar_path: Option<String>,
    ) -> Result<User, UserRepositoryError> {
        let id = Self::database_id(user_id)?;
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        let mut update = Users::update_many()
            .filter(Expr::col(UserColumn::Id).eq(id))
            .filter(Expr::col(UserColumn::Status).eq(UserStatusSchema::Active));
        if let Some(name) = display_name {
            update = update.col_expr(UserColumn::DisplayName, Expr::val(name));
        }
        if let Some(username) = username {
            update = update.col_expr(UserColumn::Username, Expr::val(username));
        }
        if let Some(path) = avatar_path {
            update = update.col_expr(UserColumn::AvatarPath, Expr::val(path));
        }
        let model = update
            .exec_with_returning(db)
            .await
            .map_err(Self::insert_error)?
            .into_iter()
            .next()
            .ok_or(UserRepositoryError::NotFound)?;
        User::try_from(model)
    }
}
