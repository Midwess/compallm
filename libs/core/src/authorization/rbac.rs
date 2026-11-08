use crate::{
    auth::AuthService, authorization::error::AuthorizationError, organization::Group,
    persistent::DbContext,
};
use rust_decimal::Error;

use crate::organization::Role;

pub struct RbacService {
    db: DbContext,
    auth_service: AuthService,
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

    async fn create_group(&self, name: &str, roles: Vec<u64>) -> Result<Group, AuthorizationError> {
        // Call the group create entities, applied some validation to entities group
        // Insert group to database
        todo!()
    }

    async fn assign_role(&self, role_id: u64, user_id: u64) -> Result<(), AuthorizationError> {
        // Check if role already assigned
        // Invalidate the user session so they will append a new role
        todo!()
    }

    async fn unassign_role(&self, role_id: u64, user_id: u64) -> Result<(), AuthorizationError> {
        todo!()
    }

    async fn assign_group(&self, group_id: u64, user_id: u64) -> Result<(), AuthorizationError> {
        todo!()
    }
}
