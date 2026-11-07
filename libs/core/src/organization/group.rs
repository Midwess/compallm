use super::Role;
use crate::authorization::SystemPolicy;
use cedar_policy::{Entity, EntityUid, RestrictedExpression, SchemaFragment};
use std::{
    collections::{HashMap, HashSet},
    sync::LazyLock,
};

/// Group-assigned roles retain their provenance; groups do not contain groups.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Group {
    pub id: u64,
    pub organization_id: u64,
    pub name: String,
    pub description: String,
    pub revision: u64,
    pub roles: Vec<Role>,
}

impl TryFrom<&Group> for Entity {
    type Error = anyhow::Error;
    fn try_from(group: &Group) -> Result<Self, Self::Error> {
        let uid: EntityUid = format!(
            "Compallm::Group::\"{}:{}\"",
            group.organization_id, group.id
        )
        .parse()?;
        let org: EntityUid =
            format!("Compallm::Organization::\"{}\"", group.organization_id).parse()?;
        let mut parents = HashSet::from([org.clone()]);
        for role in &group.roles {
            anyhow::ensure!(
                role.organization_id == group.organization_id,
                "foreign group role"
            );
            parents.insert(Entity::try_from(role)?.uid());
        }
        Ok(Entity::new(
            uid,
            HashMap::from([(
                "organization".into(),
                RestrictedExpression::new_entity_uid(org),
            )]),
            parents,
        )?)
    }
}

impl SystemPolicy for Group {
    /// `group.list/create` target the Organization. `group.metadata.read/update`
    /// protect group configuration; `group.delete` removes that group's assignments.
    /// `group.member.add/remove` identify the Member in context.member, without
    /// deleting a user; `group.role.assign/unassign` identify context.role and also
    /// require authority to delegate that Role. Both ends must share the actual
    /// organization; name edits cannot grant roles. Runtime writes invalidate sessions.
    fn policy() -> SchemaFragment {
        static VALUE: LazyLock<SchemaFragment> = LazyLock::new(|| {
            SchemaFragment::from_cedarschema_str(r#"
namespace Compallm {
 entity Group in [Role, Organization] { organization: Organization };
 action "group.list" appliesTo { principal: User, resource: Organization };
 action "group.create" appliesTo { principal: User, resource: Organization };
 action "group.metadata.read" appliesTo { principal: User, resource: Group };
 action "group.metadata.update" appliesTo { principal: User, resource: Group };
 action "group.member.add" appliesTo { principal: User, resource: Group, context: { member: Member } };
 action "group.member.remove" appliesTo { principal: User, resource: Group, context: { member: Member } };
 action "group.role.assign" appliesTo { principal: User, resource: Group, context: { role: Role } };
 action "group.role.unassign" appliesTo { principal: User, resource: Group, context: { role: Role } };
 action "group.delete" appliesTo { principal: User, resource: Group };
}
"#).expect("Group static schema").0
        });
        VALUE.clone()
    }
}
