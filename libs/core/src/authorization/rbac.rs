use crate::persistent::organization::OrganizationRepository;
use crate::{
    auth::AuthService, authorization::error::AuthorizationError, organization::Group,
    persistent::DbContext,
};

use crate::organization::Role;

pub struct RbacService {
    db: DbContext,
    auth_service: AuthService,
    organizations: Box<dyn OrganizationRepository>,
}

impl RbacService {
    async fn new_role(&self, name: &str, cedar_script: String) -> Result<Role, AuthorizationError> {
        todo!()
    }

    async fn delete_Role(&self, id: &str) -> Result<Role, AuthorizationError> {
        // Check if is there any user currently using this role and un-assign them first
        // Delete the role
        // Inside Role entities, define the role length rules follow the aws iam system length
        todo!()
    }

    async fn create_group(
        &self,
        organization_id: u64,
        name: &str,
        roles: Vec<u64>,
    ) -> Result<Group, AuthorizationError> {
        if roles.len() > Group::MAX_ROLES {
            return Err(crate::organization::group::GroupCreationError::TooManyRoles.into());
        }
        if roles.iter().collect::<std::collections::HashSet<_>>().len() != roles.len() {
            return Err(crate::organization::group::GroupCreationError::DuplicateRole.into());
        }
        let assigned = self
            .organizations
            .find_roles(organization_id, &roles)
            .await?;
        if assigned.len() != roles.len() {
            return Err(
                crate::persistent::organization::OrganizationRepositoryError::NotFound.into(),
            );
        }
        let group = Group::new(organization_id, name, "", assigned).await?;
        Ok(self.organizations.create_group(group).await?)
    }

    async fn assign_role(
        &self,
        organization_id: u64,
        role_id: u64,
        user_id: u64,
    ) -> Result<(), AuthorizationError> {
        let (mut member, _) = self
            .organizations
            .find_membership(organization_id, user_id)
            .await?
            .ok_or(crate::persistent::organization::OrganizationRepositoryError::NotFound)?;
        if member.roles.iter().any(|role| role.id == role_id) {
            return Ok(());
        }
        let role = self
            .organizations
            .find_role(organization_id, role_id)
            .await?
            .ok_or(crate::persistent::organization::OrganizationRepositoryError::NotFound)?;
        let previous = member.clone();
        member.assign_role(role)?;
        crate::transaction!(self.db, {
            self.organizations.save_member(&member, &previous).await?;
            self.auth_service
                .invalidate_permissions(organization_id, &[user_id])
                .await?;
            Ok(())
        })
    }

    async fn unassign_role(
        &self,
        organization_id: u64,
        role_id: u64,
        user_id: u64,
    ) -> Result<(), AuthorizationError> {
        let (mut member, _) = self
            .organizations
            .find_membership(organization_id, user_id)
            .await?
            .ok_or(crate::persistent::organization::OrganizationRepositoryError::NotFound)?;
        if !member.roles.iter().any(|role| role.id == role_id) {
            return Ok(());
        }
        let previous = member.clone();
        member.unassign_role(role_id)?;
        crate::transaction!(self.db, {
            self.organizations.save_member(&member, &previous).await?;
            self.auth_service
                .invalidate_permissions(organization_id, &[user_id])
                .await?;
            Ok(())
        })
    }

    async fn assign_group(
        &self,
        organization_id: u64,
        group_id: u64,
        user_id: u64,
    ) -> Result<(), AuthorizationError> {
        let (mut member, _) = self
            .organizations
            .find_membership(organization_id, user_id)
            .await?
            .ok_or(crate::persistent::organization::OrganizationRepositoryError::NotFound)?;
        if member.groups.iter().any(|group| group.id == group_id) {
            return Ok(());
        }
        let group = self
            .organizations
            .find_group(organization_id, group_id)
            .await?
            .ok_or(crate::persistent::organization::OrganizationRepositoryError::NotFound)?;
        let previous = member.clone();
        member.assign_group(group)?;
        crate::transaction!(self.db, {
            self.organizations.save_member(&member, &previous).await?;
            self.auth_service
                .invalidate_permissions(organization_id, &[user_id])
                .await?;
            Ok(())
        })
    }

    async fn unassign_group(
        &self,
        organization_id: u64,
        group_id: u64,
        user_id: u64,
    ) -> Result<(), AuthorizationError> {
        let (mut member, _) = self
            .organizations
            .find_membership(organization_id, user_id)
            .await?
            .ok_or(crate::persistent::organization::OrganizationRepositoryError::NotFound)?;
        if !member.groups.iter().any(|group| group.id == group_id) {
            return Ok(());
        }
        let previous = member.clone();
        member.unassign_group(group_id)?;
        crate::transaction!(self.db, {
            self.organizations.save_member(&member, &previous).await?;
            self.auth_service
                .invalidate_permissions(organization_id, &[user_id])
                .await?;
            Ok(())
        })
    }
}
