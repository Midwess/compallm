use crate::{OrganizationRoleSchema, postgres_connection::PostgresConnection};
use compallm_core::{
    organization::Role,
    persistent::{
        DbContext,
        role::{RoleRepository as CoreRepository, RoleRepositoryError},
    },
    transaction,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbBackend, EntityTrait, FromQueryResult,
    ModelTrait, QueryFilter, QueryOrder, Set, Statement,
};
type Roles = <OrganizationRoleSchema as ModelTrait>::Entity;
type Column = <Roles as EntityTrait>::Column;
type ActiveModel = <Roles as EntityTrait>::ActiveModel;

#[derive(Default)]
pub struct RoleRepository;

fn db_error(error: impl Into<anyhow::Error>) -> RoleRepositoryError {
    RoleRepositoryError::Database(error.into())
}
fn id(value: u64) -> Result<i64, RoleRepositoryError> {
    crate::to_bigint(value).map_err(db_error)
}

#[async_trait::async_trait]
impl CoreRepository for RoleRepository {
    async fn list(&self, organization_id: u64) -> Result<Vec<Role>, RoleRepositoryError> {
        let ctx = DbContext::current()?;
        let db = PostgresConnection::from_context(&ctx)?;
        Roles::find()
            .filter(Column::OrganizationId.eq(id(organization_id)?))
            .order_by_asc(Column::Id)
            .all(db)
            .await
            .map_err(db_error)?
            .into_iter()
            .map(|row| Role::try_from(row).map_err(db_error))
            .collect()
    }
    async fn find(
        &self,
        organization_id: u64,
        role_id: u64,
    ) -> Result<Option<Role>, RoleRepositoryError> {
        let ctx = DbContext::current()?;
        let db = PostgresConnection::from_context(&ctx)?;
        Roles::find_by_id((id(organization_id)?, id(role_id)?))
            .one(db)
            .await
            .map_err(db_error)?
            .map(|row| Role::try_from(row).map_err(db_error))
            .transpose()
    }
    async fn create(&self, role: Role) -> Result<Role, RoleRepositoryError> {
        if role.id <= Role::MEMBER_ID || role.revision != 1 {
            return Err(RoleRepositoryError::Conflict);
        }
        let ctx = DbContext::current()?;
        let db = PostgresConnection::from_context(&ctx)?;
        let model = ActiveModel {
            organization_id: Set(id(role.organization_id)?),
            id: Set(id(role.id)?),
            name: Set(role.name),
            description: Set(role.description),
            cedar_source: Set(role.cedar_source),
            revision: Set(1),
        };
        Role::try_from(model.insert(db).await.map_err(db_error)?).map_err(db_error)
    }
    async fn update(&self, role: Role) -> Result<Role, RoleRepositoryError> {
        if role.id <= Role::MEMBER_ID || role.revision == i64::MAX as u64 {
            return Err(RoleRepositoryError::Conflict);
        }
        let org = id(role.organization_id)?;
        let role_id = id(role.id)?;
        let expected = id(role.revision)?;
        let ctx = DbContext::current()?;
        transaction!(&ctx, {
            let current = DbContext::current()?;
            let db = PostgresConnection::from_context(&current)?;
            let row=db.query_one_raw(Statement::from_sql_and_values(DbBackend::Postgres,
                "UPDATE organization_roles SET name=$3,description=$4,cedar_source=$5,revision=revision+1 WHERE organization_id=$1 AND id=$2 AND revision=$6 AND id>3 RETURNING *",
                [org.into(),role_id.into(),role.name.into(),role.description.into(),role.cedar_source.into(),expected.into()])).await.map_err(db_error)?.ok_or(RoleRepositoryError::Conflict)?;
            // Conservative invalidation for every accepted edit prevents a caller
            // from suppressing invalidation by misclassifying a source change.
            db.execute_raw(Statement::from_sql_and_values(DbBackend::Postgres,r#"
UPDATE user_sessions SET
 authorization_version=CASE WHEN authorization_version=4294967295 THEN authorization_version ELSE authorization_version+1 END,
 revoked_at=CASE WHEN authorization_version=4294967295 THEN COALESCE(revoked_at,CURRENT_TIMESTAMP) ELSE revoked_at END
WHERE organization_id=$1 AND revoked_at IS NULL AND user_id IN (
 SELECT user_id FROM organization_member_roles WHERE organization_id=$1 AND role_id=$2
 UNION
 SELECT gm.user_id FROM organization_group_roles gr JOIN organization_group_members gm
 ON gm.organization_id=gr.organization_id AND gm.group_id=gr.group_id WHERE gr.organization_id=$1 AND gr.role_id=$2
)
"#,[org.into(),role_id.into()])).await.map_err(db_error)?;
            Role::try_from(OrganizationRoleSchema::from_query_result(&row, "").map_err(db_error)?)
                .map_err(db_error)
        })
    }
}
