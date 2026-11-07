use crate::authorization::SystemPolicy;
use cedar_policy::{Entity, EntityUid, PolicySet, RestrictedExpression, SchemaFragment};
use std::{
    collections::{HashMap, HashSet},
    sync::LazyLock,
};

/// One organization-owned role. Display names never establish built-in authority.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Role {
    pub id: u64,
    pub organization_id: u64,
    pub name: String,
    pub description: String,
    pub cedar_source: String,
    pub revision: u64,
}

impl Role {
    pub const OWNER_ID: u64 = 1;
    pub const ADMIN_ID: u64 = 2;
    pub const MEMBER_ID: u64 = 3;
    pub const OWNER: &str = "owner";
    pub const ADMIN: &str = "admin";
    pub const MEMBER: &str = "member";
    pub const CUSTOM: &str = "custom";

    /// Protected defaults have organization-local reserved IDs. No editable name is
    /// inspected to determine authority. Owner grants only explicit organization actions.
    pub fn defaults(organization_id: u64) -> Vec<Self> {
        [
            (Self::OWNER_ID, "Owner", include_str!("owner.cedar")),
            (Self::ADMIN_ID, "Admin", include_str!("admin.cedar")),
            (Self::MEMBER_ID, "Member", include_str!("member.cedar")),
        ]
        .into_iter()
        .map(|(id, name, source)| Self {
            id,
            organization_id,
            name: name.into(),
            description: String::new(),
            cedar_source: source.replace("__ORG__", &organization_id.to_string()),
            revision: 1,
        })
        .collect()
    }
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
    /// `role.delete` removes a custom role; protected built-ins cannot be edited.
    /// `role.assign/unassign` authorize delegating this Role, in addition to checks
    /// on the target membership/group. All changes require organization/revision
    /// guards; source publication is bounded by the default Owner/Admin action ceilings.
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
