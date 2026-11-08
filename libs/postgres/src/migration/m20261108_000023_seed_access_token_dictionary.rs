//! Immutable, manually curated vocabulary for user access-token compression.
//! Later additions belong in new versioned migrations; never rewrite this seed.
use sea_orm::{DbBackend, Statement};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

// Stable, synthetic token fields, Cedar syntax and all currently declared actions.
// Values from real users, organizations and signing keys are never included.
const ACCESS_TOKEN_DICTIONARY: &str = r###"{"exp":0,"iat":0,"data":{"user":{"session":{"id":0,"user_id":0,"organization_id":0,"organization_name":"","authorization_version":1,"revoked_at":null},"policy_snapshot":{"cedar_source":"","role_ids":[],"group_roles":{}},"validity":{"issued_at":0,"expires_at":0}}}}
permit(principal in Compallm::Role::"
,action in [
],resource in Compallm::Organization::"
)when{context.role.organization==Compallm::Organization::"
)when{context.member.organization==Compallm::Organization::"
)when{context.providerModel.organization==Compallm::Organization::"
)unless{resource==Compallm::Role::"
forbid(principal in Compallm::Role::"
,action==Compallm::Action::"model.use",resource is Compallm::Model in Compallm::Organization::"
)unless{resource.enabled};
Compallm::Action:: Compallm::Organization:: Compallm::Role:: Compallm::Group:: Compallm::Member:: Compallm::Model:: Compallm::Provider:: Compallm::ProviderModel:: Compallm::User:: Compallm::UserAuthentication:: Compallm::Session::
principal action resource context permit forbid when unless is in true false has like entity uid enabled organization user role provider providerModel member
"session" "user_id" "organization_id" "organization_name" "authorization_version" "revoked_at" "policy_snapshot" "dictionary_version" "dictionary_type" "compressed" "access_token" "cedar_source" "role_ids" "group_roles" "validity" "issued_at" "expires_at" "data" "exp" "iat" "Owner" "Admin" "Member"
Compallm::Action::"group.create"
Compallm::Action::"group.delete"
Compallm::Action::"group.list"
Compallm::Action::"group.member.add"
Compallm::Action::"group.member.remove"
Compallm::Action::"group.metadata.read"
Compallm::Action::"group.metadata.update"
Compallm::Action::"group.role.assign"
Compallm::Action::"group.role.unassign"
Compallm::Action::"member.invite"
Compallm::Action::"member.list"
Compallm::Action::"member.metadata.read"
Compallm::Action::"member.remove"
Compallm::Action::"member.role.assign"
Compallm::Action::"member.role.unassign"
Compallm::Action::"model.configuration.update"
Compallm::Action::"model.create"
Compallm::Action::"model.delete"
Compallm::Action::"model.list"
Compallm::Action::"model.metadata.read"
Compallm::Action::"model.pricing.update"
Compallm::Action::"model.route.add"
Compallm::Action::"model.route.remove"
Compallm::Action::"model.route.update"
Compallm::Action::"model.use"
Compallm::Action::"organization.delete"
Compallm::Action::"organization.settings.read"
Compallm::Action::"organization.settings.update"
Compallm::Action::"provider.configuration.read"
Compallm::Action::"provider.configuration.update"
Compallm::Action::"provider.connection.test"
Compallm::Action::"provider.create"
Compallm::Action::"provider.credential.replace"
Compallm::Action::"provider.delete"
Compallm::Action::"provider.list"
Compallm::Action::"provider.model.configuration.read"
Compallm::Action::"provider.model.configuration.update"
Compallm::Action::"provider.model.create"
Compallm::Action::"provider.model.delete"
Compallm::Action::"provider.model.list"
Compallm::Action::"provider.model.pricing.update"
Compallm::Action::"role.assign"
Compallm::Action::"role.create"
Compallm::Action::"role.delete"
Compallm::Action::"role.list"
Compallm::Action::"role.metadata.read"
Compallm::Action::"role.metadata.update"
Compallm::Action::"role.policy.read"
Compallm::Action::"role.policy.update"
Compallm::Action::"role.unassign"
Compallm::Action::"session.list"
Compallm::Action::"session.metadata.read"
Compallm::Action::"session.revoke"
Compallm::Action::"user.account.delete"
Compallm::Action::"user.authentication.read"
Compallm::Action::"user.authentication.update"
Compallm::Action::"user.profile.update"

permit(principal in Compallm::Role::"__ORG__:__OWNER__",action in[Compallm::Action::"organization.settings.read",Compallm::Action::"organization.settings.update",Compallm::Action::"organization.delete",Compallm::Action::"member.list",Compallm::Action::"member.invite",Compallm::Action::"member.metadata.read",Compallm::Action::"member.remove",Compallm::Action::"provider.list",Compallm::Action::"provider.create",Compallm::Action::"provider.connection.test",Compallm::Action::"provider.configuration.read",Compallm::Action::"provider.configuration.update",Compallm::Action::"provider.delete",Compallm::Action::"provider.credential.replace",Compallm::Action::"provider.model.list",Compallm::Action::"provider.model.create",Compallm::Action::"provider.model.configuration.read",Compallm::Action::"provider.model.configuration.update",Compallm::Action::"provider.model.pricing.update",Compallm::Action::"provider.model.delete",Compallm::Action::"model.list",Compallm::Action::"model.create",Compallm::Action::"model.metadata.read",Compallm::Action::"model.configuration.update",Compallm::Action::"model.delete",Compallm::Action::"model.use",Compallm::Action::"model.pricing.update",Compallm::Action::"role.list",Compallm::Action::"role.create",Compallm::Action::"role.metadata.read",Compallm::Action::"role.metadata.update",Compallm::Action::"role.policy.read",Compallm::Action::"role.policy.update",Compallm::Action::"role.delete",Compallm::Action::"role.assign",Compallm::Action::"role.unassign",Compallm::Action::"group.list",Compallm::Action::"group.create",Compallm::Action::"group.metadata.read",Compallm::Action::"group.metadata.update",Compallm::Action::"group.delete"],resource in Compallm::Organization::"__ORG__");permit(principal in Compallm::Role::"__ORG__:__OWNER__",action in[Compallm::Action::"member.role.assign",Compallm::Action::"member.role.unassign",Compallm::Action::"group.role.assign",Compallm::Action::"group.role.unassign"],resource in Compallm::Organization::"__ORG__")when{context.role.organization==Compallm::Organization::"__ORG__"};permit(principal in Compallm::Role::"__ORG__:__OWNER__",action in[Compallm::Action::"group.member.add",Compallm::Action::"group.member.remove"],resource in Compallm::Organization::"__ORG__")when{context.member.organization==Compallm::Organization::"__ORG__"};permit(principal in Compallm::Role::"__ORG__:__OWNER__",action in[Compallm::Action::"model.route.add",Compallm::Action::"model.route.update",Compallm::Action::"model.route.remove"],resource in Compallm::Organization::"__ORG__")when{context.providerModel.organization==Compallm::Organization::"__ORG__"};forbid(principal in Compallm::Role::"__ORG__:__OWNER__",action==Compallm::Action::"model.use",resource is Compallm::Model in Compallm::Organization::"__ORG__")unless{resource.enabled};
permit(principal in Compallm::Role::\"__ORG__:__OWNER__\",action in[Compallm::Action::\"organization.settings.read\",Compallm::Action::\"organization.settings.update\",Compallm::Action::\"organization.delete\",Compallm::Action::\"member.list\",Compallm::Action::\"member.invite\",Compallm::Action::\"member.metadata.read\",Compallm::Action::\"member.remove\",Compallm::Action::\"provider.list\",Compallm::Action::\"provider.create\",Compallm::Action::\"provider.connection.test\",Compallm::Action::\"provider.configuration.read\",Compallm::Action::\"provider.configuration.update\",Compallm::Action::\"provider.delete\",Compallm::Action::\"provider.credential.replace\",Compallm::Action::\"provider.model.list\",Compallm::Action::\"provider.model.create\",Compallm::Action::\"provider.model.configuration.read\",Compallm::Action::\"provider.model.configuration.update\",Compallm::Action::\"provider.model.pricing.update\",Compallm::Action::\"provider.model.delete\",Compallm::Action::\"model.list\",Compallm::Action::\"model.create\",Compallm::Action::\"model.metadata.read\",Compallm::Action::\"model.configuration.update\",Compallm::Action::\"model.delete\",Compallm::Action::\"model.use\",Compallm::Action::\"model.pricing.update\",Compallm::Action::\"role.list\",Compallm::Action::\"role.create\",Compallm::Action::\"role.metadata.read\",Compallm::Action::\"role.metadata.update\",Compallm::Action::\"role.policy.read\",Compallm::Action::\"role.policy.update\",Compallm::Action::\"role.delete\",Compallm::Action::\"role.assign\",Compallm::Action::\"role.unassign\",Compallm::Action::\"group.list\",Compallm::Action::\"group.create\",Compallm::Action::\"group.metadata.read\",Compallm::Action::\"group.metadata.update\",Compallm::Action::\"group.delete\"],resource in Compallm::Organization::\"__ORG__\");permit(principal in Compallm::Role::\"__ORG__:__OWNER__\",action in[Compallm::Action::\"member.role.assign\",Compallm::Action::\"member.role.unassign\",Compallm::Action::\"group.role.assign\",Compallm::Action::\"group.role.unassign\"],resource in Compallm::Organization::\"__ORG__\")when{context.role.organization==Compallm::Organization::\"__ORG__\"};permit(principal in Compallm::Role::\"__ORG__:__OWNER__\",action in[Compallm::Action::\"group.member.add\",Compallm::Action::\"group.member.remove\"],resource in Compallm::Organization::\"__ORG__\")when{context.member.organization==Compallm::Organization::\"__ORG__\"};permit(principal in Compallm::Role::\"__ORG__:__OWNER__\",action in[Compallm::Action::\"model.route.add\",Compallm::Action::\"model.route.update\",Compallm::Action::\"model.route.remove\"],resource in Compallm::Organization::\"__ORG__\")when{context.providerModel.organization==Compallm::Organization::\"__ORG__\"};forbid(principal in Compallm::Role::\"__ORG__:__OWNER__\",action==Compallm::Action::\"model.use\",resource is Compallm::Model in Compallm::Organization::\"__ORG__\")unless{resource.enabled};
permit(principal in Compallm::Role::"__ORG__:__ADMIN__",action in[Compallm::Action::"organization.settings.read",Compallm::Action::"organization.settings.update",Compallm::Action::"member.list",Compallm::Action::"member.invite",Compallm::Action::"member.metadata.read",Compallm::Action::"member.remove",Compallm::Action::"provider.list",Compallm::Action::"provider.create",Compallm::Action::"provider.connection.test",Compallm::Action::"provider.configuration.read",Compallm::Action::"provider.configuration.update",Compallm::Action::"provider.delete",Compallm::Action::"provider.credential.replace",Compallm::Action::"provider.model.list",Compallm::Action::"provider.model.create",Compallm::Action::"provider.model.configuration.read",Compallm::Action::"provider.model.configuration.update",Compallm::Action::"provider.model.pricing.update",Compallm::Action::"provider.model.delete",Compallm::Action::"model.list",Compallm::Action::"model.create",Compallm::Action::"model.metadata.read",Compallm::Action::"model.configuration.update",Compallm::Action::"model.delete",Compallm::Action::"model.use",Compallm::Action::"model.pricing.update",Compallm::Action::"role.list",Compallm::Action::"role.create",Compallm::Action::"role.metadata.read",Compallm::Action::"role.metadata.update",Compallm::Action::"role.policy.read",Compallm::Action::"role.policy.update",Compallm::Action::"role.delete",Compallm::Action::"group.list",Compallm::Action::"group.create",Compallm::Action::"group.metadata.read",Compallm::Action::"group.metadata.update",Compallm::Action::"group.delete"],resource in Compallm::Organization::"__ORG__");permit(principal in Compallm::Role::"__ORG__:__ADMIN__",action in[Compallm::Action::"member.role.assign",Compallm::Action::"member.role.unassign",Compallm::Action::"group.role.assign",Compallm::Action::"group.role.unassign"],resource in Compallm::Organization::"__ORG__")when{context.role.organization==Compallm::Organization::"__ORG__" && context.role !=Compallm::Role::"__ORG__:__OWNER__"};permit(principal in Compallm::Role::"__ORG__:__ADMIN__",action in[Compallm::Action::"group.member.add",Compallm::Action::"group.member.remove"],resource in Compallm::Organization::"__ORG__")when{context.member.organization==Compallm::Organization::"__ORG__"}unless{resource in Compallm::Role::"__ORG__:__OWNER__"};permit(principal in Compallm::Role::"__ORG__:__ADMIN__",action in[Compallm::Action::"model.route.add",Compallm::Action::"model.route.update",Compallm::Action::"model.route.remove"],resource in Compallm::Organization::"__ORG__")when{context.providerModel.organization==Compallm::Organization::"__ORG__"};permit(principal in Compallm::Role::"__ORG__:__ADMIN__",action in[Compallm::Action::"role.assign",Compallm::Action::"role.unassign"],resource is Compallm::Role in Compallm::Organization::"__ORG__")unless{resource==Compallm::Role::"__ORG__:__OWNER__"};forbid(principal in Compallm::Role::"__ORG__:__ADMIN__",action==Compallm::Action::"model.use",resource is Compallm::Model in Compallm::Organization::"__ORG__")unless{resource.enabled};
permit(principal in Compallm::Role::\"__ORG__:__ADMIN__\",action in[Compallm::Action::\"organization.settings.read\",Compallm::Action::\"organization.settings.update\",Compallm::Action::\"member.list\",Compallm::Action::\"member.invite\",Compallm::Action::\"member.metadata.read\",Compallm::Action::\"member.remove\",Compallm::Action::\"provider.list\",Compallm::Action::\"provider.create\",Compallm::Action::\"provider.connection.test\",Compallm::Action::\"provider.configuration.read\",Compallm::Action::\"provider.configuration.update\",Compallm::Action::\"provider.delete\",Compallm::Action::\"provider.credential.replace\",Compallm::Action::\"provider.model.list\",Compallm::Action::\"provider.model.create\",Compallm::Action::\"provider.model.configuration.read\",Compallm::Action::\"provider.model.configuration.update\",Compallm::Action::\"provider.model.pricing.update\",Compallm::Action::\"provider.model.delete\",Compallm::Action::\"model.list\",Compallm::Action::\"model.create\",Compallm::Action::\"model.metadata.read\",Compallm::Action::\"model.configuration.update\",Compallm::Action::\"model.delete\",Compallm::Action::\"model.use\",Compallm::Action::\"model.pricing.update\",Compallm::Action::\"role.list\",Compallm::Action::\"role.create\",Compallm::Action::\"role.metadata.read\",Compallm::Action::\"role.metadata.update\",Compallm::Action::\"role.policy.read\",Compallm::Action::\"role.policy.update\",Compallm::Action::\"role.delete\",Compallm::Action::\"group.list\",Compallm::Action::\"group.create\",Compallm::Action::\"group.metadata.read\",Compallm::Action::\"group.metadata.update\",Compallm::Action::\"group.delete\"],resource in Compallm::Organization::\"__ORG__\");permit(principal in Compallm::Role::\"__ORG__:__ADMIN__\",action in[Compallm::Action::\"member.role.assign\",Compallm::Action::\"member.role.unassign\",Compallm::Action::\"group.role.assign\",Compallm::Action::\"group.role.unassign\"],resource in Compallm::Organization::\"__ORG__\")when{context.role.organization==Compallm::Organization::\"__ORG__\" && context.role !=Compallm::Role::\"__ORG__:__OWNER__\"};permit(principal in Compallm::Role::\"__ORG__:__ADMIN__\",action in[Compallm::Action::\"group.member.add\",Compallm::Action::\"group.member.remove\"],resource in Compallm::Organization::\"__ORG__\")when{context.member.organization==Compallm::Organization::\"__ORG__\"}unless{resource in Compallm::Role::\"__ORG__:__OWNER__\"};permit(principal in Compallm::Role::\"__ORG__:__ADMIN__\",action in[Compallm::Action::\"model.route.add\",Compallm::Action::\"model.route.update\",Compallm::Action::\"model.route.remove\"],resource in Compallm::Organization::\"__ORG__\")when{context.providerModel.organization==Compallm::Organization::\"__ORG__\"};permit(principal in Compallm::Role::\"__ORG__:__ADMIN__\",action in[Compallm::Action::\"role.assign\",Compallm::Action::\"role.unassign\"],resource is Compallm::Role in Compallm::Organization::\"__ORG__\")unless{resource==Compallm::Role::\"__ORG__:__OWNER__\"};forbid(principal in Compallm::Role::\"__ORG__:__ADMIN__\",action==Compallm::Action::\"model.use\",resource is Compallm::Model in Compallm::Organization::\"__ORG__\")unless{resource.enabled};
permit(principal in Compallm::Role::"__ORG__:__MEMBER__",action==Compallm::Action::"model.list",resource==Compallm::Organization::"__ORG__");permit(principal in Compallm::Role::"__ORG__:__MEMBER__",action==Compallm::Action::"model.metadata.read",resource is Compallm::Model in Compallm::Organization::"__ORG__");permit(principal in Compallm::Role::"__ORG__:__MEMBER__",action==Compallm::Action::"model.use",resource is Compallm::Model in Compallm::Organization::"__ORG__")when{resource.enabled};
permit(principal in Compallm::Role::\"__ORG__:__MEMBER__\",action==Compallm::Action::\"model.list\",resource==Compallm::Organization::\"__ORG__\");permit(principal in Compallm::Role::\"__ORG__:__MEMBER__\",action==Compallm::Action::\"model.metadata.read\",resource is Compallm::Model in Compallm::Organization::\"__ORG__\");permit(principal in Compallm::Role::\"__ORG__:__MEMBER__\",action==Compallm::Action::\"model.use\",resource is Compallm::Model in Compallm::Organization::\"__ORG__\")when{resource.enabled};
"###;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let row = manager
            .get_connection()
            .query_one_raw(Statement::from_sql_and_values(
                DbBackend::Postgres,
                "INSERT INTO zstd_dictionary (type, content) VALUES ('access_token', $1) RETURNING version",
                [ACCESS_TOKEN_DICTIONARY.into()],
            ))
            .await?
            .ok_or_else(|| DbErr::Custom("dictionary seed did not return its version".into()))?;
        let version: i32 = row.try_get("", "version")?;
        if version != 1 {
            return Err(DbErr::Custom("initial dictionary must be version 1".into()));
        }
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Refuse to remove a prefix if later dictionary versions depend on it.
        manager.get_connection().execute_unprepared(
            "DO $$ BEGIN IF EXISTS (SELECT 1 FROM zstd_dictionary WHERE type = 'access_token' AND version > 1) THEN RAISE EXCEPTION 'later access-token dictionaries depend on version 1'; END IF; END $$"
        ).await?;
        manager.get_connection().execute_raw(Statement::from_sql_and_values(
            DbBackend::Postgres,
            "DELETE FROM zstd_dictionary WHERE type = 'access_token' AND version = 1 AND content = $1",
            [ACCESS_TOKEN_DICTIONARY.into()],
        )).await?;
        Ok(())
    }
}
