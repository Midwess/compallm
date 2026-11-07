use super::{OrganizationMember, Role};
use cedar_policy::{Entities, Entity, EntityUid, RestrictedExpression};
use std::collections::{BTreeMap, HashMap, HashSet};

impl TryFrom<&OrganizationMember> for Entities {
    type Error = anyhow::Error;
    fn try_from(member: &OrganizationMember) -> Result<Self, Self::Error> {
        anyhow::ensure!(
            member.organization_id > 0
                && member.organization_id <= i64::MAX as u64
                && member.user_id > 0
                && member.user_id <= i64::MAX as u64,
            "invalid membership identity"
        );
        let org: EntityUid =
            format!("Compallm::Organization::\"{}\"", member.organization_id).parse()?;
        let user: EntityUid = format!("Compallm::User::\"{}\"", member.user_id).parse()?;
        let membership: EntityUid = format!(
            "Compallm::Member::\"{}:{}\"",
            member.organization_id, member.user_id
        )
        .parse()?;
        let mut entities = BTreeMap::new();
        let mut insert = |entity: Entity| -> anyhow::Result<()> {
            if let Some(previous) = entities.insert(entity.uid(), entity.clone()) {
                anyhow::ensure!(previous == entity, "conflicting authorization entity");
            }
            Ok(())
        };
        insert(Entity::new_no_attrs(org.clone(), HashSet::new()))?;
        insert(Entity::new(
            membership,
            HashMap::from([
                (
                    "organization".into(),
                    RestrictedExpression::new_entity_uid(org.clone()),
                ),
                (
                    "user".into(),
                    RestrictedExpression::new_entity_uid(user.clone()),
                ),
            ]),
            HashSet::from([org.clone()]),
        )?)?;
        let mut role_records: BTreeMap<(u64, u64), &Role> = BTreeMap::new();
        for role in member.effective_roles() {
            anyhow::ensure!(
                role.organization_id == member.organization_id,
                "foreign membership role"
            );
            if let Some(previous) = role_records.insert((role.organization_id, role.id), role) {
                anyhow::ensure!(previous == role, "conflicting role source/revision");
            }
            insert(Entity::try_from(role)?)?;
        }
        let mut parents = HashSet::from([org]);
        for role in &member.roles {
            parents.insert(Entity::try_from(role)?.uid());
        }
        for group in &member.groups {
            anyhow::ensure!(
                group.organization_id == member.organization_id,
                "foreign membership group"
            );
            let entity = Entity::try_from(group)?;
            parents.insert(entity.uid());
            insert(entity)?;
        }
        insert(Entity::new_no_attrs(user, parents))?;
        Ok(Entities::from_entities(
            entities.into_values(),
            Some(crate::authorization::all_policies()),
        )?)
    }
}
