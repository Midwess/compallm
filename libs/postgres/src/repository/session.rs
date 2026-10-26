use crate::{UserSessionSchema, postgres_connection::PostgresConnection};
use anyhow::Context;
use compallm_core::{
    auth::Session,
    persistent::{
        DbContext,
        session::{SessionRepository as CoreSessionRepository, SessionRepositoryError},
    },
};
use sea_orm::{
    ColumnTrait, EntityTrait, ModelTrait, QueryFilter, QuerySelect, Set,
    prelude::{DateTimeUtc, DateTimeWithTimeZone},
    sea_query::{Expr, ExprTrait},
};

type Sessions = <UserSessionSchema as ModelTrait>::Entity;
type SessionColumn = <Sessions as EntityTrait>::Column;
type SessionActiveModel = <Sessions as EntityTrait>::ActiveModel;

#[derive(Default)]
pub struct SessionRepository;

impl SessionRepository {
    fn id(id: u64) -> Result<i64, SessionRepositoryError> {
        if id == 0 {
            return Err(anyhow::anyhow!("session and user IDs must be positive").into());
        }
        Ok(crate::to_bigint(id).context("invalid session or user ID")?)
    }

    fn timestamp(at: u64) -> Result<DateTimeWithTimeZone, SessionRepositoryError> {
        let at = i64::try_from(at).context("timestamp exceeds PostgreSQL range")?;
        Ok(DateTimeUtc::from_timestamp(at, 0)
            .context("invalid session timestamp")?
            .fixed_offset())
    }
}

#[async_trait::async_trait]
impl CoreSessionRepository for SessionRepository {
    async fn create(&self, session: Session) -> Result<(), SessionRepositoryError> {
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        let model = SessionActiveModel {
            id: Set(Self::id(session.id)?),
            user_id: Set(Self::id(session.user_id)?),
            refresh_token_id: Set(session.refresh_token_id),
            created_at: Set(Self::timestamp(session.created_at)?),
            last_used_at: Set(Self::timestamp(session.last_used_at)?),
            expires_at: Set(Self::timestamp(session.expires_at)?),
            revoked_at: Set(session.revoked_at.map(Self::timestamp).transpose()?),
        };
        let rows = Sessions::insert(model)
            .exec_without_returning(db)
            .await
            .map_err(anyhow::Error::from)?;
        if rows != 1 {
            return Err(
                anyhow::anyhow!("session insert affected an unexpected number of rows").into(),
            );
        }
        Ok(())
    }

    async fn find_for_update(
        &self,
        id: u64,
        user_id: u64,
    ) -> Result<Option<Session>, SessionRepositoryError> {
        let context = DbContext::current()?;
        if !context.postgres().is_transaction() {
            return Err(anyhow::anyhow!("session row locks require an active transaction").into());
        }
        let db = PostgresConnection::from_context(&context)?;
        Sessions::find_by_id(Self::id(id)?)
            .filter(SessionColumn::UserId.eq(Self::id(user_id)?))
            .lock_exclusive()
            .one(db)
            .await
            .map_err(anyhow::Error::from)?
            .map(Session::try_from)
            .transpose()
    }

    async fn rotate(
        &self,
        session: &Session,
        expected_token_id: &str,
    ) -> Result<bool, SessionRepositoryError> {
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        let rows = Sessions::update_many()
            .col_expr(
                SessionColumn::RefreshTokenId,
                Expr::value(session.refresh_token_id.clone()),
            )
            .col_expr(
                SessionColumn::LastUsedAt,
                Expr::value(Self::timestamp(session.last_used_at)?),
            )
            .col_expr(
                SessionColumn::ExpiresAt,
                Expr::value(Self::timestamp(session.expires_at)?),
            )
            .filter(SessionColumn::Id.eq(Self::id(session.id)?))
            .filter(SessionColumn::UserId.eq(Self::id(session.user_id)?))
            .filter(SessionColumn::RefreshTokenId.eq(expected_token_id))
            .filter(SessionColumn::RevokedAt.is_null())
            .filter(Expr::col(SessionColumn::ExpiresAt).gt(Expr::current_timestamp()))
            .exec(db)
            .await
            .map_err(anyhow::Error::from)?;
        Ok(rows.rows_affected == 1)
    }

    async fn revoke(&self, id: u64, user_id: u64, at: u64) -> Result<(), SessionRepositoryError> {
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        Sessions::update_many()
            .col_expr(SessionColumn::RevokedAt, Expr::value(Self::timestamp(at)?))
            .filter(SessionColumn::Id.eq(Self::id(id)?))
            .filter(SessionColumn::UserId.eq(Self::id(user_id)?))
            .filter(SessionColumn::RevokedAt.is_null())
            .exec(db)
            .await
            .map_err(anyhow::Error::from)?;
        Ok(())
    }
}
