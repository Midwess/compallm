use anyhow::Context;
use compallm_core::{
    organization::{Group, Organization, OrganizationMember, OrganizationStatus, Role},
    persistent::{
        DbContext,
        organization::{OrganizationRepository as CoreRepository, OrganizationRepositoryError},
    },
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbBackend, EntityTrait, FromQueryResult,
    ModelTrait, QueryFilter, QueryOrder, Set, Statement, sea_query::Expr,
};

use crate::{
    postgres_connection::PostgresConnection,
    schema::{
        OrganizationGroupSchema, OrganizationMemberSchema, OrganizationRoleSchema,
        OrganizationSchema, OrganizationStatusSchema,
    },
};

type Organizations = <OrganizationSchema as ModelTrait>::Entity;
type OrganizationActiveModel = <Organizations as EntityTrait>::ActiveModel;
type OrganizationColumn = <Organizations as EntityTrait>::Column;
type Members = <OrganizationMemberSchema as ModelTrait>::Entity;
type MemberColumn = <Members as EntityTrait>::Column;

#[derive(Default)]
pub struct OrganizationRepository;

impl OrganizationRepository {
    async fn membership(
        &self,
        user_id: u64,
        organization_id: Option<u64>,
    ) -> Result<Option<(OrganizationMember, Organization)>, OrganizationRepositoryError> {
        let user_id = Self::database_id(user_id)?;
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        let mut query = Members::find()
            .find_also_related(Organizations::default())
            .filter(MemberColumn::UserId.eq(user_id))
            .filter(MemberColumn::Status.eq(crate::schema::UserStatusSchema::Active))
            .filter(OrganizationColumn::Status.eq(OrganizationStatusSchema::Active));
        if let Some(organization_id) = organization_id {
            query =
                query.filter(MemberColumn::OrganizationId.eq(Self::database_id(organization_id)?));
        }
        let found = query
            .order_by_asc(MemberColumn::OrganizationId)
            .one(db)
            .await
            .map_err(OrganizationRepositoryError::database)?;
        let Some((member, organization)) = found else {
            return Ok(None);
        };
        let organization =
            Organization::try_from(organization.context("membership organization is missing")?)?;
        let mut member = OrganizationMember::try_from(member)?;
        let organization_id = Self::database_id(member.organization_id)?;
        let direct=db.query_all_raw(Statement::from_sql_and_values(DbBackend::Postgres,
            "SELECT r.* FROM organization_roles r JOIN organization_member_roles mr ON mr.organization_id=r.organization_id AND mr.role_id=r.id WHERE r.organization_id=$1 AND mr.user_id=$2 ORDER BY r.id",[organization_id.into(),user_id.into()])).await.map_err(OrganizationRepositoryError::database)?;
        member.roles = direct
            .iter()
            .map(|row| {
                Role::try_from(
                    OrganizationRoleSchema::from_query_result(row, "")
                        .map_err(OrganizationRepositoryError::database)?,
                )
                .map_err(OrganizationRepositoryError::database)
            })
            .collect::<Result<_, _>>()?;
        let groups=db.query_all_raw(Statement::from_sql_and_values(DbBackend::Postgres,
            "SELECT g.* FROM organization_groups g JOIN organization_group_members gm ON gm.organization_id=g.organization_id AND gm.group_id=g.id WHERE g.organization_id=$1 AND gm.user_id=$2 ORDER BY g.id",[organization_id.into(),user_id.into()])).await.map_err(OrganizationRepositoryError::database)?;
        let mut groups = groups
            .iter()
            .map(|row| OrganizationGroupSchema::from_query_result(row, ""))
            .collect::<Result<Vec<_>, _>>()
            .map_err(OrganizationRepositoryError::database)?;
        let mut grouped = std::collections::BTreeMap::<i64, Vec<Role>>::new();
        if !groups.is_empty() {
            let ids = serde_json::json!(groups.iter().map(|g| g.id).collect::<Vec<_>>());
            let rows=db.query_all_raw(Statement::from_sql_and_values(DbBackend::Postgres,
                "SELECT gr.group_id,r.* FROM organization_group_roles gr JOIN organization_roles r ON r.organization_id=gr.organization_id AND r.id=gr.role_id WHERE gr.organization_id=$1 AND gr.group_id IN (SELECT value::bigint FROM jsonb_array_elements_text($2::jsonb)) ORDER BY gr.group_id,r.id",[organization_id.into(),ids.into()])).await.map_err(OrganizationRepositoryError::database)?;
            for row in rows {
                grouped
                    .entry(
                        row.try_get("", "group_id")
                            .map_err(OrganizationRepositoryError::database)?,
                    )
                    .or_default()
                    .push(
                        Role::try_from(
                            OrganizationRoleSchema::from_query_result(&row, "")
                                .map_err(OrganizationRepositoryError::database)?,
                        )
                        .map_err(OrganizationRepositoryError::database)?,
                    );
            }
        }
        member.groups = groups
            .drain(..)
            .map(|g| {
                let roles = grouped.remove(&g.id).unwrap_or_default();
                crate::mappers::role::MappedGroup::try_from((g, roles))
                    .map(Group::from)
                    .map_err(OrganizationRepositoryError::database)
            })
            .collect::<Result<_, _>>()?;
        Ok(Some((member, organization)))
    }

    fn database_id(id: u64) -> Result<i64, OrganizationRepositoryError> {
        Ok(crate::to_bigint(id).context("organization or member ID exceeds PostgreSQL BIGINT")?)
    }
}

#[async_trait::async_trait]
impl CoreRepository for OrganizationRepository {
    async fn find_membership(
        &self,
        organization_id: u64,
        user_id: u64,
    ) -> Result<Option<(OrganizationMember, Organization)>, OrganizationRepositoryError> {
        self.membership(user_id, Some(organization_id)).await
    }

    async fn find_first_membership(
        &self,
        user_id: u64,
    ) -> Result<Option<(OrganizationMember, Organization)>, OrganizationRepositoryError> {
        self.membership(user_id, None).await
    }

    async fn update_name(
        &self,
        organization_id: u64,
        name: &str,
    ) -> Result<Organization, OrganizationRepositoryError> {
        let id = Self::database_id(organization_id)?;
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        let mut rows = Organizations::update_many()
            .col_expr(OrganizationColumn::Name, Expr::value(name))
            .filter(OrganizationColumn::Id.eq(id))
            .exec_with_returning(db)
            .await
            .map_err(OrganizationRepositoryError::database)?;
        Organization::try_from(rows.pop().ok_or(OrganizationRepositoryError::NotFound)?)
    }

    async fn create(
        &self,
        organization: Organization,
    ) -> Result<Organization, OrganizationRepositoryError> {
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        let model = OrganizationActiveModel {
            id: Set(Self::database_id(organization.id)?),
            name: Set(organization.name),
            status: Set(match organization.status {
                OrganizationStatus::Active => OrganizationStatusSchema::Active,
                OrganizationStatus::Disabled => OrganizationStatusSchema::Disabled,
            }),
        };
        Organization::try_from(
            model
                .insert(db)
                .await
                .map_err(OrganizationRepositoryError::database)?,
        )
    }

    async fn create_roles(&self, roles: Vec<Role>) -> Result<(), OrganizationRepositoryError> {
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        let mut rows = Vec::new();
        for role in roles {
            rows.push(serde_json::json!({"organization_id":role.organization_id,"id":role.id,"name":role.name,"description":role.description,"cedar_source":role.cedar_source}));
        }
        db.execute_raw(Statement::from_sql_and_values(DbBackend::Postgres,
            "INSERT INTO organization_roles SELECT * FROM jsonb_populate_recordset(NULL::organization_roles,$1::jsonb)",[serde_json::json!(rows).into()])).await.map_err(OrganizationRepositoryError::database)?;
        Ok(())
    }

    async fn add_member(
        &self,
        member: OrganizationMember,
    ) -> Result<OrganizationMember, OrganizationRepositoryError> {
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        let org = Self::database_id(member.organization_id)?;
        let user = Self::database_id(member.user_id)?;
        cedar_policy::Entities::try_from(&member).map_err(OrganizationRepositoryError::database)?;
        let roles = serde_json::json!(member.roles.iter().map(|r| r.id).collect::<Vec<_>>());
        let groups = serde_json::json!(member.groups.iter().map(|g| g.id).collect::<Vec<_>>());
        let statement = Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"
WITH member AS (
 INSERT INTO organization_members(organization_id,user_id,status) VALUES($1,$2,'active') RETURNING *
), assigned AS (
 INSERT INTO organization_member_roles(organization_id,user_id,role_id)
 SELECT m.organization_id,m.user_id,value::bigint FROM member m,jsonb_array_elements_text($3::jsonb) RETURNING role_id
), grouped AS (
 INSERT INTO organization_group_members(organization_id,group_id,user_id)
 SELECT m.organization_id,value::bigint,m.user_id FROM member m,jsonb_array_elements_text($4::jsonb) RETURNING group_id
) SELECT * FROM member
"#,
            [org.into(), user.into(), roles.into(), groups.into()],
        );
        db.query_one_raw(statement)
            .await
            .map_err(OrganizationRepositoryError::database)?
            .ok_or(OrganizationRepositoryError::NotFound)?;
        Ok(member)
    }
}
