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
    ColumnTrait, ConnectionTrait, DbBackend, EntityTrait, ExprTrait, ModelTrait, QueryFilter,
    QuerySelect, Set, Statement,
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
    async fn authorization_version(
        &self,
        session: &Session,
    ) -> Result<Option<u32>, SessionRepositoryError> {
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        Sessions::find()
            .select_only()
            .column(SessionColumn::AuthorizationVersion)
            .filter(SessionColumn::OrganizationId.eq(Self::id(session.organization_id)?))
            .filter(SessionColumn::UserId.eq(Self::id(session.user_id)?))
            .filter(SessionColumn::Id.eq(Self::id(session.id)?))
            .filter(SessionColumn::RevokedAt.is_null())
            .into_tuple::<i64>()
            .one(db)
            .await
            .map_err(anyhow::Error::from)?
            .map(|version| {
                let version =
                    u32::try_from(version).context("invalid session authorization version")?;
                if version == 0 {
                    return Err(anyhow::anyhow!("invalid session authorization version").into());
                }
                Ok(version)
            })
            .transpose()
    }

    async fn find_policy_membership(
        &self,
        session: &Session,
    ) -> Result<
        Option<(Session, compallm_core::organization::OrganizationMember)>,
        SessionRepositoryError,
    > {
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        // All subqueries share one PostgreSQL statement snapshot. No read transaction
        // or lock is needed, and role/group joins always retain organization scope.
        let rows = db.query_all_raw(Statement::from_sql_and_values(DbBackend::Postgres, r#"
SELECT s.*, o.name AS organization_name, a.*
FROM user_sessions s
JOIN users u ON u.id=s.user_id AND u.status='active'
JOIN organizations o ON o.id=s.organization_id AND o.status='active'
JOIN organization_members m ON m.organization_id=s.organization_id AND m.user_id=s.user_id AND m.status='active'
LEFT JOIN LATERAL (
 SELECT NULL::bigint AS group_id, NULL::text AS group_name, NULL::text AS group_description,
        r.id AS role_id, r.name AS role_name, r.description AS role_description, r.cedar_source, r.org_owner
 FROM organization_member_roles mr
 JOIN organization_roles r ON r.organization_id=mr.organization_id AND r.id=mr.role_id
 WHERE mr.organization_id=s.organization_id AND mr.user_id=s.user_id
 UNION ALL
 SELECT g.id, g.name, g.description, r.id, r.name, r.description, r.cedar_source, r.org_owner
 FROM organization_group_members gm
 JOIN organization_groups g ON g.organization_id=gm.organization_id AND g.id=gm.group_id
 LEFT JOIN organization_group_roles gr ON gr.organization_id=g.organization_id AND gr.group_id=g.id
 LEFT JOIN organization_roles r ON r.organization_id=gr.organization_id AND r.id=gr.role_id
 WHERE gm.organization_id=s.organization_id AND gm.user_id=s.user_id
) a ON TRUE
WHERE s.organization_id=$1 AND s.user_id=$2 AND s.id=$3 AND s.revoked_at IS NULL
LIMIT $4
"#, [Self::id(session.organization_id)?.into(), Self::id(session.user_id)?.into(), Self::id(session.id)?.into(),
        ((compallm_core::authorization::PolicySnapshot::MAX_ASSIGNMENTS + compallm_core::authorization::PolicySnapshot::MAX_GROUPS + 1) as i64).into()]))
            .await.map_err(anyhow::Error::from)?;
        if rows.is_empty() {
            return Ok(None);
        }
        Ok(Some(
            crate::mappers::session::MappedPolicyMembership::try_from(rows)?.into(),
        ))
    }

    async fn invalidate_permissions(
        &self,
        organization_id: u64,
        user_ids: &[u64],
    ) -> Result<u64, SessionRepositoryError> {
        if user_ids.is_empty() {
            return Ok(0);
        }
        let organization_id = Self::id(organization_id)?;
        let user_ids = user_ids
            .iter()
            .copied()
            .map(Self::id)
            .collect::<Result<Vec<_>, _>>()?;
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        Ok(Sessions::update_many()
            .col_expr(
                SessionColumn::AuthorizationVersion,
                Expr::col(SessionColumn::AuthorizationVersion).add(1_i64),
            )
            .filter(SessionColumn::OrganizationId.eq(organization_id))
            .filter(SessionColumn::UserId.is_in(user_ids))
            .filter(SessionColumn::RevokedAt.is_null())
            .exec(db)
            .await
            .map_err(anyhow::Error::from)?
            .rows_affected)
    }

    async fn create(&self, session: Session) -> Result<(), SessionRepositoryError> {
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        let model = SessionActiveModel {
            id: Set(Self::id(session.id)?),
            user_id: Set(Self::id(session.user_id)?),
            organization_id: Set(Self::id(session.organization_id)?),
            authorization_version: Set(i64::from(session.authorization_version)),
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
            .find_also_related(<crate::OrganizationSchema as ModelTrait>::Entity::default())
            .filter(SessionColumn::UserId.eq(Self::id(session.user_id)?))
            .filter(SessionColumn::OrganizationId.eq(Self::id(session.organization_id)?))
            .one(db)
            .await
            .map_err(anyhow::Error::from)?
            .map(|(session, organization)| {
                let organization = organization.context("session organization is missing")?;
                crate::mappers::session::MappedSession::try_from((session, organization.name))
                    .map(Session::from)
            })
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
        // organization switch, and switching must not erase revocation. Never
        // write the loaded authorization version back over concurrent invalidation.
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
