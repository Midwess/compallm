use crate::{UserSessionSchema, postgres_connection::PostgresConnection};
use anyhow::Context;
use compallm_core::{
    auth::{Session, SessionReference},
    persistent::{
        DbContext,
        session::{SessionRepository as CoreSessionRepository, SessionRepositoryError},
    },
};
use sea_orm::{
    ColumnTrait, EntityTrait, ModelTrait, QueryFilter, Set,
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
            organization_id: Set(Self::id(session.organization_id)?),
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

    async fn find(
        &self,
        reference: SessionReference,
    ) -> Result<Option<Session>, SessionRepositoryError> {
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        Sessions::find_by_id(Self::id(reference.id)?)
            .filter(SessionColumn::UserId.eq(Self::id(reference.user_id)?))
            .filter(SessionColumn::OrganizationId.eq(Self::id(reference.organization_id)?))
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
            || session.created_at != previous.created_at
            || (previous.revoked_at.is_some() && session.revoked_at != previous.revoked_at)
        {
            return Err(anyhow::anyhow!(
                "session identity, creation time, and existing revocation cannot change"
            )
            .into());
        }
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        let mut update = Sessions::update_many()
            .filter(SessionColumn::Id.eq(Self::id(previous.id)?))
            .filter(SessionColumn::UserId.eq(Self::id(previous.user_id)?))
            .filter(SessionColumn::OrganizationId.eq(Self::id(previous.organization_id)?));

        // Write only changed fields. Revocation must not copy an old credential
        // over a concurrent renewal; renewal must not erase a concurrent revocation.
        if session.organization_id != previous.organization_id {
            update = update.col_expr(
                SessionColumn::OrganizationId,
                Expr::value(Self::id(session.organization_id)?),
            );
        }
        if session.refresh_token_id != previous.refresh_token_id {
            update = update
                .col_expr(
                    SessionColumn::RefreshTokenId,
                    Expr::value(session.refresh_token_id.clone()),
                )
                .filter(SessionColumn::RefreshTokenId.eq(previous.refresh_token_id.clone()));
        }
        if session.last_used_at != previous.last_used_at {
            update = update
                .col_expr(
                    SessionColumn::LastUsedAt,
                    Expr::value(Self::timestamp(session.last_used_at)?),
                )
                .filter(SessionColumn::LastUsedAt.eq(Self::timestamp(previous.last_used_at)?));
        }
        if session.expires_at != previous.expires_at {
            update = update
                .col_expr(
                    SessionColumn::ExpiresAt,
                    Expr::value(Self::timestamp(session.expires_at)?),
                )
                .filter(SessionColumn::ExpiresAt.eq(Self::timestamp(previous.expires_at)?));
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
        if session.revoked_at.is_none() {
            update =
                update.filter(Expr::col(SessionColumn::ExpiresAt).gt(Expr::current_timestamp()));
        }
        Ok(update
            .exec(db)
            .await
            .map_err(anyhow::Error::from)?
            .rows_affected
            == 1)
    }
}
