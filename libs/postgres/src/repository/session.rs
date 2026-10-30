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
    ColumnTrait, EntityTrait, ModelTrait, QueryFilter, Set,
    prelude::{DateTimeUtc, DateTimeWithTimeZone},
    sea_query::Expr,
};

type Sessions = <UserSessionSchema as ModelTrait>::Entity;
type SessionColumn = <Sessions as EntityTrait>::Column;
type SessionActiveModel = <Sessions as EntityTrait>::ActiveModel;

#[derive(Default)]
pub struct SessionRepository;

impl SessionRepository {
    fn id(id: u64) -> Result<i64, SessionRepositoryError> {
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
            organization_id: Set(Self::id(session.organization_id)?),
            revoked_at: Set(session.revoked_at.map(Self::timestamp).transpose()?),
        };
        Sessions::insert(model)
            .exec_without_returning(db)
            .await
            .map_err(anyhow::Error::from)?;
        Ok(())
    }

    async fn find(&self, session: &Session) -> Result<Option<Session>, SessionRepositoryError> {
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        Sessions::find_by_id(Self::id(session.id)?)
            .filter(SessionColumn::UserId.eq(Self::id(session.user_id)?))
            .filter(SessionColumn::OrganizationId.eq(Self::id(session.organization_id)?))
            .one(db)
            .await
            .map_err(anyhow::Error::from)?
            .map(Session::try_from)
            .transpose()
    }

    async fn update(
        &self,
        session: &Session,
        previous: &Session,
    ) -> Result<bool, SessionRepositoryError> {
        if session.id != previous.id
            || session.user_id != previous.user_id
            || (previous.revoked_at.is_some() && session.revoked_at != previous.revoked_at)
        {
            return Err(
                anyhow::anyhow!("session identity and existing revocation cannot change").into(),
            );
        }
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        let mut update = Sessions::update_many()
            .filter(SessionColumn::Id.eq(Self::id(previous.id)?))
            .filter(SessionColumn::UserId.eq(Self::id(previous.user_id)?))
            .filter(SessionColumn::OrganizationId.eq(Self::id(previous.organization_id)?));

        // Write only changed fields. Revocation must not overwrite a concurrent
        // organization switch, and switching must not erase revocation.
        if session.organization_id != previous.organization_id {
            update = update.col_expr(
                SessionColumn::OrganizationId,
                Expr::value(Self::id(session.organization_id)?),
            );
        }
        // Setting the existing value also gives no-op updates a real row check.
        update = update.col_expr(
            SessionColumn::RevokedAt,
            Expr::value(session.revoked_at.map(Self::timestamp).transpose()?),
        );
        update = match previous.revoked_at {
            Some(at) => update.filter(SessionColumn::RevokedAt.eq(Self::timestamp(at)?)),
            None => update.filter(SessionColumn::RevokedAt.is_null()),
        };
        Ok(update
            .exec(db)
            .await
            .map_err(anyhow::Error::from)?
            .rows_affected
            == 1)
    }
}
