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

pub struct RbacService {
    auth_service: &'static AuthService,
    organizations: &'static Box<dyn OrganizationRepository>,
}

#[expect(dead_code, reason = "RBAC administration endpoints remain pending")]
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

    fn root_write_context() -> Result<DbContext, AuthorizationError> {
        if DbContext::current()?.postgres().is_transaction() {
            return Err(
                crate::persistent::ConnectionError::database(anyhow::anyhow!(
                    "RBAC writes cannot replace an active transaction"
                ))
                .into(),
            );
        }
        Ok(DbContext::root()?)
    }

    pub async fn list_roles(
        &self,
        permission: &MemberPermission,
    ) -> Result<Vec<Role>, AuthorizationError> {
        Action::RoleRead.check_permission(permission)?;
        Ok(self
            .organizations
            .list_role_info(permission.organization_id())
            .await?)
    }

    pub async fn list_groups(
        &self,
        permission: &MemberPermission,
    ) -> Result<Vec<crate::organization::group::GroupInfo>, AuthorizationError> {
        Action::GroupRead.check_permission(permission)?;
        Ok(self
            .organizations
            .list_group_info(permission.organization_id())
            .await?)
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
        Ok(self
            .organizations
            .find_group_info(permission.organization_id(), id)
            .await?)
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
        let root = Self::root_write_context()?;
        root.independent_scope(async {
            self.organizations.create_roles(vec![role.clone()]).await?;
            Ok::<Role, AuthorizationError>(role)
        })
        .await?
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
            let root = Self::root_write_context()?;
            root.independent_scope(async {
                crate::transaction!(root, {
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
            })
            .await??;
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
        let root = Self::root_write_context()?;
        root.independent_scope(async {
            crate::transaction!(root, {
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
        })
        .await?
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
        let root = Self::root_write_context()?;
        root.independent_scope(async {
            Ok::<Group, AuthorizationError>(self.organizations.create_group(group).await?)
        })
        .await?
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
        let root = Self::root_write_context()?;
        root.independent_scope(async {
            crate::transaction!(root, {
                // Assignment foreign keys coordinate with the role-deletion lock.
                self.organizations.save_member(&member, &previous).await?;
                self.auth_service
                    .invalidate_permissions(organization_id, &[user_id])
                    .await?;
                Ok::<(), AuthorizationError>(())
            })
        })
        .await?
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
        let root = Self::root_write_context()?;
        root.independent_scope(async {
            crate::transaction!(root, {
                self.organizations.save_member(&member, &previous).await?;
                self.auth_service
                    .invalidate_permissions(organization_id, &[user_id])
                    .await?;
                Ok::<(), AuthorizationError>(())
            })
        })
        .await?
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
        let root = Self::root_write_context()?;
        root.independent_scope(async {
            crate::transaction!(root, {
                self.organizations
                    .lock_group(organization_id, group_id)
                    .await?;
                self.organizations.save_member(&member, &previous).await?;
                self.auth_service
                    .invalidate_permissions(organization_id, &[user_id])
                    .await?;
                Ok::<(), AuthorizationError>(())
            })
        })
        .await?
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
        let root = Self::root_write_context()?;
        root.independent_scope(async {
            crate::transaction!(root, {
                self.organizations.save_member(&member, &previous).await?;
                self.auth_service
                    .invalidate_permissions(organization_id, &[user_id])
                    .await?;
                Ok::<(), AuthorizationError>(())
            })
        })
        .await?
    }
}
