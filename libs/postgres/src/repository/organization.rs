use crate::{
    postgres_connection::PostgresConnection,
    schema::{
        OrganizationGroupMemberSchema, OrganizationGroupRoleSchema, OrganizationGroupSchema,
        OrganizationMemberRoleSchema, OrganizationMemberSchema, OrganizationRoleSchema,
        OrganizationSchema, OrganizationStatusSchema, UserStatusSchema,
    },
};
use anyhow::Context;
use compallm_core::{
    organization::{Group, Organization, OrganizationMember, OrganizationStatus, Role},
    persistent::{
        DbContext,
        organization::{OrganizationRepository as CoreRepository, OrganizationRepositoryError},
    },
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, JoinType, ModelTrait, QueryFilter, QueryOrder,
    QuerySelect, QueryTrait, Set,
    sea_query::{Alias, Expr, Func, LockType, UnionType},
};
use std::collections::{BTreeMap, BTreeSet};

type Organizations = <OrganizationSchema as ModelTrait>::Entity;
type OrganizationActiveModel = <Organizations as EntityTrait>::ActiveModel;
type OrganizationColumn = <Organizations as EntityTrait>::Column;
type Members = <OrganizationMemberSchema as ModelTrait>::Entity;
type MemberColumn = <Members as EntityTrait>::Column;
type MemberActiveModel = <Members as EntityTrait>::ActiveModel;
type Roles = <OrganizationRoleSchema as ModelTrait>::Entity;
type RoleColumn = <Roles as EntityTrait>::Column;
type RoleActiveModel = <Roles as EntityTrait>::ActiveModel;
type Groups = <OrganizationGroupSchema as ModelTrait>::Entity;
type GroupColumn = <Groups as EntityTrait>::Column;
type GroupActiveModel = <Groups as EntityTrait>::ActiveModel;
type MemberRoles = <OrganizationMemberRoleSchema as ModelTrait>::Entity;
type MemberRoleColumn = <MemberRoles as EntityTrait>::Column;
type MemberRoleActiveModel = <MemberRoles as EntityTrait>::ActiveModel;
type GroupRoles = <OrganizationGroupRoleSchema as ModelTrait>::Entity;
type GroupRoleColumn = <GroupRoles as EntityTrait>::Column;
type GroupRoleActiveModel = <GroupRoles as EntityTrait>::ActiveModel;
type GroupMembers = <OrganizationGroupMemberSchema as ModelTrait>::Entity;
type GroupMemberColumn = <GroupMembers as EntityTrait>::Column;
type GroupMemberActiveModel = <GroupMembers as EntityTrait>::ActiveModel;

#[derive(Default)]
pub struct OrganizationRepository;

impl OrganizationRepository {
    fn creation_error(error: sea_orm::DbErr) -> OrganizationRepositoryError {
        match error.sql_err() {
            Some(sea_orm::SqlErr::UniqueConstraintViolation(_)) => {
                OrganizationRepositoryError::NameAlreadyExists
            }
            _ => OrganizationRepositoryError::database(error),
        }
    }

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
            .filter(MemberColumn::Status.eq(UserStatusSchema::Active))
            .filter(OrganizationColumn::Status.eq(OrganizationStatusSchema::Active));
        if let Some(id) = organization_id {
            query = query.filter(MemberColumn::OrganizationId.eq(Self::database_id(id)?));
        }
        let Some((member, organization)) = query
            .order_by_asc(MemberColumn::OrganizationId)
            .one(db)
            .await
            .map_err(OrganizationRepositoryError::database)?
        else {
            return Ok(None);
        };
        let organization =
            Organization::try_from(organization.context("membership organization is missing")?)?;
        let mut member = OrganizationMember::try_from(member)?;
        let org = Self::database_id(member.organization_id)?;
        member.roles = Roles::find()
            .filter(RoleColumn::OrganizationId.eq(org))
            .filter(
                RoleColumn::Id.in_subquery(
                    MemberRoles::find()
                        .select_only()
                        .column(MemberRoleColumn::RoleId)
                        .filter(MemberRoleColumn::OrganizationId.eq(org))
                        .filter(MemberRoleColumn::UserId.eq(user_id))
                        .into_query(),
                ),
            )
            .order_by_asc(RoleColumn::Id)
            .all(db)
            .await
            .map_err(OrganizationRepositoryError::database)?
            .into_iter()
            .map(Role::try_from)
            .collect::<Result<_, _>>()
            .map_err(OrganizationRepositoryError::database)?;
        let groups = Groups::find()
            .filter(GroupColumn::OrganizationId.eq(org))
            .filter(
                GroupColumn::Id.in_subquery(
                    GroupMembers::find()
                        .select_only()
                        .column(GroupMemberColumn::GroupId)
                        .filter(GroupMemberColumn::OrganizationId.eq(org))
                        .filter(GroupMemberColumn::UserId.eq(user_id))
                        .into_query(),
                ),
            )
            .order_by_asc(GroupColumn::Id)
            .all(db)
            .await
            .map_err(OrganizationRepositoryError::database)?;
        let mut grouped = BTreeMap::<i64, Vec<Role>>::new();
        if !groups.is_empty() {
            let relation = GroupRoles::belongs_to(Roles::default())
                .from((GroupRoleColumn::OrganizationId, GroupRoleColumn::RoleId))
                .to((RoleColumn::OrganizationId, RoleColumn::Id))
                .into();
            let rows = Roles::find()
                .join_rev(JoinType::InnerJoin, relation)
                .column_as(GroupRoleColumn::GroupId, "group_id")
                .filter(GroupRoleColumn::OrganizationId.eq(org))
                .filter(GroupRoleColumn::GroupId.is_in(groups.iter().map(|g| g.id)))
                .order_by_asc(GroupRoleColumn::GroupId)
                .order_by_asc(GroupRoleColumn::RoleId)
                .into_model::<crate::mappers::role::GroupRoleRow>()
                .all(db)
                .await
                .map_err(OrganizationRepositoryError::database)?;
            for row in rows {
                let role =
                    Role::try_from(row.role).map_err(OrganizationRepositoryError::database)?;
                grouped.entry(row.group_id).or_default().push(role);
            }
        }
        member.groups = groups
            .into_iter()
            .map(|group| {
                let roles = grouped.remove(&group.id).unwrap_or_default();
                crate::mappers::role::MappedGroup::try_from((group, roles))
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
    fn supports_role_policy(&self, source: &str) -> bool {
        crate::rls::validate_policy_source(source).is_ok()
    }
    async fn update_role(
        &self,
        organization_id: u64,
        role_id: u64,
        name: Option<String>,
        description: Option<String>,
        cedar_source: Option<String>,
    ) -> Result<(), OrganizationRepositoryError> {
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        let mut update = Roles::update_many()
            .filter(RoleColumn::OrganizationId.eq(Self::database_id(organization_id)?))
            .filter(RoleColumn::Id.eq(Self::database_id(role_id)?));
        if let Some(name) = name {
            update = update.col_expr(RoleColumn::Name, Expr::value(name));
        }
        if let Some(description) = description {
            update = update.col_expr(RoleColumn::Description, Expr::value(description));
        }
        if let Some(source) = cedar_source {
            update = update.col_expr(RoleColumn::CedarSource, Expr::value(source));
        }
        if update
            .exec(db)
            .await
            .map_err(Self::creation_error)?
            .rows_affected
            == 0
        {
            return Err(OrganizationRepositoryError::NotFound);
        }
        Ok(())
    }
    async fn lock_role(
        &self,
        organization_id: u64,
        role_id: u64,
    ) -> Result<Role, OrganizationRepositoryError> {
        let context = DbContext::current()?;
        if !context.postgres().is_transaction() {
            return Err(OrganizationRepositoryError::database(anyhow::anyhow!(
                "role lock requires a transaction"
            )));
        }
        let db = PostgresConnection::from_context(&context)?;
        let role = Roles::find_by_id((
            Self::database_id(organization_id)?,
            Self::database_id(role_id)?,
        ))
        .select_only()
        .columns([
            RoleColumn::OrganizationId,
            RoleColumn::Id,
            RoleColumn::Name,
            RoleColumn::Description,
            RoleColumn::OrgOwner,
        ])
        .expr_as(Expr::value(String::new()), "cedar_source")
        .lock_exclusive()
        .into_model::<OrganizationRoleSchema>()
        .one(db)
        .await
        .map_err(OrganizationRepositoryError::database)?
        .ok_or(OrganizationRepositoryError::NotFound)?;
        Role::try_from(role).map_err(OrganizationRepositoryError::database)
    }

    async fn lock_role_groups(
        &self,
        organization_id: u64,
        role_id: u64,
    ) -> Result<(), OrganizationRepositoryError> {
        let context = DbContext::current()?;
        if !context.postgres().is_transaction() {
            return Err(OrganizationRepositoryError::database(anyhow::anyhow!(
                "group lock requires a transaction"
            )));
        }
        let db = PostgresConnection::from_context(&context)?;
        let org = Self::database_id(organization_id)?;
        Groups::find()
            .filter(GroupColumn::OrganizationId.eq(org))
            .filter(
                GroupColumn::Id.in_subquery(
                    GroupRoles::find()
                        .select_only()
                        .column(GroupRoleColumn::GroupId)
                        .filter(GroupRoleColumn::OrganizationId.eq(org))
                        .filter(GroupRoleColumn::RoleId.eq(Self::database_id(role_id)?))
                        .into_query(),
                ),
            )
            .order_by_asc(GroupColumn::Id)
            .lock(LockType::NoKeyUpdate)
            .all(db)
            .await
            .map_err(OrganizationRepositoryError::database)?;
        Ok(())
    }

    async fn lock_group(
        &self,
        organization_id: u64,
        group_id: u64,
    ) -> Result<(), OrganizationRepositoryError> {
        let context = DbContext::current()?;
        if !context.postgres().is_transaction() {
            return Err(OrganizationRepositoryError::database(anyhow::anyhow!(
                "group lock requires a transaction"
            )));
        }
        let db = PostgresConnection::from_context(&context)?;
        Groups::find_by_id((
            Self::database_id(organization_id)?,
            Self::database_id(group_id)?,
        ))
        .lock(LockType::NoKeyUpdate)
        .one(db)
        .await
        .map_err(OrganizationRepositoryError::database)?
        .ok_or(OrganizationRepositoryError::NotFound)?;
        Ok(())
    }

    async fn role_assignees(
        &self,
        organization_id: u64,
        role_id: u64,
    ) -> Result<Vec<u64>, OrganizationRepositoryError> {
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        let org = Self::database_id(organization_id)?;
        let role = Self::database_id(role_id)?;
        let inherited = GroupMembers::find()
            .select_only()
            .column(GroupMemberColumn::UserId)
            .filter(GroupMemberColumn::OrganizationId.eq(org))
            .filter(
                GroupMemberColumn::GroupId.in_subquery(
                    GroupRoles::find()
                        .select_only()
                        .column(GroupRoleColumn::GroupId)
                        .filter(GroupRoleColumn::OrganizationId.eq(org))
                        .filter(GroupRoleColumn::RoleId.eq(role))
                        .into_query(),
                ),
            );
        let mut users = MemberRoles::find()
            .select_only()
            .column(MemberRoleColumn::UserId)
            .filter(MemberRoleColumn::OrganizationId.eq(org))
            .filter(MemberRoleColumn::RoleId.eq(role));
        QueryTrait::query(&mut users).union(UnionType::Distinct, inherited.into_query());
        Ok(users
            .into_tuple::<i64>()
            .all(db)
            .await
            .map_err(OrganizationRepositoryError::database)?
            .into_iter()
            .map(|id| crate::from_bigint(id).map_err(OrganizationRepositoryError::database))
            .collect::<Result<BTreeSet<_>, _>>()?
            .into_iter()
            .collect())
    }

    async fn delete_role(
        &self,
        organization_id: u64,
        role_id: u64,
    ) -> Result<(), OrganizationRepositoryError> {
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        let result = Roles::delete_by_id((
            Self::database_id(organization_id)?,
            Self::database_id(role_id)?,
        ))
        .exec(db)
        .await
        .map_err(OrganizationRepositoryError::database)?;
        if result.rows_affected == 0 {
            return Err(OrganizationRepositoryError::NotFound);
        }
        Ok(())
    }

    async fn find_roles_for_assignment(
        &self,
        organization_id: u64,
        role_ids: &[u64],
    ) -> Result<Vec<Role>, OrganizationRepositoryError> {
        let ids = role_ids
            .iter()
            .map(|id| Self::database_id(*id))
            .collect::<Result<Vec<_>, _>>()?;
        if ids.is_empty() {
            return Ok(Vec::new());
        }
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        Roles::find()
            .filter(RoleColumn::OrganizationId.eq(Self::database_id(organization_id)?))
            .filter(RoleColumn::Id.is_in(ids))
            .select_only()
            .columns([
                RoleColumn::OrganizationId,
                RoleColumn::Id,
                RoleColumn::Name,
                RoleColumn::Description,
                RoleColumn::OrgOwner,
            ])
            .order_by_asc(RoleColumn::Id)
            .into_tuple::<(i64, i64, String, String, bool)>()
            .all(db)
            .await
            .map_err(OrganizationRepositoryError::database)?
            .into_iter()
            .map(|(organization_id, id, name, description, org_owner)| {
                Ok(Role {
                    organization_id: crate::from_bigint(organization_id)
                        .map_err(OrganizationRepositoryError::database)?,
                    id: crate::from_bigint(id).map_err(OrganizationRepositoryError::database)?,
                    name,
                    description,
                    org_owner,
                    cedar_source: String::new(),
                })
            })
            .collect()
    }

    async fn create_group(&self, group: Group) -> Result<Group, OrganizationRepositoryError> {
        let org = Self::database_id(group.organization_id)?;
        let id = Self::database_id(group.id)?;
        let model = GroupActiveModel {
            organization_id: Set(org),
            id: Set(id),
            name: Set(group.name.clone()),
            description: Set(group.description.clone()),
        };
        let links = group
            .roles
            .iter()
            .map(|role| {
                Ok(GroupRoleActiveModel {
                    organization_id: Set(org),
                    group_id: Set(id),
                    role_id: Set(Self::database_id(role.id)?),
                })
            })
            .collect::<Result<Vec<_>, OrganizationRepositoryError>>()?;
        let context = DbContext::current()?;
        if links.is_empty() {
            Groups::insert(model)
                .exec_without_returning(PostgresConnection::from_context(&context)?)
                .await
                .map_err(Self::creation_error)?;
        } else {
            compallm_core::transaction!(context, {
                let current = DbContext::current()?;
                let db = PostgresConnection::from_context(&current)?;
                Groups::insert(model)
                    .exec_without_returning(db)
                    .await
                    .map_err(Self::creation_error)?;
                GroupRoles::insert_many(links)
                    .exec_without_returning(db)
                    .await
                    .map_err(Self::creation_error)?;
                Ok::<_, OrganizationRepositoryError>(())
            })?;
        }
        Ok(group)
    }

    async fn find_role(
        &self,
        organization_id: u64,
        role_id: u64,
    ) -> Result<Option<Role>, OrganizationRepositoryError> {
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        if db.rls().is_some() {
            return Roles::find_by_id((
                Self::database_id(organization_id)?,
                Self::database_id(role_id)?,
            ))
            .select_only()
            .columns([
                RoleColumn::OrganizationId,
                RoleColumn::Id,
                RoleColumn::Name,
                RoleColumn::Description,
                RoleColumn::OrgOwner,
            ])
            .expr_as(
                Func::cust(Alias::new("compallm_rls.role_policy"))
                    .arg(Expr::col(RoleColumn::OrganizationId))
                    .arg(Expr::col(RoleColumn::Id)),
                "cedar_source",
            )
            .into_model::<OrganizationRoleSchema>()
            .one(db)
            .await
            .map_err(OrganizationRepositoryError::database)?
            .map(Role::try_from)
            .transpose()
            .map_err(OrganizationRepositoryError::database);
        }
        Roles::find_by_id((
            Self::database_id(organization_id)?,
            Self::database_id(role_id)?,
        ))
        .one(db)
        .await
        .map_err(OrganizationRepositoryError::database)?
        .map(Role::try_from)
        .transpose()
        .map_err(OrganizationRepositoryError::database)
    }

    async fn list_role_info(
        &self,
        organization_id: u64,
    ) -> Result<Vec<Role>, OrganizationRepositoryError> {
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        Roles::find()
            .select_only()
            .columns([
                RoleColumn::OrganizationId,
                RoleColumn::Id,
                RoleColumn::Name,
                RoleColumn::Description,
                RoleColumn::OrgOwner,
            ])
            .filter(RoleColumn::OrganizationId.eq(Self::database_id(organization_id)?))
            .order_by_asc(RoleColumn::Id)
            .into_tuple::<(i64, i64, String, String, bool)>()
            .all(db)
            .await
            .map_err(OrganizationRepositoryError::database)?
            .into_iter()
            .map(|(organization_id, id, name, description, org_owner)| {
                Ok(Role {
                    organization_id: crate::from_bigint(organization_id)
                        .map_err(OrganizationRepositoryError::database)?,
                    id: crate::from_bigint(id).map_err(OrganizationRepositoryError::database)?,
                    name,
                    description,
                    org_owner,
                    cedar_source: String::new(),
                })
            })
            .collect()
    }

    async fn list_group_info(
        &self,
        organization_id: u64,
    ) -> Result<Vec<compallm_core::organization::group::GroupInfo>, OrganizationRepositoryError>
    {
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        Groups::find()
            .filter(GroupColumn::OrganizationId.eq(Self::database_id(organization_id)?))
            .order_by_asc(GroupColumn::Id)
            .all(db)
            .await
            .map_err(OrganizationRepositoryError::database)?
            .into_iter()
            .map(|group| {
                Ok(compallm_core::organization::group::GroupInfo {
                    id: crate::from_bigint(group.id)
                        .map_err(OrganizationRepositoryError::database)?,
                    name: group.name,
                    description: group.description,
                })
            })
            .collect()
    }

    async fn find_group_info(
        &self,
        organization_id: u64,
        group_id: u64,
    ) -> Result<Option<compallm_core::organization::group::GroupInfo>, OrganizationRepositoryError>
    {
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        let group = Groups::find_by_id((
            Self::database_id(organization_id)?,
            Self::database_id(group_id)?,
        ))
        .one(db)
        .await
        .map_err(OrganizationRepositoryError::database)?;
        group
            .map(|group| {
                Ok(compallm_core::organization::group::GroupInfo {
                    id: crate::from_bigint(group.id)
                        .map_err(OrganizationRepositoryError::database)?,
                    name: group.name,
                    description: group.description,
                })
            })
            .transpose()
    }

    async fn find_group(
        &self,
        organization_id: u64,
        group_id: u64,
    ) -> Result<Option<Group>, OrganizationRepositoryError> {
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        let org = Self::database_id(organization_id)?;
        let id = Self::database_id(group_id)?;
        let Some(group) = Groups::find_by_id((org, id))
            .one(db)
            .await
            .map_err(OrganizationRepositoryError::database)?
        else {
            return Ok(None);
        };
        let roles = Roles::find()
            .filter(RoleColumn::OrganizationId.eq(org))
            .filter(
                RoleColumn::Id.in_subquery(
                    GroupRoles::find()
                        .select_only()
                        .column(GroupRoleColumn::RoleId)
                        .filter(GroupRoleColumn::OrganizationId.eq(org))
                        .filter(GroupRoleColumn::GroupId.eq(id))
                        .into_query(),
                ),
            )
            .order_by_asc(RoleColumn::Id)
            .all(db)
            .await
            .map_err(OrganizationRepositoryError::database)?
            .into_iter()
            .map(Role::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(OrganizationRepositoryError::database)?;
        crate::mappers::role::MappedGroup::try_from((group, roles))
            .map(|g| Some(Group::from(g)))
            .map_err(OrganizationRepositoryError::database)
    }

    async fn save_member(
        &self,
        member: &OrganizationMember,
        previous: &OrganizationMember,
    ) -> Result<(), OrganizationRepositoryError> {
        if (member.organization_id, member.user_id) != (previous.organization_id, previous.user_id)
        {
            return Err(OrganizationRepositoryError::Conflict);
        }
        cedar_policy::Entities::try_from(member).map_err(OrganizationRepositoryError::database)?;
        let org = Self::database_id(member.organization_id)?;
        let user = Self::database_id(member.user_id)?;
        let roles = member.roles.iter().map(|r| r.id).collect::<BTreeSet<_>>();
        let groups = member.groups.iter().map(|g| g.id).collect::<BTreeSet<_>>();
        let previous_roles = previous.roles.iter().map(|r| r.id).collect::<BTreeSet<_>>();
        let previous_groups = previous
            .groups
            .iter()
            .map(|g| g.id)
            .collect::<BTreeSet<_>>();
        let role_links = roles
            .iter()
            .map(|id| {
                Ok(MemberRoleActiveModel {
                    organization_id: Set(org),
                    user_id: Set(user),
                    role_id: Set(Self::database_id(*id)?),
                })
            })
            .collect::<Result<Vec<_>, OrganizationRepositoryError>>()?;
        let group_links = groups
            .iter()
            .map(|id| {
                Ok(GroupMemberActiveModel {
                    organization_id: Set(org),
                    user_id: Set(user),
                    group_id: Set(Self::database_id(*id)?),
                })
            })
            .collect::<Result<Vec<_>, OrganizationRepositoryError>>()?;
        let context = DbContext::current()?;
        compallm_core::transaction!(context, {
            let current = DbContext::current()?;
            let db = PostgresConnection::from_context(&current)?;
            Members::find_by_id((org, user))
                .lock_exclusive()
                .one(db)
                .await
                .map_err(OrganizationRepositoryError::database)?
                .ok_or(OrganizationRepositoryError::NotFound)?;
            let stored_roles = MemberRoles::find()
                .select_only()
                .column(MemberRoleColumn::RoleId)
                .filter(MemberRoleColumn::OrganizationId.eq(org))
                .filter(MemberRoleColumn::UserId.eq(user))
                .into_tuple::<i64>()
                .all(db)
                .await
                .map_err(OrganizationRepositoryError::database)?
                .into_iter()
                .map(|id| crate::from_bigint(id).map_err(OrganizationRepositoryError::database))
                .collect::<Result<BTreeSet<_>, _>>()?;
            let stored_groups = GroupMembers::find()
                .select_only()
                .column(GroupMemberColumn::GroupId)
                .filter(GroupMemberColumn::OrganizationId.eq(org))
                .filter(GroupMemberColumn::UserId.eq(user))
                .into_tuple::<i64>()
                .all(db)
                .await
                .map_err(OrganizationRepositoryError::database)?
                .into_iter()
                .map(|id| crate::from_bigint(id).map_err(OrganizationRepositoryError::database))
                .collect::<Result<BTreeSet<_>, _>>()?;
            if stored_roles != previous_roles || stored_groups != previous_groups {
                return Err(OrganizationRepositoryError::Conflict);
            }
            if roles == previous_roles && groups == previous_groups {
                return Ok(());
            }
            MemberRoles::delete_many()
                .filter(MemberRoleColumn::OrganizationId.eq(org))
                .filter(MemberRoleColumn::UserId.eq(user))
                .exec(db)
                .await
                .map_err(OrganizationRepositoryError::database)?;
            GroupMembers::delete_many()
                .filter(GroupMemberColumn::OrganizationId.eq(org))
                .filter(GroupMemberColumn::UserId.eq(user))
                .exec(db)
                .await
                .map_err(OrganizationRepositoryError::database)?;
            if !role_links.is_empty() {
                MemberRoles::insert_many(role_links)
                    .exec_without_returning(db)
                    .await
                    .map_err(OrganizationRepositoryError::database)?;
            }
            if !group_links.is_empty() {
                GroupMembers::insert_many(group_links)
                    .exec_without_returning(db)
                    .await
                    .map_err(OrganizationRepositoryError::database)?;
            }
            Ok(())
        })
    }

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
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        let mut rows = Organizations::update_many()
            .col_expr(OrganizationColumn::Name, Expr::value(name))
            .filter(OrganizationColumn::Id.eq(Self::database_id(organization_id)?))
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
        let rows = roles
            .into_iter()
            .map(RoleActiveModel::try_from)
            .collect::<anyhow::Result<Vec<_>>>()?;
        if !rows.is_empty() {
            Roles::insert_many(rows)
                .exec_without_returning(db)
                .await
                .map_err(Self::creation_error)?;
        }
        Ok(())
    }
    async fn add_member(
        &self,
        member: OrganizationMember,
    ) -> Result<OrganizationMember, OrganizationRepositoryError> {
        cedar_policy::Entities::try_from(&member).map_err(OrganizationRepositoryError::database)?;
        let org = Self::database_id(member.organization_id)?;
        let user = Self::database_id(member.user_id)?;
        let model = MemberActiveModel {
            organization_id: Set(org),
            user_id: Set(user),
            status: Set(UserStatusSchema::Active),
        };
        let role_links = member
            .roles
            .iter()
            .map(|r| {
                Ok(MemberRoleActiveModel {
                    organization_id: Set(org),
                    user_id: Set(user),
                    role_id: Set(Self::database_id(r.id)?),
                })
            })
            .collect::<Result<Vec<_>, OrganizationRepositoryError>>()?;
        let group_links = member
            .groups
            .iter()
            .map(|g| {
                Ok(GroupMemberActiveModel {
                    organization_id: Set(org),
                    user_id: Set(user),
                    group_id: Set(Self::database_id(g.id)?),
                })
            })
            .collect::<Result<Vec<_>, OrganizationRepositoryError>>()?;
        let context = DbContext::current()?;
        if role_links.is_empty() && group_links.is_empty() {
            Members::insert(model)
                .exec_without_returning(PostgresConnection::from_context(&context)?)
                .await
                .map_err(OrganizationRepositoryError::database)?;
        } else {
            compallm_core::transaction!(context, {
                let current = DbContext::current()?;
                let db = PostgresConnection::from_context(&current)?;
                Members::insert(model)
                    .exec_without_returning(db)
                    .await
                    .map_err(OrganizationRepositoryError::database)?;
                if !role_links.is_empty() {
                    MemberRoles::insert_many(role_links)
                        .exec_without_returning(db)
                        .await
                        .map_err(OrganizationRepositoryError::database)?;
                }
                if !group_links.is_empty() {
                    GroupMembers::insert_many(group_links)
                        .exec_without_returning(db)
                        .await
                        .map_err(OrganizationRepositoryError::database)?;
                }
                Ok::<_, OrganizationRepositoryError>(())
            })?;
        }
        Ok(member)
    }
}
