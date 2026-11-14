use async_trait::async_trait;

use super::ConnectionError;
use crate::organization::{Group, Organization, OrganizationMember, Role};

#[derive(Debug, thiserror::Error)]
pub enum OrganizationRepositoryError {
    #[error("invalid collection query")]
    InvalidQuery,
    #[error("name already exists in this organization")]
    NameAlreadyExists,
    #[error("organization not found")]
    NotFound,
    #[error("membership assignments changed concurrently")]
    Conflict,
    #[error("database operation failed")]
    Database(#[from] anyhow::Error),
}

impl OrganizationRepositoryError {
    pub fn database(source: impl Into<anyhow::Error>) -> Self {
        Self::Database(source.into())
    }
}

impl From<ConnectionError> for OrganizationRepositoryError {
    fn from(error: ConnectionError) -> Self {
        Self::database(error)
    }
}

/// Use the current DbContext and leave transaction ownership to the caller.
#[async_trait]
pub trait OrganizationRepository: Send + Sync {
    async fn find_member_identity(
        &self,
        organization_id: u64,
        user_id: u64,
    ) -> Result<Option<OrganizationMember>, OrganizationRepositoryError>;

    async fn list_members(
        &self,
        organization_id: u64,
        actor_id: u64,
        group_id: Option<u64>,
        query: Box<dyn compallm_query::IntoQuery>,
        pagination: compallm_query::Pagination,
        ordering: Vec<compallm_query::OrderBy>,
    ) -> Result<
        compallm_query::Connection<crate::organization::group::MemberOption>,
        OrganizationRepositoryError,
    >;
    async fn group_member_ids(
        &self,
        organization_id: u64,
        group_id: u64,
    ) -> Result<Vec<u64>, OrganizationRepositoryError>;
    async fn group_member_count(
        &self,
        organization_id: u64,
        group_id: u64,
    ) -> Result<u64, OrganizationRepositoryError>;
    async fn save_group_changes(
        &self,
        group: &Group,
        changes: &crate::organization::group::GroupChanges,
    ) -> Result<Vec<u64>, OrganizationRepositoryError>;
    async fn delete_group(
        &self,
        organization_id: u64,
        group_id: u64,
    ) -> Result<(), OrganizationRepositoryError>;

    async fn find_by_id(
        &self,
        organization_id: u64,
    ) -> Result<Option<Organization>, OrganizationRepositoryError>;
    async fn update_profile(
        &self,
        organization_id: u64,
        name: Option<String>,
        avatar_path: Option<String>,
    ) -> Result<Organization, OrganizationRepositoryError>;
    fn supports_role_policy(&self, source: &str) -> bool;
    async fn update_role(
        &self,
        organization_id: u64,
        role_id: u64,
        name: Option<String>,
        description: Option<String>,
        cedar_source: Option<String>,
    ) -> Result<(), OrganizationRepositoryError>;
    /// Find a role using its organization-scoped identity.
    async fn find_role(
        &self,
        organization_id: u64,
        role_id: u64,
    ) -> Result<Option<Role>, OrganizationRepositoryError>;
    /// Load role metadata for assignment checks; the policy source is not returned.
    async fn find_roles_for_assignment(
        &self,
        organization_id: u64,
        role_ids: &[u64],
    ) -> Result<Vec<Role>, OrganizationRepositoryError>;
    /// Exclusively lock the scoped role for deletion, blocking new assignment links
    /// until the active transaction ends, and return its current record.
    async fn lock_role(
        &self,
        organization_id: u64,
        role_id: u64,
    ) -> Result<Role, OrganizationRepositoryError>;
    /// Lock groups linked to the role inside the same transaction.
    async fn lock_role_groups(
        &self,
        organization_id: u64,
        role_id: u64,
    ) -> Result<bool, OrganizationRepositoryError>;
    async fn lock_group(
        &self,
        organization_id: u64,
        group_id: u64,
    ) -> Result<(), OrganizationRepositoryError>;
    /// Find direct and group-inherited role assignees, deduplicated.
    async fn role_assignees(
        &self,
        organization_id: u64,
        role_id: u64,
    ) -> Result<Vec<u64>, OrganizationRepositoryError>;
    /// Delete the scoped role and its cascading relationships.
    async fn delete_role(
        &self,
        organization_id: u64,
        role_id: u64,
    ) -> Result<(), OrganizationRepositoryError>;
    async fn list_role_info(
        &self,
        organization_id: u64,
    ) -> Result<Vec<Role>, OrganizationRepositoryError>;
    async fn list_group_info(
        &self,
        organization_id: u64,
    ) -> Result<Vec<crate::organization::group::GroupInfo>, OrganizationRepositoryError>;
    async fn find_group_info(
        &self,
        organization_id: u64,
        group_id: u64,
    ) -> Result<Option<crate::organization::group::GroupInfo>, OrganizationRepositoryError>;
    /// Find a group and its assigned roles using its organization-scoped identity.
    async fn find_group(
        &self,
        organization_id: u64,
        group_id: u64,
    ) -> Result<Option<Group>, OrganizationRepositoryError>;
    /// Insert a group and its role links as one operation.
    async fn create_group(&self, group: Group) -> Result<Group, OrganizationRepositoryError>;
    /// Return the member and active organization, using the compound key.
    async fn find_membership(
        &self,
        organization_id: u64,
        user_id: u64,
    ) -> Result<Option<(OrganizationMember, Organization)>, OrganizationRepositoryError>;
    /// Oldest membership in an active organization, with organization ID as a tie-breaker.
    async fn find_first_membership(
        &self,
        user_id: u64,
    ) -> Result<Option<(OrganizationMember, Organization)>, OrganizationRepositoryError>;
    async fn update_name(
        &self,
        organization_id: u64,
        name: &str,
    ) -> Result<Organization, OrganizationRepositoryError>;
    async fn create(
        &self,
        organization: Organization,
    ) -> Result<Organization, OrganizationRepositoryError>;
    /// Insert initial default-role records within organization creation.
    async fn create_roles(&self, roles: Vec<Role>) -> Result<(), OrganizationRepositoryError>;
    async fn add_member(
        &self,
        member: OrganizationMember,
    ) -> Result<OrganizationMember, OrganizationRepositoryError>;
    /// Save assignment changes only if the previous membership is still current.
    async fn save_member(
        &self,
        member: &OrganizationMember,
        previous: &OrganizationMember,
    ) -> Result<(), OrganizationRepositoryError>;
}
