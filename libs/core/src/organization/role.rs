use crate::authorization::SystemPolicy;
use cedar_policy::{Entity, EntityUid, PolicySet, RestrictedExpression, SchemaFragment};
use hero::distributed_id::{generator::DistributedIdError, try_gen_id};
use std::{
    collections::{HashMap, HashSet},
    sync::LazyLock,
};

/// One organization-owned role. Display names never establish authority.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Role {
    pub id: u64,
    pub organization_id: u64,
    pub name: String,
    pub description: String,
    pub cedar_source: String,
}

#[derive(Debug, thiserror::Error)]
pub enum RoleCreationError {
    #[error("role name must contain 1 to 64 characters")]
    InvalidName(#[source] validify::ValidationErrors),
    #[error("ID generation failed")]
    IdGeneration(#[from] DistributedIdError),
}

impl Role {
    /// Maximum role name length, in Unicode characters, after trimming.
    pub const MAX_NAME_LENGTH: usize = 64;

    /// Normalize and validate a role name before a management write.
    pub fn normalize_name(name: &str) -> Result<String, validify::ValidationErrors> {
        use validify::Validify;
        let mut name = RoleName { value: name.into() };
        name.validify()?;
        Ok(name.value)
    }

    const OWNER_POLICY: &str = r###"// Application-owned initial Owner policy; organization/role IDs are trusted numeric values.
permit (
    principal in Compallm::Role::"__ORG__:__OWNER__",
    action in [Compallm::Action::"organization.settings.read", Compallm::Action::"organization.settings.update", Compallm::Action::"organization.delete", Compallm::Action::"member.list", Compallm::Action::"member.invite", Compallm::Action::"member.metadata.read", Compallm::Action::"member.remove", Compallm::Action::"provider.list", Compallm::Action::"provider.create", Compallm::Action::"provider.connection.test", Compallm::Action::"provider.configuration.read", Compallm::Action::"provider.configuration.update", Compallm::Action::"provider.delete", Compallm::Action::"provider.credential.replace", Compallm::Action::"provider.model.list", Compallm::Action::"provider.model.create", Compallm::Action::"provider.model.configuration.read", Compallm::Action::"provider.model.configuration.update", Compallm::Action::"provider.model.pricing.update", Compallm::Action::"provider.model.delete", Compallm::Action::"model.list", Compallm::Action::"model.create", Compallm::Action::"model.metadata.read", Compallm::Action::"model.configuration.update", Compallm::Action::"model.delete", Compallm::Action::"model.use", Compallm::Action::"model.pricing.update", Compallm::Action::"role.list", Compallm::Action::"role.create", Compallm::Action::"role.metadata.read", Compallm::Action::"role.metadata.update", Compallm::Action::"role.policy.read", Compallm::Action::"role.policy.update", Compallm::Action::"role.delete", Compallm::Action::"role.assign", Compallm::Action::"role.unassign", Compallm::Action::"group.list", Compallm::Action::"group.create", Compallm::Action::"group.metadata.read", Compallm::Action::"group.metadata.update", Compallm::Action::"group.delete"],
    resource in Compallm::Organization::"__ORG__"
);

permit (
    principal in Compallm::Role::"__ORG__:__OWNER__",
    action in [Compallm::Action::"member.role.assign", Compallm::Action::"member.role.unassign", Compallm::Action::"group.role.assign", Compallm::Action::"group.role.unassign"],
    resource in Compallm::Organization::"__ORG__"
) when { context.role.organization == Compallm::Organization::"__ORG__" };

permit (
    principal in Compallm::Role::"__ORG__:__OWNER__",
    action in [Compallm::Action::"group.member.add", Compallm::Action::"group.member.remove"],
    resource in Compallm::Organization::"__ORG__"
) when { context.member.organization == Compallm::Organization::"__ORG__" };

permit (
    principal in Compallm::Role::"__ORG__:__OWNER__",
    action in [Compallm::Action::"model.route.add", Compallm::Action::"model.route.update", Compallm::Action::"model.route.remove"],
    resource in Compallm::Organization::"__ORG__"
) when { context.providerModel.organization == Compallm::Organization::"__ORG__" };

forbid(principal in Compallm::Role::"__ORG__:__OWNER__", action == Compallm::Action::"model.use",
 resource is Compallm::Model in Compallm::Organization::"__ORG__") unless { resource.enabled };
"###;
    const ADMIN_POLICY: &str = r###"// Application-owned initial Admin policy; organization/role IDs are trusted numeric values.
permit (
    principal in Compallm::Role::"__ORG__:__ADMIN__",
    action in [Compallm::Action::"organization.settings.read", Compallm::Action::"organization.settings.update", Compallm::Action::"member.list", Compallm::Action::"member.invite", Compallm::Action::"member.metadata.read", Compallm::Action::"member.remove", Compallm::Action::"provider.list", Compallm::Action::"provider.create", Compallm::Action::"provider.connection.test", Compallm::Action::"provider.configuration.read", Compallm::Action::"provider.configuration.update", Compallm::Action::"provider.delete", Compallm::Action::"provider.credential.replace", Compallm::Action::"provider.model.list", Compallm::Action::"provider.model.create", Compallm::Action::"provider.model.configuration.read", Compallm::Action::"provider.model.configuration.update", Compallm::Action::"provider.model.pricing.update", Compallm::Action::"provider.model.delete", Compallm::Action::"model.list", Compallm::Action::"model.create", Compallm::Action::"model.metadata.read", Compallm::Action::"model.configuration.update", Compallm::Action::"model.delete", Compallm::Action::"model.use", Compallm::Action::"model.pricing.update", Compallm::Action::"role.list", Compallm::Action::"role.create", Compallm::Action::"role.metadata.read", Compallm::Action::"role.metadata.update", Compallm::Action::"role.policy.read", Compallm::Action::"role.policy.update", Compallm::Action::"role.delete", Compallm::Action::"group.list", Compallm::Action::"group.create", Compallm::Action::"group.metadata.read", Compallm::Action::"group.metadata.update", Compallm::Action::"group.delete"],
    resource in Compallm::Organization::"__ORG__"
);

permit (
    principal in Compallm::Role::"__ORG__:__ADMIN__",
    action in [Compallm::Action::"member.role.assign", Compallm::Action::"member.role.unassign", Compallm::Action::"group.role.assign", Compallm::Action::"group.role.unassign"],
    resource in Compallm::Organization::"__ORG__"
) when { context.role.organization == Compallm::Organization::"__ORG__" && context.role != Compallm::Role::"__ORG__:__OWNER__" };

permit (
    principal in Compallm::Role::"__ORG__:__ADMIN__",
    action in [Compallm::Action::"group.member.add", Compallm::Action::"group.member.remove"],
    resource in Compallm::Organization::"__ORG__"
) when { context.member.organization == Compallm::Organization::"__ORG__" }
unless { resource in Compallm::Role::"__ORG__:__OWNER__" };

permit (
    principal in Compallm::Role::"__ORG__:__ADMIN__",
    action in [Compallm::Action::"model.route.add", Compallm::Action::"model.route.update", Compallm::Action::"model.route.remove"],
    resource in Compallm::Organization::"__ORG__"
) when { context.providerModel.organization == Compallm::Organization::"__ORG__" };

permit (
 principal in Compallm::Role::"__ORG__:__ADMIN__",
 action in [Compallm::Action::"role.assign", Compallm::Action::"role.unassign"],
 resource is Compallm::Role in Compallm::Organization::"__ORG__"
) unless { resource == Compallm::Role::"__ORG__:__OWNER__" };

forbid(principal in Compallm::Role::"__ORG__:__ADMIN__", action == Compallm::Action::"model.use",
 resource is Compallm::Model in Compallm::Organization::"__ORG__") unless { resource.enabled };
"###;
    const MEMBER_POLICY: &str = r###"// Default members discover models and invoke enabled logical models only.
permit(principal in Compallm::Role::"__ORG__:__MEMBER__", action == Compallm::Action::"model.list",
 resource == Compallm::Organization::"__ORG__");
permit(principal in Compallm::Role::"__ORG__:__MEMBER__", action == Compallm::Action::"model.metadata.read",
 resource is Compallm::Model in Compallm::Organization::"__ORG__");
permit(principal in Compallm::Role::"__ORG__:__MEMBER__", action == Compallm::Action::"model.use",
 resource is Compallm::Model in Compallm::Organization::"__ORG__") when { resource.enabled };
"###;

    /// Initial role records rendered with application-generated IDs, never reserved identities.
    /// Labels do not establish authority; each stored Cedar source references the generated IDs.
    pub async fn defaults(organization_id: u64) -> Result<Vec<Self>, RoleCreationError> {
        let mut roles = vec![
            Self::new(organization_id, "Owner", "", "").await?,
            Self::new(organization_id, "Admin", "", "").await?,
            Self::new(organization_id, "Member", "", "").await?,
        ];
        let [owner_id, admin_id, member_id] = [roles[0].id, roles[1].id, roles[2].id];
        for (role, source) in
            roles
                .iter_mut()
                .zip([Self::OWNER_POLICY, Self::ADMIN_POLICY, Self::MEMBER_POLICY])
        {
            role.cedar_source = source
                .replace("__ORG__", &organization_id.to_string())
                .replace("__OWNER__", &owner_id.to_string())
                .replace("__ADMIN__", &admin_id.to_string())
                .replace("__MEMBER__", &member_id.to_string());
        }
        Ok(roles)
    }

    pub async fn new(
        organization_id: u64,
        name: impl Into<String>,
        description: impl Into<String>,
        cedar_source: impl Into<String>,
    ) -> Result<Self, RoleCreationError> {
        let name = Self::normalize_name(&name.into()).map_err(RoleCreationError::InvalidName)?;
        Ok(Self {
            id: try_gen_id().await?,
            organization_id,
            name,
            description: description.into(),
            cedar_source: cedar_source.into(),
        })
    }
}

#[derive(validify::Validify)]
struct RoleName {
    #[modify(trim)]
    #[validate(length(min = 1, max = 64))]
    value: String,
}

impl TryFrom<&Role> for Entity {
    type Error = anyhow::Error;
    fn try_from(role: &Role) -> Result<Self, Self::Error> {
        let uid: EntityUid =
            format!("Compallm::Role::\"{}:{}\"", role.organization_id, role.id).parse()?;
        let org: EntityUid =
            format!("Compallm::Organization::\"{}\"", role.organization_id).parse()?;
        Ok(Entity::new(
            uid,
            HashMap::from([(
                "organization".into(),
                RestrictedExpression::new_entity_uid(org.clone()),
            )]),
            HashSet::from([org]),
        )?)
    }
}

impl TryFrom<&Role> for PolicySet {
    type Error = anyhow::Error;
    fn try_from(role: &Role) -> Result<Self, Self::Error> {
        let parsed: PolicySet = role.cedar_source.parse()?;
        anyhow::ensure!(
            parsed.templates().next().is_none(),
            "unlinked role templates are unsupported"
        );
        use sha2::{Digest, Sha256};
        let mut result = PolicySet::new();
        for policy in parsed.policies() {
            let digest = Sha256::digest(serde_json::to_vec(&policy.to_json()?)?);
            let id = format!(
                "role:{}:{}:statement:{digest:x}",
                role.organization_id, role.id
            )
            .parse()
            .expect("generated policy ID");
            if result.policy(&id).is_none() {
                result.add(policy.new_id(id))?;
            }
        }
        Ok(result)
    }
}

impl SystemPolicy for Role {
    /// Declares protected role metadata, policy source, and delegation operations.
    /// `role.list` enumerates roles and `role.create` creates a custom role in an
    /// Organization. `role.metadata.read/update` inspect/change safe labels;
    /// `role.policy.read/update` inspect/publish canonical Cedar separately.
    /// `role.delete` removes a role, subject to the management workflow safeguards.
    /// `role.assign/unassign` authorize delegating this Role, in addition to checks
    /// on the target membership/group. All changes require organization-scope
    /// guards. These declarations do not implement publication or delegation checks;
    /// role management and its authorization flow remain separate work.
    fn policy() -> SchemaFragment {
        static VALUE: LazyLock<SchemaFragment> = LazyLock::new(|| {
            SchemaFragment::from_cedarschema_str(
                r#"
namespace Compallm {
 entity Role in [Organization] { organization: Organization };
 action "role.list" appliesTo { principal: User, resource: Organization };
 action "role.create" appliesTo { principal: User, resource: Organization };
 action "role.metadata.read" appliesTo { principal: User, resource: Role };
 action "role.metadata.update" appliesTo { principal: User, resource: Role };
 action "role.policy.read" appliesTo { principal: User, resource: Role };
 action "role.policy.update" appliesTo { principal: User, resource: Role };
 action "role.delete" appliesTo { principal: User, resource: Role };
 action "role.assign" appliesTo { principal: User, resource: Role };
 action "role.unassign" appliesTo { principal: User, resource: Role };
}
"#,
            )
            .expect("Role static schema")
            .0
        });
        VALUE.clone()
    }
}
