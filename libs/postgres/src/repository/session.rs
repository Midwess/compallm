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
    ColumnTrait, ConnectionTrait, EntityTrait, ExprTrait, JoinType, ModelTrait, QueryFilter,
    QuerySelect, QueryTrait, Set,
    prelude::{DateTimeUtc, DateTimeWithTimeZone},
    sea_query::{Expr, UnionType},
};

type Sessions = <UserSessionSchema as ModelTrait>::Entity;
type SessionColumn = <Sessions as EntityTrait>::Column;
type Users = <crate::UserSchema as ModelTrait>::Entity;
type UserColumn = <Users as EntityTrait>::Column;
type Organizations = <crate::OrganizationSchema as ModelTrait>::Entity;
type OrganizationColumn = <Organizations as EntityTrait>::Column;
type Members = <crate::OrganizationMemberSchema as ModelTrait>::Entity;
type MemberColumn = <Members as EntityTrait>::Column;
type Roles = <crate::OrganizationRoleSchema as ModelTrait>::Entity;
type RoleColumn = <Roles as EntityTrait>::Column;
type MemberRoles = <crate::OrganizationMemberRoleSchema as ModelTrait>::Entity;
type MemberRoleColumn = <MemberRoles as EntityTrait>::Column;
type Groups = <crate::OrganizationGroupSchema as ModelTrait>::Entity;
type GroupColumn = <Groups as EntityTrait>::Column;
type GroupMembers = <crate::OrganizationGroupMemberSchema as ModelTrait>::Entity;
type GroupMemberColumn = <GroupMembers as EntityTrait>::Column;
type GroupRoles = <crate::OrganizationGroupRoleSchema as ModelTrait>::Entity;
type GroupRoleColumn = <GroupRoles as EntityTrait>::Column;

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
        // Keep session state and all effective assignments in one MVCC statement.
        let direct_relation = Roles::belongs_to(MemberRoles::default())
            .from((RoleColumn::OrganizationId, RoleColumn::Id))
            .to((MemberRoleColumn::OrganizationId, MemberRoleColumn::RoleId))
            .into();
        let mut direct = Roles::find()
            .select_only()
            .column_as(Expr::value(None::<i64>).cast_as("bigint"), "group_id")
            .column_as(Expr::value(None::<String>).cast_as("text"), "group_name")
            .column_as(
                Expr::value(None::<String>).cast_as("text"),
                "group_description",
            )
            .column_as(RoleColumn::Id, "role_id")
            .column_as(RoleColumn::Name, "role_name")
            .column_as(RoleColumn::Description, "role_description")
            .column(RoleColumn::CedarSource)
            .column(RoleColumn::OrgOwner)
            .join(JoinType::InnerJoin, direct_relation)
            .filter(
                Expr::col((MemberRoles::default(), MemberRoleColumn::OrganizationId))
                    .equals((Sessions::default(), SessionColumn::OrganizationId)),
            )
            .filter(
                Expr::col((MemberRoles::default(), MemberRoleColumn::UserId))
                    .equals((Sessions::default(), SessionColumn::UserId)),
            );
        let group_relation = GroupMembers::belongs_to(Groups::default())
            .from((
                GroupMemberColumn::OrganizationId,
                GroupMemberColumn::GroupId,
            ))
            .to((GroupColumn::OrganizationId, GroupColumn::Id))
            .into();
        let group_role_relation = Groups::belongs_to(GroupRoles::default())
            .from((GroupColumn::OrganizationId, GroupColumn::Id))
            .to((GroupRoleColumn::OrganizationId, GroupRoleColumn::GroupId))
            .into();
        let role_relation = GroupRoles::belongs_to(Roles::default())
            .from((GroupRoleColumn::OrganizationId, GroupRoleColumn::RoleId))
            .to((RoleColumn::OrganizationId, RoleColumn::Id))
            .into();
        let inherited = GroupMembers::find()
            .select_only()
            .column_as(GroupColumn::Id, "group_id")
            .column_as(GroupColumn::Name, "group_name")
            .column_as(GroupColumn::Description, "group_description")
            .column_as(RoleColumn::Id, "role_id")
            .column_as(RoleColumn::Name, "role_name")
            .column_as(RoleColumn::Description, "role_description")
            .column(RoleColumn::CedarSource)
            .column(RoleColumn::OrgOwner)
            .join(JoinType::InnerJoin, group_relation)
            .join(JoinType::LeftJoin, group_role_relation)
            .join(JoinType::LeftJoin, role_relation)
            .filter(
                Expr::col((GroupMembers::default(), GroupMemberColumn::OrganizationId))
                    .equals((Sessions::default(), SessionColumn::OrganizationId)),
            )
            .filter(
                Expr::col((GroupMembers::default(), GroupMemberColumn::UserId))
                    .equals((Sessions::default(), SessionColumn::UserId)),
            );
        QueryTrait::query(&mut direct).union(UnionType::All, inherited.into_query());
        let membership_relation = Sessions::belongs_to(Members::default())
            .from((SessionColumn::OrganizationId, SessionColumn::UserId))
            .to((MemberColumn::OrganizationId, MemberColumn::UserId))
            .into();
        let mut query = Sessions::find()
            .inner_join(Users::default())
            .inner_join(Organizations::default())
            .join(JoinType::InnerJoin, membership_relation)
            .column_as(OrganizationColumn::Name, "organization_name")
            .filter(UserColumn::Status.eq(crate::UserStatusSchema::Active))
            .filter(OrganizationColumn::Status.eq(crate::OrganizationStatusSchema::Active))
            .filter(MemberColumn::Status.eq(crate::UserStatusSchema::Active))
            .filter(SessionColumn::OrganizationId.eq(Self::id(session.organization_id)?))
            .filter(SessionColumn::UserId.eq(Self::id(session.user_id)?))
            .filter(SessionColumn::Id.eq(Self::id(session.id)?))
            .filter(SessionColumn::RevokedAt.is_null());
        QueryTrait::query(&mut query).join_lateral(
            JoinType::LeftJoin,
            direct.into_query(),
            "authority",
            Expr::value(true),
        );
        for field in [
            "group_id",
            "group_name",
            "group_description",
            "role_id",
            "role_name",
            "role_description",
            "cedar_source",
            "org_owner",
        ] {
            query = query.column_as(Expr::col(("authority", field)), field);
        }
        let rows = db
            .query_all(&query.into_query())
            .await
            .map_err(anyhow::Error::from)?;
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
