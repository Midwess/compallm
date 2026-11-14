use sea_orm::{ConnectionTrait, sea_query::Query};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

const ORGANIZATION_ID: i64 = 7_000_000_000_000_000_000;
const OWNER_ID: i64 = ORGANIZATION_ID + 1;
const ADMIN_ID: i64 = ORGANIZATION_ID + 2;
const MEMBER_ID: i64 = ORGANIZATION_ID + 3;
const USERS: [(&str, &str); 10] = [
    ("owner", "Morgan Hayes"),
    ("admin", "Jordan Blake"),
    ("emma.wilson", "Emma Wilson"),
    ("liam.chen", "Liam Chen"),
    ("olivia.martinez", "Olivia Martinez"),
    ("noah.patel", "Noah Patel"),
    ("ava.nguyen", "Ava Nguyen"),
    ("ethan.brooks", "Ethan Brooks"),
    ("sophia.kim", "Sophia Kim"),
    ("lucas.anderson", "Lucas Anderson"),
];

const OWNER_SOURCE: &str = r###"// Application-owned initial Owner template.
permit (
    principal in ?principal,
    action in [Compallm::Action::"organization.settings.read", Compallm::Action::"organization.settings.update", Compallm::Action::"organization.delete", Compallm::Action::"member.read", Compallm::Action::"member.invite", Compallm::Action::"member.remove", Compallm::Action::"provider.read", Compallm::Action::"provider.create", Compallm::Action::"provider.configuration.update", Compallm::Action::"provider.delete", Compallm::Action::"provider.credential.replace", Compallm::Action::"provider.model.read", Compallm::Action::"provider.model.create", Compallm::Action::"provider.model.configuration.update", Compallm::Action::"provider.model.pricing.update", Compallm::Action::"provider.model.delete", Compallm::Action::"model.read", Compallm::Action::"model.create", Compallm::Action::"model.configuration.update", Compallm::Action::"model.delete", Compallm::Action::"model.use", Compallm::Action::"model.pricing.update", Compallm::Action::"role.read", Compallm::Action::"role.create", Compallm::Action::"role.metadata.update", Compallm::Action::"role.policy.read", Compallm::Action::"role.policy.update", Compallm::Action::"role.delete", Compallm::Action::"role.assign", Compallm::Action::"role.unassign", Compallm::Action::"group.read", Compallm::Action::"group.create", Compallm::Action::"group.metadata.update", Compallm::Action::"group.delete"],
    resource in ?resource
);

permit (
    principal in ?principal,
    action in [Compallm::Action::"member.role.assign", Compallm::Action::"member.role.unassign", Compallm::Action::"group.role.assign", Compallm::Action::"group.role.unassign"],
    resource in ?resource
) when { context.role.organization == Compallm::Organization::"__ORG__" };

permit (
    principal in ?principal,
    action in [Compallm::Action::"group.member.add", Compallm::Action::"group.member.remove"],
    resource in ?resource
) when { context.member.organization == Compallm::Organization::"__ORG__" };

permit (
    principal in ?principal,
    action in [Compallm::Action::"model.route.add", Compallm::Action::"model.route.update", Compallm::Action::"model.route.remove"],
    resource in ?resource
) when { context.providerModel.organization == Compallm::Organization::"__ORG__" };

forbid(principal in ?principal, action == Compallm::Action::"model.use",
 resource is Compallm::Model in ?resource) unless { resource.enabled };
"###;
const ADMIN_SOURCE: &str = r###"// Application-owned initial Admin template.
permit (
    principal in ?principal,
    action in [Compallm::Action::"organization.settings.read", Compallm::Action::"organization.settings.update", Compallm::Action::"member.read", Compallm::Action::"member.invite", Compallm::Action::"member.remove", Compallm::Action::"provider.read", Compallm::Action::"provider.create", Compallm::Action::"provider.configuration.update", Compallm::Action::"provider.delete", Compallm::Action::"provider.credential.replace", Compallm::Action::"provider.model.read", Compallm::Action::"provider.model.create", Compallm::Action::"provider.model.configuration.update", Compallm::Action::"provider.model.pricing.update", Compallm::Action::"provider.model.delete", Compallm::Action::"model.read", Compallm::Action::"model.create", Compallm::Action::"model.configuration.update", Compallm::Action::"model.delete", Compallm::Action::"model.use", Compallm::Action::"model.pricing.update", Compallm::Action::"role.read", Compallm::Action::"role.create", Compallm::Action::"role.metadata.update", Compallm::Action::"role.policy.read", Compallm::Action::"role.policy.update", Compallm::Action::"role.delete", Compallm::Action::"group.read", Compallm::Action::"group.create", Compallm::Action::"group.metadata.update", Compallm::Action::"group.delete"],
    resource in ?resource
);

permit (
    principal in ?principal,
    action in [Compallm::Action::"member.role.assign", Compallm::Action::"member.role.unassign", Compallm::Action::"group.role.assign", Compallm::Action::"group.role.unassign"],
    resource in ?resource
) when { context.role.organization == Compallm::Organization::"__ORG__" && context.role != Compallm::Role::"__ORG__:__OWNER__" };

permit (
    principal in ?principal,
    action in [Compallm::Action::"group.member.add", Compallm::Action::"group.member.remove"],
    resource in ?resource
) when { context.member.organization == Compallm::Organization::"__ORG__" }
unless { resource in Compallm::Role::"__ORG__:__OWNER__" };

permit (
    principal in ?principal,
    action in [Compallm::Action::"model.route.add", Compallm::Action::"model.route.update", Compallm::Action::"model.route.remove"],
    resource in ?resource
) when { context.providerModel.organization == Compallm::Organization::"__ORG__" };

permit (
 principal in ?principal,
 action in [Compallm::Action::"role.assign", Compallm::Action::"role.unassign"],
 resource is Compallm::Role in ?resource
) unless { resource == Compallm::Role::"__ORG__:__OWNER__" };

forbid(principal in ?principal, action == Compallm::Action::"model.use",
 resource is Compallm::Model in ?resource) unless { resource.enabled };
"###;
const MEMBER_SOURCE: &str = r###"// Default members discover models and invoke enabled logical models only.
permit(principal in ?principal, action == Compallm::Action::"model.read",
 resource == ?resource);
permit(principal in ?principal, action == Compallm::Action::"model.use",
 resource is Compallm::Model in ?resource) when { resource.enabled };
"###;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        if !cfg!(debug_assertions) {
            return Ok(());
        }
        let db = manager.get_connection();
        db.execute(
            Query::insert()
                .into_table(Alias::new("organizations"))
                .columns([Alias::new("id"), Alias::new("name")])
                .values_panic([ORGANIZATION_ID.into(), "Compallm org".into()]),
        )
        .await?;
        for (id, name, source, owner) in [
            (OWNER_ID, "Owner", OWNER_SOURCE, true),
            (ADMIN_ID, "Admin", ADMIN_SOURCE, false),
            (MEMBER_ID, "Member", MEMBER_SOURCE, false),
        ] {
            let source = source
                .replace("__ORG__", &ORGANIZATION_ID.to_string())
                .replace("__OWNER__", &OWNER_ID.to_string());
            let source = compallm_core::organization::Role::prepare_policy_source(
                &source,
                ORGANIZATION_ID as u64,
                id as u64,
            )
            .map_err(|error| DbErr::Custom(error.to_string()))?;
            db.execute(
                Query::insert()
                    .into_table(Alias::new("organization_roles"))
                    .columns(
                        [
                            "organization_id",
                            "id",
                            "name",
                            "description",
                            "cedar_source",
                            "org_owner",
                        ]
                        .map(Alias::new),
                    )
                    .values_panic([
                        ORGANIZATION_ID.into(),
                        id.into(),
                        name.into(),
                        "".into(),
                        source.into(),
                        owner.into(),
                    ]),
            )
            .await?;
        }
        for (index, (username, name)) in USERS.into_iter().enumerate() {
            let id = ORGANIZATION_ID + 100 + index as i64;
            db.execute(
                Query::insert()
                    .into_table(Alias::new("users"))
                    .columns(
                        [
                            "id",
                            "email",
                            "display_name",
                            "username",
                            "email_verified_at",
                        ]
                        .map(Alias::new),
                    )
                    .values_panic([
                        Expr::val(id),
                        Expr::val(format!("{username}@compallm.com")),
                        Expr::val(name),
                        Expr::val(username),
                        Expr::current_timestamp(),
                    ]),
            )
            .await?;
            db.execute(
                Query::insert()
                    .into_table(Alias::new("organization_members"))
                    .columns(["organization_id", "user_id"].map(Alias::new))
                    .values_panic([ORGANIZATION_ID.into(), id.into()]),
            )
            .await?;
            let role = match index {
                0 => OWNER_ID,
                1 => ADMIN_ID,
                _ => MEMBER_ID,
            };
            db.execute(
                Query::insert()
                    .into_table(Alias::new("organization_member_roles"))
                    .columns(["organization_id", "user_id", "role_id"].map(Alias::new))
                    .values_panic([ORGANIZATION_ID.into(), id.into(), role.into()]),
            )
            .await?;
            let preferences =
                serde_json::to_value(compallm_core::preference::UserPreferences::default())
                    .map_err(|error| DbErr::Custom(error.to_string()))?;
            let fields = [
                ("navigation", "navigation"),
                ("sidebar", "sidebar"),
                ("member_display", "memberDisplay"),
                ("calendar", "calendar"),
                ("theme", "theme"),
                ("typography", "typography"),
                ("accessibility", "accessibility"),
                ("message_composition", "messageComposition"),
                ("code_appearance", "codeAppearance"),
            ];
            let mut insert = Query::insert();
            insert.into_table(Alias::new("user_preferences")).columns(
                std::iter::once(Alias::new("user_id"))
                    .chain(fields.iter().map(|(column, _)| Alias::new(*column))),
            );
            let mut values = vec![id.into()];
            values.extend(
                fields
                    .iter()
                    .map(|(_, field)| preferences[*field].clone().into()),
            );
            insert.values_panic(values);
            db.execute(&insert).await?;
        }
        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        if cfg!(debug_assertions) {
            return Err(DbErr::Custom(
                "testing workspace data may own later work and cannot be downgraded".into(),
            ));
        }
        Ok(())
    }
}
