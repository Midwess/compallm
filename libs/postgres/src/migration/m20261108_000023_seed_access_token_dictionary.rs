//! Synthetic, ordered and human-reviewable items for the initial LZ4 dictionary.
use sea_orm::{DbBackend, Statement, Value};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

const ITEMS: &[&str] = &[
    r####"{"u":{"s":{"i":101,"u":202,"o":303,"n":"Workspace","v":1,"r":null},"p":{"c":"","r":[404],"g":{}},"v":{"i":1700000000,"e":1700001800}}}"####,
    r####"{"r":{"s":{"i":101,"u":202,"o":303,"n":"Workspace","v":1,"r":null},"v":{"i":1700000000,"e":1702592000}}}"####,
    r####"{"e":"v1.encrypted_email_challenge"}"####,
    r####"{"c":{"v":1,"d":"compressed_token_payload"}}"####,
    r####"{"o":{"e":"person@example.test","c":"123456","v":{"i":1700000000,"e":1700000600}}}"####,
    r####"permit(principal in Compallm::Role::""####,
    r####",action in ["####,
    r####"],resource in Compallm::Organization::""####,
    r####")when{context.role.organization==Compallm::Organization::""####,
    r####")when{context.member.organization==Compallm::Organization::""####,
    r####")when{context.providerModel.organization==Compallm::Organization::""####,
    r####")unless{resource==Compallm::Role::""####,
    r####"forbid(principal in Compallm::Role::""####,
    r####",action==Compallm::Action::"model.use",resource is Compallm::Model in Compallm::Organization::""####,
    r####")unless{resource.enabled};"####,
    r####"Compallm::Action:: Compallm::Organization:: Compallm::Role:: Compallm::Group:: Compallm::Member:: Compallm::Model:: Compallm::Provider:: Compallm::ProviderModel:: Compallm::User:: Compallm::UserAuthentication:: Compallm::Session::"####,
    r####"principal action resource context permit forbid when unless is in true false has like entity uid enabled organization user role provider providerModel member"####,
    r####""session" "user_id" "organization_id" "organization_name" "authorization_version" "revoked_at" "policy_snapshot" "dictionary_version" "dictionary_type" "compressed" "access_token" "cedar_source" "role_ids" "group_roles" "validity" "issued_at" "expires_at" "data" "exp" "iat" "Owner" "Admin" "Member""####,
    r####"Compallm::Action::"group.create""####,
    r####"Compallm::Action::"group.delete""####,
    r####"Compallm::Action::"group.list""####,
    r####"Compallm::Action::"group.member.add""####,
    r####"Compallm::Action::"group.member.remove""####,
    r####"Compallm::Action::"group.metadata.read""####,
    r####"Compallm::Action::"group.metadata.update""####,
    r####"Compallm::Action::"group.role.assign""####,
    r####"Compallm::Action::"group.role.unassign""####,
    r####"Compallm::Action::"member.invite""####,
    r####"Compallm::Action::"member.list""####,
    r####"Compallm::Action::"member.metadata.read""####,
    r####"Compallm::Action::"member.remove""####,
    r####"Compallm::Action::"member.role.assign""####,
    r####"Compallm::Action::"member.role.unassign""####,
    r####"Compallm::Action::"model.configuration.update""####,
    r####"Compallm::Action::"model.create""####,
    r####"Compallm::Action::"model.delete""####,
    r####"Compallm::Action::"model.list""####,
    r####"Compallm::Action::"model.metadata.read""####,
    r####"Compallm::Action::"model.pricing.update""####,
    r####"Compallm::Action::"model.route.add""####,
    r####"Compallm::Action::"model.route.remove""####,
    r####"Compallm::Action::"model.route.update""####,
    r####"Compallm::Action::"model.use""####,
    r####"Compallm::Action::"organization.delete""####,
    r####"Compallm::Action::"organization.settings.read""####,
    r####"Compallm::Action::"organization.settings.update""####,
    r####"Compallm::Action::"provider.configuration.read""####,
    r####"Compallm::Action::"provider.configuration.update""####,
    r####"Compallm::Action::"provider.connection.test""####,
    r####"Compallm::Action::"provider.create""####,
    r####"Compallm::Action::"provider.credential.replace""####,
    r####"Compallm::Action::"provider.delete""####,
    r####"Compallm::Action::"provider.list""####,
    r####"Compallm::Action::"provider.model.configuration.read""####,
    r####"Compallm::Action::"provider.model.configuration.update""####,
    r####"Compallm::Action::"provider.model.create""####,
    r####"Compallm::Action::"provider.model.delete""####,
    r####"Compallm::Action::"provider.model.list""####,
    r####"Compallm::Action::"provider.model.pricing.update""####,
    r####"Compallm::Action::"role.assign""####,
    r####"Compallm::Action::"role.create""####,
    r####"Compallm::Action::"role.delete""####,
    r####"Compallm::Action::"role.list""####,
    r####"Compallm::Action::"role.metadata.read""####,
    r####"Compallm::Action::"role.metadata.update""####,
    r####"Compallm::Action::"role.policy.read""####,
    r####"Compallm::Action::"role.policy.update""####,
    r####"Compallm::Action::"role.unassign""####,
    r####"Compallm::Action::"session.list""####,
    r####"Compallm::Action::"session.metadata.read""####,
    r####"Compallm::Action::"session.revoke""####,
    r####"Compallm::Action::"user.account.delete""####,
    r####"Compallm::Action::"user.authentication.read""####,
    r####"Compallm::Action::"user.authentication.update""####,
    r####"Compallm::Action::"user.profile.update""####,
    r####"permit(principal in Compallm::Role::"__ORG__:__OWNER__",action in[Compallm::Action::"organization.settings.read",Compallm::Action::"organization.settings.update",Compallm::Action::"organization.delete",Compallm::Action::"member.list",Compallm::Action::"member.invite",Compallm::Action::"member.metadata.read",Compallm::Action::"member.remove",Compallm::Action::"provider.list",Compallm::Action::"provider.create",Compallm::Action::"provider.connection.test",Compallm::Action::"provider.configuration.read",Compallm::Action::"provider.configuration.update",Compallm::Action::"provider.delete",Compallm::Action::"provider.credential.replace",Compallm::Action::"provider.model.list",Compallm::Action::"provider.model.create",Compallm::Action::"provider.model.configuration.read",Compallm::Action::"provider.model.configuration.update",Compallm::Action::"provider.model.pricing.update",Compallm::Action::"provider.model.delete",Compallm::Action::"model.list",Compallm::Action::"model.create",Compallm::Action::"model.metadata.read",Compallm::Action::"model.configuration.update",Compallm::Action::"model.delete",Compallm::Action::"model.use",Compallm::Action::"model.pricing.update",Compallm::Action::"role.list",Compallm::Action::"role.create",Compallm::Action::"role.metadata.read",Compallm::Action::"role.metadata.update",Compallm::Action::"role.policy.read",Compallm::Action::"role.policy.update",Compallm::Action::"role.delete",Compallm::Action::"role.assign",Compallm::Action::"role.unassign",Compallm::Action::"group.list",Compallm::Action::"group.create",Compallm::Action::"group.metadata.read",Compallm::Action::"group.metadata.update",Compallm::Action::"group.delete"],resource in Compallm::Organization::"__ORG__");permit(principal in Compallm::Role::"__ORG__:__OWNER__",action in[Compallm::Action::"member.role.assign",Compallm::Action::"member.role.unassign",Compallm::Action::"group.role.assign",Compallm::Action::"group.role.unassign"],resource in Compallm::Organization::"__ORG__")when{context.role.organization==Compallm::Organization::"__ORG__"};permit(principal in Compallm::Role::"__ORG__:__OWNER__",action in[Compallm::Action::"group.member.add",Compallm::Action::"group.member.remove"],resource in Compallm::Organization::"__ORG__")when{context.member.organization==Compallm::Organization::"__ORG__"};permit(principal in Compallm::Role::"__ORG__:__OWNER__",action in[Compallm::Action::"model.route.add",Compallm::Action::"model.route.update",Compallm::Action::"model.route.remove"],resource in Compallm::Organization::"__ORG__")when{context.providerModel.organization==Compallm::Organization::"__ORG__"};forbid(principal in Compallm::Role::"__ORG__:__OWNER__",action==Compallm::Action::"model.use",resource is Compallm::Model in Compallm::Organization::"__ORG__")unless{resource.enabled};"####,
    r####"permit(principal in Compallm::Role::\"__ORG__:__OWNER__\",action in[Compallm::Action::\"organization.settings.read\",Compallm::Action::\"organization.settings.update\",Compallm::Action::\"organization.delete\",Compallm::Action::\"member.list\",Compallm::Action::\"member.invite\",Compallm::Action::\"member.metadata.read\",Compallm::Action::\"member.remove\",Compallm::Action::\"provider.list\",Compallm::Action::\"provider.create\",Compallm::Action::\"provider.connection.test\",Compallm::Action::\"provider.configuration.read\",Compallm::Action::\"provider.configuration.update\",Compallm::Action::\"provider.delete\",Compallm::Action::\"provider.credential.replace\",Compallm::Action::\"provider.model.list\",Compallm::Action::\"provider.model.create\",Compallm::Action::\"provider.model.configuration.read\",Compallm::Action::\"provider.model.configuration.update\",Compallm::Action::\"provider.model.pricing.update\",Compallm::Action::\"provider.model.delete\",Compallm::Action::\"model.list\",Compallm::Action::\"model.create\",Compallm::Action::\"model.metadata.read\",Compallm::Action::\"model.configuration.update\",Compallm::Action::\"model.delete\",Compallm::Action::\"model.use\",Compallm::Action::\"model.pricing.update\",Compallm::Action::\"role.list\",Compallm::Action::\"role.create\",Compallm::Action::\"role.metadata.read\",Compallm::Action::\"role.metadata.update\",Compallm::Action::\"role.policy.read\",Compallm::Action::\"role.policy.update\",Compallm::Action::\"role.delete\",Compallm::Action::\"role.assign\",Compallm::Action::\"role.unassign\",Compallm::Action::\"group.list\",Compallm::Action::\"group.create\",Compallm::Action::\"group.metadata.read\",Compallm::Action::\"group.metadata.update\",Compallm::Action::\"group.delete\"],resource in Compallm::Organization::\"__ORG__\");permit(principal in Compallm::Role::\"__ORG__:__OWNER__\",action in[Compallm::Action::\"member.role.assign\",Compallm::Action::\"member.role.unassign\",Compallm::Action::\"group.role.assign\",Compallm::Action::\"group.role.unassign\"],resource in Compallm::Organization::\"__ORG__\")when{context.role.organization==Compallm::Organization::\"__ORG__\"};permit(principal in Compallm::Role::\"__ORG__:__OWNER__\",action in[Compallm::Action::\"group.member.add\",Compallm::Action::\"group.member.remove\"],resource in Compallm::Organization::\"__ORG__\")when{context.member.organization==Compallm::Organization::\"__ORG__\"};permit(principal in Compallm::Role::\"__ORG__:__OWNER__\",action in[Compallm::Action::\"model.route.add\",Compallm::Action::\"model.route.update\",Compallm::Action::\"model.route.remove\"],resource in Compallm::Organization::\"__ORG__\")when{context.providerModel.organization==Compallm::Organization::\"__ORG__\"};forbid(principal in Compallm::Role::\"__ORG__:__OWNER__\",action==Compallm::Action::\"model.use\",resource is Compallm::Model in Compallm::Organization::\"__ORG__\")unless{resource.enabled};"####,
    r####"permit(principal in Compallm::Role::"__ORG__:__ADMIN__",action in[Compallm::Action::"organization.settings.read",Compallm::Action::"organization.settings.update",Compallm::Action::"member.list",Compallm::Action::"member.invite",Compallm::Action::"member.metadata.read",Compallm::Action::"member.remove",Compallm::Action::"provider.list",Compallm::Action::"provider.create",Compallm::Action::"provider.connection.test",Compallm::Action::"provider.configuration.read",Compallm::Action::"provider.configuration.update",Compallm::Action::"provider.delete",Compallm::Action::"provider.credential.replace",Compallm::Action::"provider.model.list",Compallm::Action::"provider.model.create",Compallm::Action::"provider.model.configuration.read",Compallm::Action::"provider.model.configuration.update",Compallm::Action::"provider.model.pricing.update",Compallm::Action::"provider.model.delete",Compallm::Action::"model.list",Compallm::Action::"model.create",Compallm::Action::"model.metadata.read",Compallm::Action::"model.configuration.update",Compallm::Action::"model.delete",Compallm::Action::"model.use",Compallm::Action::"model.pricing.update",Compallm::Action::"role.list",Compallm::Action::"role.create",Compallm::Action::"role.metadata.read",Compallm::Action::"role.metadata.update",Compallm::Action::"role.policy.read",Compallm::Action::"role.policy.update",Compallm::Action::"role.delete",Compallm::Action::"group.list",Compallm::Action::"group.create",Compallm::Action::"group.metadata.read",Compallm::Action::"group.metadata.update",Compallm::Action::"group.delete"],resource in Compallm::Organization::"__ORG__");permit(principal in Compallm::Role::"__ORG__:__ADMIN__",action in[Compallm::Action::"member.role.assign",Compallm::Action::"member.role.unassign",Compallm::Action::"group.role.assign",Compallm::Action::"group.role.unassign"],resource in Compallm::Organization::"__ORG__")when{context.role.organization==Compallm::Organization::"__ORG__" && context.role !=Compallm::Role::"__ORG__:__OWNER__"};permit(principal in Compallm::Role::"__ORG__:__ADMIN__",action in[Compallm::Action::"group.member.add",Compallm::Action::"group.member.remove"],resource in Compallm::Organization::"__ORG__")when{context.member.organization==Compallm::Organization::"__ORG__"}unless{resource in Compallm::Role::"__ORG__:__OWNER__"};permit(principal in Compallm::Role::"__ORG__:__ADMIN__",action in[Compallm::Action::"model.route.add",Compallm::Action::"model.route.update",Compallm::Action::"model.route.remove"],resource in Compallm::Organization::"__ORG__")when{context.providerModel.organization==Compallm::Organization::"__ORG__"};permit(principal in Compallm::Role::"__ORG__:__ADMIN__",action in[Compallm::Action::"role.assign",Compallm::Action::"role.unassign"],resource is Compallm::Role in Compallm::Organization::"__ORG__")unless{resource==Compallm::Role::"__ORG__:__OWNER__"};forbid(principal in Compallm::Role::"__ORG__:__ADMIN__",action==Compallm::Action::"model.use",resource is Compallm::Model in Compallm::Organization::"__ORG__")unless{resource.enabled};"####,
    r####"permit(principal in Compallm::Role::\"__ORG__:__ADMIN__\",action in[Compallm::Action::\"organization.settings.read\",Compallm::Action::\"organization.settings.update\",Compallm::Action::\"member.list\",Compallm::Action::\"member.invite\",Compallm::Action::\"member.metadata.read\",Compallm::Action::\"member.remove\",Compallm::Action::\"provider.list\",Compallm::Action::\"provider.create\",Compallm::Action::\"provider.connection.test\",Compallm::Action::\"provider.configuration.read\",Compallm::Action::\"provider.configuration.update\",Compallm::Action::\"provider.delete\",Compallm::Action::\"provider.credential.replace\",Compallm::Action::\"provider.model.list\",Compallm::Action::\"provider.model.create\",Compallm::Action::\"provider.model.configuration.read\",Compallm::Action::\"provider.model.configuration.update\",Compallm::Action::\"provider.model.pricing.update\",Compallm::Action::\"provider.model.delete\",Compallm::Action::\"model.list\",Compallm::Action::\"model.create\",Compallm::Action::\"model.metadata.read\",Compallm::Action::\"model.configuration.update\",Compallm::Action::\"model.delete\",Compallm::Action::\"model.use\",Compallm::Action::\"model.pricing.update\",Compallm::Action::\"role.list\",Compallm::Action::\"role.create\",Compallm::Action::\"role.metadata.read\",Compallm::Action::\"role.metadata.update\",Compallm::Action::\"role.policy.read\",Compallm::Action::\"role.policy.update\",Compallm::Action::\"role.delete\",Compallm::Action::\"group.list\",Compallm::Action::\"group.create\",Compallm::Action::\"group.metadata.read\",Compallm::Action::\"group.metadata.update\",Compallm::Action::\"group.delete\"],resource in Compallm::Organization::\"__ORG__\");permit(principal in Compallm::Role::\"__ORG__:__ADMIN__\",action in[Compallm::Action::\"member.role.assign\",Compallm::Action::\"member.role.unassign\",Compallm::Action::\"group.role.assign\",Compallm::Action::\"group.role.unassign\"],resource in Compallm::Organization::\"__ORG__\")when{context.role.organization==Compallm::Organization::\"__ORG__\" && context.role !=Compallm::Role::\"__ORG__:__OWNER__\"};permit(principal in Compallm::Role::\"__ORG__:__ADMIN__\",action in[Compallm::Action::\"group.member.add\",Compallm::Action::\"group.member.remove\"],resource in Compallm::Organization::\"__ORG__\")when{context.member.organization==Compallm::Organization::\"__ORG__\"}unless{resource in Compallm::Role::\"__ORG__:__OWNER__\"};permit(principal in Compallm::Role::\"__ORG__:__ADMIN__\",action in[Compallm::Action::\"model.route.add\",Compallm::Action::\"model.route.update\",Compallm::Action::\"model.route.remove\"],resource in Compallm::Organization::\"__ORG__\")when{context.providerModel.organization==Compallm::Organization::\"__ORG__\"};permit(principal in Compallm::Role::\"__ORG__:__ADMIN__\",action in[Compallm::Action::\"role.assign\",Compallm::Action::\"role.unassign\"],resource is Compallm::Role in Compallm::Organization::\"__ORG__\")unless{resource==Compallm::Role::\"__ORG__:__OWNER__\"};forbid(principal in Compallm::Role::\"__ORG__:__ADMIN__\",action==Compallm::Action::\"model.use\",resource is Compallm::Model in Compallm::Organization::\"__ORG__\")unless{resource.enabled};"####,
    r####"permit(principal in Compallm::Role::"__ORG__:__MEMBER__",action==Compallm::Action::"model.list",resource==Compallm::Organization::"__ORG__");permit(principal in Compallm::Role::"__ORG__:__MEMBER__",action==Compallm::Action::"model.metadata.read",resource is Compallm::Model in Compallm::Organization::"__ORG__");permit(principal in Compallm::Role::"__ORG__:__MEMBER__",action==Compallm::Action::"model.use",resource is Compallm::Model in Compallm::Organization::"__ORG__")when{resource.enabled};"####,
    r####"permit(principal in Compallm::Role::\"__ORG__:__MEMBER__\",action==Compallm::Action::\"model.list\",resource==Compallm::Organization::\"__ORG__\");permit(principal in Compallm::Role::\"__ORG__:__MEMBER__\",action==Compallm::Action::\"model.metadata.read\",resource is Compallm::Model in Compallm::Organization::\"__ORG__\");permit(principal in Compallm::Role::\"__ORG__:__MEMBER__\",action==Compallm::Action::\"model.use\",resource is Compallm::Model in Compallm::Organization::\"__ORG__\")when{resource.enabled};"####,
    r####""c":{"v":"####,
    r####""d":""####,
    r####""u":{"s":{"i":"####,
    r####""r":{"s":{"i":"####,
    r####""p":{"c":""####,
    r####""r":["####,
    r####""g":{},"####,
    r####""v":{"i":"####,
    r####""e":"####,
    r####"permit(principal in Compallm::Role::\""####,
    r####"action in[Compallm::Action::\""####,
    r####"resource in Compallm::Organization::\""####,
];

const DOMAINS: &[&str] = &[
    "@gmail.com",
    "@googlemail.com",
    "@outlook.com",
    "@hotmail.com",
    "@yahoo.com",
    "@icloud.com",
    "@proton.me",
    ".com",
    ".org",
    ".ai",
    ".net",
    ".io",
    ".edu",
    ".gov",
    ".vn",
];

fn items() -> Vec<String> {
    let mut items = ITEMS.to_vec();
    // LZ4's match selection depends on dictionary order. This ordering was
    // measured against synthetic CBOR tokens for the built-in roles.
    items.sort_by_key(|item| (!item.contains('\\'), item.len()));
    items.extend_from_slice(DOMAINS);
    items.into_iter().map(str::to_owned).collect()
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let items = items();
        let placeholders = (0..items.len())
            .map(|index| {
                format!(
                    "('access_token'::lz4_dictionary_type, ${}::text)",
                    index + 1
                )
            })
            .collect::<Vec<_>>()
            .join(",");
        let sql = format!("INSERT INTO lz4_dictionary (type,content) VALUES {placeholders}");
        let values = items.into_iter().map(Value::from).collect::<Vec<_>>();
        manager
            .get_connection()
            .execute_raw(Statement::from_sql_and_values(
                DbBackend::Postgres,
                sql,
                values,
            ))
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let rows = manager
            .get_connection()
            .query_all_raw(Statement::from_string(
                DbBackend::Postgres,
                "SELECT content FROM lz4_dictionary WHERE type = 'access_token' ORDER BY version"
                    .to_owned(),
            ))
            .await?;
        let expected = items();
        if rows.len() != expected.len() {
            return Err(DbErr::Custom(
                "dictionary contains modified or later items".into(),
            ));
        }
        for (row, expected) in rows.iter().zip(&expected) {
            let content: String = row.try_get("", "content")?;
            if content != *expected {
                return Err(DbErr::Custom(
                    "dictionary contains modified or later items".into(),
                ));
            }
        }
        manager
            .get_connection()
            .execute_unprepared("DELETE FROM lz4_dictionary WHERE type = 'access_token'")
            .await?;
        Ok(())
    }
}
