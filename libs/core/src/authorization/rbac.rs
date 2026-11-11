use crate::persistent::organization::OrganizationRepository;
use crate::{
    authentication::AuthService, authorization::error::AuthorizationError, organization::Group,
    persistent::DbContext,
};

use super::{
    MemberPermission,
    condition::{CheckPermission, ForAction},
    policy::Action,
};
use crate::organization::Role;

#[expect(dead_code, reason = "RBAC administration endpoints remain pending")]
pub struct RbacService {
    auth_service: &'static AuthService,
    organizations: &'static Box<dyn OrganizationRepository>,
}

#[expect(dead_code, reason = "RBAC administration endpoints remain pending")]
impl RbacService {
    pub fn new(auth_service: &'static AuthService, organizations: &'static Box<dyn OrganizationRepository>) -> Self {
        Self { auth_service, organizations }
    }

    async fn new_role(
        &self,
        permission: &MemberPermission,
        name: &str,
        cedar_script: String,
    ) -> Result<Role, AuthorizationError> {
        Action::RoleCreate.check_permission(permission)?;
        let role = Role::new(permission.organization_id(), name, "", cedar_script).await?;
        self.organizations.create_roles(vec![role.clone()]).await?;
        Ok(role)
    }

    async fn delete_role(
        &self,
        permission: &MemberPermission,
        role_id: u64,
    ) -> Result<Role, AuthorizationError> {
        let organization_id = permission.organization_id();
        let role = self
            .organizations
            .find_role(organization_id, role_id)
            .await?
            .ok_or(crate::persistent::organization::OrganizationRepositoryError::NotFound)?;
        role.for_action(Action::RoleDelete)
            .check_permission(permission)?;
        role.ensure_deletable()?;
        crate::transaction!(DbContext::current()?, {
            let role = self
                .organizations
                .lock_role(organization_id, role_id)
                .await?;
            role.ensure_deletable()?;
            self.organizations
                .lock_role_groups(organization_id, role_id)
                .await?;
            let users = self
                .organizations
                .role_assignees(organization_id, role_id)
                .await?;
            self.organizations
                .delete_role(organization_id, role_id)
                .await?;
            self.auth_service
                .invalidate_permissions(organization_id, &users)
                .await?;
            Ok(role)
        })
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
        Ok(self.organizations.create_group(group).await?)
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
            .find_role(organization_id, role_id)
            .await?
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
        crate::transaction!(DbContext::current()?, {
            // Assignment foreign keys coordinate with the role-deletion lock.
            self.organizations.save_member(&member, &previous).await?;
            self.auth_service
                .invalidate_permissions(organization_id, &[user_id])
                .await?;
            Ok(())
        })
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
            .find_role(organization_id, role_id)
            .await?
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
        crate::transaction!(DbContext::current()?, {
            self.organizations.save_member(&member, &previous).await?;
            self.auth_service
                .invalidate_permissions(organization_id, &[user_id])
                .await?;
            Ok(())
        })
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
        crate::transaction!(DbContext::current()?, {
            self.organizations
                .lock_group(organization_id, group_id)
                .await?;
            self.organizations.save_member(&member, &previous).await?;
            self.auth_service
                .invalidate_permissions(organization_id, &[user_id])
                .await?;
            Ok(())
        })
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
        crate::transaction!(DbContext::current()?, {
            self.organizations.save_member(&member, &previous).await?;
            self.auth_service
                .invalidate_permissions(organization_id, &[user_id])
                .await?;
            Ok(())
        })
    }
}
