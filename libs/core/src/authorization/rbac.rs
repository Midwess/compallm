use crate::persistent::organization::OrganizationRepository;
use crate::{
    authentication::AuthService, authorization::error::AuthorizationError, organization::Group,
    persistent::db_context,
};

use super::{
    MemberPermission, MemberPermissionError,
    condition::{CheckPermission, ForAction},
    policy::Action,
};
use crate::organization::Role;

pub struct RbacService {
    auth_service: &'static AuthService,
    organizations: &'static Box<dyn OrganizationRepository>,
}

#[allow(dead_code, reason = "RBAC assignment endpoints remain pending")]
impl RbacService {
    pub fn new(
        auth_service: &'static AuthService,
        organizations: &'static Box<dyn OrganizationRepository>,
    ) -> Self {
        Self {
            auth_service,
            organizations,
        }
    }

    pub async fn list_roles(
        &self,
        permission: &MemberPermission,
    ) -> Result<Vec<Role>, AuthorizationError> {
        Action::RoleRead.check_permission(permission)?;
        Ok(self.organizations.list_role_info().await?)
    }

    pub async fn list_groups(
        &self,
        permission: &MemberPermission,
    ) -> Result<Vec<crate::organization::group::GroupInfo>, AuthorizationError> {
        Action::GroupRead.check_permission(permission)?;
        Ok(self.organizations.list_group_info().await?)
    }

    pub async fn get_role(
        &self,
        permission: &MemberPermission,
        id: u64,
    ) -> Result<Option<Role>, AuthorizationError> {
        Action::RoleRead.check_permission(permission)?;
        Ok(self
            .organizations
            .find_roles_for_assignment(permission.organization_id(), &[id])
            .await?
            .into_iter()
            .next())
    }

    pub async fn get_group(
        &self,
        permission: &MemberPermission,
        id: u64,
    ) -> Result<Option<crate::organization::group::GroupInfo>, AuthorizationError> {
        Action::GroupRead.check_permission(permission)?;
        Ok(self.organizations.find_group_info(id).await?)
    }

    pub async fn new_role(
        &self,
        permission: &MemberPermission,
        name: &str,
        description: &str,
        cedar_script: String,
    ) -> Result<Role, AuthorizationError> {
        Action::RoleCreate.check_permission(permission)?;
        let role = Role::new(
            permission.organization_id(),
            name,
            description,
            cedar_script,
        )
        .await?;
        if !self
            .organizations
            .supports_role_policy(&cedar_policy::PolicySet::try_from(&role)?.to_string())
        {
            return Err(AuthorizationError::UnsupportedPolicy);
        }
        self.insert_role(role).await
    }

    pub async fn update_role(
        &self,
        permission: &MemberPermission,
        id: u64,
        name: Option<String>,
        description: Option<String>,
        cedar_source: Option<String>,
    ) -> Result<Role, AuthorizationError> {
        let mut cedar_source =
            cedar_source.map(|source| super::policy::compact_policy_source(&source));
        let organization_id = permission.organization_id();
        let role = self
            .organizations
            .find_roles_for_assignment(organization_id, &[id])
            .await?
            .into_iter()
            .next()
            .ok_or(crate::persistent::organization::OrganizationRepositoryError::NotFound)?;
        if role.org_owner() {
            return Err(super::MemberPermissionError::Denied.into());
        }
        if name.is_some() || description.is_some() {
            role.for_action(Action::RoleMetadataUpdate)
                .check_permission(permission)?;
        }
        if let Some(source) = cedar_source.as_mut() {
            role.for_action(Action::RolePolicyUpdate)
                .check_permission(permission)?;
            *source = Role::prepare_policy_source(source, organization_id, id)
                .map_err(crate::organization::role::RoleCreationError::InvalidPolicy)?;
            if !self.organizations.supports_role_policy(
                &super::policy::role_policy_set(source, organization_id, id)?.to_string(),
            ) {
                return Err(AuthorizationError::UnsupportedPolicy);
            }
        }
        let name = name
            .map(|value| {
                Role::normalize_name(&value)
                    .map_err(crate::organization::role::RoleCreationError::InvalidName)
            })
            .transpose()?;
        if let Some(description) = description.as_ref() {
            Role::validate_description(description)
                .map_err(crate::organization::role::RoleCreationError::InvalidDescription)?;
        }
        if name.is_some() || description.is_some() || cedar_source.is_some() {
            self.save_role_changes(organization_id, id, name, description, cedar_source)
                .await?;
        }
        self.organizations
            .find_roles_for_assignment(organization_id, &[id])
            .await?
            .into_iter()
            .next()
            .ok_or(crate::persistent::organization::OrganizationRepositoryError::NotFound.into())
    }

    pub async fn delete_role(
        &self,
        permission: &MemberPermission,
        role_id: u64,
    ) -> Result<Role, AuthorizationError> {
        let organization_id = permission.organization_id();
        let role = self
            .organizations
            .find_roles_for_assignment(organization_id, &[role_id])
            .await?
            .into_iter()
            .next()
            .ok_or(crate::persistent::organization::OrganizationRepositoryError::NotFound)?;
        role.for_action(Action::RoleDelete)
            .check_permission(permission)?;
        role.ensure_deletable()?;
        self.delete_role_record(organization_id, role_id).await
    }

    pub async fn create_group(
        &self,
        permission: &MemberPermission,
        name: &str,
        description: &str,
        roles: Vec<u64>,
    ) -> Result<Group, AuthorizationError> {
        Action::GroupCreate.check_permission(permission)?;
        let organization_id = permission.organization_id();
        if roles.len() > Group::MAX_ROLES {
            return Err(crate::organization::group::GroupCreationError::TooManyRoles.into());
        }
        if roles.iter().collect::<std::collections::HashSet<_>>().len() != roles.len() {
            return Err(crate::organization::group::GroupCreationError::DuplicateRole.into());
        }
        let assigned = self
            .organizations
            .find_roles_for_assignment(organization_id, &roles)
            .await?;
        if assigned.len() != roles.len() {
            return Err(
                crate::persistent::organization::OrganizationRepositoryError::NotFound.into(),
            );
        }
        let group = Group::new(organization_id, name, description, assigned).await?;
        for role in &group.roles {
            (&group, role)
                .for_action(Action::GroupRoleAssign)
                .and(role.for_action(Action::RoleAssign))
                .check_permission(permission)?;
        }
        self.insert_group(group).await
    }

    pub async fn update_group(
        &self,
        permission: &MemberPermission,
        id: u64,
        mut changes: crate::organization::group::GroupChanges,
    ) -> Result<Group, AuthorizationError> {
        use crate::organization::group::GroupCreationError;
        use std::collections::BTreeSet;
        let org = permission.organization_id();
        let distinct = |added: &[u64], removed: &[u64]| {
            let a: BTreeSet<_> = added.iter().copied().collect();
            let r: BTreeSet<_> = removed.iter().copied().collect();
            a.len() == added.len()
                && r.len() == removed.len()
                && a.is_disjoint(&r)
                && !a.contains(&0)
                && !r.contains(&0)
        };
        if changes.add_member_ids.len() > 100
            || changes.remove_member_ids.len() > 100
            || changes.add_role_ids.len() > 32
            || changes.remove_role_ids.len() > 32
            || !distinct(&changes.add_role_ids, &changes.remove_role_ids)
            || !distinct(&changes.add_member_ids, &changes.remove_member_ids)
        {
            return Err(GroupCreationError::InvalidChanges.into());
        }
        if let Some(name) = changes.name.as_mut() {
            *name = Group::normalize_name(name).map_err(GroupCreationError::InvalidName)?;
        }
        if let Some(description) = changes.description.as_ref() {
            Group::validate_description(description)
                .map_err(GroupCreationError::InvalidDescription)?;
        }
        let before = self.load_group(org, id).await?;
        let ids: BTreeSet<_> = changes
            .add_role_ids
            .iter()
            .chain(&changes.remove_role_ids)
            .copied()
            .collect();
        if !ids.is_empty() {
            Action::RoleRead.check_permission(permission)?;
        }
        let roles = self
            .organizations
            .find_roles_for_assignment(org, &ids.iter().copied().collect::<Vec<_>>())
            .await?;
        if roles.len() != ids.len() {
            return Err(
                crate::persistent::organization::OrganizationRepositoryError::NotFound.into(),
            );
        }
        let mut members = Vec::new();
        if !changes.add_member_ids.is_empty() || !changes.remove_member_ids.is_empty() {
            Action::MemberRead.check_permission(permission)?;
            for user in changes
                .add_member_ids
                .iter()
                .chain(&changes.remove_member_ids)
            {
                let member = self
                    .organizations
                    .find_member_identity(*user)
                    .await?
                    .ok_or(
                        crate::persistent::organization::OrganizationRepositoryError::NotFound,
                    )?;
                members.push(member);
            }
        }
        self.check_group_changes(permission, &before, &changes, &roles, &members)?;
        self.save_group_changes(permission, id, &changes, &roles, &members)
            .await
    }

    fn check_group_changes(
        &self,
        permission: &MemberPermission,
        group: &Group,
        changes: &crate::organization::group::GroupChanges,
        roles: &[Role],
        members: &[crate::organization::OrganizationMember],
    ) -> Result<(), AuthorizationError> {
        if changes.name.is_some() || changes.description.is_some() {
            group
                .for_action(Action::GroupMetadataUpdate)
                .check_permission(permission)?;
        }
        let resulting = group
            .roles
            .iter()
            .filter(|r| !changes.remove_role_ids.contains(&r.id))
            .map(|r| r.id)
            .chain(changes.add_role_ids.iter().copied())
            .collect::<std::collections::BTreeSet<_>>();
        if resulting.len() > Group::MAX_ROLES {
            return Err(crate::organization::group::GroupCreationError::TooManyRoles.into());
        }
        for role in roles {
            let (action, delegation) = if changes.add_role_ids.contains(&role.id) {
                (Action::GroupRoleAssign, Action::RoleAssign)
            } else {
                (Action::GroupRoleUnassign, Action::RoleUnassign)
            };
            (group, role)
                .for_action(action)
                .and(role.for_action(delegation))
                .check_permission(permission)?;
        }
        for member in members {
            let action = if changes.add_member_ids.contains(&member.user_id) {
                Action::GroupMemberAdd
            } else {
                Action::GroupMemberRemove
            };
            (group, member)
                .for_action(action)
                .check_permission(permission)?;
        }
        Ok(())
    }

    /// Evaluate group authority using complete relationship facts without returning them.
    #[db_context::root]
    pub async fn check_group_permission(
        &self,
        permission: &MemberPermission,
        id: u64,
        action: Action,
        role_id: Option<u64>,
        member_id: Option<u64>,
    ) -> Result<(), AuthorizationError> {
        Action::GroupRead.check_permission(permission)?;
        let org = permission.organization_id();
        let group = self
            .organizations
            .find_group(org, id)
            .await?
            .ok_or(crate::persistent::organization::OrganizationRepositoryError::NotFound)?;
        if let Some(role_id) = role_id {
            Action::RoleRead.check_permission(permission)?;
            let role = self
                .organizations
                .find_roles_for_assignment(org, &[role_id])
                .await?
                .pop()
                .ok_or(crate::persistent::organization::OrganizationRepositoryError::NotFound)?;
            let delegation = match action {
                Action::GroupRoleAssign => Action::RoleAssign,
                Action::GroupRoleUnassign => Action::RoleUnassign,
                _ => return Err(MemberPermissionError::Evaluation.into()),
            };
            (&group, &role)
                .for_action(action)
                .and((&role).for_action(delegation))
                .check_permission(permission)?;
        } else if let Some(member_id) = member_id {
            Action::MemberRead.check_permission(permission)?;
            let member = self
                .organizations
                .find_membership(org, member_id)
                .await?
                .map(|(member, _)| member)
                .ok_or(crate::persistent::organization::OrganizationRepositoryError::NotFound)?;
            (&group, &member)
                .for_action(action)
                .check_permission(permission)?;
        } else {
            (&group).for_action(action).check_permission(permission)?;
        }
        Ok(())
    }

    /// Evaluate member authority using verified organization scope and complete assignments.
    #[db_context::root]
    pub async fn check_member_permission(
        &self,
        permission: &MemberPermission,
        id: u64,
        action: Action,
        role_id: Option<u64>,
    ) -> Result<(), AuthorizationError> {
        Action::MemberRead.check_permission(permission)?;
        let org = permission.organization_id();
        let (member, _) = self
            .organizations
            .find_membership(org, id)
            .await?
            .ok_or(crate::persistent::organization::OrganizationRepositoryError::NotFound)?;
        if let Some(role_id) = role_id {
            Action::RoleRead.check_permission(permission)?;
            let role = self
                .organizations
                .find_roles_for_assignment(org, &[role_id])
                .await?
                .pop()
                .ok_or(crate::persistent::organization::OrganizationRepositoryError::NotFound)?;
            let delegation = match action {
                Action::MemberRoleAssign => Action::RoleAssign,
                Action::MemberRoleUnassign => Action::RoleUnassign,
                _ => return Err(MemberPermissionError::Evaluation.into()),
            };
            (&member, &role)
                .for_action(action)
                .and((&role).for_action(delegation))
                .check_permission(permission)?;
        } else {
            (&member).for_action(action).check_permission(permission)?;
        }
        Ok(())
    }

    pub async fn delete_group(
        &self,
        permission: &MemberPermission,
        id: u64,
    ) -> Result<(), AuthorizationError> {
        let org = permission.organization_id();
        let group = self.load_group(org, id).await?;
        group
            .for_action(Action::GroupDelete)
            .check_permission(permission)?;
        self.delete_group_record(permission, id).await
    }

    async fn assign_role(
        &self,
        permission: &MemberPermission,
        role_id: u64,
        user_id: u64,
    ) -> Result<(), AuthorizationError> {
        let organization_id = permission.organization_id();
        let (mut member, _) = self
            .organizations
            .find_membership(organization_id, user_id)
            .await?
            .ok_or(crate::persistent::organization::OrganizationRepositoryError::NotFound)?;
        let role = self
            .organizations
            .find_roles_for_assignment(organization_id, &[role_id])
            .await?
            .into_iter()
            .next()
            .ok_or(crate::persistent::organization::OrganizationRepositoryError::NotFound)?;
        (&member, &role)
            .for_action(Action::MemberRoleAssign)
            .and(role.for_action(Action::RoleAssign))
            .check_permission(permission)?;
        if member.roles.iter().any(|role| role.id == role_id) {
            return Ok(());
        }
        let previous = member.clone();
        member.assign_role(role)?;
        self.save_member_changes(&member, &previous).await
    }

    async fn unassign_role(
        &self,
        permission: &MemberPermission,
        role_id: u64,
        user_id: u64,
    ) -> Result<(), AuthorizationError> {
        let organization_id = permission.organization_id();
        let (mut member, _) = self
            .organizations
            .find_membership(organization_id, user_id)
            .await?
            .ok_or(crate::persistent::organization::OrganizationRepositoryError::NotFound)?;
        let role = self
            .organizations
            .find_roles_for_assignment(organization_id, &[role_id])
            .await?
            .into_iter()
            .next()
            .ok_or(crate::persistent::organization::OrganizationRepositoryError::NotFound)?;
        (&member, &role)
            .for_action(Action::MemberRoleUnassign)
            .and(role.for_action(Action::RoleUnassign))
            .check_permission(permission)?;
        if !member.roles.iter().any(|role| role.id == role_id) {
            return Ok(());
        }
        let previous = member.clone();
        member.unassign_role(role_id)?;
        self.save_member_changes(&member, &previous).await
    }

    async fn assign_group(
        &self,
        permission: &MemberPermission,
        group_id: u64,
        user_id: u64,
    ) -> Result<(), AuthorizationError> {
        let organization_id = permission.organization_id();
        let (mut member, _) = self
            .organizations
            .find_membership(organization_id, user_id)
            .await?
            .ok_or(crate::persistent::organization::OrganizationRepositoryError::NotFound)?;
        let group = self
            .organizations
            .find_group(organization_id, group_id)
            .await?
            .ok_or(crate::persistent::organization::OrganizationRepositoryError::NotFound)?;
        (&group, &member)
            .for_action(Action::GroupMemberAdd)
            .check_permission(permission)?;
        if member.groups.iter().any(|group| group.id == group_id) {
            return Ok(());
        }
        let previous = member.clone();
        member.assign_group(group)?;
        self.save_group_member(group_id, &member, &previous).await
    }

    async fn unassign_group(
        &self,
        permission: &MemberPermission,
        group_id: u64,
        user_id: u64,
    ) -> Result<(), AuthorizationError> {
        let organization_id = permission.organization_id();
        let (mut member, _) = self
            .organizations
            .find_membership(organization_id, user_id)
            .await?
            .ok_or(crate::persistent::organization::OrganizationRepositoryError::NotFound)?;
        let group = self
            .organizations
            .find_group(organization_id, group_id)
            .await?
            .ok_or(crate::persistent::organization::OrganizationRepositoryError::NotFound)?;
        (&group, &member)
            .for_action(Action::GroupMemberRemove)
            .check_permission(permission)?;
        if !member.groups.iter().any(|group| group.id == group_id) {
            return Ok(());
        }
        let previous = member.clone();
        member.unassign_group(group_id)?;
        self.save_member_changes(&member, &previous).await
    }

    #[db_context::root]
    async fn save_role_changes(
        &self,
        organization_id: u64,
        id: u64,
        name: Option<String>,
        description: Option<String>,
        cedar_source: Option<String>,
    ) -> Result<(), AuthorizationError> {
        crate::transaction!(db, {
            let current = self.organizations.lock_role(organization_id, id).await?;
            if current.org_owner() {
                return Err(AuthorizationError::from(
                    super::MemberPermissionError::Denied,
                ));
            }
            self.organizations
                .lock_role_groups(organization_id, id)
                .await?;
            self.organizations
                .update_role(organization_id, id, name, description, cedar_source)
                .await?;
            let users = self
                .organizations
                .role_assignees(organization_id, id)
                .await?;
            self.auth_service
                .invalidate_permissions(organization_id, &users)
                .await?;
            Ok::<(), AuthorizationError>(())
        })
    }

    #[db_context::root]
    async fn delete_role_record(
        &self,
        organization_id: u64,
        role_id: u64,
    ) -> Result<Role, AuthorizationError> {
        crate::transaction!(db, {
            let role = self
                .organizations
                .lock_role(organization_id, role_id)
                .await?;
            role.ensure_deletable()?;
            let has_groups = self
                .organizations
                .lock_role_groups(organization_id, role_id)
                .await?;
            let users = self
                .organizations
                .role_assignees(organization_id, role_id)
                .await?;
            if has_groups || !users.is_empty() {
                return Err(crate::organization::role::RoleDeletionError::Assigned.into());
            }
            self.organizations
                .delete_role(organization_id, role_id)
                .await?;
            Ok::<Role, AuthorizationError>(role)
        })
    }

    #[db_context::root]
    async fn save_group_changes(
        &self,
        permission: &MemberPermission,
        id: u64,
        changes: &crate::organization::group::GroupChanges,
        roles: &[Role],
        members: &[crate::organization::OrganizationMember],
    ) -> Result<Group, AuthorizationError> {
        let org = permission.organization_id();
        crate::transaction!(db, {
            for role in roles {
                self.organizations.lock_role(org, role.id).await?;
            }
            self.organizations.lock_group(org, id).await?;
            let group =
                self.organizations.find_group(org, id).await?.ok_or(
                    crate::persistent::organization::OrganizationRepositoryError::NotFound,
                )?;
            self.check_group_changes(permission, &group, changes, roles, members)?;
            let users = self
                .organizations
                .save_group_changes(&group, changes)
                .await?;
            self.auth_service
                .invalidate_permissions(org, &users)
                .await?;
            self.organizations.find_group(org, id).await?.ok_or(
                crate::persistent::organization::OrganizationRepositoryError::NotFound.into(),
            )
        })
    }

    #[db_context::root]
    async fn delete_group_record(
        &self,
        permission: &MemberPermission,
        id: u64,
    ) -> Result<(), AuthorizationError> {
        let org = permission.organization_id();
        crate::transaction!(db, {
            self.organizations.lock_group(org, id).await?;
            let group =
                self.organizations.find_group(org, id).await?.ok_or(
                    crate::persistent::organization::OrganizationRepositoryError::NotFound,
                )?;
            group
                .for_action(Action::GroupDelete)
                .check_permission(permission)?;
            if self.organizations.group_member_count(org, id).await? != 0 {
                return Err(crate::organization::group::GroupDeletionError::Assigned.into());
            }
            self.organizations.delete_group(org, id).await?;
            Ok(())
        })
    }

    #[db_context::root]
    async fn insert_role(&self, role: Role) -> Result<Role, AuthorizationError> {
        self.organizations.create_roles(vec![role.clone()]).await?;
        Ok(role)
    }

    #[db_context::root]
    async fn insert_group(&self, group: Group) -> Result<Group, AuthorizationError> {
        Ok(self.organizations.create_group(group).await?)
    }

    #[db_context::root]
    async fn load_group(&self, organization_id: u64, id: u64) -> Result<Group, AuthorizationError> {
        self.organizations
            .find_group(organization_id, id)
            .await?
            .ok_or(crate::persistent::organization::OrganizationRepositoryError::NotFound.into())
    }

    #[db_context::root]
    async fn save_member_changes(
        &self,
        member: &crate::organization::OrganizationMember,
        previous: &crate::organization::OrganizationMember,
    ) -> Result<(), AuthorizationError> {
        crate::transaction!(db, {
            self.organizations.save_member(member, previous).await?;
            self.auth_service
                .invalidate_permissions(member.organization_id, &[member.user_id])
                .await?;
            Ok::<(), AuthorizationError>(())
        })
    }

    #[db_context::root]
    async fn save_group_member(
        &self,
        group_id: u64,
        member: &crate::organization::OrganizationMember,
        previous: &crate::organization::OrganizationMember,
    ) -> Result<(), AuthorizationError> {
        crate::transaction!(db, {
            self.organizations
                .lock_group(member.organization_id, group_id)
                .await?;
            self.organizations.save_member(member, previous).await?;
            self.auth_service
                .invalidate_permissions(member.organization_id, &[member.user_id])
                .await?;
            Ok::<(), AuthorizationError>(())
        })
    }
}
