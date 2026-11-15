use crate::{
    postgres_connection::PostgresConnection, query::IntoSeaQuery,
    schema::OrganizationMemberInvitationSchema,
};
use compallm_core::{
    authentication::AuthError,
    organization::invitation::Invitation,
    persistent::{DbContext, organization::OrganizationRepositoryError},
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QuerySelect, Set, sea_query::LockType,
};

type Entity = <OrganizationMemberInvitationSchema as sea_orm::ModelTrait>::Entity;
type Column = <Entity as EntityTrait>::Column;
type ActiveModel = <Entity as EntityTrait>::ActiveModel;

#[derive(Default)]
pub struct InvitationRepository;
fn error(value: impl Into<anyhow::Error>) -> AuthError {
    OrganizationRepositoryError::database(value).into()
}
fn id(value: u64) -> Result<i64, AuthError> {
    i64::try_from(value).map_err(error)
}

#[async_trait::async_trait]
impl compallm_core::persistent::invitation::InvitationRepository for InvitationRepository {
    async fn find(
        &self,
        organization_id: u64,
        invitation_id: u64,
        lock: bool,
    ) -> Result<Option<Invitation>, AuthError> {
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        let mut query = Entity::find_by_id((id(organization_id)?, id(invitation_id)?));
        if lock {
            query = query.lock(LockType::Update);
        }
        query
            .one(db)
            .await
            .map_err(error)?
            .map(TryInto::try_into)
            .transpose()
    }
    async fn lock_organization(&self, organization_id: u64) -> Result<(), AuthError> {
        use crate::schema::{OrganizationSchema, OrganizationStatusSchema};
        use sea_orm::ModelTrait;
        type Organizations = <OrganizationSchema as ModelTrait>::Entity;
        type OrgColumn = <Organizations as EntityTrait>::Column;
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        Organizations::find_by_id(id(organization_id)?)
            .filter(OrgColumn::Status.eq(OrganizationStatusSchema::Active))
            .lock(LockType::Update)
            .one(db)
            .await
            .map_err(error)?
            .ok_or(AuthError::InvalidInvitation)?;
        Ok(())
    }
    async fn has_pending(
        &self,
        organization_id: u64,
        email: &str,
        now: u64,
    ) -> Result<bool, AuthError> {
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        Ok(Entity::find()
            .filter(Column::OrganizationId.eq(id(organization_id)?))
            .filter(Column::Email.eq(email))
            .filter(Column::Status.eq("pending"))
            .filter(Column::ExpiresAt.gt(id(now)?))
            .one(db)
            .await
            .map_err(error)?
            .is_some())
    }
    async fn create(&self, value: Invitation) -> Result<Invitation, AuthError> {
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        ActiveModel {
            organization_id: Set(id(value.organization_id)?),
            id: Set(id(value.id)?),
            email: Set(value.email),
            inviter_id: Set(id(value.inviter_id)?),
            status: Set(value.status),
            expires_at: Set(id(value.expires_at)?),
            accepted_at: Set(None),
            accepted_by: Set(None),
            revoked_at: Set(None),
        }
        .insert(db)
        .await
        .map_err(error)?
        .try_into()
    }
    async fn revoke(
        &self,
        organization_id: u64,
        invitation_id: u64,
        now: u64,
    ) -> Result<Invitation, AuthError> {
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        let row = self
            .find(organization_id, invitation_id, true)
            .await?
            .ok_or(AuthError::InvalidInvitation)?;
        if row.status != "pending" {
            return Err(AuthError::InvalidInvitation);
        }
        Entity::update_many()
            .col_expr(Column::Status, sea_orm::sea_query::Expr::value("revoked"))
            .col_expr(Column::RevokedAt, sea_orm::sea_query::Expr::value(id(now)?))
            .filter(Column::OrganizationId.eq(id(organization_id)?))
            .filter(Column::Id.eq(id(invitation_id)?))
            .exec(db)
            .await
            .map_err(error)?;
        self.find(organization_id, invitation_id, false)
            .await?
            .ok_or(AuthError::InvalidInvitation)
    }
    async fn accept(
        &self,
        organization_id: u64,
        invitation_id: u64,
        user_id: u64,
        now: u64,
    ) -> Result<(), AuthError> {
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        let result = Entity::update_many()
            .col_expr(Column::Status, sea_orm::sea_query::Expr::value("accepted"))
            .col_expr(
                Column::AcceptedAt,
                sea_orm::sea_query::Expr::value(id(now)?),
            )
            .col_expr(
                Column::AcceptedBy,
                sea_orm::sea_query::Expr::value(id(user_id)?),
            )
            .filter(Column::OrganizationId.eq(id(organization_id)?))
            .filter(Column::Id.eq(id(invitation_id)?))
            .filter(Column::Status.eq("pending"))
            .filter(Column::ExpiresAt.gt(id(now)?))
            .exec(db)
            .await
            .map_err(error)?;
        if result.rows_affected != 1 {
            return Err(AuthError::InvalidInvitation);
        }
        Ok(())
    }
    async fn membership_exists(
        &self,
        organization_id: u64,
        user_id: u64,
    ) -> Result<bool, AuthError> {
        use sea_orm::ModelTrait;
        type Members = <crate::schema::OrganizationMemberSchema as ModelTrait>::Entity;
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        Ok(Members::find_by_id((id(organization_id)?, id(user_id)?))
            .one(db)
            .await
            .map_err(error)?
            .is_some())
    }
    async fn member_role(&self, organization_id: u64) -> Result<u64, AuthError> {
        use crate::schema::OrganizationRoleSchema;
        use sea_orm::ModelTrait;
        type Roles = <OrganizationRoleSchema as ModelTrait>::Entity;
        type RoleColumn = <Roles as EntityTrait>::Column;
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        let row = Roles::find()
            .filter(RoleColumn::OrganizationId.eq(id(organization_id)?))
            .filter(RoleColumn::DefaultMember.eq(true))
            .one(db)
            .await
            .map_err(error)?
            .ok_or(AuthError::InvalidInvitation)?;
        u64::try_from(row.id).map_err(error)
    }
    async fn list(
        &self,
        query: Box<dyn compallm_query::IntoQuery>,
        pagination: compallm_query::Pagination,
        ordering: Vec<compallm_query::OrderBy>,
        scope: &str,
    ) -> Result<compallm_query::Connection<Invitation>, AuthError> {
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        let page = query
            .plan_page(
                Entity::find()
                    .filter(Column::Status.eq("pending"))
                    .filter(Column::ExpiresAt.gt(id(compallm_utils::time::unix_timestamp())?)),
                pagination,
                ordering,
                100,
                &format!("{scope}:active"),
            )
            .map_err(|_| OrganizationRepositoryError::InvalidQuery)?
            .fetch(db)
            .await
            .map_err(error)?;
        Ok(compallm_query::Connection {
            page_info: page.page_info,
            edges: page
                .edges
                .into_iter()
                .map(|edge| {
                    Ok(compallm_query::Edge {
                        cursor: edge.cursor,
                        node: edge.node.try_into()?,
                    })
                })
                .collect::<Result<_, AuthError>>()?,
        })
    }
}
