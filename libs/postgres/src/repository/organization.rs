use anyhow::Context;
use compallm_core::{
    organization::{Organization, OrganizationMember, OrganizationRole, OrganizationStatus},
    persistent::{
        DbContext,
        organization::{OrganizationRepository as CoreRepository, OrganizationRepositoryError},
    },
};
use sea_orm::{ActiveModelTrait, EntityTrait, ModelTrait, Set};

use crate::{
    postgres_connection::PostgresConnection,
    schema::{
        OrganizationMemberSchema, OrganizationRoleSchema, OrganizationSchema,
        OrganizationStatusSchema,
    },
};

type Organizations = <OrganizationSchema as ModelTrait>::Entity;
type OrganizationActiveModel = <Organizations as EntityTrait>::ActiveModel;
type Members = <OrganizationMemberSchema as ModelTrait>::Entity;
type MemberActiveModel = <Members as EntityTrait>::ActiveModel;

#[derive(Default)]
pub struct OrganizationRepository;

impl OrganizationRepository {
    fn database_id(id: u64) -> Result<i64, OrganizationRepositoryError> {
        if id == 0 {
            return Err(OrganizationRepositoryError::database(anyhow::anyhow!(
                "organization and member IDs must be positive"
            )));
        }
        Ok(crate::to_bigint(id).context("organization or member ID exceeds PostgreSQL BIGINT")?)
    }
}

#[async_trait::async_trait]
impl CoreRepository for OrganizationRepository {
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
            created_by: Set(Self::database_id(organization.created_by)?),
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
