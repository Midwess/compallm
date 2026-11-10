use std::{
    collections::{BTreeMap, HashMap, HashSet},
    sync::LazyLock,
};

use cedar_policy::{Entities, Entity, EntityUid, RestrictedExpression, SchemaFragment};

use super::{Group, Role};
use crate::authorization::{
    SystemPolicy,
    policy::{EntityType, Policy},
};

#[derive(Clone, Debug)]
pub struct OrganizationMember {
    pub organization_id: u64,
    pub user_id: u64,
    pub roles: Vec<Role>,
    pub groups: Vec<Group>,
}

#[derive(Debug, thiserror::Error)]
pub enum MembershipAssignmentError {
    #[error("assignment belongs to another organization")]
    ForeignOrganization,
    #[error("the organization owner role cannot be unassigned")]
    OwnerRole,
}

impl SystemPolicy for OrganizationMember {
    /// Declares membership enumeration, metadata, and administration use cases.
    ///
    /// - `member.read`: list memberships and inspect protected state and role assignments, not a
    ///   public user profile. It exposes no authentication or private account data;
    ///   the membership response contract must define its exact projection.
    /// - `member.invite`: invite someone into the destination Organization; invitation
    ///   verification and any initial role assignment require their own checks.
    /// - `member.remove`: remove the targeted membership, never the global user.
    ///   Self-leave and last-owner safeguards belong to the membership workflow.
    /// - `member.role.assign` / `member.role.unassign`: change a specific Member/Role
    ///   relationship using `context.role`. Both must belong to the active organization.
    ///   Delegation ceilings and concurrent removal guards
    ///   are additional checks; permission to edit a member is not permission to escalate.
    ///
    /// Membership administration is separate from unrestricted public profile reading. Role
    /// context is a typed reference, not evidence that the role exists or is assignable.
    fn policy() -> SchemaFragment {
        static DEFINITION: LazyLock<SchemaFragment> =
            LazyLock::new(|| Policy::new(EntityType::Member).into());
        DEFINITION.clone()
    }
}

impl OrganizationMember {
    pub fn assign_role(&mut self, role: Role) -> Result<(), MembershipAssignmentError> {
        if role.organization_id != self.organization_id {
            return Err(MembershipAssignmentError::ForeignOrganization);
        }
        if !self.roles.iter().any(|existing| existing.id == role.id) {
            self.roles.push(role);
        }
        Ok(())
    }

    pub fn assign_group(&mut self, group: Group) -> Result<(), MembershipAssignmentError> {
        if group.organization_id != self.organization_id
            || group
                .roles
                .iter()
                .any(|role| role.organization_id != self.organization_id)
        {
            return Err(MembershipAssignmentError::ForeignOrganization);
        }
        if !self.groups.iter().any(|existing| existing.id == group.id) {
            self.groups.push(group);
        }
        Ok(())
    }

    /// Remove a direct role assignment. Group-inherited roles are unaffected.
    pub fn unassign_role(&mut self, role_id: u64) -> Result<(), MembershipAssignmentError> {
        if self
            .roles
            .iter()
            .any(|role| role.id == role_id && role.org_owner())
        {
            return Err(MembershipAssignmentError::OwnerRole);
        }
        self.roles.retain(|role| role.id != role_id);
        Ok(())
    }

    /// Remove a group membership, without changing the group's roles.
    pub fn unassign_group(&mut self, group_id: u64) -> Result<(), MembershipAssignmentError> {
        if self
            .groups
            .iter()
            .any(|group| group.id == group_id && group.roles.iter().any(Role::org_owner))
        {
            return Err(MembershipAssignmentError::OwnerRole);
        }
        self.groups.retain(|group| group.id != group_id);
        Ok(())
    }

    pub fn effective_roles(&self) -> impl Iterator<Item = &Role> {
        self.roles
            .iter()
            .chain(self.groups.iter().flat_map(|g| g.roles.iter()))
    }
}

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
                anyhow::ensure!(previous == role, "conflicting role definition");
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
