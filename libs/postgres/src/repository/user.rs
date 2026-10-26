use anyhow::Context;
use compallm_core::{
    persistent::{
        DbContext,
        user::{UserRepository as CoreUserRepository, UserRepositoryError},
    },
    user::{User, UserStatus},
};
use compallm_utils::email::normalize_email;
use sea_orm::{
    ActiveModelTrait, DbErr, EntityTrait, ModelTrait, QueryFilter, RuntimeErr, Set,
    prelude::ChronoUtc,
    sea_query::{Expr, ExprTrait, Func},
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
        if id == 0 {
            return Err(UserRepositoryError::database(anyhow::anyhow!(
                "user ID must be positive"
            )));
        }
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
impl CoreUserRepository for UserRepository {
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, UserRepositoryError> {
        let email = normalize_email(email).context("invalid user email")?;
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
        let email = normalize_email(&user.email).context("invalid user email")?;
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        let model = UserActiveModel {
            id: Set(id),
            email: Set(email),
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
