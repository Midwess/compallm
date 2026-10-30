use anyhow::Context;
use compallm_core::{
    organization::{Organization, OrganizationMember, OrganizationRole, OrganizationStatus},
    persistent::{
        DbContext,
        organization::{OrganizationRepository as CoreRepository, OrganizationRepositoryError},
    },
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, ModelTrait, QueryFilter, QueryOrder, Set,
    sea_query::Expr,
};

use crate::{
    postgres_connection::PostgresConnection,
    schema::{
        OrganizationMemberSchema, OrganizationRoleSchema, OrganizationSchema,
        OrganizationStatusSchema,
    },
};

type Organizations = <OrganizationSchema as ModelTrait>::Entity;
type OrganizationActiveModel = <Organizations as EntityTrait>::ActiveModel;
type OrganizationColumn = <Organizations as EntityTrait>::Column;
type Members = <OrganizationMemberSchema as ModelTrait>::Entity;
type MemberActiveModel = <Members as EntityTrait>::ActiveModel;
type MemberColumn = <Members as EntityTrait>::Column;

#[derive(Default)]
pub struct OrganizationRepository;

impl OrganizationRepository {
    async fn membership(
        &self,
        user_id: u64,
        organization_id: Option<u64>,
        admin_only: bool,
    ) -> Result<Option<OrganizationMember>, OrganizationRepositoryError> {
        let user_id = Self::database_id(user_id)?;
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        let mut query = Members::find()
            .inner_join(Organizations::default())
            .filter(MemberColumn::UserId.eq(user_id))
            .filter(OrganizationColumn::Status.eq(OrganizationStatusSchema::Active));
        if let Some(organization_id) = organization_id {
            query =
                query.filter(MemberColumn::OrganizationId.eq(Self::database_id(organization_id)?));
        }
        if admin_only {
            query = query.filter(
                MemberColumn::Role
                    .is_in([OrganizationRoleSchema::Owner, OrganizationRoleSchema::Admin]),
            );
        }
        query
            .order_by_asc(MemberColumn::OrganizationId)
            .one(db)
            .await
            .map_err(OrganizationRepositoryError::database)?
            .map(OrganizationMember::try_from)
            .transpose()
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
    ) -> Result<Option<OrganizationMember>, OrganizationRepositoryError> {
        self.membership(user_id, Some(organization_id), false).await
    }

    async fn find_first_membership(
        &self,
        user_id: u64,
    ) -> Result<Option<OrganizationMember>, OrganizationRepositoryError> {
        self.membership(user_id, None, false).await
    }

    async fn find_first_admin_membership(
        &self,
        user_id: u64,
    ) -> Result<Option<OrganizationMember>, OrganizationRepositoryError> {
        self.membership(user_id, None, true).await
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
            ..Default::default()
        };
        Organization::try_from(
            model
                .insert(db)
                .await
                .map_err(OrganizationRepositoryError::database)?,
        )
    }

    async fn add_member(
        &self,
        member: OrganizationMember,
    ) -> Result<OrganizationMember, OrganizationRepositoryError> {
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        let model = MemberActiveModel {
            organization_id: Set(Self::database_id(member.organization_id)?),
            user_id: Set(Self::database_id(member.user_id)?),
            role: Set(match member.role {
                OrganizationRole::Owner => OrganizationRoleSchema::Owner,
                OrganizationRole::Admin => OrganizationRoleSchema::Admin,
                OrganizationRole::Member => OrganizationRoleSchema::Member,
            }),
            ..Default::default()
        };
        OrganizationMember::try_from(
            model
                .insert(db)
                .await
                .map_err(OrganizationRepositoryError::database)?,
        )
    }
}
