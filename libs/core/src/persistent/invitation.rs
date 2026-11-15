use crate::{authentication::AuthError, organization::invitation::Invitation};
use async_trait::async_trait;

#[async_trait]
pub trait InvitationRepository: Send + Sync {
    async fn find(
        &self,
        organization_id: u64,
        id: u64,
        lock: bool,
    ) -> Result<Option<Invitation>, AuthError>;
    async fn lock_organization(&self, organization_id: u64) -> Result<(), AuthError>;
    async fn has_pending(
        &self,
        organization_id: u64,
        email: &str,
        now: u64,
    ) -> Result<bool, AuthError>;
    async fn create(&self, invitation: Invitation) -> Result<Invitation, AuthError>;
    async fn revoke(
        &self,
        organization_id: u64,
        id: u64,
        now: u64,
    ) -> Result<Invitation, AuthError>;
    async fn accept(
        &self,
        organization_id: u64,
        id: u64,
        user_id: u64,
        now: u64,
    ) -> Result<(), AuthError>;
    async fn membership_exists(
        &self,
        organization_id: u64,
        user_id: u64,
    ) -> Result<bool, AuthError>;
    async fn member_role(&self, organization_id: u64) -> Result<u64, AuthError>;
    async fn list(
        &self,
        query: Box<dyn compallm_query::IntoQuery>,
        pagination: compallm_query::Pagination,
        ordering: Vec<compallm_query::OrderBy>,
        scope: &str,
    ) -> Result<compallm_query::Connection<Invitation>, AuthError>;
}
