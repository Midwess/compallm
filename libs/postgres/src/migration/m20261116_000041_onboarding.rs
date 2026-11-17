use sea_orm_migration::prelude::*;
#[derive(DeriveMigrationName)]
pub struct Migration;
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(include_str!("m20261116_000041_onboarding/up.sql"))
            .await?;
        backfill(manager).await?;
        Ok(())
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.get_connection().execute_unprepared("DROP TABLE public.analytic_users_profile; DROP TABLE public.organization_member_onboarding; DROP TABLE public.organization_onboarding_settings;").await?;
        Ok(())
    }
}

async fn backfill(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    use crate::schema::{OrganizationMemberSchema, OrganizationRoleSchema, UserSessionSchema};
    use sea_orm::{ColumnTrait, EntityTrait, ModelTrait, QueryFilter, sea_query::Expr};
    type Roles = <OrganizationRoleSchema as ModelTrait>::Entity;
    type RoleColumn = <Roles as EntityTrait>::Column;
    type Members = <OrganizationMemberSchema as ModelTrait>::Entity;
    type MemberColumn = <Members as EntityTrait>::Column;
    type Sessions = <UserSessionSchema as ModelTrait>::Entity;
    type SessionColumn = <Sessions as EntityTrait>::Column;
    let db = manager.get_connection();
    let roles = Roles::find().all(db).await?;
    for role in &roles {
        let owner = roles
            .iter()
            .find(|r| r.organization_id == role.organization_id && r.org_owner);
        let Some(owner) = owner else {
            continue;
        };
        let legacy = include_str!("m20261116_000041_onboarding/admin.cedar")
            .replace("__ORG__", &role.organization_id.to_string())
            .replace("__OWNER__", &owner.id.to_string());
        let compact = compallm_core::authorization::policy::compact_policy_source;
        if !role.org_owner && compact(&role.cedar_source) != compact(&legacy) {
            continue;
        }
        let grant = String::from(
            "permit(principal in ?principal, action in [Compallm::Action::\"organization.onboarding.read\",Compallm::Action::\"organization.onboarding.update\"], resource in ?resource);",
        );
        let source = compact(&format!("{}\n{}", role.cedar_source, grant));
        Roles::update_many()
            .col_expr(RoleColumn::CedarSource, Expr::value(source))
            .filter(RoleColumn::OrganizationId.eq(role.organization_id))
            .filter(RoleColumn::Id.eq(role.id))
            .exec(db)
            .await?;
        let users = Members::find()
            .filter(MemberColumn::OrganizationId.eq(role.organization_id))
            .filter(MemberColumn::RoleId.eq(role.id))
            .all(db)
            .await?;
        let mut user_ids: std::collections::BTreeSet<_> =
            users.into_iter().map(|m| m.user_id).collect();
        type GroupRoles = <crate::schema::OrganizationGroupRoleSchema as ModelTrait>::Entity;
        type GroupRoleColumn = <GroupRoles as EntityTrait>::Column;
        type GroupMembers = <crate::schema::OrganizationGroupMemberSchema as ModelTrait>::Entity;
        type GroupMemberColumn = <GroupMembers as EntityTrait>::Column;
        let groups = GroupRoles::find()
            .filter(GroupRoleColumn::OrganizationId.eq(role.organization_id))
            .filter(GroupRoleColumn::RoleId.eq(role.id))
            .all(db)
            .await?;
        let group_ids: Vec<_> = groups.into_iter().map(|g| g.group_id).collect();
        if !group_ids.is_empty() {
            let members = GroupMembers::find()
                .filter(GroupMemberColumn::OrganizationId.eq(role.organization_id))
                .filter(GroupMemberColumn::GroupId.is_in(group_ids))
                .all(db)
                .await?;
            user_ids.extend(members.into_iter().map(|m| m.user_id));
        }
        if !user_ids.is_empty() {
            Sessions::update_many()
                .col_expr(
                    SessionColumn::AuthorizationVersion,
                    Expr::col(SessionColumn::AuthorizationVersion).add(1),
                )
                .filter(SessionColumn::OrganizationId.eq(role.organization_id))
                .filter(SessionColumn::UserId.is_in(user_ids))
                .exec(db)
                .await?;
        }
    }
    Ok(())
}
